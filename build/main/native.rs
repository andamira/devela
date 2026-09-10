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

    // Avoid mistaking host libraries for target libraries when cross-compiling.
    let _native_target = std::env::var("HOST").unwrap() == std::env::var("TARGET").unwrap();

    #[cfg(feature = "alsa")]
    if _native_target {
        let _ = Build::emit_flag_if_lib("ffi_alsa··", "asound");
    } else {
        Build::emit_check_cfg("ffi_alsa··");
    }

    #[cfg(feature = "x11")]
    if _native_target {
        let _ = Build::emit_flag_if_lib("ffi_xcb_shm··", "xcb-shm");
    } else {
        Build::emit_check_cfg("ffi_xcb_shm··");
    }

    Ok(())
}
