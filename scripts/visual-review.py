"""One self-contained HTML page of every gallery component whose render moved
since a git ref. Old side = the ref's committed baselines, so nothing is rebuilt.
Usage: visual-review.py [<git-ref>] [--themes a,b] [--fresh]"""
import argparse, shutil, base64, html, os, pathlib, subprocess, sys, tempfile

FUZZ, PAD = 1, 16
SHOTS = "rust/tests/visual/__screenshots__"

ap = argparse.ArgumentParser()
ap.add_argument("ref", nargs="?", default="origin/main")
ap.add_argument("--themes", default="dark")
ap.add_argument("--fresh", action="store_true", help="capture first and use dist/captured/ as the new side")
args = ap.parse_args()
themes = args.themes.split(",")

root = pathlib.Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip())


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
sid = lambda i: f"r{i}"
index = "".join(f"<li><a href='#{sid(i)}'>{html.escape(l)}</a> — {p:.2f} %</li>" for i, (p, l, *_) in enumerate(rows))
body = "".join(
    f"<h2 id='{sid(i)}'>{html.escape(l)} — {p:.2f} % of pixels differ</h2>"
    f"<div class=row><figure><figcaption>diff</figcaption><img src='{d}'></figure>"
    f"<figure><figcaption>old</figcaption><img src='{o}'></figure>"
    f"<figure><figcaption>new</figcaption><img src='{n}'></figure></div>"
    for i, (p, l, d, o, n) in enumerate(rows)
)
side = lambda title, names: f"<p>{title}: {', '.join(map(html.escape, names))}</p>" if names else ""
page = f"""<!doctype html><meta charset=utf-8><title>visual review vs {html.escape(args.ref)}</title>
<style>
body{{background:#1b1b1b;color:#ddd;font:14px system-ui,sans-serif;margin:24px}}
h2{{margin:36px 0 8px;border-top:1px solid #444;padding-top:14px;font-size:16px}}
.row{{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:12px;align-items:start}}
figure{{margin:0;overflow:auto}} figcaption{{color:#999;margin-bottom:4px}}
img{{display:block;max-width:100%;outline:1px solid #555}} a{{color:#8ab4ff}}
</style>
<h1>Visual review: {html.escape(args.ref)} baselines vs {'fresh capture' if args.fresh else 'working-tree baselines'}</h1>
<p>Themes: {', '.join(themes)}. A pixel counts as changed when old and new differ by more than {FUZZ} % (fuzz).
% = changed pixels / pixels of the component's area (union of old and new content boxes + {PAD}px padding), not the 1280x720 page.
Diff panel: old, dimmed and greyscale; red = changed pixels. {identical} identical renders not listed; the combined board (index) is skipped.</p>
{side('Added pages', added)}{side('Removed pages', removed)}
<ol>{index}</ol>{body}"""
out = root / "tmp/visual-review.html"
out.parent.mkdir(exist_ok=True)
out.write_text(page)
shutil.rmtree(tmp)
print(out)
