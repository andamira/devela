// devela::num::grain::niche::macros
//
//! Defines [`niche!`], (`NicheNew`) and [`niche_prim!`].
//
// TOC
// - niche!
// - (NicheNew)
// - (_generate_niche_prim!) generates niche_prim!
// - tests

#[doc = crate::_tags!(num niche construction)]
/// Creates a primitive-backed niche value with compile-time type selection.
#[doc = crate::_doc_meta!{location("num/grain/niche")}]
///
/// `niche!` constructs a niche wrapper over a primitive carrier,
/// using the excluded value as a small invariant.
///
/// - `!= 0` creates a `NonZero*`.
/// - `!= MIN` creates a signed `NonMin*`.
/// - `!= MAX` creates an unsigned `NonMax*`.
/// - `!= V` creates a `NonValue*<V>`.
///
/// Use `lossy` to map a prohibited value to a nearby valid value instead of panicking.
///
/// # Examples
/// ```
/// # use devela::{niche, NonZeroU8, NonMaxU8, NonMinI8, NonValueU8};
/// let a: NonZeroU8     = niche!(42_u8;  != 0);
/// let b: NonMaxU8      = niche!(42_u8;  != MAX);
/// let c: NonMinI8      = niche!(-42_i8; != MIN);
/// let d: NonValueU8<7> = niche!(42_u8;  != 7);
///
/// // The carrier type may also be written separately.
/// let e: NonMaxU8 = niche!(42, u8; != MAX);
///
/// // Lossy construction avoids panicking on the excluded value.
/// assert_eq![niche!(lossy 0_u8;     != 0).get(), 1];
/// assert_eq![niche!(lossy u8::MAX;  != MAX).get(), u8::MAX - 1];
/// assert_eq![niche!(lossy i8::MIN;  != MIN).get(), i8::MIN + 1];
///
/// // These would fail:
/// // let x: NonMaxU32 = niche!(42; != MAX);      // needs a suffix or explicit type
/// // let y: NonMaxU8  = niche!(u8::MAX; != MAX); // strict construction rejects MAX
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! niche {
    /* lossy: specific */
    (lossy $num:expr; != 0) => {{ $crate::NicheNew($num).to_lossy_non_zero() }};
    (lossy $num:expr, $T:ty; != 0) => {{ $crate::NicheNew::<$T>($num).to_lossy_non_zero() }};
    (lossy $num:expr; != MIN) => {{ $crate::NicheNew($num).to_lossy_non_extreme() }};
    (lossy $num:expr, $T:ty; != MIN) => {{ $crate::NicheNew::<$T>($num).to_lossy_non_extreme() }};
    (lossy $num:expr; != MAX) => {{ $crate::NicheNew($num).to_lossy_non_extreme() }};
    (lossy $num:expr, $T:ty; != MAX) => {{ $crate::NicheNew::<$T>($num).to_lossy_non_extreme() }};

    /* lossy: fallback */
    (lossy $num:expr; != $V:expr) => {{ $crate::NicheNew($num).to_lossy_non_value::<{ $V }>() }};
    (lossy $num:expr, $T:ty; != $V:expr) => {{ $crate::NicheNew::<$T>($num).to_lossy_non_value::<{ $V }>() }};

    /* strict: specific */
    ($num:expr; != 0) => {{ $crate::NicheNew($num).to_non_zero() }};
    ($num:expr, $T:ty; != 0) => {{ $crate::NicheNew::<$T>($num).to_non_zero() }};
    ($num:expr; != MIN) => {{ $crate::NicheNew($num).to_non_extreme() }};
    ($num:expr, $T:ty; != MIN) => {{ $crate::NicheNew::<$T>($num).to_non_extreme() }};
    ($num:expr; != MAX) => {{ $crate::NicheNew($num).to_non_extreme() }};
    ($num:expr, $T:ty; != MAX) => {{ $crate::NicheNew::<$T>($num).to_non_extreme() }};

    /* strict: fallback */
    ($num:expr; != $V:expr) => {{ $crate::NicheNew($num).to_non_value::<{ $V }>() }};
    ($num:expr, $T:ty; != $V:expr) => {{ $crate::NicheNew::<$T>($num).to_non_value::<{ $V }>() }};
}
#[doc(inline)]
pub use niche;

#[doc = crate::_tags!(num niche construction)]
/// Private helper to construct niche types.
#[doc = crate::_doc_meta!{location("num/grain/niche")}]
#[doc(hidden)]
#[derive(Debug)]
pub struct NicheNew<T>(pub T);
macro_rules! impl_niche_new {
    () => {
        impl_niche_new!(U MAX: u8, u16, u32, u64, u128, usize);
        impl_niche_new!(I MIN: i8, i16, i32, i64, i128, isize);
    };
    ($SIGN:ident $XTR:ident: $($prim:ty),+) => { $crate::paste! {
        $( impl_niche_new!(@NZ [<NonZero $prim:camel>], $prim); )+
        $( impl_niche_new!(@NV [<NonValue $prim:camel>], $prim); )+
        $( impl_niche_new!(@NM [<Non $XTR:camel $prim:camel>], $prim, $XTR); )+
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
    (@NM $name:ident, $prim:ty, $LIMIT:ident) => {
        impl NicheNew<$prim> {
            pub const fn to_non_extreme(self) -> $crate::$name {
                if self.0 == <$prim>::$LIMIT {
                    panic![concat!("value must not be ", stringify!($LIMIT))];
                } else {
                    $crate::$name::new(self.0).unwrap()
                }
            }
            pub const fn to_lossy_non_extreme(self) -> $crate::$name {
                $crate::$name::new_lossy(self.0)
            }
        }
    };
}
impl_niche_new!();

/// Generates the `niche_prim!` macro for a closed set of primitive carrier types.
///
/// It exists solely to avoid duplication and keep the supported primitive list centralized.
///
/// About using $ as a token delimiter:
///   this part:  ($P $_d($_:tt)*) => { $P };
///   expands to: (u8 $($_:tt)*) => { u8 };
macro_rules! _generate_niche_prim {
    ($_d:tt $( $XTR:ident: $($P:ty),+ $(,)? );+ $(;)?) => { $crate::paste! {
        #[doc = crate::_tags!(niche primitive)]
        /// Maps a niche representation type to its primitive carrier type.
        #[doc = crate::_doc_meta!{location("num/grain/niche")}]
        ///
        /// `niche_prim!` performs a purely syntactic, compile-time mapping from a
        /// supported niche type (for example `NonMaxU8`, `NonValueU16<…>`,
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
        /// - `NonMaxP` (for unsigned primitives)
        /// - `NonMinP` (for signed primitives)
        /// - `NonValueP<…>`
        /// - `MaybeNiche<P>`
        /// - `MaybeNiche<NonNiche<P>>`
        /// - `MaybeNiche<NonZero<P>>` and `MaybeNiche<NonZeroP>`
        /// - `MaybeNiche<NonMaxP> (for unsigned primitives)`
        /// - `MaybeNiche<NonMinP> (for signed primitives)`
        /// - `MaybeNiche<NonValueP<…>>`
        ///
        /// The mapping is shallow and exact: only the explicitly supported spellings are matched.
        ///
        /// ## Closed-world behavior
        /// The set of recognized niche types is intentionally fixed and mirrors the
        /// niche representations provided by this crate. User-defined wrappers or
        /// arbitrary nesting are not supported.
        ///
        /// # Examples
        /// ```
        /// # use devela::{NonValueU8, niche_prim};
        /// let x: niche_prim!(NonValueU8<43>) = 3_u8;
        /// ```
        #[macro_export]
        #[cfg_attr(cargo_primary_package, doc(hidden))]
        macro_rules! niche_prim· {
            // strip one leading path segment at a time
            ($head:ident :: $_d($rest:tt)+) => { $crate::niche_prim!($_d($rest)+) };
            $($(
                ($P $_d($_:tt)*) => { $P };
                (NonNiche<$P> $_d($_:tt)*) => { $P };
                (NonZero<$P> $_d($_:tt)*) => { $P };
                ([<NonZero$P:camel>] $_d($_:tt)*) => { $P };
                ([<Non $XTR $P:camel>] $_d($_:tt)*) => { $P };
                ([<NonValue$P:camel>] $_d($_:tt)*) => { $P };
                (MaybeNiche<$P> $_d($_:tt)*) => { $P };
                (MaybeNiche<NonNiche<$P>> $_d($_:tt)*) => { $P };
                (MaybeNiche<NonZero<$P>> $_d($_:tt)*) => { $P };
                (MaybeNiche<[<NonZero$P:camel>]> $_d($_:tt)*) => { $P };
                (MaybeNiche<[<Non $XTR $P:camel>]> $_d($_:tt)*) => { $P };
                (MaybeNiche<[<NonValue$P:camel>]> $_d($_:tt)*) => { $P };
            )+)+
        }
        #[doc(inline)]
        pub use niche_prim· as niche_prim;
    }};
}
_generate_niche_prim![$ Min: i8, i16, i32, i64, i128, isize; Max: u8, u16, u32, u64, u128, usize; ];

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
        macro_rules! _example_macro {
            // ($P:ty) => { const fn prim3() -> crate::niche_prim![$P] { 32 } }; // would fail
            ($($P:tt)+) => { const fn prim2() -> crate::niche_prim![$($P)+] { 32 } }; // works
        }
        _example_macro![NonValueU8<43>];
    }

    #[test]
    fn test_niche_non0() {
        assert_eq![niche![20i32; != 0], crate::NonZeroI32::new(20).unwrap()];
        assert_eq![niche![20u8;  != 0], crate::NonZeroU8::new(20).unwrap()];
        // alternative syntax
        assert_eq![niche![20, i32; != 0], crate::NonZeroI32::new(20).unwrap()];
        assert_eq![niche![20, u8;  != 0], crate::NonZeroU8::new(20).unwrap()];
    }
    #[test]
    #[should_panic(expected = "value must not be 0")]
    fn test_niche_non0_panic() { let _panic = niche![0i64; != 0]; }

    #[test]
    fn test_niche_non_value() {
        assert_eq![niche![20u8; != 3], crate::NonValueU8::new(20).unwrap()];
        // alternative syntax
        assert_eq![niche![20, i32; != 3], crate::NonValueI32::new(20).unwrap()];
    }
    #[test]
    #[should_panic(expected = "Invalid value")]
    fn test_niche_non_value_panic() { let _panic = niche![3, i32; != 3]; }

    #[test]
    fn test_niche_non_extreme() {
        assert_eq![niche![20i8; != MIN], crate::NonMinI8::new(20).unwrap()];
        assert_eq![niche![20u8; != MAX], crate::NonMaxU8::new(20).unwrap()];
        // alternative syntax
        assert_eq![niche![20, i8; != MIN], crate::NonMinI8::new(20).unwrap()];
        assert_eq![niche![20, u8; != MAX], crate::NonMaxU8::new(20).unwrap()];
    }
    #[test]
    #[should_panic(expected = "value must not be MIN")]
    fn test_niche_non_min_panic() { let min = i8::MIN; let _panic = niche![min; != MIN]; }
    #[test]
    #[should_panic(expected = "value must not be MAX")]
    fn test_niche_non_max_panic() { let max = u8::MAX; let _panic = niche![max; != MAX]; }


}
