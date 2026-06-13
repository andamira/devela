// devela/src/sys/os/browser/web/js/window.js

import { strDecode, strEncodeInto } from "./shared.js";

function evalNow(jsCode) {
  try {
    eval(jsCode);
  } catch (err) {
    console.error("Error evaluating JavaScript:", err);
  }
}

export function makeWindowApi(env) {
  return {
    window_is_closed: () => window.closed,
    window_is_coi: () => window.crossOriginIsolated,
    window_is_secure: () => window.isSecureContext,
    window_is_popup: () => { return window.menubar ? !window.menubar.visible : false; },

    // text
    window_name: (ptr, maxLen) => { return strEncodeInto(env, window.name, ptr, maxLen); },
    window_set_name: (ptr, len) => { window.name = strDecode(env, ptr, len); },

    // CHECK extern
    window_location: (ptr, maxLen) => {
      return strEncodeInto(env, window.location.href, ptr, maxLen);
    },
    // CHECK extern
    window_set_location: (ptr, len) => {
      window.location.href = strDecode(env, ptr, len);
    },

    // state
    window_state: (dataPtr) => {
      const view = new DataView(env.wasm.exports.memory.buffer);
      let off = dataPtr; // in sync with ../window.rs::WebWindowState::__ASSERT_FIELD_OFFSETS
      // window: 16 bytes
      view.setUint32(off, window.innerWidth, true); off += 4;  // inner_size.w
      view.setUint32(off, window.innerHeight, true); off += 4; // inner_size.h
      view.setUint32(off, window.outerWidth, true); off += 4;  // outer_size.w
      view.setUint32(off, window.outerHeight, true); off += 4; // outer_size.h
      // screen: 24 bytes
      view.setInt32(off, window.screenLeft, true); off += 4;          // screen_offset.x
      view.setInt32(off, window.screenTop, true); off += 4;           // screen_offset.y
      view.setUint32(off, window.screen.width, true); off += 4;       // screen_size.w
      view.setUint32(off, window.screen.height, true); off += 4;      // screen_size.w
      view.setUint32(off, window.screen.availWidth, true); off += 4;  // *_usable_size.w
      view.setUint32(off, window.screen.availHeight, true); off += 4; // *_usable_size.w
      // misc: 5 bytes + 3 bytes explicit Rust padding
      view.setFloat32(off, Math.fround(window.devicePixelRatio), true); off += 4;
      view.setUint8(off, window.screen.colorDepth); off += 1;
      // padding is intentionally left untouched.
    },

    // timeout
    window_set_timeout: (callbackPtr, delayMs) => {
      return setTimeout(() => { env.wasm.exports.wasm_callback(callbackPtr); }, delayMs);
    },
    window_set_interval: (callbackPtr, intervalMs) => {
      return setInterval(() => { env.wasm.exports.wasm_callback(callbackPtr); }, intervalMs);
    },
    window_clear_timeout: (timeoutId) => { clearTimeout(timeoutId); },

    // eval
    window_eval: (jsCodePtr, jsCodeLen) => {
      evalNow(strDecode(env, jsCodePtr, jsCodeLen));
    },
    window_eval_timeout: (jsCodePtr, jsCodeLen, delayMs) => {
      const jsCode = strDecode(env, jsCodePtr, jsCodeLen);
      return setTimeout(() => evalNow(jsCode), delayMs);
    },
    window_eval_interval: (jsCodePtr, jsCodeLen, intervalMs) => {
      const jsCode = strDecode(env, jsCodePtr, jsCodeLen);
      return setInterval(() => evalNow(jsCode), intervalMs);
    },

    // animation
    window_request_animation_frame: (callbackPtr) => {
      return requestAnimationFrame(() => { env.wasm.exports.wasm_callback(callbackPtr); });
    },
    window_cancel_animation_frame: (requestId) => {
      cancelAnimationFrame(requestId);
    },
  };
}
