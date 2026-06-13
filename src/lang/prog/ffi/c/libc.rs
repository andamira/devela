// devela/src/lang/prog/ffi/c/libc.rs

#![allow(non_camel_case_types)]

#[doc = crate::_tags!(primitive logic)]
/// Equivalent to C99 `_Bool` / `<stdbool.h>` `bool`.
#[doc = crate::_doc_meta!{location("lang/ffi/c")}]
pub type c_bool = bool;

#[doc = crate::_tags!(primitive num)]
/// Equivalent to C's `mode_t` type.
#[doc = crate::_doc_meta!{location("lang/ffi/c")}]
///
/// POSIX numeric type used to encode file modes and permission bits.
pub type c_mode_t = u32;

#[doc = crate::_tags!(primitive num)]
/// Equivalent to C's `off_t` type.
#[doc = crate::_doc_meta!{location("lang/ffi/c")}]
///
/// POSIX signed integer type used to represent file offsets and sizes.
pub type c_off_t = i64;

#[doc = crate::_tags!(primitive num)]
/// Equivalent to C's `size_t` type.
#[doc = crate::_doc_meta!{location("lang/ffi/c")}]
///
/// Unsigned integer type (C standard)
/// used to represent the size of objects in bytes and the result of `sizeof`.
pub type c_size_t = usize;

#[doc = crate::_tags!(primitive num)]
/// Equivalent to C's `ssize_t` type.
#[doc = crate::_doc_meta!{location("lang/ffi/c")}]
///
/// POSIX signed integer type used for functions
/// that return either a non‑negative byte count or -1 to indicate an error.
pub type c_ssize_t = isize;
