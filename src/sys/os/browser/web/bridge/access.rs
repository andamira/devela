// devela/src/sys/os/browser/web/bridge/access.rs
// In sync with js/permissions.js
//
//! Implements the web permissions API.
//

use crate::{_js_doc, _js_extern, Web, WebPermission, WebPermissionState, js_int32};

/// # Web API permissions
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API>
#[rustfmt::skip]
impl Web {
    #[doc = _js_doc!("Permissions", "query")]
    /// Queries the status of a given permission.
    ///
    /// Returns `Granted`, `Denied`, `Prompt`, or `Unknown` if unsupported.
    pub fn permissions_query(permission: WebPermission) -> WebPermissionState {
        unsafe { permissions_query(permission.as_str().as_ptr(), permission.as_str().len()) }
        .into()
    }
}
_js_extern! {
    [ module: "api_permissions" ]
    unsafe fn permissions_query(name_ptr: *const u8, name_len: usize) -> js_int32;
}
