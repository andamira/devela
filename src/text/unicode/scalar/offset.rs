// devela/src/text/unicode/scalar/offset.rs
//
//! Defines the [`scalar_offset!`] generator macro.
//
//! A scalar-offset type stores a compact integer displacement
//! from the first Unicode scalar in a contiguous interval.
//!
//! The defining relation is: `scalar = START + offset`
//!
//! Intervals containing Unicode surrogate code points are therefore rejected.
//

#[doc = crate::_tags!(construction text)]
/// Defines a compact Unicode scalar type backed by an integer offset.
#[doc = crate::_doc_meta!{
    location("text/unicode/scalar", macro scalar_offset),
}]
/// The generated type represents every Unicode scalar in the inclusive `range`
/// as an integer displacement from its first scalar.
///
/// `offset: Prim` stores the primitive carrier directly. `offset: Prim + Repr`
/// keeps `Prim` as the semantic offset while using `Repr` as its representation,
/// allowing niche-optimized types such as [`NonMaxU8`][crate::NonMaxU8] and
/// [`NonMaxU16`][crate::NonMaxU16].
///
/// The initial implementation supports `u8` and `u16` offset carriers.
/// The selected representation must encode every offset in
/// `0..=MAX_OFFSET`; this is checked at compile time.
///
/// The scalar interval must be non-empty, fit the selected offset carrier,
/// and must not contain Unicode surrogate code points. This keeps the mapping
/// exactly `START + offset`, without skipped values or lookup tables.
///
/// # Examples
/// A full 8-bit interval uses every byte representation:
/// ```
/// # use devela::scalar_offset;
/// scalar_offset! {
///     [
///         range: '\u{2800}'..='\u{28FF}';
///         offset: u8;
///     ]
///     pub BrailleChar;
/// }
///
/// let ch = BrailleChar::try_from_offset(0x81).unwrap();
/// assert_eq!(ch.to_char(), '\u{2881}');
/// assert_eq!(ch.offset(), 0x81);
/// assert_eq!(BrailleChar::VALUES, 256);
/// ```
///
/// A smaller interval may select a niche-aware representation:
/// ```
/// # use devela::{NonMaxU8, scalar_offset};
/// scalar_offset! {
///     [
///         range: '\u{2500}'..='\u{257F}';
///         offset: u8 + NonMaxU8;
///     ]
///     pub BoxChar;
/// }
/// assert_eq!(size_of::<BoxChar>(), 1);
/// assert_eq!(size_of::<Option<BoxChar>>(), 1);
/// assert_eq!(BoxChar::try_from_char('\u{253C}').unwrap().offset(), 0x3C);
/// ```
///
/// Ranges that cross the surrogate block are rejected:
/// ```compile_fail
/// # use devela::scalar_offset;
/// scalar_offset! {
///     [range: '\u{D7FF}'..='\u{E000}'; offset: u16;]
///     InvalidScalarRange;
/// }
/// ```
///
/// The range must also fit the chosen carrier:
/// ```compile_fail
/// # use devela::scalar_offset;
/// scalar_offset! {
///     [range: '\u{2800}'..='\u{2900}'; offset: u8;]
///     TooWideForU8;
/// }
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! scalar_offset· {
    /* entry */
    (
        [
            range: $start:literal ..= $end:literal;
            offset: $Prim:ident;
        ]
        $(#[$attr:meta])*
        $vis:vis $Name:ident $(;)?
    ) => {
        $crate::scalar_offset! { %dispatch_plain
            range[$start ..= $end]
            offset[$Prim]
            attrs[$(#[$attr])*]
            vis[$vis]
            name[$Name]
        }
    };
    (
        [
            range: $start:literal ..= $end:literal;
            offset: $Prim:ident + $Repr:ty;
        ]
        $(#[$attr:meta])*
        $vis:vis $Name:ident $(;)?
    ) => {
        $crate::scalar_offset! { %dispatch
            range[$start ..= $end]
            offset[$Prim + $Repr]
            explicit_repr[true]
            attrs[$(#[$attr])*]
            vis[$vis]
            name[$Name]
        }
    };

    /* carrier dispatch */
    (%dispatch
        range[$start:literal ..= $end:literal]
        offset[u8 + $Repr:ty]
        explicit_repr[$explicit_repr:tt]
        attrs[$($attr:tt)*]
        vis[$vis:vis]
        name[$Name:ident]
    ) => {
        $crate::scalar_offset! { %define
            range[$start ..= $end]
            offset[u8 + $Repr]
            offset_max[u8::MAX as u32]
            explicit_repr[$explicit_repr]
            attrs[$($attr)*]
            vis[$vis]
            name[$Name]
        }
    };
    (%dispatch
        range[$start:literal ..= $end:literal]
        offset[u16 + $Repr:ty]
        explicit_repr[$explicit_repr:tt]
        attrs[$($attr:tt)*]
        vis[$vis:vis]
        name[$Name:ident]
    ) => {
        $crate::scalar_offset! { %define
            range[$start ..= $end]
            offset[u16 + $Repr]
            offset_max[u16::MAX as u32]
            explicit_repr[$explicit_repr]
            attrs[$($attr)*]
            vis[$vis]
            name[$Name]
        }
    };
    (%dispatch
        range[$start:literal ..= $end:literal]
        offset[$bad:ident + $Repr:ty]
        explicit_repr[$explicit_repr:tt]
        attrs[$($attr:tt)*]
        vis[$vis:vis]
        name[$Name:ident]
    ) => {
        compile_error!(concat!("scalar_offset!: unsupported offset carrier `",
            stringify!($bad), "`; expected `u8` or `u16`"));
    };
    (%dispatch_plain
        range[$start:literal ..= $end:literal]
        offset[u8]
        attrs[$($attr:tt)*]
        vis[$vis:vis]
        name[$Name:ident]
    ) => {
        $crate::scalar_offset! { %dispatch
            range[$start ..= $end]
            offset[u8 + u8]
            explicit_repr[false]
            attrs[$($attr)*]
            vis[$vis]
            name[$Name]
        }
    };
    (%dispatch_plain
        range[$start:literal ..= $end:literal]
        offset[u16]
        attrs[$($attr:tt)*]
        vis[$vis:vis]
        name[$Name:ident]
    ) => {
        $crate::scalar_offset! { %dispatch
            range[$start ..= $end]
            offset[u16 + u16]
            explicit_repr[false]
            attrs[$($attr)*]
            vis[$vis]
            name[$Name]
        }
    };
    (%dispatch_plain
        range[$start:literal ..= $end:literal]
        offset[$bad:ident]
        attrs[$($attr:tt)*]
        vis[$vis:vis]
        name[$Name:ident]
    ) => {
        compile_error!(concat!(
            "scalar_offset!: unsupported offset carrier `", stringify!($bad),
            "`; expected `u8` or `u16`"
        ));
    };

    /* normalized kernel */
    (%define
        range[$start:literal ..= $end:literal]
        offset[$Prim:ident + $Repr:ty]
        offset_max[$OFFSET_MAX:expr]
        explicit_repr[$explicit_repr:tt]
        attrs[$($attr:tt)*]
        vis[$vis:vis]
        name[$Name:ident]
    ) => {
        $($attr)*
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
        $vis struct $Name($crate::MaybeNiche<$Repr>);

        // Order by the semantic offset rather than by the niche representation.
        impl $crate::PartialOrd for $Name {
            fn partial_cmp(&self, other: &Self) -> Option<$crate::Ordering> {
                Some($crate::Ord::cmp(self, other))
            }
        }
        impl $crate::Ord for $Name {
            fn cmp(&self, other: &Self) -> $crate::Ordering {
                $crate::Ord::cmp(&self.offset(), &other.offset())
            }
        }

        #[allow(dead_code)]
        impl $Name {
            /* domain */

            /// First Unicode scalar represented by this type.
            $vis const START: char = $start;
            /// Last Unicode scalar represented by this type.
            $vis const END: char = $end;
            /// Number of Unicode scalars represented by this type.
            $vis const VALUES: u32 = Self::END as u32 - Self::START as u32 + 1;
            /// Largest semantic offset represented by this type.
            $vis const MAX_OFFSET: $Prim = (Self::VALUES - 1) as $Prim;
            /// Number of bits in the primitive offset carrier.
            $vis const OFFSET_BITS: u32 = <$Prim>::BITS;
            /// Whether the selected representation provides a memory niche.
            $vis const IS_NICHE: bool = $crate::MaybeNiche::<$Repr>::IS_NICHE;
            /// Lowest scalar represented by this type.
            $vis const MIN: Self = Self::_from_offset_invariant(0);
            /// Highest scalar represented by this type.
            $vis const MAX: Self = Self::_from_offset_invariant(Self::MAX_OFFSET);

            /* construction */

            /// Tries to construct the scalar from its Unicode `char` value.
            #[must_use]
            $vis const fn try_from_char(value: char) -> Option<Self> {
                Self::try_from_scalar(value as u32)
            }
            /// Tries to construct the scalar from its Unicode scalar value.
            #[must_use]
            $vis const fn try_from_scalar(value: u32) -> Option<Self> {
                let (start, end) = (Self::START as u32, Self::END as u32);
                if value < start || value > end { return None; }
                Self::try_from_offset((value - start) as $Prim)
            }
            /// Tries to construct the scalar from its semantic offset.
            #[must_use]
            $vis const fn try_from_offset(offset: $Prim) -> Option<Self> {
                if offset > Self::MAX_OFFSET { return None; }
                match $crate::MaybeNiche::<$Repr>::try_from_prim(offset) {
                    Ok(repr) => Some(Self(repr)),
                    Err(_) => None,
                }
            }
            /// Tries to reconstruct the scalar from its validated representation.
            #[must_use]
            $vis const fn try_from_repr(repr: $Repr) -> Option<Self> {
                let repr = $crate::MaybeNiche::<$Repr>::new(repr);
                if repr.get_prim() <= Self::MAX_OFFSET { Some(Self(repr)) } else { None }
            }

            /* projection */

            /// Returns the semantic integer offset from [`START`](Self::START).
            #[must_use]
            $vis const fn offset(self) -> $Prim { self.0.get_prim() }

            /// Returns the validated underlying representation.
            #[must_use]
            $vis const fn repr(self) -> $Repr { self.0.get() }

            /// Returns the Unicode scalar value as `u32`.
            #[must_use]
            $vis const fn to_scalar(self) -> u32 {
                Self::START as u32 + self.offset() as u32
            }
            /// Returns the Unicode scalar value as `char`.
            #[must_use]
            $vis const fn to_char(self) -> char {
                $crate::unwrap![some ::core::primitive::char::from_u32(self.to_scalar())]
            }

            /* queries */

            /// Returns whether `value` belongs to this scalar interval.
            #[must_use]
            $vis const fn contains_char(value: char) -> bool {
                Self::contains_scalar(value as u32)
            }
            /// Returns whether `value` belongs to this scalar interval.
            #[must_use]
            $vis const fn contains_scalar(value: u32) -> bool {
                value >= Self::START as u32 && value <= Self::END as u32
            }

            /* internal */

            const fn _from_offset_invariant(offset: $Prim) -> Self {
                match $crate::MaybeNiche::<$Repr>::try_from_prim(offset) {
                    Ok(repr) => Self(repr),
                    Err(_) => panic!("scalar_offset!: internal representation invariant failed"),
                }
            }
        }

        // Eagerly validate the semantic interval and the selected representation.
        const _: () = {
            let (start, end) = ($Name::START as u32, $Name::END as u32);
            assert!(start <= end, "scalar_offset!: range start must not exceed range end");
            assert!(end < $crate::Char::<u32>::SURROGATE_START
                || start > $crate::Char::<u32>::SURROGATE_END,
                "scalar_offset!: range must not contain Unicode surrogate code points"
            );
            let max_offset_u32 = end - start;
            assert!(max_offset_u32 <= $OFFSET_MAX,
                "scalar_offset!: range does not fit the selected offset carrier");
            $crate::scalar_offset! { %validate_repr
                explicit_repr[$explicit_repr] offset[$Prim + $Repr] max_u32[max_offset_u32]
            }
        };
    };

    /* representation validation */

    // With the primitive itself as representation every carrier value is valid.
    (%validate_repr
        explicit_repr[false]
        offset[$Prim:ident + $Repr:ty]
        max_u32[$max_offset_u32:ident]
    ) => {};
    // A distinct representation must admit the entire semantic offset interval.
    (%validate_repr
        explicit_repr[true]
        offset[$Prim:ident + $Repr:ty]
        max_u32[$max_offset_u32:ident]
    ) => {
        let max_offset: $Prim = $max_offset_u32 as $Prim;
        let repr_min: $Prim = $crate::MaybeNiche::<$Repr>::MIN.get_prim();
        let repr_max: $Prim = $crate::MaybeNiche::<$Repr>::MAX.get_prim();
        if $crate::MaybeNiche::<$Repr>::IS_CONTIGUOUS {
            assert!(repr_min == 0 && repr_max >= max_offset,
                "scalar_offset!: representation cannot encode every semantic offset");
        } else {
            let mut offset: $Prim = 0;
            loop {
                match $crate::MaybeNiche::<$Repr>::try_from_prim(offset) {
                    Ok(_) => {}
                    Err(_) => panic!(
                        "scalar_offset!: representation cannot encode every semantic offset"),
                }
                if offset == max_offset { break; }
                offset += 1;
            }
        }
    };

    /* diagnostics */

    ($($bad:tt)+) => {
        compile_error!("invalid scalar_offset! syntax; expected \
             `[range: START..=END; offset: Prim [+ Repr];] attrs visibility Name;`");
    };
}
#[doc(inline)]
pub use scalar_offset· as scalar_offset;

#[cfg(test)]
mod _test {
    use crate::{NonMaxU8, NonMaxU16, scalar_offset};

    scalar_offset! {
        [
            range: '\u{2800}'..='\u{28FF}';
            offset: u8;
        ]
        BrailleScalar;
    }
    scalar_offset! {
        [
            range: '\u{2500}'..='\u{257F}';
            offset: u8 + NonMaxU8;
        ]
        BoxScalar;
    }
    scalar_offset! {
        [
            range: '\u{1F300}'..='\u{1F4FF}';
            offset: u16 + NonMaxU16;
        ]
        EmojiScalar;
    }
    scalar_offset! {
        [
            range: '\u{10000}'..='\u{1FFFF}';
            offset: u16;
        ]
        FullU16Scalar;
    }
    #[test]
    fn full_u8_interval() {
        assert_eq!(BrailleScalar::VALUES, 256);
        assert_eq!(BrailleScalar::MAX_OFFSET, u8::MAX);
        assert!(!BrailleScalar::IS_NICHE);
        assert_eq!(size_of::<BrailleScalar>(), 1);
        assert_eq!(size_of::<Option<BrailleScalar>>(), 2);
        let min = BrailleScalar::try_from_offset(0).unwrap();
        let max = BrailleScalar::try_from_offset(u8::MAX).unwrap();
        assert_eq!(min, BrailleScalar::MIN);
        assert_eq!(max, BrailleScalar::MAX);
        assert_eq!(min.to_char(), '\u{2800}');
        assert_eq!(max.to_char(), '\u{28FF}');
        assert_eq!(max.to_scalar(), 0x28FF);
    }
    #[test]
    fn niche_u8_interval() {
        assert_eq!(BoxScalar::VALUES, 128);
        assert_eq!(BoxScalar::MAX_OFFSET, 127);
        assert!(BoxScalar::IS_NICHE);
        assert_eq!(size_of::<BoxScalar>(), 1);
        assert_eq!(size_of::<Option<BoxScalar>>(), 1);
        let cross = BoxScalar::try_from_char('\u{253C}').unwrap();
        assert_eq!(cross.offset(), 0x3C);
        assert_eq!(cross.to_char(), '\u{253C}');
        assert!(BoxScalar::try_from_char('\u{2580}').is_none());
        assert!(BoxScalar::try_from_offset(128).is_none());
    }
    #[test]
    fn niche_u16_interval() {
        assert_eq!(EmojiScalar::VALUES, 0x200);
        assert!(EmojiScalar::IS_NICHE);
        assert_eq!(size_of::<EmojiScalar>(), 2);
        assert_eq!(size_of::<Option<EmojiScalar>>(), 2);
        assert_eq!(EmojiScalar::MAX.to_char(), '\u{1F4FF}');
    }
    #[test]
    fn full_u16_interval() {
        assert_eq!(FullU16Scalar::VALUES, 65_536);
        assert_eq!(FullU16Scalar::MAX_OFFSET, u16::MAX);
        assert!(!FullU16Scalar::IS_NICHE);
        assert_eq!(size_of::<FullU16Scalar>(), 2);
        assert!(size_of::<Option<FullU16Scalar>>() > 2);
        assert_eq!(FullU16Scalar::MAX.to_char(), '\u{1FFFF}');
    }
    #[test]
    fn representation_roundtrip() {
        let value = BoxScalar::try_from_offset(37).unwrap();
        let repr = value.repr();
        assert_eq!(BoxScalar::try_from_repr(repr), Some(value));
    }
}
