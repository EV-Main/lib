import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { bool, int, list, num, str } from '../src/index';

// Shared with ev_lib::settings (rust/tests/settings.rs).
const CONTRACT = JSON.parse(readFileSync(join(import.meta.dirname, '../../../tests/fixtures/settings/contract.json'), 'utf8')) as {
  bool: { true: string[]; false: string[]; invalid: string[]; error: string };
  string: string[];
  int: { accept: [string, number][]; reject: string[] };
  float: { accept: [string, number][]; reject: string[] };
  list: [string, string[]][];
  int_list: [string, number[]][];
  int_list_error: { raw: string; prefix: string; item: string };
};

describe('parsing contract', () => {
  it.each(CONTRACT.bool.true)('bool %j is true', (raw) => {
    expect(bool().parse(raw)).toBe(true);
  });

  it.each(CONTRACT.bool.false)('bool %j is false', (raw) => {
    expect(bool().parse(raw)).toBe(false);
  });

  it.each(CONTRACT.bool.invalid)('bool %j is invalid', (raw) => {
    expect(() => bool().parse(raw)).toThrowError(CONTRACT.bool.error);
  });

  it.each(CONTRACT.string)('string %j is kept as is', (raw) => {
    expect(str().parse(raw)).toBe(raw);
  });

  it.each(CONTRACT.int.accept)('int %j is %d', (raw, parsed) => {
    expect(int().parse(raw)).toBe(parsed);
  });

  it.each(CONTRACT.int.reject)('int %j is invalid', (raw) => {
    expect(() => int().parse(raw)).toThrowError();
  });

  it.each(CONTRACT.float.accept)('float %j is %d', (raw, parsed) => {
    expect(num().parse(raw)).toBe(parsed);
  });

  it.each(CONTRACT.float.reject)('float %j is invalid', (raw) => {
    expect(() => num().parse(raw)).toThrowError();
  });

  it.each(CONTRACT.list)('list %j is %j', (raw, parsed) => {
    expect(list().parse(raw)).toEqual(parsed);
  });

  it.each(CONTRACT.int_list)('int list %j is %j', (raw, parsed) => {
    expect(list(int()).parse(raw)).toEqual(parsed);
  });

  it('numbers a list error by kept item and never leaks the item value', () => {
    const { raw, prefix, item } = CONTRACT.int_list_error;
    let message = '';
    try {
      list(int()).parse(raw);
    } catch (error) {
      message = (error as Error).message;
    }
    expect(message.startsWith(prefix)).toBe(true);
    expect(message).not.toContain(item);
  });

  it('diverges from Rust where documented', () => {
    // Rust `f64` also accepts `inf`/`NaN` where `num()` requires finite; Rust
    // 64-bit integers parse exactly where `int()` stops at the safe range.
    expect(() => num().parse('NaN')).toThrowError();
    expect(() => num().parse('Infinity')).toThrowError();
    expect(() => int().parse('9007199254740993')).toThrowError(/safe integer/);
  });
});
