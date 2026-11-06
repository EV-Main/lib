import { readFileSync } from "node:fs";
import { join } from "node:path";

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  captureBody,
  createBeaconSink,
  createPostHogSink,
  deliverBeacon,
  type PostHogLike,
} from "../src/index";

// Shared with ev_lib::analytics (rust/tests/analytics.rs).
const VECTORS = JSON.parse(readFileSync(join(import.meta.dirname, "../../../tests/fixtures/analytics/beacon.json"), "utf8")) as {
  capture: [string, string, string, Record<string, unknown>, string][];
};

const prop = (value: unknown): unknown =>
  typeof value === "object" && value !== null && "f64" in value ? Number(value.f64) : value;

const BARE = captureBody("phc_test", "anon-1", "app_booted");

describe("captureBody — byte parity with ev_lib::analytics::capture_body", () => {
  it.each(VECTORS.capture)("%j sends %j as %j with %j", (apiKey, distinctId, event, props, body) => {
    const decoded = Object.fromEntries(Object.entries(props).map(([key, value]) => [key, prop(value)]));
    expect(captureBody(apiKey, distinctId, event, decoded)).toBe(body);
  });

  it("skips values Rust's PropValue cannot express", () => {
    const body = captureBody("k", "d", "e", {
      nested: { a: 1 },
      list: [1],
      missing: undefined,
      nothing: null,
      ok: "yes",
    });
    expect(body).toBe(
      '{"api_key":"k","distinct_id":"d","event":"e","properties":{"$lib":"ev-analytics","ok":"yes"}}',
    );
  });
});

describe("createBeaconSink", () => {
  let sendBeacon: ReturnType<typeof vi.fn>;
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    sendBeacon = vi.fn(() => true);
    fetchMock = vi.fn(() => Promise.resolve(new Response(null)));
    vi.stubGlobal("navigator", { sendBeacon });
    vi.stubGlobal("fetch", fetchMock);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("sends the Rust capture_body payload through navigator.sendBeacon", () => {
    const sink = createBeaconSink({
      key: "phc_test",
      region: "eu",
      distinctId: "anon-1",
    });
    sink.capture("app_booted");
    expect(sendBeacon).toHaveBeenCalledTimes(1);
    expect(sendBeacon).toHaveBeenCalledWith(
      "https://eu.i.posthog.com/capture/",
      BARE,
    );
    expect(fetchMock).not.toHaveBeenCalled();
  });

  it("uses an explicit host, trimming trailing slashes like the Rust client", () => {
    const sink = createBeaconSink({
      key: "phc_test",
      host: "https://ph.example.com//",
      distinctId: "d",
    });
    sink.capture("e");
    expect(sendBeacon.mock.calls[0]?.[0]).toBe(
      "https://ph.example.com/capture/",
    );
  });

  it("falls back to a keepalive fetch when sendBeacon refuses the payload", () => {
    sendBeacon.mockReturnValue(false);
    const sink = createBeaconSink({
      key: "phc_test",
      region: "us",
      distinctId: "anon-1",
    });
    sink.capture("app_booted");
    expect(fetchMock).toHaveBeenCalledWith("https://us.i.posthog.com/capture/", {
      method: "POST",
      body: BARE,
      keepalive: true,
    });
  });

  it("falls back to a keepalive fetch when sendBeacon does not exist", () => {
    vi.stubGlobal("navigator", {});
    const sink = createBeaconSink({ key: "k", region: "eu", distinctId: "d" });
    sink.capture("e");
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  it("applies globalProps and the allow-list before serializing", () => {
    const sink = createBeaconSink({
      key: "phc_test",
      region: "eu",
      distinctId: "anon-42",
      allowedProps: ["brand_id", "location_id", "channel"],
      globalProps: { brand_id: "aquafix", location_id: "warsaw" },
      strict: false,
    });
    sink.capture("contact_intent_click", {
      channel: "phone",
      phone: "+48 600 000 000",
    });
    expect(sendBeacon).toHaveBeenCalledWith(
      "https://eu.i.posthog.com/capture/",
      captureBody("phc_test", "anon-42", "contact_intent_click", {
        brand_id: "aquafix",
        location_id: "warsaw",
        channel: "phone",
      }),
    );
  });

  it("keeps one in-memory distinct id per sink and a fresh one per sink", () => {
    const first = createBeaconSink({ key: "k", region: "eu" });
    first.capture("a");
    first.capture("b");
    createBeaconSink({ key: "k", region: "eu" }).capture("c");
    const ids = sendBeacon.mock.calls.map(
      (call) => JSON.parse(String(call[1])).distinct_id,
    );
    expect(ids[0]).toBe(ids[1]);
    expect(ids[2]).not.toBe(ids[0]);
  });

  it("requires an explicit region or host at the type level", () => {
    // @ts-expect-error — a new sink has no legacy region to fall back to.
    createBeaconSink({ key: "k" });
  });

  it("never throws when the transport itself throws", () => {
    sendBeacon.mockImplementation(() => {
      throw new Error("blocked");
    });
    const sink = createBeaconSink({ key: "k", region: "eu" });
    expect(() => sink.capture("e")).not.toThrow();
  });
});

describe("deliverBeacon", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("does nothing when neither sendBeacon nor fetch exists", () => {
    vi.stubGlobal("navigator", undefined);
    vi.stubGlobal("fetch", undefined);
    expect(() => deliverBeacon("https://x/capture/", "{}")).not.toThrow();
  });
});

describe("without a PostHog key", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("makes no network call from any sink", () => {
    const sendBeacon = vi.fn(() => true);
    const fetchMock = vi.fn();
    const xhr = vi.fn();
    vi.stubGlobal("navigator", { sendBeacon });
    vi.stubGlobal("fetch", fetchMock);
    vi.stubGlobal("XMLHttpRequest", xhr);
    const posthog = { init: vi.fn(), capture: vi.fn() } satisfies PostHogLike;

    const sinks = [
      createBeaconSink({ region: "eu" }),
      createBeaconSink({ key: "", host: "https://ph.example.com" }),
      createPostHogSink(posthog, {}),
      createPostHogSink(posthog, { cookieless: true, region: "eu" }),
    ];
    for (const sink of sinks) {
      sink.capture("contact_intent_click", { channel: "phone" });
      sink.capture("x", {}, { transport: "beacon" });
    }

    expect(sendBeacon).not.toHaveBeenCalled();
    expect(fetchMock).not.toHaveBeenCalled();
    expect(xhr).not.toHaveBeenCalled();
    expect(posthog.init).not.toHaveBeenCalled();
    expect(posthog.capture).not.toHaveBeenCalled();
  });

  it("still enforces the allow-list, so development catches PII keys early", () => {
    vi.stubEnv("NODE_ENV", "development");
    const sink = createBeaconSink({ region: "eu", allowedProps: ["channel"] });
    expect(() => sink.capture("e", { phone: "+33" })).toThrow();
    vi.unstubAllEnvs();
  });
});
