use crate::{Slice, is, whilst};

/// Generates const slice equality implementations for primitive types.
macro_rules! _impl_slice_eq_prim {
    (
        #[cfg($gate:meta)]
        $($t:ty),+ $(,)?
    ) => {
        #[cfg($gate)]
        _impl_slice_eq_prim! { % $($t),+ }
    };
    (% $($t:ty),+ $(,)?) => {
        $(
            impl Slice<$t> {
                /// Checks the equality of two slices in compile-time.
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
                /// Checks the equality of two slices of slices in compile-time.
                #[must_use]
                pub const fn eq(a: &[&[$t]], b: &[&[$t]]) -> bool {
                    is! { a.len() != b.len(), return false }
                    whilst! { i in 0..a.len(); {
                        is! { !Slice::<$t>::eq(a[i], b[i]), return false }
                    }}
                    true
                }
            }
        )+
    };
}

// Fundamental byte equality.
// Required by strings, codecs, const_assert!'s default buffer form, etc.
_impl_slice_eq_prim! {
    #[cfg(all())]
    u8
}

// Required by existing crate tests.
_impl_slice_eq_prim! {
    #[cfg(any(feature = "slice_eq", test))]
    u32
}

// Extended primitive equality.
_impl_slice_eq_prim! {
    #[cfg(feature = "slice_eq")]
    u16, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    bool, char
}

#[cfg(nightly_float)]
_impl_slice_eq_prim! {
    #[cfg(feature = "slice_eq")]
    f16, f128
}

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
