// devela::sys::mem::view::slice::namespace::eq

use crate::{Slice, is, whilst};

/// Helper for implementing slice operations for primitives.
macro_rules! impl_prim {
    () => {
        impl_prim![
            u8, u16, u32, u64, u128, usize,
            i8, i16, i32, i64, i128, isize,
            f32, f64,
            bool, char
        ];
        #[cfg(nightly_float)]
        impl_prim![f16, f128];
    };
    ($($t:ty),+) => { $( impl_prim![@$t]; )+ };
    (@$t:ty) => {
        impl Slice<$t> {
            /// Checks the equality of two slices of primitives in compile-time.
            #[must_use]
            pub const fn eq(a: &[$t], b: &[$t]) -> bool {
                is! { a.len() != b.len(), return false }
                whilst! { i in 0..a.len(); {
                    is! { a[i] != b[i], return false }
                }}
                true
            }
        }
        impl Slice<&[$t]> {
            /// Checks the equality of two slices of slices of primitives in compile-time.
            #[must_use]
            pub const fn eq(a: &[&[$t]], b: &[&[$t]]) -> bool {
                is! { a.len() != b.len(), return false }
                whilst! { i in 0..a.len(); {
                    is! { !Slice::<$t>::eq(a[i], b[i]), return false }
                }}
                true
            }
        }
    };
}
impl_prim!();

/// # Methods for string slices.
impl Slice<&str> {
    /// Checks the equality of two string slices in compile-time.
    #[must_use]
    pub const fn eq(a: &str, b: &str) -> bool {
        Slice::<u8>::eq(a.as_bytes(), b.as_bytes())
    }
}
impl Slice<&[&str]> {
    /// Checks the equality of two slices of string slices in compile-time.
    #[must_use]
    pub const fn eq(a: &[&str], b: &[&str]) -> bool {
        is! { a.len() != b.len(), return false }
        whilst! { i in 0..a.len(); {
            is! { !Slice::<&str>::eq(a[i], b[i]), return false }
        }}
        true
    }
}
