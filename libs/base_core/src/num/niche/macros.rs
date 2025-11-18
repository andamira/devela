// devela_base_core::num::niche::macros
//
//! Defines [`ne!`], [`nv!`] [`nz!`], and the [`NicheNew`] hidden helper.
//
// TOC
// - macro ne!
// - macro nv!
// - macro nz!
// - struct NicheNew
// - tests

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NICHE!()]
/// Creates a `NonExtreme*` [`niche`] instance with compile-time checking.
///
/// # Example
/// ```
/// # use devela_base_core::ne;
/// let x = ne!(42_u32);  // NonExtremeU32
/// let y = ne!(42, u32); // alternative syntax
/// // let y: NonExtremeU32 = ne!(42); // Would fail to compile (needs type suffix)
///
/// // alternative lossy constructor, non-panicking
/// assert_eq![ne!(lossy u8::MAX).get(), u8::MAX-1]; // MAX prohibited value is -1 mapped
/// assert_eq![ne!(lossy 254_u8).get(), 254];
/// assert_eq![ne!(lossy -127_i8).get(), -127];
/// assert_eq![ne!(lossy i8::MIN).get(), i8::MIN+1]; // MIN prohibited value is +1 mapped
/// ```
#[doc = crate::doclink!(custom crate "[`niche`]" "num/niche" @mod)]
#[macro_export]
#[doc(hidden)]
macro_rules! ne {
    (lossy $num:expr) => {{ $crate::NicheNew($num).to_lossy_non_extreme() }};
    (lossy $num:expr, $T:ty) => {{ $crate::NicheNew::<$T>($num).to_lossy_non_extreme() }};
    ($num:expr) => {{ $crate::NicheNew($num).to_non_extreme() }};
    ($num:expr, $T:ty) => {{ $crate::NicheNew::<$T>($num).to_non_extreme() }};
}
#[doc(inline)]
pub use ne;

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NICHE!()]
/// Creates a `NonValue*` [`niche`] value with compile-time checking.
///
/// # Example
/// ```
/// # use devela_base_core::nv;
/// let x = nv!(3: 42_u8);  // NonValueU8<3>
/// let y = nv!(3: 42, u8); // alternative syntax
/// // let y: NonValue<u8> = nv!(3: 42); // Would fail to compile (needs type suffix)
///
/// // alternative lossy constructor, non-panicking
/// assert_eq![nv!(lossy 255: 255_u8).get(), 254]; // most prohibited values are -1 mapped
/// assert_eq![nv!(lossy 255: 254_u8).get(), 254];
/// assert_eq![nv!(lossy 2: 2, u8).get(), 1];
/// assert_eq![nv!(lossy 2: 1, u8).get(), 1];
/// assert_eq![nv!(lossy 0: 0, u8).get(), 1]; // MIN prohibited value is +1 mapped
/// ```
#[doc = crate::doclink!(custom crate "[`niche`]" "num/niche" @mod)]
#[macro_export]
#[doc(hidden)]
macro_rules! nv {
    (lossy $val:literal: $num:expr) => {{ $crate::NicheNew($num).to_lossy_non_value::<$val>() }};
    (lossy $val:literal: $num:expr, $T:ty) => {{ $crate::NicheNew::<$T>($num).to_lossy_non_value::<$val>() }};

    ($val:literal: $num:expr) => {{ $crate::NicheNew($num).to_non_value::<$val>() }};
    ($val:literal: $num:expr, $T:ty) => {{ $crate::NicheNew::<$T>($num).to_non_value::<$val>() }};
}
#[doc(inline)]
pub use nv;

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NICHE!()]
/// Creates a `NonZero*` [`niche`] value with compile-time checking.
///
/// # Example
/// ```
/// # use devela_base_core::nz;
/// let x = nz!(42u32);   // NonZeroU32
/// let y = nz!(42, u32); // alternative syntax
/// // let y: NonZero<u8> = nz!(42); // Would fail to compile (needs type suffix)
///
/// // alternative lossy constructor, non-panicking
/// assert_eq![nz!(lossy u8::MAX).get(), u8::MAX];
/// assert_eq![nz!(lossy 1, u8).get(), 1];
/// assert_eq![nz!(lossy 0_u8).get(), 1]; // 0 value is 1 mapped
/// assert_eq![nz!(lossy i8::MIN).get(), i8::MIN];
/// ```
#[doc = crate::doclink!(custom crate "[`niche`]" "num/niche" @mod)]
#[macro_export]
#[doc(hidden)]
macro_rules! nz {
    (lossy $num:expr) => {{ $crate::NicheNew($num).to_lossy_non_zero() }};
    (lossy $num:expr, $T:ty) => {{ $crate::NicheNew::<$T>($num).to_lossy_non_zero() }};

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

macro_rules! impl_niche_new {
    () => {
        impl_niche_new!(U: u8, u16, u32, u64, u128, usize);
        impl_niche_new!(I: i8, i16, i32, i64, i128, isize);
    };
    ($SIGN:ident: $($prim:ty),+) => { $crate::paste! {
        $( impl_niche_new!(@NZ [<NonZero $prim:camel>], $prim); )+
        $( impl_niche_new!(@NV [<NonValue $prim:camel>], $prim); )+ // TODO
        $( impl_niche_new!(@$SIGN [<NonExtreme $prim:camel>], $prim); )+
    }};
    (@NZ $name:ident, $prim:ty) => {
        impl NicheNew<$prim> {
            pub const fn to_non_zero(self) -> $crate::$name {
                if self.0 == 0 { panic!["value must not be 0"]; }
                else { $crate::$name::new(self.0).unwrap() }
            }
            pub const fn to_lossy_non_zero(self) -> $crate::$name {{
                let value = if self.0 == 0 { 1 } else { self.0 };
                $crate::$name::new(value).unwrap()
            }}
        }
    };
    (@NV $name:ident, $prim:ty) => {
        impl NicheNew<$prim> {
            pub const fn to_non_value<const V: $prim>(self) -> $crate::$name<V> {
                $crate::$name::new(self.0).expect("Invalid value")
            }
            pub const fn to_lossy_non_value<const V: $prim>(self) -> $crate::$name<V> {
                $crate::$name::new_lossy(self.0)
            }
        }
    };
    (@I $name:ident, $prim:ty) => {
        impl NicheNew<$prim> {
            pub const fn to_non_extreme(self) -> $crate::$name {
                if self.0 == <$prim>::MIN { panic!["value must not be MIN"]; }
                else { $crate::$name::new(self.0).unwrap() }
            }
            pub const fn to_lossy_non_extreme(self) -> $crate::$name {
                $crate::$name::new_lossy(self.0)
            }
        }
    };
    (@U $name:ident, $prim:ty) => {
        impl NicheNew<$prim> {
            pub const fn to_non_extreme(self) -> $crate::$name {
                if self.0 == <$prim>::MAX { panic!["value must not be MAX"]; }
                else { $crate::$name::new(self.0).unwrap() }
            }
            pub const fn to_lossy_non_extreme(self) -> $crate::$name {
                $crate::$name::new_lossy(self.0)
            }
        }
    };
}
impl_niche_new!();

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
