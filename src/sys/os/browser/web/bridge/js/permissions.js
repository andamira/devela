// devela/src/sys/os/browser/web/bridge/js/permissions.js

import { strDecode } from "./shared.js";

// In sync with ../access.rs permission_query_from_js
const PERMISSION_GRANTED = 1;
const PERMISSION_PROMPT = 0;
const PERMISSION_DENIED = -1;
const PERMISSION_PENDING = -2;
const PERMISSION_UNSUPPORTED = -3;
const PERMISSION_FAILED = -4;
const PERMISSION_UNQUERIED   = -5;

function permissionStateCode(state) {
  switch (state) {
    case "granted": return PERMISSION_GRANTED;
    case "prompt":  return PERMISSION_PROMPT;
    case "denied":  return PERMISSION_DENIED;
    default:        return PERMISSION_FAILED;
  }
}

function permissionErrorCode(error) {
  return error instanceof TypeError || error?.name === "TypeError"
    ? PERMISSION_UNSUPPORTED
    : PERMISSION_FAILED;
}

export function makePermissionsApi(env) {
  const cache = new Map();   // permission name -> resolved code
  const pending = new Set(); // permission names being queried

  function cachedPermissionCode(name) {
    if (cache.has(name)) return cache.get(name);
    if (pending.has(name)) return PERMISSION_PENDING;
    return PERMISSION_UNQUERIED;
  }
  function startPermissionQuery(name) {
    if (cache.has(name) || pending.has(name)) return;
    pending.add(name);
    let query;
    try {
      query = navigator.permissions.query({ name });
    } catch (error) {
      cache.set(name, permissionErrorCode(error));
      pending.delete(name);
      return;
    }
    query
      .then((status) => {
        const update = () => { cache.set(name, permissionStateCode(status.state)); };
        status.addEventListener("change", update);
        update();
        pending.delete(name);
      })
      .catch((error) => {
        cache.set(name, permissionErrorCode(error));
        pending.delete(name);
      });
  }
  return {
    permissions_cached: (namePtr, nameLen) => {
      const name = strDecode(env, namePtr, nameLen);
      return cachedPermissionCode(name);
    },
    permissions_query: (namePtr, nameLen) => {
      const name = strDecode(env, namePtr, nameLen);
      const cached = cachedPermissionCode(name);
      if (cached !== PERMISSION_UNQUERIED) return cached;
      if (!navigator.permissions?.query) { return PERMISSION_UNSUPPORTED; }
      startPermissionQuery(name);
      return cachedPermissionCode(name);
    },
  };
}
