// devela::num::niche::macros
//
//! Defines the `ne!` and `nz!` macros.
//

/// Creates a `NonExtreme*` [`niche`][crate::num::niche] value with compile-time checking.
///
/// # Example
/// ```
/// # use devela::ne;
/// let x = ne!(42u32); // NonExtremeU32
/// // let y: NonExtremeI32 = ne!(20); // Would fail to compile (needs type suffix)
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! ne {
    (
    // Only the value (needs the type suffix to aid inference)
    $num:expr) => {{ $crate::NewNicheHelper($num).to_non_extreme() }};
    (
    // Specify the type separately.
    $num:expr, $T:ty) => {{ $crate::NewNicheHelper::<$T>($num).to_non_extreme() }};
}
#[doc(inline)]
pub use ne;

/// Creates a `NonZero*` [`niche`][crate::num::niche] value with compile-time checking.
///
/// # Example
/// ```
/// # use devela::nz;
/// let x = nz!(42u32); // NonZeroU32
/// // let y: NonZeroI32 = nz!(20); // Would fail to compile (needs type suffix)
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! nz {
    (
    // Only the value (needs the type suffix to aid inference)
    $num:expr) => {{ $crate::NewNicheHelper($num).to_non_zero() }};
    (
    // Specify the type separately
    $num:expr, $T:ty) => {{ $crate::NewNicheHelper::<$T>($num).to_non_zero() }};
}
#[doc(inline)]
pub use nz;

/* helpers */

/// Private helper to construct niche types.
#[doc(hidden)]
#[derive(Debug)]
pub struct NewNicheHelper<T>(pub T);
macro_rules! impl_niche {
    () => {
        impl_niche!(U: u8, u16, u32, u64, u128, usize);
        impl_niche!(I: i8, i16, i32, i64, i128, isize);
    };
    ($SIGN:ident: $($prim:ty),+) => { $crate::paste! {
        $( impl_niche!(@ALL [<NonZero $prim:camel>], $prim); )+
        $( impl_niche!(@$SIGN [<NonExtreme $prim:camel>], $prim); )+
    }};
    (@ALL $nz:ident, $prim:ty) => {
        impl NewNicheHelper<$prim> {
            pub const fn to_non_zero(self) -> $crate::$nz {
                if self.0 == 0 { panic!["value must not be 0"]; }
                else { $crate::$nz::new(self.0).unwrap() }
            }
        }
    };
    (@I $ne:ident, $prim:ty) => {
        impl NewNicheHelper<$prim> {
            pub const fn to_non_extreme(self) -> $crate::$ne {
                if self.0 == <$prim>::MIN { panic!["value must not be MIN"]; }
                else { $crate::$ne::new(self.0).unwrap() }
            }
        }
    };
    (@U $ne:ident, $prim:ty) => {
        impl NewNicheHelper<$prim> {
            pub const fn to_non_extreme(self) -> $crate::$ne {
                if self.0 == <$prim>::MAX { panic!["value must not be MAX"]; }
                else { $crate::$ne::new(self.0).unwrap() }
            }
        }
    };
}
use impl_niche;
impl_niche!();

#[cfg(test)]
crate::items! {
    #[test]
    fn test_nz() {
        // const NZI8: crate::NonZeroI8 = nz![23i8];
        assert_eq![nz![20i32], crate::NonZeroI32::new(20).unwrap()];
        assert_eq![nz![20u8], crate::NonZeroU8::new(20).unwrap()];
        // alternative syntax
        assert_eq![nz![20, i32], crate::NonZeroI32::new(20).unwrap()];
        assert_eq![nz![20, u8], crate::NonZeroU8::new(20).unwrap()];
    }
    #[test]
    #[should_panic(expected = "value must not be 0")]
    // NOTE: using a runtime value so that should_panic catches the panic
    fn test_nz_panic() { let zero = 0i64; let _panic = nz![zero]; }

    #[test]
    fn test_ne() {
        // const NEI8: crate::NonExtremeI8 = ne![23i8];
        assert_eq![ne![20i8], crate::NonExtremeI8::new(20).unwrap()];
        assert_eq![ne![20u8], crate::NonExtremeU8::new(20).unwrap()];
        // alternative syntax
        assert_eq![ne![20, i8], crate::NonExtremeI8::new(20).unwrap()];
        assert_eq![ne![20, u8], crate::NonExtremeU8::new(20).unwrap()];
    }
    #[test]
    #[should_panic(expected = "value must not be MIN")]
    fn test_ne_min_panic() { let min = i8::MIN; let _panic = ne![min]; }
    #[test]
    #[should_panic(expected = "value must not be MAX")]
    fn test_ne_max_panic() { let max = u8::MAX; let _panic = ne![max]; }
}
