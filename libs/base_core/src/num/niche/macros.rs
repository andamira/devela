// devela_base_core::num::niche::macros
//
//! Defines the [`ne!`] and [`nz!`] macros, and the hidden [`NicheNew`].
//
// TOC
// - macro ne!
// - macro nv!
// - macro nz!
// - struct NicheNew
// - tests

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NICHE!()]
/// Creates a `NonExtreme*` [`niche`][crate::num::niche] instance with compile-time checking.
///
/// # Example
/// ```
/// # use devela_base_core::ne;
/// let x = ne!(42_u32);  // NonExtremeU32
/// let y = ne!(42, u32); // alternative syntax
/// // let y: NonExtremeU32 = ne!(42); // Would fail to compile (needs type suffix)
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! ne {
    ($num:expr) => {{ $crate::NicheNew($num).to_non_extreme() }};
    ($num:expr, $T:ty) => {{ $crate::NicheNew::<$T>($num).to_non_extreme() }};
}
#[doc(inline)]
pub use ne;

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NICHE!()]
/// Creates a `NonValue*` [`niche`][crate::num::niche] value with compile-time checking.
///
/// # Example
/// ```
/// # use devela_base_core::nv;
/// let x = nv!(3: 42_u8);  // NonValueU8<3>
/// let y = nv!(3: 42, u8); // alternative syntax
/// // let y: NonValue<u8> = nv!(3: 42); // Would fail to compile (needs type suffix)
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! nv {
    ($val:literal: $num:expr) => {{ $crate::NicheNew($num).to_non_value::<$val>() }};
    ($val:literal: $num:expr, $T:ty) => {{ $crate::NicheNew::<$T>($num).to_non_value::<$val>() }};
}
#[doc(inline)]
pub use nv;

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NICHE!()]
/// Creates a `NonZero*` [`niche`][crate::num::niche] value with compile-time checking.
///
/// # Example
/// ```
/// # use devela_base_core::nz;
/// let x = nz!(42u32);   // NonZeroU32
/// let y = nz!(42, u32); // alternative syntax
/// // let y: NonZero<u8> = nz!(42); // Would fail to compile (needs type suffix)
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! nz {
    ($num:expr) => {{ $crate::NicheNew($num).to_non_zero() }};
    ($num:expr, $T:ty) => {{ $crate::NicheNew::<$T>($num).to_non_zero() }};
}
#[doc(inline)]
pub use nz;

/* helpers */

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NICHE!()]
/// Private helper to construct niche types.
#[doc(hidden)]
#[derive(Debug)]
pub struct NicheNew<T>(pub T);

macro_rules! impl_niche {
    () => {
        impl_niche!(U: u8, u16, u32, u64, u128, usize);
        impl_niche!(I: i8, i16, i32, i64, i128, isize);
    };
    ($SIGN:ident: $($prim:ty),+) => { $crate::paste! {
        $( impl_niche!(@NZ [<NonZero $prim:camel>], $prim); )+
        $( impl_niche!(@NV [<NonValue $prim:camel>], $prim); )+ // TODO
        $( impl_niche!(@$SIGN [<NonExtreme $prim:camel>], $prim); )+
    }};
    (@NZ $name:ident, $prim:ty) => {
        impl NicheNew<$prim> {
            pub const fn to_non_zero(self) -> $crate::$name {
                if self.0 == 0 { panic!["value must not be 0"]; }
                else { $crate::$name::new(self.0).unwrap() }
            }
        }
    };
    (@NV $name:ident, $prim:ty) => {
        impl NicheNew<$prim> {
            pub const fn to_non_value<const V: $prim>(self) -> $crate::$name<V> {
                $crate::$name::new(self.0).expect("Invalid value")
            }
        }
    };
    (@I $name:ident, $prim:ty) => {
        impl NicheNew<$prim> {
            pub const fn to_non_extreme(self) -> $crate::$name {
                if self.0 == <$prim>::MIN { panic!["value must not be MIN"]; }
                else { $crate::$name::new(self.0).unwrap() }
            }
        }
    };
    (@U $name:ident, $prim:ty) => {
        impl NicheNew<$prim> {
            pub const fn to_non_extreme(self) -> $crate::$name {
                if self.0 == <$prim>::MAX { panic!["value must not be MAX"]; }
                else { $crate::$name::new(self.0).unwrap() }
            }
        }
    };
}
impl_niche!();

#[cfg(test)]
crate::items! {
    #[test]
    fn test_nz() {
        assert_eq![nz![20i32], crate::NonZeroI32::new(20).unwrap()];
        assert_eq![nz![20u8], crate::NonZeroU8::new(20).unwrap()];
        // alternative syntax
        assert_eq![nz![20, i32], crate::NonZeroI32::new(20).unwrap()];
        assert_eq![nz![20, u8], crate::NonZeroU8::new(20).unwrap()];
    }
    #[test]
    #[should_panic(expected = "value must not be 0")]
    fn test_nz_panic() { let _panic = nz![0i64]; }

    #[test]
    fn test_nv() {
        assert_eq![nv![3: 20u8], crate::NonValueU8::new(20).unwrap()];
        // alternative syntax
        assert_eq![nv![3: 20, i32], crate::NonValueI32::new(20).unwrap()];
    }
    #[test]
    #[should_panic(expected = "Invalid value")]
    fn test_nv_panic() { let _panic = nv![3: 3, i32]; }

    #[test]
    fn test_ne() {
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
