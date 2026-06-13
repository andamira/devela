// devela/src/sys/os/browser/web/js/events.js
// (in sync with ../api/events.rs)

import { strDecode, queryElement } from "./shared.js";

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
