import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { sseMessages } from '../src/mcp';
import { issued, polled } from '../src/render';

// Figma's answers, verbatim from real calls.
const answer = (name: string) => readFileSync(new URL(`./fixtures/figma/${name}.txt`, import.meta.url), 'utf8');

describe("Figma's answers", () => {
  it('an issued capture names its id and endpoint', () => {
    expect(issued(answer('issue'))).toEqual({
      captureId: '78c18467-5589-4a48-b732-ce9791bf0312',
      endpoint: 'https://mcp.figma.com/mcp/capture/78c18467-5589-4a48-b732-ce9791bf0312/submit?bindVariables=true&promptId=5b0c8e64-31a2-4c55-9d0e-6f2a7e1b9c43',
    });
  });

  it('a poll is the frame once captured, pending before, and refused when unknown', () => {
    expect(polled(answer('done'))).toBe('16:2');
    expect(polled(answer('pending'))).toBeNull();
    expect(() => polled('Capture failed: 413')).toThrow(/Capture failed: 413/);
  });

  it('an event stream yields its JSON-RPC messages', () => {
    const body = 'event: message\ndata: {"jsonrpc":"2.0","method":"notifications/progress"}\n\nid: 2\ndata: {"jsonrpc":"2.0",\ndata: "id":1,"result":{}}\n\n';
    expect(sseMessages(body)).toEqual([{ jsonrpc: '2.0', method: 'notifications/progress' }, { jsonrpc: '2.0', id: 1, result: {} }]);
  });
});
