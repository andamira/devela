// devela::sys::os::browser::web::api::namespace
// (in sync with ./web_api.js)
//
//! Implements the web history & location API.
//

use crate::Web;
use crate::{_js_doc, _js_extern, js_int32};

/// # Web API history & location
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/History>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Location>
#[rustfmt::skip]
impl Web {
    #[doc = _js_doc!("History", "back")]
    /// Moves the browser back one step in the session history.
    pub fn history_back() { unsafe { history_back(); } }

    #[doc = _js_doc!("History", "forward")]
    /// Moves the browser forward one step in the session history.
    pub fn history_forward() { unsafe { history_forward(); } }

    #[doc = _js_doc!("History", "go")]
    /// Moves the browser to a specific point in the session history.
    /// Use negative values to go back, positive to go forward.
    pub fn history_go(delta: js_int32) { unsafe { history_go(delta); } }

    #[doc = _js_doc!("History", "pushState")]
    /// Adds an entry to the session history stack.
    pub fn history_push_state(state: &str, title: &str, url: &str) {
        unsafe { history_push_state(state.as_ptr(), state.len(), title.as_ptr(), title.len(),
            url.as_ptr(), url.len()); }
    }

    #[doc = _js_doc!("History", "replaceState")]
    /// Modifies the current history entry without creating a new one.
    pub fn history_replace_state(state: &str, title: &str, url: &str) {
        unsafe { history_replace_state(state.as_ptr(), state.len(), title.as_ptr(), title.len(),
            url.as_ptr(), url.len()); }
    }

    #[doc = _js_doc!("Location", "reload")]
    /// Reloads the current document.
    pub fn location_reload() { unsafe { location_reload(); } }

    #[doc = _js_doc!("Location", "assign")]
    /// Loads the specified URL.
    pub fn location_assign(url: &str) { unsafe { location_assign(url.as_ptr(), url.len()); } }

    #[doc = _js_doc!("Location", "replace")]
    /// Replaces the current document with the given URL,
    /// without creating a new entry in the history.
    pub fn location_replace(url: &str) { unsafe { location_replace(url.as_ptr(), url.len()); } }
}
_js_extern! {
    [ module: "api_history_location" ]
    unsafe fn history_back();
    unsafe fn history_forward();
    unsafe fn history_go(delta: js_int32);
    unsafe fn "history_pushState" history_push_state(state_ptr: *const u8, state_len: usize,
        title_ptr: *const u8, title_len: usize, url_ptr: *const u8, url_len: usize);
    unsafe fn "history_replaceState" history_replace_state(state_ptr: *const u8, state_len: usize,
        title_ptr: *const u8, title_len: usize, url_ptr: *const u8, url_len: usize);
    //
    unsafe fn location_reload();
    unsafe fn location_assign(url_ptr: *const u8, url_len: usize);
    unsafe fn location_replace(url_ptr: *const u8, url_len: usize);
}
