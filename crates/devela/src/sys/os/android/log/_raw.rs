//
//! Raw Android logging bindings.
//

use crate::{c_char, c_int};

#[link(name = "log")]
unsafe extern "C" {
    pub(super) fn __android_log_write(
        prio: c_int,
        tag: *const c_char,
        text: *const c_char,
    ) -> c_int;
}
