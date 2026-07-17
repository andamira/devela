// devela/src/sys/os/browser/web/bridge/js/document.js

import { strEncodeInto } from "./shared.js";

export function makeDocumentApi(env) {
  return {
    // flags
    document_is_compat_mode: () => { return document.compatMode === "CSS1Compat"; },
    document_is_hidden: () => { return document.hidden; },
    // text
    document_content_type: (ptr, maxLen) => {
      return strEncodeInto(env, document.contentType, ptr, maxLen);
    },
    // document_create_element: (ptr, len) => {
    //   return document.createElement(strDecode(env, ptr, len));
    // },
  };
}
