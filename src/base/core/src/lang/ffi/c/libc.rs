// devela_base_core::lang::ffi::c::libc
//
//!
//

#![allow(non_camel_case_types)]

#[doc = crate::_tags!(primitive num)]
/// Equivalent to C's `mode_t` type.
#[doc = crate::_doc_location!("lang/ffi/c")]
///
/// Numeric type used in POSIX interfaces to encode file modes and permission bits.
pub type c_mode_t = u32;

#[doc = crate::_tags!(primitive num)]
/// Equivalent to C's `off_t` type.
#[doc = crate::_doc_location!("lang/ffi/c")]
///
/// Signed integer type used by POSIX interfaces to represent file offsets and sizes.
pub type c_off_t = i64;
