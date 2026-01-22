// devela_base_core::num::niche::macros
//
//! Defines [`ne!`], [`nv!`] [`nz!`], and the [`NicheNew`] hidden helper.
//
// TOC
// - macro niche_prim! (via generate_niche_prim!)
// - macro ne!
// - macro nv!
// - macro nz!
// - struct NicheNew (hidden)
// - tests

/// Generates the `niche_prim!` macro for a closed set of primitive carrier types.
///
/// It exists solely to avoid duplication and keep the supported primitive list centralized.
///
/// About using $ as a token delimiter:
///   this part:  ($P $_d($_:tt)*) => { $P };
///   expands to: (u8 $($_:tt)*) => { u8 };
macro_rules! generate_niche_prim {
    ($_d:tt $($P:ty),+ $(,)?) => { $crate::paste! {
        #[doc = crate::_tags!(niche)]
        /// Maps a niche representation type to its primitive carrier type.
        #[doc = crate::_doc_location!("num/niche")]
        ///
        /// `niche_prim!` performs a purely syntactic, compile-time mapping from a
        /// supported niche type (for example `NonExtremeU8`, `NonValueU16<…>`,
        /// `MaybeNiche<NonZero<i32>>`) to its underlying primitive numeric type
        /// (`u8`, `u16`, `i32`, …).
        ///
        /// This macro is intended for use in type position, including type aliases,
        /// signatures, and const contexts.
        ///
        /// ## Supported forms
        /// For each supported primitive `P`, the following spellings are recognized:
        /// - `P`
        /// - `NonNiche<P>`
        /// - `NonZero<P>` and `NonZeroP`
        /// - `NonExtremeP`
        /// - `NonValueP<…>`
        /// - `MaybeNiche<P>`
        /// - `MaybeNiche<NonNiche<P>>`
        /// - `MaybeNiche<NonZero<P>>` and `MaybeNiche<NonZeroP>`
        /// - `MaybeNiche<NonExtremeP>`
        /// - `MaybeNiche<NonValueP<…>>`
        ///
        /// The mapping is shallow and exact: only the explicitly supported spellings are matched.
        ///
        /// ## Closed-world behavior
        /// The set of recognized niche types is intentionally fixed and mirrors the
        /// niche representations provided by this crate. User-defined wrappers or
        /// arbitrary nesting are not supported.
        ///
        /// # Example
        /// ```
        /// # use devela_base_core::{NonValueU8, niche_prim};
        /// let x: niche_prim!(NonValueU8<43>) = 3_u8;
        /// ```
        #[doc(hidden)]
        #[macro_export]
        macro_rules! _niche_prim {
            // strip one leading path segment at a time
            ($head:ident :: $_d($rest:tt)+) => { $crate::niche_prim!($_d($rest)+) };
            $(
                ($P $_d($_:tt)*) => { $P };
                (NonNiche<$P> $_d($_:tt)*) => { $P };
                (NonZero<$P> $_d($_:tt)*) => { $P };
                ([<NonZero$P:camel>] $_d($_:tt)*) => { $P };
                ([<NonExtreme$P:camel>] $_d($_:tt)*) => { $P };
                ([<NonValue$P:camel>] $_d($_:tt)*) => { $P };
                (MaybeNiche<$P> $_d($_:tt)*) => { $P };
                (MaybeNiche<NonNiche<$P>> $_d($_:tt)*) => { $P };
                (MaybeNiche<NonZero<$P>> $_d($_:tt)*) => { $P };
                (MaybeNiche<[<NonZero$P:camel>]> $_d($_:tt)*) => { $P };
                (MaybeNiche<[<NonExtreme$P:camel>]> $_d($_:tt)*) => { $P };
                (MaybeNiche<[<NonValue$P:camel>]> $_d($_:tt)*) => { $P };
            )+
        }
        #[doc(inline)]
        pub use _niche_prim as niche_prim;
    }};
}
generate_niche_prim![$ u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

#[doc = crate::_tags!(num niche construction)]
/// Creates a `NonExtreme*` niche instance with compile-time checking.
#[doc = crate::_doc_location!("num/niche")]
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

#[doc = crate::_tags!(num niche construction)]
/// Creates a `NonValue*` niche value with compile-time checking.
#[doc = crate::_doc_location!("num/niche")]
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

#[doc = crate::_tags!(num niche construction)]
/// Creates a `NonZero*` niche value with compile-time checking.
#[doc = crate::_doc_location!("num/niche")]
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

#[doc = crate::_tags!(num niche construction)]
/// Private helper to construct niche types.
#[doc = crate::_doc_location!("num/niche")]
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
    #[allow(unused)]
    fn test_niche_prim() {
        let a: niche_prim!(NonValueU8<43>) = 3;
        let b: u8 = a + 4_u8;

        let a: niche_prim!(MaybeNiche<NonZeroIsize>) = 3;
        let b: isize = a + 4_isize;

        // works with paths
        let a: niche_prim!(crate::num::NonValueU8<43>) = 3;

        const fn prim() -> crate::niche_prim![u8] { 32 }

        // NOTE: the following works because the macro matches on arbitrary tokens.
        // But it wouldn't compile if it'd match over $P:ty, for example.
        macro_rules! example_macro {
            // ($P:ty) => { const fn prim3() -> crate::niche_prim![$P] { 32 } }; // would fail
            ($($P:tt)+) => { const fn prim2() -> crate::niche_prim![$($P)+] { 32 } }; // works
        }
        example_macro![NonValueU8<43>];
    }

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
