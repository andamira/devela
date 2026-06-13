// devela/src/sys/os/browser/web/js/shared.js
//
// This should hold helpers that need wasm.memory, decoding, writing memory,
// null checks, maybe selector helpers.

export function makeEnv() {
  return {
    wasm: null,
    canvas: null,
    ctx: null,
    textDecoder: new TextDecoder("utf-8"),
    textEncoder: new TextEncoder(),
  };
}

// Decodes a UTF-8 string from WASM memory starting at `ptr`, with `len` bytes. (rust → js)
export function strDecode(env, ptr, len) {
  const src = new Uint8Array(env.wasm.exports.memory.buffer, ptr, len);
  return env.textDecoder.decode(src);
}

// Encodes a JS `string` into UTF-8 bytes. js → rust
export function strEncode(env, string) {
  return env.textEncoder.encode(String(string));
}

// Encodes a JS `string` into UTF-8 bytes in WASM memory at `ptr`. js → rust
//
// - Writes up to `maxLen` bytes.
// - Returns the number of bytes written.
// - If the buffer is too small, returns the negated required length.
export function strEncodeInto(env, string, ptr, maxLen) {
  const encoded = strEncode(env, string);
  const required = encoded.length | 0;

  if (maxLen < required) return (-required) | 0;

  const dst = new Uint8Array(env.wasm.exports.memory.buffer, ptr, required);
  dst.set(encoded);

  return required;
}

export function queryElement(selector) {
  if (selector === "window") return window;
  if (selector === "document") return document;
  return document.querySelector(selector);
}

export function requireCtx(env) {
  if (!env.ctx) { console.error("No active canvas context. Call set_canvas first."); return null; }
  return env.ctx;
}
