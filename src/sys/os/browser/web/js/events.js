// devela/src/sys/os/browser/web/js/events.js
// In sync with ../api/events.rs

import { strDecode, queryElement } from "./shared.js";

/* helpers */

// Maps DOM event names to `WebEventKind` values, matching Rust's repr.
function getEventKind(eventName) {
  const eventMap = {
    "click": 1,
    "keydown": 2,
    "keyup": 3,
    "mousedown": 4,
    "mouseup": 5,
    "mousemove": 6,
    "pointerdown": 7,
    "pointerup": 8,
    "pointermove": 9,
    "wheel": 10,
    // 11 reserved / unused
    "resize": 12,
  };
  return eventMap[eventName] ?? 0; // 0 = unknown / none
}

// Returns KeyMods web bitmask for a DOM input event.
function eventMods(e) {
  let mods = 0;
  if (e.ctrlKey)  mods |= 1 << 0;
  if (e.shiftKey) mods |= 1 << 1;
  if (e.altKey)   mods |= 1 << 2;
  if (e.metaKey)  mods |= 1 << 3;
  if (e.getModifierState) {
    if (e.getModifierState("AltGraph"))   mods |= 1 << 4;
    if (e.getModifierState("CapsLock"))   mods |= 1 << 5;
    if (e.getModifierState("NumLock"))    mods |= 1 << 6;
    if (e.getModifierState("ScrollLock")) mods |= 1 << 7;
  }
  return mods;
}

// Returns a Unicode scalar for single-scalar KeyboardEvent.key values.
// Returns 0 for non-text keys such as "Enter", "Shift", "ArrowLeft".
function eventKeyScalar(e) {
  if (!e.key) return 0;
  const scalars = Array.from(e.key);
  if (scalars.length !== 1) return 0;
  return scalars[0].codePointAt(0) || 0;
}

// Returns EventPointerKind web code.
function eventPointerKind(e) {
  switch (e.pointerType) {
    case "touch": return 1;
    case "pen":   return 2;
    default:      return 0;
  }
}

function listenerKey(selector, event, id) {
  return `${selector}\0${event}\0${id}`;
}

function decodeTargetAndEvent(env, ePtr, eLen, eventPtr, eventLen) {
  const selector = strDecode(env, ePtr, eLen);
  const event = strDecode(env, eventPtr, eventLen);
  const element = queryElement(selector);
  return { selector, event, element };
}

const WEB_KEY_UNKNOWN = 0x00000000;
const WEB_KEY_CHAR    = 0x01000000;
const WEB_KEY_NAMED   = 0x02000000;
const WEB_KEY_FN      = 0x03000000;
const WEB_KEY_PAD     = 0x04000000;
const WEB_KEY_MEDIA   = 0x05000000;
const WEB_KEY_MOD     = 0x06000000;

const namedKey = (id) => WEB_KEY_NAMED | id;
const fnKey = (n) => WEB_KEY_FN | n;
const padKey = (id) => WEB_KEY_PAD | id;
const mediaKey = (id) => WEB_KEY_MEDIA | id;
const modKey = (id) => WEB_KEY_MOD | id;

function charKey(s) {
  if (!s) return WEB_KEY_UNKNOWN;
  const a = s.charCodeAt(0);
  // Single UTF-16 code unit, excluding lone surrogate.
  if (s.length === 1) {
    if (a >= 0xD800 && a <= 0xDFFF) return WEB_KEY_UNKNOWN;
    return WEB_KEY_CHAR | a;
  }
  // One surrogate pair.
  if (s.length === 2) {
    const b = s.charCodeAt(1);
    if (a >= 0xD800 && a <= 0xDBFF && b >= 0xDC00 && b <= 0xDFFF) {
      const scalar = 0x10000 + ((a - 0xD800) << 10) + (b - 0xDC00);
      return WEB_KEY_CHAR | scalar;
    }
  }
  return WEB_KEY_UNKNOWN;
}

function webKeyFromKey(key) {
  switch (key) {
    case "Backspace": return namedKey(1); //
    case "Enter": return namedKey(2);
    case "Tab": return namedKey(3);
    case "Escape": return namedKey(4);
    case " ": return namedKey(5);
    case "ArrowLeft": return namedKey(10); //
    case "ArrowRight": return namedKey(11);
    case "ArrowUp": return namedKey(12);
    case "ArrowDown": return namedKey(13);
    case "Home": return namedKey(20); //
    case "End": return namedKey(21);
    case "PageUp": return namedKey(22);
    case "PageDown": return namedKey(23);
    case "Delete": return namedKey(30); //
    case "Insert": return namedKey(31);
    case "CapsLock": return namedKey(40); //
    case "ScrollLock": return namedKey(41);
    case "NumLock": return namedKey(42);
    case "PrintScreen": return namedKey(50); //
    case "Pause": return namedKey(51);
    case "ContextMenu": return namedKey(52);
    case "Shift": return modKey(0); //
    case "Control": return modKey(1);
    case "Alt": return modKey(2);
    case "Meta": return modKey(3);
    case "AltGraph": return modKey(4);
    case "Level5Shift": return modKey(5);
    case "Play": return mediaKey(0); //
    case "Pause": return mediaKey(1); // ambiguous with Key::Pause in DOM practice
    case "MediaPlayPause": return mediaKey(2);
    case "MediaReverse": return mediaKey(3);
    case "Stop": return mediaKey(4);
    case "FastForward": return mediaKey(5);
    case "Rewind": return mediaKey(6);
    case "NextTrack": return mediaKey(7);
    case "PreviousTrack": return mediaKey(8);
    case "Record": return mediaKey(9);
    case "VolumeDown": return mediaKey(10); //
    case "VolumeUp": return mediaKey(11);
    case "VolumeMute": return mediaKey(12);
    case "Eject": return mediaKey(20); //
    case "MediaSelect": return mediaKey(21);
    case "LaunchMedia": return mediaKey(22);
    case "Bass Boost": return mediaKey(23);
    case "Bass Up": return mediaKey(24);
    case "Bass Down": return mediaKey(25);
    case "Treble Up": return mediaKey(26);
    case "Treble Down": return mediaKey(27);
    case "MicMute": return mediaKey(28);
    case "MicVolumeUp": return mediaKey(29);
    case "MicVolumeDown": return mediaKey(30); //
    case "BrightnessUp": return mediaKey(31);
    case "BrightnessDown": return mediaKey(32);
    case "Sleep": return mediaKey(33);
    case "Wake": return mediaKey(34);
    case "Power": return mediaKey(35);
  }
  if (/^F([1-9]|[1-3][0-9]|4[0-8])$/.test(key)) {
    return fnKey(Number(key.slice(1)));
  }
  if (/^[A-Z]$/.test(key)) return namedKey(100 + key.charCodeAt(0) - 65);
  if (/^[0-9]$/.test(key)) return namedKey(200 + Number(key));
  switch (key) {
    case "*": return padKey(10);
    case "+": return padKey(11);
    case "-": return padKey(12);
    case "/": return padKey(13);
    case ".": return padKey(14);
    case "=": return padKey(16);
    case ",": return padKey(17);
  }
  return charKey(key);
}

function webKeyFromCode(code) {
  switch (code) {
    case "Backspace": return namedKey(1); //
    case "Enter": return namedKey(2);
    case "Tab": return namedKey(3);
    case "Escape": return namedKey(4);
    case "Space": return namedKey(5);
    case "ArrowLeft": return namedKey(10); //
    case "ArrowRight": return namedKey(11);
    case "ArrowUp": return namedKey(12);
    case "ArrowDown": return namedKey(13);
    case "Home": return namedKey(20); //
    case "End": return namedKey(21);
    case "PageUp": return namedKey(22);
    case "PageDown": return namedKey(23);
    case "Delete": return namedKey(30); //
    case "Insert": return namedKey(31);
    case "CapsLock": return namedKey(40); //
    case "ScrollLock": return namedKey(41);
    case "NumLock": return namedKey(42);
    case "PrintScreen": return namedKey(50); //
    case "Pause": return namedKey(51);
    case "ContextMenu": return namedKey(52);
    case "ShiftLeft": //
    case "ShiftRight": return modKey(0);
    case "ControlLeft":
    case "ControlRight": return modKey(1);
    case "AltLeft":
    case "AltRight": return modKey(2);
    case "MetaLeft":
    case "MetaRight": return modKey(3);
    case "AltGraph": return modKey(4);
    case "Level5Shift": return modKey(5);
    case "NumpadMultiply": return padKey(10); //
    case "NumpadAdd": return padKey(11);
    case "NumpadSubtract": return padKey(12);
    case "NumpadDivide": return padKey(13);
    case "NumpadDecimal": return padKey(14);
    case "NumpadEnter": return padKey(15);
    case "NumpadEqual": return padKey(16);
    case "NumpadComma": return padKey(17);
    case "MediaPlay": return mediaKey(0); //
    case "MediaPause": return mediaKey(1);
    case "MediaPlayPause": return mediaKey(2);
    case "MediaReverse": return mediaKey(3);
    case "MediaStop": return mediaKey(4);
    case "MediaFastForward": return mediaKey(5);
    case "MediaRewind": return mediaKey(6);
    case "MediaTrackNext": return mediaKey(7);
    case "MediaTrackPrevious": return mediaKey(8);
    case "MediaRecord": return mediaKey(9);
    case "AudioVolumeDown": return mediaKey(10); //
    case "AudioVolumeUp": return mediaKey(11);
    case "AudioVolumeMute": return mediaKey(12);
    case "Eject": return mediaKey(20); //
    case "MediaSelect": return mediaKey(21);
    case "LaunchMedia": return mediaKey(22);
    case "BassBoost": return mediaKey(23);
    case "BassUp": return mediaKey(24);
    case "BassDown": return mediaKey(25);
    case "TrebleUp": return mediaKey(26);
    case "TrebleDown": return mediaKey(27);
    case "MicrophoneMute": return mediaKey(28);
    case "MicrophoneVolumeUp": return mediaKey(29);
    case "MicrophoneVolumeDown": return mediaKey(30); //
    case "BrightnessUp": return mediaKey(31);
    case "BrightnessDown": return mediaKey(32);
    case "Sleep": return mediaKey(33);
    case "Wake": return mediaKey(34);
    case "Power": return mediaKey(35);
  }
  if (/^Key[A-Z]$/.test(code)) { return namedKey(100 + code.charCodeAt(3) - 65); }
  if (/^Digit[0-9]$/.test(code)) { return namedKey(200 + Number(code.slice(5))); }
  if (/^F([1-9]|[1-3][0-9]|4[0-8])$/.test(code)) { return fnKey(Number(code.slice(1))); }
  if (/^Numpad[0-9]$/.test(code)) { return padKey(Number(code.slice(6))); }
  return WEB_KEY_UNKNOWN;
}

/* API */

export function makeEventsApi(env) {
  const callbacks = new Map();
  const callbacksJs = new Map();

  return {
    // Generic event listener registration
    event_addListener: (ePtr, eLen, eventPtr, eventLen, callbackPtr) => {
      const { selector, event, element } =
        decodeTargetAndEvent(env, ePtr, eLen, eventPtr, eventLen);
      if (!element) {
        console.error(`Element not found for event '${event}' on '${selector}'`);
        return;
      }
      const callback = () => env.wasm.exports.wasm_callback(callbackPtr);
      const key = listenerKey(selector, event, callbackPtr);
      callbacks.set(key, callback);
      element.addEventListener(event, callback);
    },
    //
    event_removeListener: (ePtr, eLen, eventPtr, eventLen, callbackPtr) => {
      const { selector, event, element } =
        decodeTargetAndEvent(env, ePtr, eLen, eventPtr, eventLen);
      const key = listenerKey(selector, event, callbackPtr);
      const callback = callbacks.get(key);
      if (!element || !callback) return;
      element.removeEventListener(event, callback);
      callbacks.delete(key);
    },
    //
    event_addListenerJs: (ePtr, eLen, eventPtr, eventLen, jsFnPtr, jsFnLen) => {
      const { selector, event, element } =
        decodeTargetAndEvent(env, ePtr, eLen, eventPtr, eventLen);
      const jsFnName = strDecode(env, jsFnPtr, jsFnLen);
      if (!element) {
        console.error(`Element not found for event '${event}' on '${selector}'`);
        return;
      }
      if (typeof window[jsFnName] !== "function") {
        console.error(`JS function '${jsFnName}' not found.`);
        return;
      }
      const callback = window[jsFnName];
      const key = listenerKey(selector, event, jsFnName);
      callbacksJs.set(key, callback);
      element.addEventListener(event, callback);
    },
    //
    event_removeListenerJs: (ePtr, eLen, eventPtr, eventLen, jsFnPtr, jsFnLen) => {
      const { selector, event, element } =
        decodeTargetAndEvent(env, ePtr, eLen, eventPtr, eventLen);
      const jsFnName = strDecode(env, jsFnPtr, jsFnLen);
      const key = listenerKey(selector, event, jsFnName);
      const callback = callbacksJs.get(key);
      if (!element) {
        console.error(`Element not found for event '${event}' on '${selector}'`);
        return;
      }
      if (!callback) {
        console.error(`No JS listener '${jsFnName}' found for '${event}'.`);
        return;
      }
      element.removeEventListener(event, callback);
      callbacksJs.delete(key);
    },
    //
    event_addListenerKey: (ePtr, eLen, eventPtr, eventLen, callbackPtr) => {
      const { selector, event, element } = decodeTargetAndEvent(env, ePtr, eLen, eventPtr, eventLen);
      if (!element) {
        console.error(`Element not found for key listener on '${selector}'`);
        return;
      }
      const callback = (e) => {
        env.wasm.exports.wasm_callback_key(
          callbackPtr,
          webKeyFromKey(e.key),
          webKeyFromCode(e.code),
          e.location | 0,
          e.repeat ? 1 : 0,
          eventMods(e),
          getEventKind(e.type),
          e.timeStamp,
        );
      };
      const key = listenerKey(selector, event, callbackPtr);
      callbacks.set(key, callback);
      element.addEventListener(event, callback);
    },
    //
    event_addListenerMouse: (ePtr, eLen, eventPtr, eventLen, callbackPtr) => {
      const { selector, event, element } =
        decodeTargetAndEvent(env, ePtr, eLen, eventPtr, eventLen);
      if (!element) {
        console.error(`Element not found for mouse listener on '${selector}'`);
        return;
      }
      const callback = (e) => {
        const button = e.type === "mousemove" ? -1 : e.button; // -1 (255) for no clicks
        const buttons = e.buttons;
        const mods = eventMods(e);
        const etype = getEventKind(e.type);
        const timeStamp = e.timeStamp;
        env.wasm.exports.wasm_callback_mouse(
          callbackPtr,
          e.clientX,
          e.clientY,
          button,
          buttons,
          mods,
          etype,
          timeStamp,
        );
      };
      const key = listenerKey(selector, event, callbackPtr);
      callbacks.set(key, callback);
      element.addEventListener(event, callback);
    },
    //
    event_addListenerPointer: (ePtr, eLen, eventPtr, eventLen, callbackPtr) => {
      const { selector, event, element } =
        decodeTargetAndEvent(env, ePtr, eLen, eventPtr, eventLen);
      if (!element) {
        console.error(`Element not found for pointer listener on '${selector}'`);
        return;
      }
      const callback = (e) => {
        const button = e.type === "pointermove" ? -1 : e.button; // -1 (255) for no clicks
        const buttons = e.buttons;
        const mods = eventMods(e);
        const kind = eventPointerKind(e);
        const etype = getEventKind(e.type);
        const timeStamp = e.timeStamp;
        env.wasm.exports.wasm_callback_pointer(
          callbackPtr,
          e.pointerId,
          kind,
          e.clientX,
          e.clientY,
          e.pressure,
          e.tiltX,
          e.tiltY,
          e.twist || 0,
          button,
          buttons,
          mods,
          etype,
          timeStamp,
        );
      };
      const key = listenerKey(selector, event, callbackPtr);
      callbacks.set(key, callback);
      element.addEventListener(event, callback);
    },
    //
    event_addListenerWheel: (ePtr, eLen, eventPtr, eventLen, callbackPtr) => {
      const { selector, event, element } =
        decodeTargetAndEvent(env, ePtr, eLen, eventPtr, eventLen);
      if (!element) {
        console.error(`Element not found for wheel listener on '${selector}'`);
        return;
      }
      const callback = (e) => {
        const buttons = e.buttons;
        const mods = eventMods(e);
        const timeStamp = e.timeStamp;
        env.wasm.exports.wasm_callback_wheel(
          callbackPtr,
          e.clientX,
          e.clientY,
          e.deltaX,
          e.deltaY,
          buttons,
          mods,
          e.deltaMode,
          timeStamp,
        );
      };
      const key = listenerKey(selector, event, callbackPtr);
      callbacks.set(key, callback);
      element.addEventListener(event, callback);
    },
  };
}
