const data = JSON.parse(document.getElementById("decisions").textContent);
const pristine = document.documentElement.outerHTML; // saved with only the JSON block swapped, so no UI state leaks into the file
const storeKey = `visual-review:${data.ref}:${data.stamp}`;
const stored = localStorage.getItem(storeKey);
const picks = stored ? JSON.parse(stored) : data.decisions;
const rows = [...document.querySelectorAll(".row")];
let cur = -1, handle = null;

const render = () => {
  for (const row of rows) {
    const p = picks[row.dataset.label] || "";
    row.dataset.pick = p;
    const li = document.querySelector(`li[data-label="${row.dataset.label}"] .mark`);
    li.textContent = p && `→ ${p}`;
  }
  const n = Object.keys(picks).length;
  document.getElementById("count").textContent = `${n} / ${rows.length} decided`;
  localStorage.setItem(storeKey, JSON.stringify(picks));
};
const pick = (row, v) => {
  const l = row.dataset.label;
  if (picks[l] === v) delete picks[l]; else picks[l] = v;
  render();
};
const go = (i) => {
  cur = Math.max(0, Math.min(rows.length - 1, i));
  rows.forEach((r, j) => r.classList.toggle("cur", j === cur));
  rows[cur].scrollIntoView({ block: "start" });
};

const save = async () => {
  const json = JSON.stringify({ ...data, decisions: picks });
  const html = "<!doctype html>" + pristine.replace(
    /(<script type="application\/json" id="decisions">)[\s\S]*?(<\/script>)/, (_, a, b) => a + json + b);
  const name = location.pathname.split("/").pop() || "visual-review.html";
  const status = document.getElementById("saved");
  if (window.showSaveFilePicker) {
    try {
      handle ??= await showSaveFilePicker({ suggestedName: name, types: [{ accept: { "text/html": [".html"] } }] });
      const w = await handle.createWritable();
      await w.write(html);
      await w.close();
      status.textContent = `saved ${new Date().toLocaleTimeString()}`;
      return;
    } catch (e) {
      if (e.name === "AbortError") return;
      throw e;
    }
  }
  const a = document.createElement("a");
  a.href = URL.createObjectURL(new Blob([html], { type: "text/html" }));
  a.download = name;
  a.click();
  status.textContent = "downloaded (File System Access API unavailable)";
};

document.querySelectorAll("[data-choose]").forEach((b) =>
  b.addEventListener("click", () => pick(b.closest(".row"), b.dataset.choose)));
document.getElementById("save").addEventListener("click", save);

const zoom = document.getElementById("zoom");
const view = { s: 1, x: 0, y: 0 };
const panels = [...zoom.querySelectorAll(".zp")];
const apply = () => panels.forEach((p) => {
  p.firstElementChild.style.transform = `translate(${view.x}px,${view.y}px) scale(${view.s})`;
});
const zoomAt = (px, py, k) => {
  const s = Math.max(0.1, Math.min(32, view.s * k));
  view.x = px - (px - view.x) * (s / view.s);
  view.y = py - (py - view.y) * (s / view.s);
  view.s = s;
  apply();
};
const open = (row) => {
  row.querySelectorAll("img").forEach((img, i) => (panels[i].firstElementChild.src = img.src));
  Object.assign(view, { s: 1, x: 0, y: 0 });
  apply();
  zoom.hidden = false;
};
const origin = (e, p) => { const r = p.getBoundingClientRect(); return [e.clientX - r.left, e.clientY - r.top]; };
rows.forEach((row) => row.querySelectorAll("img").forEach((img) =>
  img.addEventListener("dblclick", () => open(row))));
panels.forEach((p) => {
  p.addEventListener("wheel", (e) => {
    e.preventDefault();
    zoomAt(...origin(e, p), Math.exp(-e.deltaY * (e.ctrlKey ? 0.01 : 0.002))); // ctrlKey = trackpad pinch
  }, { passive: false });
  p.addEventListener("dblclick", (e) => zoomAt(...origin(e, p), 2));
  p.addEventListener("pointerdown", (e) => {
    p.setPointerCapture(e.pointerId);
    p.onpointermove = (m) => { view.x += m.movementX; view.y += m.movementY; apply(); };
    p.onpointerup = () => { p.onpointermove = p.onpointerup = null; };
  });
});
zoom.addEventListener("click", (e) => { if (e.target === zoom) zoom.hidden = true; });

document.addEventListener("keydown", (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key === "s") { e.preventDefault(); save(); return; }
  if (e.ctrlKey || e.metaKey || e.altKey) return;
  if (e.key === "Escape") { zoom.hidden = true; return; }
  if (!zoom.hidden) return;
  if (e.key === "j") go(cur + 1);
  else if (e.key === "k") go(cur - 1);
  else if (cur < 0) return;
  else if (e.key === "1") pick(rows[cur], "old");
  else if (e.key === "2") pick(rows[cur], "new");
  else if (e.key === "3") pick(rows[cur], "both");
  else if (e.key === "0") { delete picks[rows[cur].dataset.label]; render(); }
  else if (e.key === "z") open(rows[cur]);
});
render();
