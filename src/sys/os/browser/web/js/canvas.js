// devela/src/sys/os/browser/web/js/canvas.js
// (in sync with ../api/canvas.rs)

import { strDecode, requireCtx } from "./shared.js";

// Sets the active canvas.
function setCanvas(env, selector) {
  const canvas = document.querySelector(selector);
  if (!canvas) { console.error(`Canvas with selector "${selector}" not found`); return; }
  const ctx = canvas.getContext("2d");
  if (!ctx) { console.error(`Failed to get 2D context for canvas "${selector}"`); return; }
  env.canvas = canvas;
  env.ctx = ctx;
}

export function makeCanvasApi(env) {
  return {
    /* misc. */
    set_canvas: (ptr, len) => { setCanvas(env, strDecode(env, ptr, len)); },

    /* color settings */
    fillStyle: (r, g, b) => {
      const ctx = requireCtx(env);
      if (ctx) ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
    },
    strokeStyle: (r, g, b) => {
      const ctx = requireCtx(env);
      if (ctx) ctx.strokeStyle = `rgb(${r}, ${g}, ${b})`;
    },

    /* drawing rectangles */
    fillRect: (x, y, w, h) => {
      const ctx = requireCtx(env);
      if (ctx) ctx.fillRect(x, y, w, h);
    },
    strokeRect: (x, y, w, h) => {
      const ctx = requireCtx(env);
      if (ctx) ctx.strokeRect(x, y, w, h);
    },
    clearRect: (x, y, w, h) => {
      const ctx = requireCtx(env);
      if (ctx) ctx.clearRect(x, y, w, h);
    },

    /* drawing paths */

    /* drawing shapes */
    draw_line: (x1, y1, x2, y2) => {
      const ctx = requireCtx(env);
      if (!ctx) return;
      ctx.beginPath();
      ctx.moveTo(x1, y1);
      ctx.lineTo(x2, y2);
      ctx.stroke();
    },
    draw_circle: (x, y, r) => {
      const ctx = requireCtx(env);
      if (!ctx) return;
      ctx.beginPath();
      ctx.arc(x, y, r, 0, Math.PI * 2);
      ctx.fill();
    },

    /* drawing text */
    fillText: (ptr, len, x, y) => {
      const ctx = requireCtx(env);
      if (ctx) ctx.fillText(strDecode(env, ptr, len), x, y);
    },
    strokeText: (ptr, len, x, y) => {
      const ctx = requireCtx(env);
      if (ctx) ctx.strokeText(strDecode(env, ptr, len), x, y);
    },
    measureText: (textPtr, textLen, outMetricsPtr) => {
      const ctx = requireCtx(env);
      if (!ctx) return;
      const text = strDecode(env, textPtr, textLen);
      const metrics = ctx.measureText(text);
      const out = new Float32Array(env.wasm.exports.memory.buffer, outMetricsPtr, 3);
      out[0] = metrics.width;
      out[1] = metrics.actualBoundingBoxAscent;
      out[2] = metrics.actualBoundingBoxDescent;
    },
    measureTextFull: (textPtr, textLen, outMetricsPtr) => {
      const ctx = requireCtx(env);
      if (!ctx) return;
      const text = strDecode(env, textPtr, textLen);
      const metrics = ctx.measureText(text);
      const out = new Float32Array(env.wasm.exports.memory.buffer, outMetricsPtr, 12);
      out[0] = metrics.width;
      out[1] = metrics.actualBoundingBoxLeft;
      out[2] = metrics.actualBoundingBoxRight;
      out[3] = metrics.actualBoundingBoxAscent;
      out[4] = metrics.actualBoundingBoxDescent;
      out[5] = metrics.fontBoundingBoxAscent ?? 0.0;
      out[6] = metrics.fontBoundingBoxDescent ?? 0.0;
      out[7] = metrics.emHeightAscent ?? 0.0;
      out[8] = metrics.emHeightDescent ?? 0.0;
      out[9] = metrics.hangingBaseline ?? 0.0;
      out[10] = metrics.alphabeticBaseline ?? 0.0;
      out[11] = metrics.ideographicBaseline ?? 0.0;
    },
  };
}
