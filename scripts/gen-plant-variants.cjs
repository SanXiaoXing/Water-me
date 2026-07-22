// Generate plant_wilt.json and plant_watered.json from Plant.json
const fs = require('fs');
const path = require('path');

const ROOT = path.resolve(__dirname, '..');
const SRC = path.join(ROOT, 'src/assets/Plant.json');
const WILT = path.join(ROOT, 'src/assets/Plant_wilt.json');
const WATERED = path.join(ROOT, 'src/assets/Plant_watered.json');

const original = JSON.parse(fs.readFileSync(SRC, 'utf-8'));

// ============ Plant_wilt.json ============
const wilt = JSON.parse(JSON.stringify(original));
// Layer 1 (ind:1) — left leaf: droop -45° (smaller swing)
wilt.layers[0].ks.r.k[0].s = [-45];
wilt.layers[0].ks.r.k[0].e = [-30];
// Layer 2 (ind:2) — stem: bent forward, scale Y 92%
wilt.layers[1].ks.r.k[0].s = [22];
wilt.layers[1].ks.r.k[0].e = [17];
wilt.layers[1].ks.s.k = [100, 92, 100];
// Layer 3 (ind:3) — right leaf: droop 42°
wilt.layers[2].ks.r.k[0].s = [42];
wilt.layers[2].ks.r.k[0].e = [36];
// Leaves (Layer 1 & 3) → grayish green #95A89A
const wiltLeafColor = [0.58431373, 0.65882353, 0.60392157, 1];
// Stem (Layer 2) → grayish green #8AA590
const wiltStemColor = [0.54117647, 0.64705882, 0.56470588, 1];
function recolor(layer, color) {
  if (layer.shapes) {
    layer.shapes.forEach(g => {
      if (g.it) g.it.forEach(it => {
        if (it.ty === 'fl') it.c.k = color;
      });
    });
  }
}
recolor(wilt.layers[0], wiltLeafColor);
recolor(wilt.layers[1], wiltStemColor);
recolor(wilt.layers[2], wiltLeafColor);

fs.writeFileSync(WILT, JSON.stringify(wilt));
console.log('✓ Plant_wilt.json written');

// ============ Plant_watered.json ============
const watered = JSON.parse(JSON.stringify(original));
// Layer 1 (ind:1) — left leaf: stand UP (was -12→3, now -30→-20)
watered.layers[0].ks.r.k[0].s = [-30];
watered.layers[0].ks.r.k[0].e = [-20];
// Layer 2 (ind:2) — stem: perfectly upright (was 15→5, now 2→-3) + 15% bigger
watered.layers[1].ks.r.k[0].s = [2];
watered.layers[1].ks.r.k[0].e = [-3];
watered.layers[1].ks.s.k = [115, 115, 100];
// Layer 3 (ind:3) — right leaf: stand UP (was -5→10, now 25→35)
watered.layers[2].ks.r.k[0].s = [25];
watered.layers[2].ks.r.k[0].e = [35];
// Leaves → vivid fresh green #45C16F
const wateredLeafColor = [0.27058824, 0.75686275, 0.43529412, 1];
// Stem → vivid fresh green #52D18A
const wateredStemColor = [0.32156863, 0.81960784, 0.54117647, 1];
recolor(watered.layers[0], wateredLeafColor);
recolor(watered.layers[1], wateredStemColor);
recolor(watered.layers[2], wateredLeafColor);

// ---- Add 5 dewdrop layers (with falling animation) + 3 sparkle effects ----
const dropBlue = [0.70196078, 0.85098039, 0.94901961, 1];
const dropWhite = [1, 1, 1, 1];
// Drops: each one forms on a leaf, slides down along the leaf arc, fades out, loops
// startX/Y = where the drop forms (on the leaf)
// endX/Y = where it lands / fades out
// appear/peakEnd/fadeEnd = timing in frames (0..144)
const drops = [
  { ind: 6,  name: 'Drop 1', startX: 310, startY: 420, endX: 270, endY: 600, w: 24, h: 32, appear: 8,  peakEnd: 70, fadeEnd: 88 },
  { ind: 7,  name: 'Drop 2', startX: 460, startY: 420, endX: 500, endY: 600, w: 20, h: 28, appear: 30, peakEnd: 92, fadeEnd: 110 },
  { ind: 8,  name: 'Drop 3', startX: 380, startY: 360, endX: 380, endY: 540, w: 14, h: 20, appear: 4,  peakEnd: 55, fadeEnd: 72 },
  { ind: 9,  name: 'Drop 4', startX: 370, startY: 580, endX: 365, endY: 780, w: 10, h: 14, appear: 22, peakEnd: 88, fadeEnd: 104 },
  { ind: 10, name: 'Drop 5', startX: 395, startY: 815, endX: 395, endY: 815, w: 8,  h: 12, appear: 50, peakEnd: 60, fadeEnd: 75 }  // splash on pot
];
const sparkles = [
  { ind: 11, name: 'Sparkle 1', x: 290, y: 380, size: 18, delay: 12 },
  { ind: 12, name: 'Sparkle 2', x: 480, y: 380, size: 14, delay: 48 },
  { ind: 13, name: 'Sparkle 3', x: 380, y: 320, size: 16, delay: 84 }
];
const allFx = drops.concat(sparkles);
function makeKeyframes(d) {
  const mid = 72 + d.delay;
  const kf = [
    { i: { x: [0.4], y: [1] }, o: { x: [0.6], y: [0] }, n: ['0p4_1_0p6_0'], t: 0, s: [d.opStart], e: [d.opPeak] },
    { i: { x: [0.4], y: [1] }, o: { x: [0.6], y: [0] }, n: ['0p4_1_0p6_0'], t: mid, s: [d.opPeak], e: [d.opStart] },
    { t: 144 }
  ];
  const kfScl = [
    { i: { x: [0.4, 0.4, 0.4], y: [1, 1, 1] }, o: { x: [0.6, 0.6, 0.6], y: [0, 0, 0] }, n: ['0p4_1_0p6_0', '0p4_1_0p6_0', '0p4_1_0p6_0'], t: 0, s: [d.sclStart, d.sclStart, 100], e: [d.sclPeak, d.sclPeak, 100] },
    { i: { x: [0.4, 0.4, 0.4], y: [1, 1, 1] }, o: { x: [0.6, 0.6, 0.6], y: [0, 0, 0] }, n: ['0p4_1_0p6_0', '0p4_1_0p6_0', '0p4_1_0p6_0'], t: mid, s: [d.sclPeak, d.sclPeak, 100], e: [d.sclStart, d.sclStart, 100] },
    { t: 144 }
  ];
  return { kf, kfScl };
}

// ---- Drops: each one forms on a leaf, slides down, fades out, loops ----
// Position + opacity both cycle every 144 frames via loopOut('cycle')
const cycleLoop = "var $bm_rt;\n$bm_rt = loopOut('cycle');";
function makeDropKeyframes(d) {
  const startPos = [d.startX, d.startY, 0];
  const endPos = [d.endX, d.endY, 0];
  // Position: 5 keyframes covering the full 144f cycle
  //   t=0: at start, invisible
  //   t=appear: still at start, now visible
  //   t=peakEnd: at end, still visible
  //   t=fadeEnd: at end, invisible
  //   t=144: snap back to start, invisible (invisible jump = seamless loop)
  const posKf = [
    { i: { x: 0.6, y: 1 }, o: { x: 0.4, y: 0 }, t: 0,        s: startPos, e: startPos },
    { i: { x: 0.6, y: 1 }, o: { x: 0.4, y: 0 }, t: d.appear, s: startPos, e: startPos },
    // ease-in (gravity): slow start, fast end
    { i: { x: 1,   y: 1 }, o: { x: 0.42, y: 0 }, t: d.peakEnd, s: endPos, e: endPos },
    { i: { x: 0.6, y: 1 }, o: { x: 0.4, y: 0 }, t: d.fadeEnd, s: endPos, e: endPos },
    { t: 144, s: startPos, e: startPos }
  ];
  // Opacity: 0 → 100 (fade in) → 100 (visible during slide) → 0 (fade out) → 0
  const opKf = [
    { i: { x: [0.4], y: [1] }, o: { x: [0.6], y: [0] }, n: ['0p4_1_0p6_0'], t: 0,        s: [0],   e: [0] },
    { i: { x: [0.4], y: [1] }, o: { x: [0.6], y: [0] }, n: ['0p4_1_0p6_0'], t: d.appear, s: [100], e: [100] },
    { i: { x: [0.4], y: [1] }, o: { x: [0.6], y: [0] }, n: ['0p4_1_0p6_0'], t: d.peakEnd, s: [100], e: [100] },
    { i: { x: [0.4], y: [1] }, o: { x: [0.6], y: [0] }, n: ['0p4_1_0p6_0'], t: d.fadeEnd, s: [0],   e: [0] },
    { t: 144, s: [0], e: [0] }
  ];
  return { posKf, opKf };
}
drops.forEach(d => {
  const { posKf, opKf } = makeDropKeyframes(d);
  const hlW = d.w * 0.3;
  const hlH = d.h * 0.3;
  const hlOffX = -d.w * 0.2;
  const hlOffY = -d.h * 0.2;

  watered.layers.push({
    ddd: 0,
    ind: d.ind,
    ty: 4,
    nm: d.name,
    sr: 1,
    ks: {
      o: { a: 1, k: opKf,  ix: 11, x: cycleLoop },
      r: { a: 0, k: 0, ix: 10 },
      p: { a: 1, k: posKf, ix: 2,  x: cycleLoop },
      a: { a: 0, k: [0, 0, 0], ix: 1 },
      s: { a: 0, k: [100, 100, 100], ix: 6 }
    },
    ao: 0,
    shapes: [
      {
        ty: 'gr',
        it: [
          { ind: 0, ty: 'el', s: { a: 0, k: [d.w, d.h], ix: 2 }, p: { a: 0, k: [0, 0], ix: 3 }, nm: 'Drop Path', hd: false },
          { ind: 1, ty: 'fl', c: { a: 0, k: dropBlue, ix: 4 }, o: { a: 0, k: 80, ix: 5 }, r: 1, nm: 'Drop Fill', hd: false },
          { ind: 2, ty: 'tr', p: { a: 0, k: [0, 0], ix: 2 }, a: { a: 0, k: [0, 0], ix: 1 }, s: { a: 0, k: [100, 100], ix: 3 }, r: { a: 0, k: 0, ix: 6 }, o: { a: 0, k: 100, ix: 7 }, sk: { a: 0, k: 0, ix: 4 }, sa: { a: 0, k: 0, ix: 5 }, nm: 'Transform' }
        ],
        nm: d.name + ' Body',
        np: 2,
        cix: 2,
        ix: 1,
        mn: 'ADBE Vector Group',
        hd: false
      },
      {
        ty: 'gr',
        it: [
          { ind: 0, ty: 'el', s: { a: 0, k: [hlW, hlH], ix: 2 }, p: { a: 0, k: [hlOffX, hlOffY], ix: 3 }, nm: 'Highlight', hd: false },
          { ind: 1, ty: 'fl', c: { a: 0, k: dropWhite, ix: 4 }, o: { a: 0, k: 70, ix: 5 }, r: 1, nm: 'Highlight Fill', hd: false },
          { ind: 2, ty: 'tr', p: { a: 0, k: [0, 0], ix: 2 }, a: { a: 0, k: [0, 0], ix: 1 }, s: { a: 0, k: [100, 100], ix: 3 }, r: { a: 0, k: 0, ix: 6 }, o: { a: 0, k: 100, ix: 7 }, sk: { a: 0, k: 0, ix: 4 }, sa: { a: 0, k: 0, ix: 5 }, nm: 'Transform' }
        ],
        nm: d.name + ' Highlight',
        np: 2,
        cix: 2,
        ix: 2,
        mn: 'ADBE Vector Group',
        hd: false
      }
    ],
    ip: 0,
    op: 144,
    st: 0,
    bm: 0
  });
});

// ---- Sparkles: 4-pointed star with rotation twinkle ----
function makeStarPath(R, innerRatio) {
  const r = R * innerRatio;
  const k = 1 / Math.SQRT2;
  const verts = [
    [0, -R], [r * k, -r * k], [R, 0], [r * k, r * k],
    [0, R], [-r * k, r * k], [-R, 0], [-r * k, -r * k]
  ];
  return {
    i: verts.map(() => [0, 0]),
    o: verts.map(() => [0, 0]),
    v: verts,
    c: true
  };
}
sparkles.forEach(s => {
  const { kf, kfScl } = makeKeyframes(s);
  const R = s.size;
  // Rotation twinkle: 0 → 180 (flips like a sparkle)
  const rotKf = [
    { i: { x: [0.4], y: [1] }, o: { x: [0.6], y: [0] }, n: ['0p4_1_0p6_0'], t: 0, s: [0], e: [90] },
    { i: { x: [0.4], y: [1] }, o: { x: [0.6], y: [0] }, n: ['0p4_1_0p6_0'], t: 72 + s.delay, s: [90], e: [180] },
    { t: 144 }
  ];

  watered.layers.push({
    ddd: 0,
    ind: s.ind,
    ty: 4,
    nm: s.name,
    sr: 1,
    ks: {
      o: { a: 1, k: kf, ix: 11, x: "var $bm_rt;\n$bm_rt = loopOut('pingpong');" },
      r: { a: 1, k: rotKf, ix: 10, x: "var $bm_rt;\n$bm_rt = loopOut('pingpong');" },
      p: { a: 0, k: [s.x, s.y, 0], ix: 2 },
      a: { a: 0, k: [0, 0, 0], ix: 1 },
      s: { a: 1, k: kfScl, ix: 6, x: "var $bm_rt;\n$bm_rt = loopOut('pingpong');" }
    },
    ao: 0,
    shapes: [
      {
        ty: 'gr',
        it: [
          {
            ind: 0, ty: 'sh', ix: 1,
            ks: { a: 0, k: makeStarPath(R, 0.3), ix: 2 },
            nm: 'Star Path', mn: 'ADBE Vector Shape - Group', hd: false
          },
          { ind: 1, ty: 'fl', c: { a: 0, k: dropWhite, ix: 4 }, o: { a: 0, k: 95, ix: 5 }, r: 1, nm: 'Star Fill', hd: false },
          {
            ind: 2, ty: 'tr',
            p: { a: 0, k: [0, 0], ix: 2 },
            a: { a: 0, k: [0, 0], ix: 1 },
            s: { a: 0, k: [100, 100], ix: 3 },
            r: { a: 0, k: 0, ix: 6 },
            o: { a: 0, k: 100, ix: 7 },
            sk: { a: 0, k: 0, ix: 4 },
            sa: { a: 0, k: 0, ix: 5 },
            nm: 'Transform'
          }
        ],
        nm: s.name,
        np: 2,
        cix: 2,
        ix: 1,
        mn: 'ADBE Vector Group',
        hd: false
      }
    ],
    ip: 0,
    op: 144,
    st: 0,
    bm: 0
  });
});

fs.writeFileSync(WATERED, JSON.stringify(watered));
console.log('✓ Plant_watered.json written');

// Verify
console.log('\n--- Verification ---');
[
  { file: 'Plant_wilt.json', path: WILT },
  { file: 'Plant_watered.json', path: WATERED }
].forEach(({ file, path: p }) => {
  const j = JSON.parse(fs.readFileSync(p, 'utf-8'));
  console.log(`\n${file} (${j.layers.length} layers):`);
  j.layers.forEach(l => {
    const r = l.ks.r;
    const rot = r.a ? (r.k[0].s[0] + '→' + r.k[0].e[0]) : '0';
    const scl = l.ks.s.k.join(',');
    const fills = [];
    if (l.shapes) l.shapes.forEach(g => { if (g.it) g.it.forEach(it => { if (it.ty === 'fl') fills.push('#' + (it.c.k[0]*255|0).toString(16).padStart(2,'0') + (it.c.k[1]*255|0).toString(16).padStart(2,'0') + (it.c.k[2]*255|0).toString(16).padStart(2,'0')); }); });
    console.log(`  ind:${l.ind} ${l.nm.padEnd(15)} rot:${rot.padEnd(10)} scl:${scl.padEnd(15)} fills:[${fills.join(', ')}]`);
  });
});
