// Playback UI for the dining-philosophers simulation. All simulation logic
// lives in the Rust engine (see engine/); this file only drives it and renders
// snapshots. Snapshot shape: engine/src/lib.rs.

const $ = (id) => document.getElementById(id);
const SVG_NS = "http://www.w3.org/2000/svg";
const STATE_COLORS = { thinking: "var(--thinking)", hungry: "var(--hungry)", eating: "var(--eating)" };
const INK = "var(--ink)";
const MONO = "'Geist Mono', ui-monospace, monospace";

let WasmSimulation = null;
let sim = null;
let currentTick = 0;
let playing = false;
let playTimer = null;

function showBanner(msg) {
  const el = $("banner");
  el.textContent = msg;
  el.classList.remove("hidden");
}

function hideBanner() {
  $("banner").classList.add("hidden");
}

// Design-preview stand-in, enabled with ?mock in the URL. Cycles states on a
// seeded script so the page can be styled/tested before the engine exists.
// Deliberately no acquisition logic — this is NOT a dining-philosophers
// implementation (see docs/adr/0002).
class MockSimulation {
  constructor(config) {
    this.n = config.philosophers;
    let s = Number(config.seed) >>> 0 || 1;
    this.rand = () => ((s = (s * 1664525 + 1013904223) >>> 0) / 2 ** 32);
    this.history = [];
    this.phils = Array.from({ length: this.n }, () => ({
      state: "thinking", left: Math.floor(this.rand() * 12) + 4, meals: 0, hunger: 0,
    }));
    this.record(false);
  }
  record() {
    const forks = Array.from({ length: this.n }, () => ({ holder: null, dirty: null }));
    this.phils.forEach((p, i) => {
      if (p.state === "eating") {
        forks[i].holder = i;
        forks[(i - 1 + this.n) % this.n].holder = i;
      }
    });
    this.history.push({
      tick: this.history.length,
      philosophers: this.phils.map((p) => ({
        state: p.state,
        holding: [],
        meals: p.meals,
        hunger_streak: p.hunger,
      })),
      forks,
      deadlocked: false,
    });
  }
  tick() {
    const next = { thinking: "hungry", hungry: "eating", eating: "thinking" };
    this.phils.forEach((p) => {
      if (p.state === "hungry") p.hunger += 1;
      if (--p.left <= 0) {
        if (p.state === "eating") { p.meals += 1; p.hunger = 0; }
        p.state = next[p.state];
        p.left = Math.floor(this.rand() * 14) + 4;
      }
    });
    this.record();
  }
  snapshot(t) { return this.history[Number(t)]; }
  ticks() { return this.history.length; }
}

async function loadWasm() {
  if (new URLSearchParams(location.search).has("mock")) {
    WasmSimulation = MockSimulation;
    return;
  }
  try {
    const mod = await import("./pkg/wasm_glue.js");
    await mod.default();
    WasmSimulation = mod.WasmSimulation;
  } catch (e) {
    showBanner(
      "WASM module not found or failed to load.\n" +
      "Build it with:  wasm-pack build wasm --target web --out-dir ../web/pkg\n" +
      `(${e})`
    );
  }
}

function readConfig() {
  return {
    strategy: $("strategy").value,
    philosophers: Math.min(15, Math.max(2, Number($("count").value) || 5)),
    seed: BigInt(Math.max(0, Number($("seed").value) || 0)),
    scheduler: $("scheduler").value,
  };
}

function run() {
  stopPlayback();
  hideBanner();
  sim = null;
  try {
    sim = new WasmSimulation(readConfig());
    currentTick = 0;
    render();
  } catch (e) {
    showBanner(
      "Engine error — likely an unimplemented todo!() (check the browser console for the panic location).\n" +
      `(${e})`
    );
    renderEmpty();
  }
}

// Advance the engine so snapshot(currentTick) exists.
function ensureSimulated(t) {
  while (sim.ticks() <= t) sim.tick();
}

function snapshotAt(t) {
  ensureSimulated(t);
  return sim.snapshot(BigInt(t));
}

// --- playback -------------------------------------------------------------

function stopPlayback() {
  playing = false;
  clearTimeout(playTimer);
  $("play").textContent = "▶";
}

function scheduleNext() {
  const delay = 1000 / Number($("speed").value);
  playTimer = setTimeout(() => {
    if (!playing || !sim) return;
    try {
      currentTick += 1;
      const snap = snapshotAt(currentTick);
      render(snap);
      if (snap.deadlocked) stopPlayback();
      else scheduleNext();
    } catch (e) {
      stopPlayback();
      showBanner(`Engine error at tick ${currentTick}: ${e}`);
    }
  }, delay);
}

function togglePlay() {
  if (!sim) return;
  if (playing) {
    stopPlayback();
  } else {
    playing = true;
    $("play").textContent = "⏸";
    scheduleNext();
  }
}

function step(delta) {
  if (!sim) return;
  stopPlayback();
  const t = Math.max(0, currentTick + delta);
  try {
    currentTick = t;
    render();
  } catch (e) {
    showBanner(`Engine error at tick ${t}: ${e}`);
  }
}

// --- rendering ------------------------------------------------------------

function el(tag, attrs, text) {
  const node = document.createElementNS(SVG_NS, tag);
  for (const [k, v] of Object.entries(attrs)) node.setAttribute(k, v);
  if (text !== undefined) node.textContent = text;
  return node;
}

function polar(cx, cy, r, angle) {
  return [cx + r * Math.cos(angle), cy + r * Math.sin(angle)];
}

// Fork silhouette, ~26px tall, origin at its center, pointing up.
// Swap point for a Noun Project icon: replace this path's `d` with the
// icon's path data (normalized to roughly a -8..8 × -13..13 box).
const FORK_D =
  "M -4.5 -13 v 7 a 4.5 4.5 0 0 0 3 4.2 V 11 a 1.5 1.5 0 0 0 3 0 V -1.8 " +
  "a 4.5 4.5 0 0 0 3 -4.2 v -7 h -2.2 v 6.5 h -1.8 v -6.5 h -2 v 6.5 h -1.8 v -6.5 Z";

function drawFork(x, y, deg, fill) {
  return el("path", {
    d: FORK_D,
    transform: `translate(${x} ${y}) rotate(${deg})`,
    fill,
    stroke: INK,
    "stroke-width": 0.8,
    "stroke-linejoin": "round",
  });
}

// The signature: tiny state-dependent faces (ink strokes on a paper head).
function drawFace(state, x, y) {
  const g = el("g", {
    transform: `translate(${x} ${y})`,
    stroke: INK,
    "stroke-width": 1.7,
    "stroke-linecap": "round",
    fill: "none",
  });
  if (state === "thinking") {
    // Closed eyes, flat mouth, a trail of musing dots.
    g.append(el("path", { d: "M -10 -3 q 3.5 3 7 0" }));
    g.append(el("path", { d: "M 3 -3 q 3.5 3 7 0" }));
    g.append(el("path", { d: "M -4 8 h 8" }));
    g.append(el("circle", { cx: 18, cy: -20, r: 1.4, fill: INK, stroke: "none" }));
    g.append(el("circle", { cx: 23, cy: -26, r: 2, fill: INK, stroke: "none" }));
  } else if (state === "hungry") {
    // Wide eyes fixed on the forks, small open mouth.
    g.append(el("circle", { cx: -6.5, cy: -4, r: 2.6, fill: INK, stroke: "none" }));
    g.append(el("circle", { cx: 6.5, cy: -4, r: 2.6, fill: INK, stroke: "none" }));
    g.append(el("circle", { cx: 0, cy: 8, r: 3.2 }));
  } else if (state === "eating") {
    // Content: happy closed eyes, full cheeks.
    g.append(el("path", { d: "M -10 -3 q 3.5 -4 7 0" }));
    g.append(el("path", { d: "M 3 -3 q 3.5 -4 7 0" }));
    g.append(el("ellipse", { cx: 0, cy: 8, rx: 5, ry: 3.5, fill: INK, stroke: "none" }));
  } else {
    // Unknown state from a future strategy: neutral.
    g.append(el("circle", { cx: -6.5, cy: -4, r: 2.2, fill: INK, stroke: "none" }));
    g.append(el("circle", { cx: 6.5, cy: -4, r: 2.2, fill: INK, stroke: "none" }));
    g.append(el("path", { d: "M -4 8 h 8" }));
  }
  return g;
}

function renderEmpty() {
  $("table").replaceChildren();
  $("deadlock").classList.add("hidden");
  $("scrub").max = 0;
  $("scrub").value = 0;
  $("tick-label").textContent = "tick 0";
}

function render(snap) {
  if (!sim) return;
  snap = snap ?? snapshotAt(currentTick);
  const svg = $("table");
  svg.replaceChildren();

  const n = snap.philosophers.length;
  const cx = 230, cy = 230;
  const angleOf = (i) => (i / n) * 2 * Math.PI - Math.PI / 2;

  svg.append(el("circle", { cx, cy, r: 90, fill: "var(--parchment-deep)", stroke: "var(--hairline)", "stroke-width": 1 }));

  // Forks: fork i sits between philosophers i and (i+1)%n; drawn at its
  // holder's side when held. Chandy–Misra: dirty = orange, clean = paper.
  snap.forks.forEach((fork, i) => {
    const restAngle = angleOf(i) + Math.PI / n;
    const angle = fork.holder === null || fork.holder === undefined
      ? restAngle
      : angleOf(fork.holder) + (i === fork.holder ? -0.28 : 0.28);
    const [x, y] = polar(cx, cy, 70, angle);
    const fill = fork.dirty === true ? "var(--dirty)" : fork.dirty === false ? "var(--paper)" : "var(--faint)";
    svg.append(drawFork(x, y, (angle * 180) / Math.PI + 90, fill));
  });

  snap.philosophers.forEach((p, i) => {
    const [x, y] = polar(cx, cy, 138, angleOf(i));
    const color = STATE_COLORS[p.state] ?? "var(--other)";
    // Paper head with a state-colored ring; the face carries the state too.
    svg.append(el("circle", { cx: x, cy: y, r: 22, fill: "var(--paper)", stroke: color, "stroke-width": 3.5 }));
    svg.append(drawFace(p.state, x, y));
    const [lx, ly] = polar(cx, cy, 192, angleOf(i));
    svg.append(el("text", {
      x: lx, y: ly - 7, "text-anchor": "middle", "dominant-baseline": "middle",
      "font-size": 12, "font-family": MONO, "font-weight": 500, fill: INK,
    }, `P${i}`));
    svg.append(el("text", {
      x: lx, y: ly + 8, "text-anchor": "middle", "dominant-baseline": "middle",
      "font-size": 10.5, "font-family": MONO, fill: "var(--muted)",
    }, `${p.meals} meals · ${p.hunger_streak}h`));
  });

  const dl = $("deadlock");
  if (snap.deadlocked) {
    dl.textContent = `Deadlocked at tick ${snap.tick}`;
    dl.classList.remove("hidden");
  } else {
    dl.classList.add("hidden");
  }

  const scrub = $("scrub");
  scrub.max = Number(sim.ticks()) - 1;
  scrub.value = currentTick;
  $("tick-label").textContent = `tick ${currentTick}`;
}

// --- wiring ---------------------------------------------------------------

$("run").addEventListener("click", run);
$("play").addEventListener("click", togglePlay);
$("step-back").addEventListener("click", () => step(-1));
$("step-fwd").addEventListener("click", () => step(1));
$("randomize").addEventListener("click", () => {
  $("seed").value = Math.floor(Math.random() * 2 ** 32);
});
$("speed").addEventListener("input", () => {
  $("speed-label").textContent = `${$("speed").value} t/s`;
});
$("scrub").addEventListener("input", () => {
  if (!sim) return;
  stopPlayback();
  currentTick = Number($("scrub").value);
  render();
});

await loadWasm();
if (WasmSimulation) run();
