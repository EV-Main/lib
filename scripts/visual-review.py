"""One self-contained HTML page of every gallery component whose render moved
since a git ref. Old side = the ref's committed baselines, so nothing is rebuilt.
Usage: visual-review.py [<git-ref>] [--themes a,b] [--fresh]"""
import argparse, json, re, shutil, time, base64, html, os, pathlib, subprocess, sys, tempfile

FUZZ, PAD = 1, 16
SHOTS = "rust/tests/visual/__screenshots__"

ap = argparse.ArgumentParser()
ap.add_argument("ref", nargs="?", default="origin/main")
ap.add_argument("--themes", default="dark")
ap.add_argument("--decisions", nargs="?", const="tmp/visual-review.html", metavar="PATH", help="print the decisions embedded in a saved page")
ap.add_argument("--fresh", action="store_true", help="capture first and use dist/captured/ as the new side")
args = ap.parse_args()
themes = args.themes.split(",")

root = pathlib.Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip())
out = root / "tmp/visual-review.html"
DECISIONS = re.compile(r'<script type="application/json" id="decisions">(.*?)</script>', re.S)


def embedded(path):
    return json.loads(DECISIONS.search(pathlib.Path(path).read_text()).group(1))


if args.decisions:
    d = embedded(root / args.decisions)
    for title, labels in (("marked", [l for l in d["labels"] if l in d["decisions"]]), ("unmarked", [l for l in d["labels"] if l not in d["decisions"]])):
        print(f"# {title}")
        for l in labels:
            print(f"{l}: {d['decisions'][l]}" if title == "marked" else l)
    sys.exit()

previous = embedded(out)["decisions"] if out.exists() and DECISIONS.search(out.read_text()) else {}


def run(*a, **kw):
    r = subprocess.run(a, capture_output=True, **kw)
    assert r.returncode == 0, f"{a}: {r.stderr}"
    return r.stdout


def magick(*a):
    return run("magick", *a, text=True)


if args.fresh:
    # the test must not fail on mismatch: it is the capture we want
    subprocess.run(
        [os.environ.get("EV_CARGO", "cargo"), "test", "--features", "uikit_dioxus", "--test", "gallery"],
        cwd=root / "rust", check=True, env={**os.environ, "EV_CAPTURE_ONLY": "1"},
    )
    new_dir = root / "rust/tests/visual/dist/captured"
else:
    new_dir = root / SHOTS
new = {p.name: p for p in new_dir.glob("*.png")}

old_names = run("git", "-C", str(root), "ls-tree", "--name-only", f"{args.ref}:{SHOTS}/", text=True).split()


def old_bytes(name):
    label, theme, _ = name.rsplit(".", 2)
    for n in (name, f"{label}.png" if theme == "dark" else None):  # pre-theme baselines are dark
        if n in old_names:
            return run("git", "-C", str(root), "show", f"{args.ref}:{SHOTS}/{n}")
    return None


def key(name):
    label, theme, _ = name.rsplit(".", 2)
    return label, theme


wanted = sorted(n for n in new if key(n)[0] != "index" and key(n)[1] in themes)
old_keys = {name if name.count(".") == 2 else f"{name[:-4]}.dark.png" for name in old_names}  # pre-theme baselines are dark
removed = sorted(n for n in old_keys if key(n)[0] != "index" and key(n)[1] in themes and n not in new)

(root / "tmp").mkdir(exist_ok=True)
tmp = pathlib.Path(tempfile.mkdtemp(prefix="visual-review-", dir=root / "tmp"))  # not $TMPDIR: playwright wipes it


def uri(path):
    return "data:image/png;base64," + base64.b64encode(pathlib.Path(path).read_bytes()).decode()


def dims(path):
    w, h = magick(str(path), "-format", "%w %h", "info:").split()
    return int(w), int(h)


def content_box(path):
    # trims against the top-left pixel, which is the page background
    g = magick(str(path), "-fuzz", "2%", "-format", "%@", "info:").strip()
    wh, x, y = g.split("+")
    w, h = map(int, wh.split("x"))
    return int(x), int(y), int(x) + w, int(y) + h


rows, added, identical = [], [], 0
for name in wanted:
    b = old_bytes(name)
    if b is None:
        added.append(name)
        continue
    if b == new[name].read_bytes():
        identical += 1
        continue
    po, pn = tmp / "old.png", new[name]
    po.write_bytes(b)
    (ow, oh), (nw, nh) = dims(po), dims(pn)
    W, H = max(ow, nw), max(oh, nh)
    bg = magick(str(po), "-format", "%[pixel:p{0,0}]", "info:")
    canvas = ["-background", bg, "-gravity", "northwest", "-extent", f"{W}x{H}"]
    ob, nb = content_box(po), content_box(pn)
    x0, y0 = max(min(ob[0], nb[0]) - PAD, 0), max(min(ob[1], nb[1]) - PAD, 0)
    x1, y1 = min(max(ob[2], nb[2]) + PAD, W), min(max(ob[3], nb[3]) + PAD, H)
    cw, ch = x1 - x0, y1 - y0
    crop = ["-crop", f"{cw}x{ch}+{x0}+{y0}", "+repage"]
    co, cn, mask, diff = (tmp / f for f in ("co.png", "cn.png", "mask.png", "diff.png"))
    magick(str(po), *canvas, *crop, str(co))
    magick(str(pn), *canvas, *crop, str(cn))
    # compare prints the differing-pixel count on stderr and exits 1 when any differ
    r = subprocess.run(
        ["magick", "compare", "-fuzz", f"{FUZZ}%", "-metric", "AE", str(co), str(cn),
         "-compose", "src", "-highlight-color", "red", "-lowlight-color", "none", str(mask)],
        capture_output=True, text=True)
    assert r.returncode in (0, 1), r.stderr
    changed = int(float(r.stderr.split()[0]))
    if changed == 0:
        identical += 1  # bytes differ, but only below the fuzz
        continue
    magick(str(co), "-colorspace", "Gray", "-colorspace", "sRGB", "-evaluate", "multiply", "0.45", "-evaluate", "add", "10%",
           str(mask), "-compose", "over", "-composite", str(diff))
    label, theme = key(name)
    rows.append((changed / (cw * ch) * 100, f"{label}" + (f" ({theme})" if len(themes) > 1 else ""), uri(diff), uri(co), uri(cn)))

rows.sort(key=lambda r: -r[0])
labels = [r[1] for r in rows]
meta = {"ref": args.ref, "stamp": int(time.time()), "labels": labels, "decisions": {l: previous[l] for l in labels if l in previous}}
e = html.escape
index = "".join(f"<li data-label='{e(l)}'><a href='#r{i}'>{e(l)}</a> — {p:.2f} % <span class=mark></span></li>" for i, (p, l, *_) in enumerate(rows))
body = "".join(
    f"<section class=row id='r{i}' data-label='{e(l)}'><h2>{e(l)} — {p:.2f} % of pixels differ</h2><div class=panels>"
    f"<figure data-p=both><figcaption><button data-choose=both>diff · both are fine</button></figcaption><img src='{d}'></figure>"
    f"<figure data-p=old><figcaption><button data-choose=old>old · keep old</button></figcaption><img src='{o}'></figure>"
    f"<figure data-p=new><figcaption><button data-choose=new>new · keep new</button></figcaption><img src='{n}'></figure></div></section>"
    for i, (p, l, d, o, n) in enumerate(rows)
)
side = lambda title, names: f"<p>{title}: {', '.join(map(e, names))}</p>" if names else ""
page = f"""<!doctype html><meta charset=utf-8><title>visual review vs {e(args.ref)}</title>
<style>
body{{background:#1b1b1b;color:#ddd;font:14px system-ui,sans-serif;margin:24px}}
h2{{margin:0 0 8px;font-size:16px}} a{{color:#8ab4ff}}
.row{{margin-top:36px;border-top:1px solid #444;padding-top:14px;scroll-margin-top:50px}} .row.cur h2::before{{content:'▶ ';color:#d29922}}
.panels{{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:16px;align-items:start}}
figure{{margin:0;position:relative}} img{{display:block;max-width:100%;outline:1px solid #555;cursor:zoom-in}}
figcaption button{{all:unset;cursor:pointer;color:#999;margin-bottom:4px;padding:2px 4px}} figcaption button:hover{{color:#fff;background:#333}}
.row[data-pick=old] figure[data-p=old] img,.row[data-pick=new] figure[data-p=new] img,
.row[data-pick=both] figure[data-p=old] img,.row[data-pick=both] figure[data-p=new] img{{outline:5px solid #3fb950}}
.row[data-pick=old] figure[data-p=old]::after,.row[data-pick=new] figure[data-p=new]::after{{content:'✓ chosen';position:absolute;top:24px;right:6px;background:#3fb950;color:#000;padding:2px 8px;font-weight:700}}
.row[data-pick=both] figure[data-p=old]::after,.row[data-pick=both] figure[data-p=new]::after{{content:'✓ either';position:absolute;top:24px;right:6px;background:#d29922;color:#000;padding:2px 8px;font-weight:700}}
.row[data-pick=both] figure[data-p=old] img,.row[data-pick=both] figure[data-p=new] img{{outline-color:#d29922}}
#bar{{position:sticky;top:0;background:#1b1b1b;padding:6px 0;z-index:2;display:flex;gap:16px;align-items:center}}
#zoom{{position:fixed;inset:0;background:#000d;z-index:9;display:grid;grid-template-columns:repeat(3,1fr);gap:8px;padding:8px}} #zoom[hidden]{{display:none}}
.zp{{overflow:hidden;background:#111;cursor:grab;touch-action:none}} .zp img{{transform-origin:0 0;image-rendering:pixelated;user-select:none;-webkit-user-drag:none;max-width:none;outline:0;cursor:inherit}}
.mark{{color:#3fb950}}
</style>
<div id=bar><button id=save>Save (Ctrl+S)</button><span id=count></span><span id=saved></span></div>
<h1>Visual review: {e(args.ref)} baselines vs {'fresh capture' if args.fresh else 'working-tree baselines'}</h1>
<p><b>Deciding:</b> per component click the <b>old</b> or <b>new</b> caption to keep that render, or <b>diff</b> when either is fine; click again to clear.
Keys: <kbd>j</kbd>/<kbd>k</kbd> next/previous row, <kbd>1</kbd> old, <kbd>2</kbd> new, <kbd>3</kbd> either, <kbd>0</kbd> clear.
Choices survive reloads (browser storage) and are written into the file by <b>Ctrl+S</b> or the Save button; the first save asks where to write (Chromium; elsewhere it downloads).
Read them back with <code>nix run .#visual-review -- --decisions</code>.<br>
<b>Zoom:</b> double-click a panel (or <kbd>z</kbd>) for a full-screen side-by-side at native resolution; wheel or pinch zooms, drag pans, all three in lockstep, double-click zooms in, Esc or a click outside closes.</p>
<p>Themes: {', '.join(themes)}. A pixel counts as changed when old and new differ by more than {FUZZ} % (fuzz).
% = changed pixels / pixels of the component's area (union of old and new content boxes + {PAD}px padding), not the 1280x720 page.
Diff panel: old, dimmed and greyscale; red = changed pixels. {identical} identical renders not listed; the combined board (index) is skipped.</p>
{side('Added pages', added)}{side('Removed pages', removed)}
<ol>{index}</ol>{body}
<div id=zoom hidden><div class=zp><img></div><div class=zp><img></div><div class=zp><img></div></div>
<script type="application/json" id="decisions">{json.dumps(meta)}</script>
<script>{(pathlib.Path(__file__).parent / "visual-review.js").read_text()}</script>"""
out.parent.mkdir(exist_ok=True)
out.write_text(page)
shutil.rmtree(tmp)
print(out)
