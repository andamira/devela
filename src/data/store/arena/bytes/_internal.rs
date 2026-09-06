// devela/src/data/store/arena/bytes/_internal.rs
//
//! Defines [`__ArenaBytesArray`], [`__arena_bytes!`].
//!
//! This module groups all the arena-related safety feature-gates.
//

use crate::{MaybeByte, Slice};

#[doc(hidden)]
/// Internal arena methods.
///
/// Abstracts away over safe `u8` or unsafe `MaybeUninit<u8>` representations.
///
/// # Features
/// Uses the `unsafe_array` and `unsafe_slice` features.
#[derive(Debug)]
pub struct __ArenaBytesArray<const CAP: usize>;

#[rustfmt::skip]
impl<const CAP: usize> __ArenaBytesArray<CAP> {
    ///
    pub const fn new_array<const N: usize>() -> [MaybeByte; N] {
        cfg_select! { all(feature = "unsafe_array", not(feature = "safe_mem")) => {
            [MaybeByte::uninit(); N]
        } _ => {
            [0_u8; N]
        }}
    }
    ///
    pub const fn read_byte(data: &[MaybeByte], i: usize) -> u8 {
        cfg_select! { all(feature = "unsafe_array", not(feature = "safe_mem")) => {
            unsafe { data[i].assume_init_read() }
        } _ => {
            data[i]
        }}
    }
    ///
    pub const fn read_byte_mut(data: &mut [MaybeByte], i: usize) -> &mut u8 {
        cfg_select! { all(feature = "unsafe_array", not(feature = "safe_mem")) => {
            unsafe { data[i].assume_init_mut() }
        } _ => {
            &mut data[i]
        }}
    }
    ///
    pub const fn write_byte(data: &mut [MaybeByte], i: usize, b: u8) {
        cfg_select! { all(feature = "unsafe_array", not(feature = "safe_mem")) => {
            data[i].write(b);
        } _ => {
            data[i] = b;
        }}
    }
    ///
    pub const fn slice_bytes(data: &[MaybeByte], start: usize, end: usize) -> &[u8] {
        cfg_select! { // unsafest, unsafe, safe:
            all(feature = "unsafe_array", feature = "unsafe_slice", not(feature = "safe_mem")) => {
                unsafe { Slice::range_unchecked(
                    Slice::from_raw_parts(data.as_ptr().cast::<u8>(), CAP), start, end)
            }} all(feature = "unsafe_array", not(feature = "safe_mem")) => {
                unsafe { Slice::range(Slice::from_raw_parts(
                    data.as_ptr().cast::<u8>(), CAP), start, end)
            }}
            _ => { Slice::range(data, start, end) }}
    }
    ///
    pub const fn slice_bytes_mut(data: &mut [MaybeByte], start: usize, end: usize) -> &mut [u8] {
        cfg_select! { // unsafest, unsafe, safe:
            all(feature = "unsafe_array", feature = "unsafe_slice", not(feature = "safe_mem")) => {
                unsafe { Slice::range_mut_unchecked(
                    Slice::from_raw_parts_mut(data.as_mut_ptr().cast::<u8>(), CAP), start, end)
            }} all(feature = "unsafe_array", not(feature = "safe_mem")) => {
                unsafe { Slice::range_mut(Slice::from_raw_parts_mut(
                    data.as_mut_ptr().cast::<u8>(), CAP), start, end)
            }}
            _ => { Slice::range_mut(data, start, end) }}
    }
}

/// Private API of [`arena_bytes`][crate::arena_bytes].
#[doc(hidden)]
#[macro_export]
macro_rules! __arena_bytes· {
    (
        [cursor: $cprim:ident $(+ $Cursor:ty)?;]

        $(#[$arena_attr:meta])*
        $vis:vis $Arena:ident $( : $kind:ident )?;

        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident;

        $(
            $(#[$mark_attr:meta])*
            $mvis:vis $Mark:ident $(;)?
        )?
    ) => {
        $crate::__arena_bytes! { %normalize_cursor
            [kind: $($kind)?]
            [cursor: $cprim $(+ $Cursor)?]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $(#[$handle_attr])* $hvis $Handle]
            [mark: $($(#[$mark_attr])* $mvis $Mark)?]
        }
    };
    (%normalize_cursor
        [kind: $($kind:ident)?]
        [cursor: $cprim:ident]
        $($rest:tt)*
    ) => {
        $crate::__arena_bytes! { %generate
            [kind: $($kind)?]
            [cursor: $cprim + $cprim]
            $($rest)*
        }
    };
    (%normalize_cursor
        [kind: $($kind:ident)?]
        [cursor: $cprim:ident + $Cursor:ty]
        $($rest:tt)*
    ) => {
        $crate::__arena_bytes! { %generate
            [kind: $($kind)?]
            [cursor: $cprim + $Cursor]
            $($rest)*
        }
    };
    (%generate
        [kind: $($kind:ident)?]
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $(#[$handle_attr:meta])* $hvis:vis $Handle:ident]
        [mark: $($(#[$mark_attr:meta])* $mvis:vis $Mark:ident)?]
    ) => {
        $crate::handle_span! {
            [offset: $cprim + $Cursor;]
            $(#[$handle_attr])*
            $hvis $Handle;
        }
        $(
            $(#[$mark_attr])*
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $mvis struct $Mark($cprim);

            #[allow(dead_code)]
            impl $Mark {
                const fn new(cursor: $cprim) -> Self {
                    Self(cursor)
                }
            }
        )?
        $crate::__arena_bytes! { %backend
            [kind: $($kind)?]
            [cursor: $cprim + $Cursor]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($mvis $Mark)?]
        }
    };
    (%backend
        [kind:]
        $($rest:tt)*) => {
        $crate::__arena_bytes! { %backend [kind: static] $($rest)* }
    };
    (%backend
        [kind: static]
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $($mvis:vis $Mark:ident)?]
    ) => {
        $crate::paste! { $crate::__arena_bytes_impl_array! {
            [cursor: $cprim]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($mvis $Mark)?]
            [internal: $crate::__ArenaBytesArray::<CAP>]
            [module: [<_arena_bytes_impl_ $Arena>]]
            ($)
        }}
    };
    (%backend
        [kind: alloc]
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $($mvis:vis $Mark:ident)?]
    ) => {
        $crate::paste! { $crate::__arena_bytes_impl_vec! {
            [cursor: $cprim + $Cursor]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($mvis $Mark)?]
            [module: [<_arena_bytes_impl_ $Arena>]]
            ($)
        }}
    };
}
pub use __arena_bytes· as __arena_bytes;
