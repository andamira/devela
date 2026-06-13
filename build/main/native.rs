// devela/build/main/native.rs
//
//! Native library availability probing.
//

#[allow(unused_imports, reason = "gated")]
use super::Build;
use std::io::Error as IoError;

pub(crate) fn main() -> Result<(), IoError> {
    #[cfg(feature = "__dbg")]
    cfg_select! {
        feature = "__disable_native_libs" =>
        Build::println_heading("Native libraries detection DISABLED:"),
        _ => Build::println_heading("Native libraries detection requested:"),
    };

    #[cfg(feature = "alsa")]
    let _ = Build::emit_flag_if_lib("ffi_alsa··", "asound");

    #[cfg(feature = "x11")]
    let _ = Build::emit_flag_if_lib("ffi_xcb_shm··", "xcb-shm");

    Ok(())
}
