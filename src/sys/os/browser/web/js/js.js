// devela/src/sys/os/browser/web/js/js.js

import { strDecode } from "./shared.js";

export function makeJsApi(env) {
  return {
    api_console: {
      console_clear: () => console.clear(),

      console_debug: (ptr, len) => console.debug(strDecode(env, ptr, len)),
      console_error: (ptr, len) => console.error(strDecode(env, ptr, len)),
      console_info:  (ptr, len) => console.info(strDecode(env, ptr, len)),
      console_log:   (ptr, len) => console.log(strDecode(env, ptr, len)),
      console_warn:  (ptr, len) => console.warn(strDecode(env, ptr, len)),
      console_trace: () => console.trace(),

      console_count:       (ptr, len) => console.count(strDecode(env, ptr, len)),
      console_count_reset: (ptr, len) => console.countReset(strDecode(env, ptr, len)),

      console_group:           (ptr, len) => console.group(strDecode(env, ptr, len)),
      console_group_collapsed: (ptr, len) => console.groupCollapsed(strDecode(env, ptr, len)),
      console_group_end:       () => console.groupEnd(),

      console_time:     (ptr, len) => console.time(strDecode(env, ptr, len)),
      console_time_end: (ptr, len) => console.timeEnd(strDecode(env, ptr, len)),
      console_time_log: (ptr, len) => console.timeLog(strDecode(env, ptr, len)),
    },

    api_object: {
      // TODO
    },
  };
}
