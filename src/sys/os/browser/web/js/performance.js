// devela/src/sys/os/browser/web/js/performance.js

import { strDecode } from "./shared.js";

export function makePerformanceApi(env) {
  return {
    now: () => performance.now(),
    timeOrigin: () => performance.timeOrigin,
    eventCounts: (eventPtr, eventLen) => {
      const eventName = strDecode(env, eventPtr, eventLen);
      return performance.eventCounts?.get(eventName) ?? 0;
    },
  };
}
