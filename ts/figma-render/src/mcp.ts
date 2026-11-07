import { spawn } from "node:child_process";
import { createHash, randomBytes } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { homedir } from "node:os";
import { dirname, join } from "node:path";
import { version } from "../package.json";

const MCP = "https://mcp.figma.com/mcp";
const ISSUER = "https://api.figma.com";
const AUTHORIZE = "https://www.figma.com/oauth/mcp";
const TOKEN = "https://api.figma.com/v1/oauth/token";
const REGISTER = "https://api.figma.com/v1/oauth/mcp/register";
const CLIENT_NAME = "Claude Code"; // Figma's dynamic registration admits allowlisted client names only
const REDIRECT = "http://127.0.0.1:47613/callback";

interface Client {
  id: string;
  secret: string;
}
interface Token {
  access: string;
  refresh: string;
  expiresAt: number;
}
interface State {
  client: Client;
  token?: Token;
}

const STATE_PATH = join(
  process.env["XDG_STATE_HOME"]?.startsWith("/") ? process.env["XDG_STATE_HOME"] : join(homedir(), ".local/state"),
  "evinvest-figma-render/oauth.json",
);

function save(state: State): void {
  mkdirSync(dirname(STATE_PATH), { recursive: true });
  writeFileSync(STATE_PATH, JSON.stringify(state, null, 2), { mode: 0o600 });
}

function load(): State | null {
  if (!existsSync(STATE_PATH)) return null;
  const state = JSON.parse(readFileSync(STATE_PATH, "utf8")) as State;
  if (typeof state.client?.id !== "string" || typeof state.client.secret !== "string")
    throw new Error(`${STATE_PATH}: no client id/secret; delete it to register again`);
  return state;
}

async function register(): Promise<Client> {
  const res = await fetch(REGISTER, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      client_name: CLIENT_NAME,
      redirect_uris: [REDIRECT],
      grant_types: ["authorization_code", "refresh_token"],
      response_types: ["code"],
      token_endpoint_auth_method: "client_secret_post",
    }),
  });
  const body = await res.text();
  if (!res.ok) throw new Error(`Figma OAuth registration: ${res.status} ${body}`);
  const { client_id, client_secret } = JSON.parse(body) as { client_id?: unknown; client_secret?: unknown };
  if (typeof client_id !== "string" || typeof client_secret !== "string") throw new Error(`Figma OAuth registration: ${body}`);
  return { id: client_id, secret: client_secret };
}

/** The token endpoint's answer; null when it refuses the grant (400/401). */
async function grant(client: Client, params: Record<string, string>, previous?: Token): Promise<Token | null> {
  const res = await fetch(TOKEN, {
    method: "POST",
    headers: { "content-type": "application/x-www-form-urlencoded" },
    body: new URLSearchParams({ ...params, client_id: client.id, client_secret: client.secret, resource: MCP }),
  });
  const body = await res.text();
  if (res.status === 400 || res.status === 401) {
    console.error(`Figma refused the ${params["grant_type"]} grant: ${body}`);
    return null;
  }
  if (!res.ok) throw new Error(`Figma OAuth token: ${res.status} ${body}`);
  const t = JSON.parse(body) as { access_token?: unknown; refresh_token?: unknown; expires_in?: unknown };
  const refresh = t.refresh_token ?? previous?.refresh; // a refresh grant may keep the old refresh token
  if (typeof t.access_token !== "string" || typeof refresh !== "string" || typeof t.expires_in !== "number")
    throw new Error(`Figma OAuth token: ${body}`);
  return { access: t.access_token, refresh, expiresAt: Date.now() + t.expires_in * 1000 };
}

const base64url = (b: Buffer) => b.toString("base64url");

async function login(client: Client): Promise<Token> {
  const verifier = base64url(randomBytes(32));
  const state = base64url(randomBytes(16));
  const url = new URL(AUTHORIZE);
  for (const [k, v] of Object.entries({
    response_type: "code",
    client_id: client.id,
    redirect_uri: REDIRECT,
    scope: "mcp:connect",
    state,
    code_challenge: base64url(createHash("sha256").update(verifier).digest()),
    code_challenge_method: "S256",
    resource: MCP,
  }))
    url.searchParams.set(k, v);

  const callback = new URL(REDIRECT);
  const code = await new Promise<string>((resolve, reject) => {
    const server = createServer((req, res) => {
      const got = new URL(req.url ?? "/", REDIRECT);
      if (got.pathname !== callback.pathname) return void res.writeHead(404).end();
      const p = got.searchParams;
      const why =
        p.get("state") !== state ? "state mismatch"
        : p.has("iss") && p.get("iss") !== ISSUER ? `issuer ${p.get("iss")}`
        : p.has("error") ? `${p.get("error")}: ${p.get("error_description")}`
        : p.get("code") === null ? "no code"
        : null;
      res.writeHead(why ? 400 : 200, { "content-type": "text/plain" }).end(why ? `Figma login failed: ${why}` : "Logged in to Figma; this tab can close.");
      server.close();
      if (why) reject(new Error(`Figma login: ${why}`));
      else resolve(p.get("code") as string);
    });
    server.on("error", reject);
    server.listen(Number(callback.port), callback.hostname, () => {
      console.error(`Log in to Figma: ${url}`);
      const opener = process.platform === "darwin" ? "open" : "xdg-open";
      // The printed URL is the way in; the opener only saves a click.
      spawn(opener, [url.href], { stdio: "ignore", detached: true })
        .on("error", e => console.error(`${opener}: ${e.message}`))
        .unref();
    });
  });
  const token = await grant(client, { grant_type: "authorization_code", code, redirect_uri: REDIRECT, code_verifier: verifier });
  if (token === null) throw new Error(`Figma refused the login's code; if the client is gone, delete ${STATE_PATH}`);
  return token;
}

async function accessToken(): Promise<string> {
  let state = load();
  if (state === null) {
    state = { client: await register() };
    save(state);
  }
  if (state.token !== undefined && state.token.expiresAt > Date.now() + 60_000) return state.token.access;
  const token =
    (state.token && (await grant(state.client, { grant_type: "refresh_token", refresh_token: state.token.refresh }, state.token))) ??
    (await login(state.client));
  save({ ...state, token });
  return token.access;
}

interface Message {
  id?: number;
  result?: unknown;
  error?: unknown;
}

/** The JSON-RPC messages of a `text/event-stream` body. */
export function sseMessages(body: string): Message[] {
  return body
    .split(/\r?\n\r?\n/)
    .map(event =>
      event
        .split(/\r?\n/)
        .filter(line => line.startsWith("data:"))
        .map(line => line.slice(5).replace(/^ /, ""))
        .join("\n"),
    )
    .filter(data => data !== "")
    .map(data => JSON.parse(data) as Message);
}

/** A session with Figma's MCP server, logging in first if needed: calls a tool, answers its text. */
export async function connect(): Promise<(tool: string, args: Record<string, unknown>) => Promise<string>> {
  const token = await accessToken();
  const headers: Record<string, string> = {
    authorization: `Bearer ${token}`,
    "content-type": "application/json",
    accept: "application/json, text/event-stream",
  };
  const post = async (message: object) => {
    const res = await fetch(MCP, { method: "POST", headers, body: JSON.stringify(message) });
    if (!res.ok) throw new Error(`${MCP}: ${res.status} ${await res.text()}`);
    const session = res.headers.get("mcp-session-id");
    if (session !== null) headers["mcp-session-id"] = session;
    return res;
  };
  let lastId = 0;
  const request = async (method: string, params: object) => {
    const id = ++lastId;
    const res = await post({ jsonrpc: "2.0", id, method, params });
    const body = await res.text();
    const messages = res.headers.get("content-type")?.startsWith("text/event-stream") ? sseMessages(body) : [JSON.parse(body) as Message];
    const reply = messages.find(m => m.id === id);
    if (reply === undefined || reply.error !== undefined) throw new Error(`${method}: ${body}`);
    return reply.result;
  };

  const init = (await request("initialize", {
    protocolVersion: "2025-06-18",
    capabilities: {},
    clientInfo: { name: "evinvest-figma-render", version },
  })) as { protocolVersion: string };
  headers["mcp-protocol-version"] = init.protocolVersion;
  await post({ jsonrpc: "2.0", method: "notifications/initialized" });

  return async (tool, args) => {
    const result = (await request("tools/call", { name: tool, arguments: args })) as { content: { type: string; text?: string }[]; isError?: boolean };
    const text = result.content.flatMap(c => (c.type === "text" && c.text !== undefined ? [c.text] : [])).join("\n");
    if (result.isError) throw new Error(`${tool}: ${text}`);
    return text;
  };
}
