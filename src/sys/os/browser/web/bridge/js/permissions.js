// devela/src/sys/os/browser/web/bridge/js/permissions.js

import { strDecode } from "./shared.js";

// In sync with ../access.rs permission_query_from_js
const PERMISSION_GRANTED = 1;
const PERMISSION_PROMPT = 0;
const PERMISSION_DENIED = -1;
const PERMISSION_PENDING = -2;
const PERMISSION_UNSUPPORTED = -3;
const PERMISSION_FAILED = -4;

function permissionStateCode(state) {
  switch (state) {
    case "granted": return PERMISSION_GRANTED;
    case "prompt":  return PERMISSION_PROMPT;
    case "denied":  return PERMISSION_DENIED;
    default:        return PERMISSION_FAILED;
  }
}

export function makePermissionsApi(env) {
  const cache = new Map();   // permission name -> state code
  const pending = new Set(); // permission name
  return {
    permissions_query: (namePtr, nameLen) => {
      const name = strDecode(env, namePtr, nameLen);
      if (!navigator.permissions?.query) { return PERMISSION_UNSUPPORTED; }
      if (cache.has(name)) { return cache.get(name); }
      if (!pending.has(name)) {
        pending.add(name);
        navigator.permissions.query({ name })
          .then((status) => {
            cache.set(name, permissionStateCode(status.state));
            pending.delete(name);
            status.onchange = () => { cache.set(name, permissionStateCode(status.state)); };
          })
          .catch((error) => {
            cache.set(name, error instanceof TypeError ? PERMISSION_UNSUPPORTED : PERMISSION_FAILED);
            pending.delete(name);
          });
      }
      return PERMISSION_PENDING;
    },
  };
}
