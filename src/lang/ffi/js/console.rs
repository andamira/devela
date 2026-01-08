// devela::lang::ffi::js::console
//
//! Defines [`JsConsole`].
//

#[allow(unused_imports)]
use devela::{_js_doc, _js_extern};

#[doc = crate::_tags!(runtime namespace)]
/// Javascript Console.
#[doc = crate::_doc_location!("lang/ffi/js")]
#[derive(Debug)]
pub struct JsConsole;

/// # Javascript API console
///
/// - <https://console.spec.whatwg.org/>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/console>
#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl JsConsole {
    #[doc = _js_doc!(console "clear")]
    /// Clears the console if possible.
    pub fn clear() { console_clear() }

    #[doc = _js_doc!(console "debug")]
    /// Outputs a message to the console with the debug log level.
    pub fn debug(text: &str) { unsafe { console_debug(text.as_ptr(), text.len()); } }

    #[doc = _js_doc!(console "error")]
    /// Outputs a message to the console with the error log level.
    pub fn error(text: &str) { unsafe { console_error(text.as_ptr(), text.len()); } }

    #[doc = _js_doc!(console "info")]
    /// Outputs a message to the console with the info log level.
    pub fn info(text: &str) { unsafe { console_info(text.as_ptr(), text.len()); } }

    #[doc = _js_doc!(console "log")]
    /// Outputs a message to the console.
    pub fn log(text: &str) { unsafe { console_log(text.as_ptr(), text.len()); } }

    #[doc = _js_doc!(console "trace")]
    /// Outputs a stack trace.
    pub fn trace() { console_trace(); }

    #[doc = _js_doc!(console "warn")]
    /// Outputs a message to the console with the warning log level.
    pub fn warn(text: &str) { unsafe { console_warn(text.as_ptr(), text.len()); } }

    //

    #[doc = _js_doc!(console "count")]
    /// Logs the number of times that this particular call to count has been called.
    pub fn count(label: &str) { unsafe { console_count(label.as_ptr(), label.len()); } }

    #[doc = _js_doc!(console "countReset")]
    /// Resets the counter used with [`count`][Self::count].
    pub fn count_reset(label: &str) {
        unsafe { console_count_reset(label.as_ptr(), label.len()); } }

    //

    #[doc = _js_doc!(console "group")]
    /// Creates a new inline group, indenting all following output by another level.
    pub fn group(text: &str) { unsafe { console_group(text.as_ptr(), text.len()); } }

    #[doc = _js_doc!(console "groupCollapsed")]
    /// Like [`group`][Self::group] but starts with the inline group collapsed.
    pub fn group_collapsed(text: &str) {
        unsafe { console_group_collapsed(text.as_ptr(), text.len()); } }

    #[doc = _js_doc!(console "groupEnd")]
    /// Exits the current inline group.
    pub fn group_end() { console_group_end(); }

    //

    #[doc = _js_doc!(console "time")]
    /// Starts a timer with the given `name`.
    pub fn time(name: &str) { unsafe { console_time(name.as_ptr(), name.len()); } }

    #[doc = _js_doc!(console "timeEnd")]
    /// Stops a timer with the given name, started with [`time_end`][Self::time_end].
    pub fn time_end(name: &str) { unsafe { console_time_end(name.as_ptr(), name.len()); } }

    #[doc = _js_doc!(console "timeLog")]
    /// Logs a timer with the given name, started with [`time_log`][Self::time_log].
    pub fn time_log(name: &str) { unsafe { console_time_log(name.as_ptr(), name.len()); } }
}
_js_extern! {
    [ module: "api_console" ]
    safe fn console_clear();
    unsafe fn console_debug(str_ptr: *const u8, str_len: usize);
    unsafe fn console_error(str_ptr: *const u8, str_len: usize);
    unsafe fn console_info(str_ptr: *const u8, str_len: usize);
    unsafe fn console_log(str_ptr: *const u8, str_len: usize);
    safe fn console_trace();
    unsafe fn console_warn(str_ptr: *const u8, str_len: usize);
    //
    unsafe fn console_count(str_ptr: *const u8, str_len: usize);
    unsafe fn console_count_reset(str_ptr: *const u8, str_len: usize);
    //
    unsafe fn console_group(str_ptr: *const u8, str_len: usize);
    unsafe fn console_group_collapsed(str_ptr: *const u8, str_len: usize);
    safe fn console_group_end();
    //
    unsafe fn console_time(str_ptr: *const u8, str_len: usize);
    unsafe fn console_time_end(str_ptr: *const u8, str_len: usize);
    unsafe fn console_time_log(str_ptr: *const u8, str_len: usize);
}
