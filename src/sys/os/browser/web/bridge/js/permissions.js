// devela/src/sys/os/browser/web/bridge/js/permissions.js

import { strDecode } from "./shared.js";

// In sync with ../access/permission.rs::WebPermissionState
const PERMISSION_GRANTED = 1;
const PERMISSION_PROMPT = 0;
const PERMISSION_DENIED = -1;
const PERMISSION_UNKNOWN = -2;
const PERMISSION_ERROR = -3;

function permissionStateCode(state) {
  switch (state) {
    case "granted": return PERMISSION_GRANTED;
    case "prompt":  return PERMISSION_PROMPT;
    case "denied":  return PERMISSION_DENIED;
    default:        return PERMISSION_UNKNOWN;
  }
}

export function makePermissionsApi(env) {
  const cache = new Map();   // permission name -> state code
  const pending = new Set(); // permission name

  return {
    permissions_query: (namePtr, nameLen) => {
      const name = strDecode(env, namePtr, nameLen);
      if (!navigator.permissions || !navigator.permissions.query) { return PERMISSION_UNKNOWN; }
      if (cache.has(name)) { return cache.get(name); }
      if (!pending.has(name)) {
        pending.add(name);
        navigator.permissions.query({ name })
          .then((result) => {
            cache.set(name, permissionStateCode(result.state));
            pending.delete(name);
            result.onchange = () => { cache.set(name, permissionStateCode(result.state)); };
          })
          .catch(() => { cache.set(name, PERMISSION_ERROR); pending.delete(name); });
      }
      return PERMISSION_UNKNOWN;
    },
  };
}
