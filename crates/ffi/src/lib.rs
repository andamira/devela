// devela_ffi
//
//! C ABI layer for selected devela functionality.
//

#![allow(non_camel_case_types)]

use devela::c_char;

// #[cfg(feature = "term")]
// #[cfg(feature = "linux")]
// mod term;

/// ABI status values.
//
// In sync with:
// - src/bin/devela_ffi_bindgen.rs: STATUS_CONSTANTS
// - generated include/devela_ffi.h
// - generated odin/common/gen_common.odin
//
// Values are part of the public C ABI.
pub type devela_status = i32;
pub const DEVELA_OK: devela_status = 0;
pub const DEVELA_NO_EVENT: devela_status = 1;
pub const DEVELA_ERR_NULL: devela_status = -1;
pub const DEVELA_ERR_INVALID: devela_status = -2;
pub const DEVELA_ERR_PANIC: devela_status = -3;
pub const DEVELA_ERR_UNSUPPORTED: devela_status = -4;

/// ABI version: major.minor.patch packed as 0x00MM_mm_pp.
#[unsafe(no_mangle)]
pub extern "C" fn devela_abi_version() -> u32 {
    0x0000_0001
}

/// Tiny call-with-argument smoke test.
#[unsafe(no_mangle)]
pub extern "C" fn devela_add_i32(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

/// Tiny pointer/string smoke test.
///
/// Returns a status instead of printing directly, so C/Odin argument passing
/// can be tested before terminal state enters the picture.
#[unsafe(no_mangle)]
pub extern "C" fn devela_bytes_len(
    bytes: *const u8,
    len: usize,
    out_len: *mut usize,
) -> devela_status {
    if out_len.is_null() {
        return DEVELA_ERR_NULL;
    }
    if bytes.is_null() && len != 0 {
        return DEVELA_ERR_NULL;
    }
    unsafe {
        *out_len = len;
    }
    DEVELA_OK
}

#[unsafe(no_mangle)]
pub extern "C" fn devela_error_string(status: devela_status) -> *const c_char {
    match status {
        DEVELA_OK => c"ok".as_ptr(),
        DEVELA_NO_EVENT => c"no event".as_ptr(),
        DEVELA_ERR_NULL => c"null pointer".as_ptr(),
        DEVELA_ERR_INVALID => c"invalid argument".as_ptr(),
        DEVELA_ERR_PANIC => c"panic".as_ptr(),
        DEVELA_ERR_UNSUPPORTED => c"unsupported".as_ptr(),
        _ => c"unknown error".as_ptr(),
    }
}
