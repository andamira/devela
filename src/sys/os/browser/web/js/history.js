// devela/src/sys/os/browser/web/js/history.js
// (in sync with ../api/history.rs)

import { strDecode } from "./shared.js";

export function makeHistoryApi(env) {
  return {
    history_back: () => history.back(),
    history_forward: () => history.forward(),
    history_go: delta => history.go(delta),
    history_pushState: (statePtr, stateLen, titlePtr, titleLen, urlPtr, urlLen) => {
      history.pushState(
        strDecode(env, statePtr, stateLen),
        strDecode(env, titlePtr, titleLen),
        strDecode(env, urlPtr, urlLen),
      );
    },
    history_replaceState: (statePtr, stateLen, titlePtr, titleLen, urlPtr, urlLen) => {
      history.replaceState(
        strDecode(env, statePtr, stateLen),
        strDecode(env, titlePtr, titleLen),
        strDecode(env, urlPtr, urlLen),
      );
    },
    location_reload: () => location.reload(),
    location_assign: (urlPtr, urlLen) => location.assign(strDecode(env, urlPtr, urlLen)),
    location_replace: (urlPtr, urlLen) => location.replace(strDecode(env, urlPtr, urlLen)),
  };
}
