// Host mode: a host on another origin writes attributes with `hostAttributes`,
// the element reads them with `readHostProps`. The contract pinned here: what
// the host writes is exactly what the element renders, a value the reader would
// refuse is never written, and an href that could run script or leave the
// origin never reaches an `<a href>`. The safeHref and parseGroups vectors are
// carried over from banking `cabinet/frontend/features/account-chip/lib/host-attributes.test.ts`.
import { describe, expect, it } from 'vitest';

import {
  HOST_ATTRIBUTE_NAMES,
  HOST_ATTRS,
  hostAttributes,
  isHostMode,
  parseGroups,
  readHostProps,
  safeHref,
  type HostAttributes,
  type MenuGroupInput,
} from '../src/index';

/** An attribute reader over a written record, standing in for `el.getAttribute.bind(el)`. */
function readerOf(attrs: HostAttributes) {
  return (name: string): string | null => (attrs as Record<string, string | undefined>)[name] ?? null;
}

const item = (id: string, extra: Record<string, unknown> = {}) => ({ id, href: `/${id}`, label: id.toUpperCase(), ...extra });

describe('safeHref: accepted', () => {
  it('should_accept_a_root_relative_href_as_written', () => {
    expect(safeHref('/x')).toBe('/x');
    expect(safeHref('/en/cabinet/settings?tab=1#a')).toBe('/en/cabinet/settings?tab=1#a');
  });

  it('should_accept_an_absolute_http_or_https_href_as_written', () => {
    expect(safeHref('https://sa.evinvest.ltd/places')).toBe('https://sa.evinvest.ltd/places');
    expect(safeHref('http://localhost:3000/')).toBe('http://localhost:3000/');
  });

  it('should_trim_surrounding_spaces_from_an_accepted_href', () => {
    expect(safeHref('  /x  ')).toBe('/x');
  });

  it('should_accept_2048_characters_and_refuse_2049', () => {
    const at = `/${'a'.repeat(2047)}`;
    expect(safeHref(at)).toBe(at);
    expect(safeHref(`${at}a`)).toBeNull();
  });
});

describe('safeHref: refused', () => {
  it('should_refuse_javascript_in_any_case_and_with_leading_space', () => {
    expect(safeHref('javascript:alert(1)')).toBeNull();
    expect(safeHref('JavaScript:alert(1)')).toBeNull();
    expect(safeHref(' javascript:alert(1)')).toBeNull();
  });

  it('should_refuse_a_data_href', () => {
    expect(safeHref('data:text/html,<script>alert(1)</script>')).toBeNull();
  });

  it('should_refuse_a_protocol_relative_href', () => {
    expect(safeHref('//evil.example/x')).toBeNull();
  });

  it('should_refuse_a_backslash_after_the_leading_slash', () => {
    expect(safeHref('/\\evil.example')).toBeNull();
  });

  it('should_refuse_a_tab_or_newline_the_browser_would_strip_into_protocol_relative', () => {
    expect(safeHref('/\t/evil.example')).toBeNull();
    expect(safeHref('/\n/evil.example')).toBeNull();
    expect(safeHref('/\r\n/evil.example')).toBeNull();
  });

  it('should_refuse_a_control_character_or_backslash_anywhere', () => {
    expect(safeHref('/a\\b')).toBeNull();
    expect(safeHref('https://sa.example/\\evil')).toBeNull();
    expect(safeHref('/a\u0000b')).toBeNull();
    expect(safeHref('/a\u007Fb')).toBeNull();
  });

  it('should_refuse_a_bare_word_a_non_http_scheme_blank_and_non_strings', () => {
    expect(safeHref('settings')).toBeNull();
    expect(safeHref('mailto:a@b.c')).toBeNull();
    expect(safeHref('ftp://x.example/')).toBeNull();
    expect(safeHref('')).toBeNull();
    expect(safeHref('   ')).toBeNull();
    expect(safeHref(42)).toBeNull();
    expect(safeHref(null)).toBeNull();
  });

  it('should_refuse_plain_http_to_a_real_host', () => {
    expect(safeHref('http://sa.evinvest.ltd/places')).toBeNull();
  });
});

describe('parseGroups', () => {
  it('should_fill_flag_defaults_and_a_null_label', () => {
    const raw = JSON.stringify([{ id: 'more', items: [item('help')] }]);
    expect(parseGroups(raw)).toEqual([
      { id: 'more', label: null, items: [{ id: 'help', href: '/help', label: 'HELP', external: false, current: false }] },
    ]);
  });

  it('should_read_missing_broken_or_non_array_json_as_no_groups', () => {
    expect(parseGroups(null)).toEqual([]);
    expect(parseGroups('[{')).toEqual([]);
    expect(parseGroups('null')).toEqual([]);
    expect(parseGroups(JSON.stringify({ id: 'g', items: [item('a')] }))).toEqual([]);
  });

  it('should_empty_the_whole_list_when_one_item_has_an_unsafe_href', () => {
    const raw = JSON.stringify([
      { id: 'good', items: [item('a')] },
      { id: 'bad', items: [item('c', { href: 'javascript:alert(1)' })] },
    ]);
    expect(parseGroups(raw)).toEqual([]);
  });

  it('should_refuse_a_non_boolean_flag', () => {
    expect(parseGroups(JSON.stringify([{ id: 'g', items: [item('a', { external: 'true' })] }]))).toEqual([]);
    expect(parseGroups(JSON.stringify([{ id: 'g', items: [item('a', { current: null })] }]))).toEqual([]);
  });

  it('should_accept_8_groups_and_refuse_9', () => {
    const groups = (n: number) => JSON.stringify(Array.from({ length: n }, (_, i) => ({ id: `g${i}`, items: [item('a')] })));
    expect(parseGroups(groups(8))).toHaveLength(8);
    expect(parseGroups(groups(9))).toEqual([]);
  });

  it('should_accept_16_items_and_refuse_0_or_17', () => {
    const withItems = (n: number) => JSON.stringify([{ id: 'g', items: Array.from({ length: n }, (_, i) => item(`i${i}`)) }]);
    expect(parseGroups(withItems(16))[0]?.items).toHaveLength(16);
    expect(parseGroups(withItems(0))).toEqual([]);
    expect(parseGroups(withItems(17))).toEqual([]);
  });
});

describe('attribute names', () => {
  it('should_keep_the_frozen_v2_attribute_names', () => {
    expect(HOST_ATTRS).toEqual({
      mode: 'data-mode',
      name: 'data-name',
      email: 'data-email',
      profileHref: 'data-profile-href',
      switchHref: 'data-switch-href',
      groups: 'data-groups',
      intent: 'data-intent',
      returnTo: 'data-return-to',
      layout: 'data-layout',
    });
  });

  it('should_observe_every_attribute_name', () => {
    expect([...HOST_ATTRIBUTE_NAMES].sort()).toEqual(Object.values(HOST_ATTRS).sort());
  });
});

describe('hostAttributes → readHostProps', () => {
  const groups: MenuGroupInput[] = [
    {
      id: 'services',
      label: 'Services',
      items: [
        { id: 'panel', href: '/panel', label: 'Panel', current: true },
        { id: 'docs', href: 'https://docs.example/', label: 'Docs', external: true },
      ],
    },
  ];

  it('should_round_trip_every_value_the_host_sets', () => {
    const attrs = hostAttributes({
      name: 'Valentina Kuznetsova',
      email: 'v@example.com',
      profileHref: '/admin/account',
      switchHref: '/switch',
      groups,
      layout: 'menu',
    });

    expect(isHostMode(readerOf(attrs))).toBe(true);
    expect(attrs[HOST_ATTRS.layout]).toBe('menu');
    expect(readHostProps(readerOf(attrs))).toEqual({
      name: 'Valentina Kuznetsova',
      email: 'v@example.com',
      profileHref: '/admin/account',
      switchHref: '/switch',
      groups: [
        {
          id: 'services',
          label: 'Services',
          items: [
            { id: 'panel', href: '/panel', label: 'Panel', external: false, current: true },
            { id: 'docs', href: 'https://docs.example/', label: 'Docs', external: true, current: false },
          ],
        },
      ],
    });
  });

  it('should_write_groups_as_the_reader_parses_them', () => {
    const panel = { id: ' panel ', href: ' /panel ', label: ' Panel ', current: true, tooltip: 'dropped' };
    const help = { id: 'help', href: '/help', label: 'Help' };
    const input = [
      { id: ' services ', label: ' Services ', items: [panel], icon: 'dropped' },
      { id: 'more', items: [help] },
    ];

    const attrs = hostAttributes({ groups: input });

    expect(JSON.parse(attrs[HOST_ATTRS.groups] ?? 'null')).toStrictEqual([
      {
        id: 'services',
        label: 'Services',
        items: [{ id: 'panel', href: '/panel', label: 'Panel', external: false, current: true }],
      },
      { id: 'more', items: [{ id: 'help', href: '/help', label: 'Help', external: false, current: false }] },
    ]);
  });

  it('should_write_only_data_mode_for_empty_props', () => {
    expect(hostAttributes({})).toEqual({ 'data-mode': 'host' });
  });

  it('should_leave_out_an_unsafe_href_and_keep_the_others', () => {
    const attrs = hostAttributes({ name: 'Valentina', profileHref: '/account', switchHref: '//evil.example' });

    expect(attrs).toEqual({ 'data-mode': 'host', 'data-name': 'Valentina', 'data-profile-href': '/account' });
  });

  it('should_leave_out_an_email_longer_than_254_instead_of_cutting_it', () => {
    const email = `${'a'.repeat(243)}@example.com`;
    expect(email).toHaveLength(255);

    const attrs = hostAttributes({ email });

    expect(attrs[HOST_ATTRS.email]).toBeUndefined();
    expect(readHostProps(readerOf(attrs)).email).toBeNull();
  });

  it('should_keep_an_email_of_exactly_254', () => {
    const email = `${'a'.repeat(242)}@example.com`;

    expect(readHostProps(readerOf(hostAttributes({ email }))).email).toBe(email);
  });

  it('should_cut_a_long_ascii_name_to_254_characters_that_the_reader_accepts', () => {
    const attrs = hostAttributes({ name: 'N'.repeat(300) });

    expect(attrs[HOST_ATTRS.name]).toBe('N'.repeat(254));
    expect(readHostProps(readerOf(attrs)).name).toBe('N'.repeat(254));
  });

  it('should_cut_a_long_name_by_code_points_without_splitting_a_surrogate_pair', () => {
    const attrs = hostAttributes({ name: `a${'😀'.repeat(300)}` });

    expect(Array.from(attrs[HOST_ATTRS.name] ?? '')).toEqual(['a', ...Array<string>(253).fill('😀')]);
  });

  // The written name is what the host is told the element renders; the reader
  // must not refuse a value the writer produced.
  it('should_render_the_cut_name_of_astral_characters_that_the_host_wrote', () => {
    const attrs = hostAttributes({ name: '😀'.repeat(300) });

    expect(readHostProps(readerOf(attrs)).name).toBe(attrs[HOST_ATTRS.name]);
  });

  it('should_render_a_cut_name_exactly_as_written_when_the_cut_lands_after_a_space', () => {
    const attrs = hostAttributes({ name: `${'a'.repeat(253)} ${'b'.repeat(10)}` });

    expect(readHostProps(readerOf(attrs)).name).toBe(attrs[HOST_ATTRS.name]);
  });

  it('should_not_write_more_than_8_groups', () => {
    const nine = Array.from({ length: 9 }, (_, i) => ({ id: `g${i}`, items: [{ id: 'a', href: '/a', label: 'A' }] }));

    const attrs = hostAttributes({ groups: nine });

    expect(attrs[HOST_ATTRS.groups]).toBeUndefined();
  });

  it('should_not_write_a_group_with_no_items', () => {
    const attrs = hostAttributes({ groups: [{ id: 'empty', items: [] }] });

    expect(attrs[HOST_ATTRS.groups]).toBeUndefined();
  });

  it('should_not_write_groups_when_one_item_href_is_unsafe', () => {
    const attrs = hostAttributes({
      groups: [{ id: 'g', items: [{ id: 'x', href: 'javascript:alert(1)', label: 'X' }] }],
    });

    expect(attrs[HOST_ATTRS.groups]).toBeUndefined();
  });

  it('should_not_write_a_blank_name', () => {
    expect(hostAttributes({ name: '   ' })[HOST_ATTRS.name]).toBeUndefined();
  });
});

describe('isHostMode', () => {
  it('should_be_on_only_for_data_mode_exactly_host', () => {
    expect(isHostMode(readerOf({ 'data-mode': 'host' }))).toBe(true);
    expect(isHostMode(readerOf({ 'data-mode': 'Host' }))).toBe(false);
    expect(isHostMode(readerOf({}))).toBe(false);
  });
});
