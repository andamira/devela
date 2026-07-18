// devela/src/sys/os/browser/web/bridge/access.rs
// In sync with js/permissions.js
//
//! Implements the web permissions API.
//

use crate::{_js_extern, js_int32};
use crate::{AsyncPoll, PermissionError, PermissionQuery, PermissionState};
use crate::{Web, WebPermission};

/// # Web API permissions
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API>
impl Web {
    /// Starts or polls a non-blocking query for a browser permission.
    ///
    /// Repeated calls for the same permission observe the query cached by the JavaScript bridge.
    pub fn permissions_query(permission: WebPermission) -> PermissionQuery {
        let name = permission.as_str();
        let code = unsafe { permissions_query(name.as_ptr(), name.len()) };
        permission_query_from_js(code).unwrap_or(AsyncPoll::Ready(Err(PermissionError::Failed)))
    }
    /// Returns the cached query for a browser permission without starting one.
    pub fn permissions_cached(permission: WebPermission) -> Option<PermissionQuery> {
        let name = permission.as_str();
        let code = unsafe { permissions_cached(name.as_ptr(), name.len()) };
        permission_query_from_js(code)
    }
}
_js_extern! {
    [ module: "api_permissions" ]
    unsafe fn permissions_query(name_ptr: *const u8, name_len: usize) -> js_int32;
    unsafe fn permissions_cached(name_ptr: *const u8, name_len: usize) -> js_int32;
}

const fn permission_query_from_js(code: js_int32) -> Option<PermissionQuery> {
    match code {
        1 => Some(AsyncPoll::Ready(Ok(PermissionState::Granted))),
        0 => Some(AsyncPoll::Ready(Ok(PermissionState::Prompt))),
        -1 => Some(AsyncPoll::Ready(Ok(PermissionState::Denied))),
        -2 => Some(AsyncPoll::Pending),
        -3 => Some(AsyncPoll::Ready(Err(PermissionError::Unsupported))),
        -4 => Some(AsyncPoll::Ready(Err(PermissionError::Failed))),
        -5 => None,
        _ => Some(AsyncPoll::Ready(Err(PermissionError::Failed))),
    }
}
