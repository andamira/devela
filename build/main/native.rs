// devela::build::native
//
//! Native library availability probing.
//

#[allow(unused_imports, reason = "gated")]
use super::Build;
use std::io::Error as IoError;

pub(crate) fn main() -> Result<(), IoError> {
    #[cfg(feature = "__dbg")]
    Build::println_heading("Native libraries:");

    #[cfg(feature = "x11")]
    let _ = Build::emit_flag_if_lib("ffi_xcb_shm··", "xcb-shm");

    Ok(())
}
