// devela_base_core::lang::ffi::c::libc
//
//!
//

#![allow(non_camel_case_types)]

use crate::{_TAG_NUM, _TAG_PRIMITIVE};

#[doc = _TAG_PRIMITIVE!()]
#[doc = _TAG_NUM!()]
/// Equivalent to C’s `mode_t` type.
///
/// Numeric type used in POSIX interfaces to encode file modes and permission bits.
pub type c_mode_t = u32;

#[doc = _TAG_PRIMITIVE!()]
#[doc = _TAG_NUM!()]
/// Equivalent to C’s `off_t` type.
///
/// Signed integer type used by POSIX interfaces to represent file offsets and sizes.
pub type c_off_t = i64;
