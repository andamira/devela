// devela/src/sys/os/browser/web/js/api.js

import { makeEnv } from "./shared.js";

import { makeJsApi } from "./js.js";

import { makeCanvasApi } from "./canvas.js";
import { makeDocumentApi } from "./document.js";
// import { makeElementApi } from "./element.js";
import { makeEventsApi } from "./events.js";
import { makeHistoryApi } from "./history.js";
import { makePermissionsApi } from "./permissions.js";
import { makePerformanceApi } from "./performance.js";
import { makeWindowApi } from "./window.js";
import { makeWorkersApi } from "./workers.js";

export async function initWasm(wasmPath, imports = {}) {
  const env = makeEnv();

  const jsApi = makeJsApi(env);
  const webApi = {
    api_canvas: makeCanvasApi(env),
    api_document: makeDocumentApi(env),
    // api_element: makeElementApi(env),
    api_events: makeEventsApi(env),
    api_history: makeHistoryApi(env),
    api_permissions: makePermissionsApi(env),
    api_performance: makePerformanceApi(env),
    api_window: makeWindowApi(env),
    api_workers: makeWorkersApi(env),
  };

  // TEMP legacy globals while migrating callbacks / worker helpers.
  window.api_events = webApi.api_events;
  window.api_workers = webApi.api_workers;

  const finalImports = {
    ...jsApi,
    ...webApi,
    ...imports,
  };

  try {
    const response = await fetch(wasmPath);
    if (!response.ok) throw new Error(`Failed to load WASM: ${response.statusText}`);

    const wasmModule = await WebAssembly.instantiateStreaming(response, finalImports);
    env.wasm = wasmModule.instance;

    wasmModule.instance.exports.main();

    return wasmModule.instance;
  } catch (error) { console.error("WASM loading failed:", error); return null; }
}
