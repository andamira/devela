//
//! Raw Android logging bindings.
//

use crate::{c_char, c_int};

pub(super) const ANDROID_LOG_VERBOSE: c_int = 2;
pub(super) const ANDROID_LOG_DEBUG: c_int = 3;
pub(super) const ANDROID_LOG_INFO: c_int = 4;
pub(super) const ANDROID_LOG_WARN: c_int = 5;
pub(super) const ANDROID_LOG_ERROR: c_int = 6;
pub(super) const ANDROID_LOG_FATAL: c_int = 7;

#[link(name = "log")]
unsafe extern "C" {
    pub(super) fn __android_log_write(
        prio: c_int,
        tag: *const c_char,
        text: *const c_char,
    ) -> c_int;
    pub(super) fn __android_log_print(
        prio: c_int,
        tag: *const c_char,
        fmt: *const c_char,
        ...
    ) -> c_int;
}
