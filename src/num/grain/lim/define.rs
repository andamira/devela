// devela/src/num/grain/lim/define.rs
//
//! Defines [`bound_int!`].
//

#[doc = crate::_tags!(construction num)]
/// Defines a bounded integer wrapper with embedded boundary metadata.
#[doc = crate::_doc_meta!{location("num/grain/lim")}]
/// The generated type stores a primitive carrier through [`MaybeNiche`][crate::MaybeNiche].
///
/// The `value_bits(...)` argument chooses how many low bits encode the payload.
/// The remaining high bits encode boundary metadata:
/// - one direction bit, reporting the last lower/upper boundary event,
/// - the remaining metadata bits as a saturating event counter.
///
/// For signed carriers, the raw carrier minimum value is reserved as invalid.
/// Non-canonical raw values with `count == 0` and a set direction bit are
/// accepted by `from_raw` but canonicalized by clearing the direction bit.
///
/// `range(...)` chooses the decoded payload interval:
///
/// - `full`: uses every payload value representable by `value_bits`.
/// - `symmetric`: for signed carriers, excludes the negative extra endpoint
///   so the range is symmetric around zero.
///
/// The default is `full`.
///
/// `symmetric` is only valid for signed carriers.
///
/// # Invariants
///
/// - `value_bits` must be at least `1` and must leave at least two metadata bits:
///   one count bit and one direction bit.
/// - Methods ending in `_meta` preserve or merge existing boundary metadata.
///   Unsuffixed methods are value-first and only record boundary events caused
///   by the operation itself.
///
/// # Operation groups
///
/// - `sat`: saturating arithmetic.
/// - `che`: checked arithmetic.
/// - `mod`: explicit-modulus arithmetic.
/// - `up`: exact arithmetic returning the upcasted primitive.
/// - `all`: all supported operation groups.
/// - `default`: `sat` and `che`.
///
/// # Syntax
/// ```ignore
/// bound_int! {
///     pub struct Name: repr(Representation => carrier);
///
///     value_bits(N);
///     range(full); // optional: full | symmetric
///     ops(sat, che, mod, up);
/// }
/// ```
///
/// # Examples
///
/// See [`BoundI8Example`][crate::BoundI8Example].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! bound_int {
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : repr($Repr:ty => $Carrier:tt);

        value_bits($VALUE_BITS:expr);
        ops($($op:ident),* $(,)?);

        $($user_impls:tt)*
    ) => {
        $crate::bound_int! {
            $(#[$attr])*
            $vis struct $Name: repr($Repr => $Carrier);
            value_bits($VALUE_BITS);
            range(full);
            ops($($op),*);
            $($user_impls)*
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : repr($Repr:ty => $Carrier:tt);

        value_bits($VALUE_BITS:expr);
        range($Range:ident);
        ops($($op:ident),* $(,)?);

        $($user_impls:tt)*
    ) => {
        $crate::bound_int!(%dispatch_carrier
            attrs[$(#[$attr])*] vis[$vis] name[$Name] repr[$Repr] carrier[$Carrier]
            value_bits[$VALUE_BITS] range[$Range] ops[$($op),*] user_impls[$($user_impls)*]
        );
    };

    // carrier validation and dispatch
    (%dispatch_carrier
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty] carrier[i8]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        $crate::bound_int!(%validate_range
            attrs[$($attr)*] vis[$vis] name[$Name] repr[$Repr]
            signed[true] carrier[i8] unsigned[u8] up[i16]
            value_bits[$VALUE_BITS] range[$Range] ops[$($op),*]
            user_impls[$($user_impls)*]
        );
    };
    (%dispatch_carrier
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty] carrier[i16]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        $crate::bound_int!(%validate_range
            attrs[$($attr)*] vis[$vis] name[$Name] repr[$Repr]
            signed[true] carrier[i16] unsigned[u16] up[i32]
            value_bits[$VALUE_BITS] range[$Range] ops[$($op),*] user_impls[$($user_impls)*]
        );
    };
    (%dispatch_carrier
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty] carrier[i32]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        $crate::bound_int!(%validate_range
            attrs[$($attr)*] vis[$vis] name[$Name] repr[$Repr]
            signed[true] carrier[i32] unsigned[u32] up[i64]
            value_bits[$VALUE_BITS] range[$Range] ops[$($op),*] user_impls[$($user_impls)*]
        );
    };
    (%dispatch_carrier
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty] carrier[i64]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        $crate::bound_int!(%validate_range
            attrs[$($attr)*] vis[$vis] name[$Name] repr[$Repr]
            signed[true] carrier[i64] unsigned[u64] up[i128]
            value_bits[$VALUE_BITS] range[$Range] ops[$($op),*] user_impls[$($user_impls)*]
        );
    };
    // TODO: unsigned dispatch
    (%dispatch_carrier
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty] carrier[$bad:tt]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        compile_error!(concat!("bound_int!: unsupported carrier `",
            stringify!($bad), "`; currently only `i8`, `i16`, `i32` and `i64` are implemented"));
    };

    // range validation / normalization
    (%validate_range
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty]
        signed[$signed:tt] carrier[$Carrier:ty] unsigned[$Unsigned:ty] up[$Up:ty]
        value_bits[$VALUE_BITS:expr] range[full] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        $crate::bound_int!(%validate_range_backend
            attrs[$($attr)*] vis[$vis] name[$Name] repr[$Repr]
            signed[$signed] carrier[$Carrier] unsigned[$Unsigned] up[$Up]
            value_bits[$VALUE_BITS] range[full] range_requires_signed[false]
            ops[$($op),*] user_impls[$($user_impls)*]
        );
    };
    (%validate_range
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty]
        signed[$signed:tt] carrier[$Carrier:ty] unsigned[$Unsigned:ty] up[$Up:ty]
        value_bits[$VALUE_BITS:expr] range[symmetric] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        $crate::bound_int!(%validate_range_backend
            attrs[$($attr)*] vis[$vis] name[$Name] repr[$Repr]
            signed[$signed] carrier[$Carrier] unsigned[$Unsigned] up[$Up]
            value_bits[$VALUE_BITS] range[symmetric] range_requires_signed[true]
            ops[$($op),*] user_impls[$($user_impls)*]
        );
    };
    (%validate_range
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty]
        signed[$signed:tt] carrier[$Carrier:ty] unsigned[$Unsigned:ty] up[$Up:ty]
        value_bits[$VALUE_BITS:expr] range[$bad:ident] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        compile_error!(concat!("bound_int!: unknown range mode `", stringify!($bad),
            "`; expected `full` or `symmetric`"));
    };

    // semantic compatibility validation
    (%validate_range_backend
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty]
        signed[false] carrier[$Carrier:ty] unsigned[$Unsigned:ty] up[$Up:ty]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] range_requires_signed[true]
        ops[$($op:ident),*] user_impls[$($user_impls:tt)*]
    ) => {
        compile_error!(concat!("bound_int!: range(`", stringify!($Range),
            "`) requires a signed carrier"));
    };
    (%validate_range_backend
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty]
        signed[$signed:tt] carrier[$Carrier:ty] unsigned[$Unsigned:ty] up[$Up:ty]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] range_requires_signed[$requires_signed:tt]
        ops[$($op:ident),*] user_impls[$($user_impls:tt)*]
    ) => {
        $crate::bound_int!(%dispatch
            attrs[$($attr)*] vis[$vis] name[$Name] repr[$Repr]
            signed[$signed] carrier[$Carrier] unsigned[$Unsigned] up[$Up]
            value_bits[$VALUE_BITS] range[$Range]
            ops[$($op),*] user_impls[$($user_impls)*]
        );
    };

    // main dispatch
    (%dispatch
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty]
        signed[true] carrier[$Carrier:ty] unsigned[$Unsigned:ty] up[$Up:ty]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        $crate::bound_int!(%impl_common
            attrs[$($attr)*] vis[$vis] name[$Name] repr[$Repr]
            signed[true] carrier[$Carrier] unsigned[$Unsigned] up[$Up]
            value_bits[$VALUE_BITS] range[$Range] ops[$($op),*] user_impls[$($user_impls)*]
        );
        $crate::__bound_int_impl_signed!(
            attrs[$($attr)*] vis[$vis] name[$Name]
            repr[$Repr] carrier[$Carrier] unsigned[$Unsigned] up[$Up]
            value_bits[$VALUE_BITS] range[$Range] ops[$($op),*] user_impls[$($user_impls)*]
        );
    };
    // TODO: unsigned
    // (%dispatch
    //     attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty]
    //     signed[false] carrier[$Carrier:ty] unsigned[$Unsigned:ty] up[$Up:ty]
    //     value_bits[$VALUE_BITS:expr] range[$Range:ident] ops[$($op:ident),*]
    //     user_impls[$($user_impls:tt)*]
    // ) => {
    //     $crate::bound_int!(%impl_common
    //         attrs[$($attr)*] vis[$vis] name[$Name] repr[$Repr]
    //         signed[false] carrier[$Carrier] unsigned[$Unsigned] up[$Up]
    //         value_bits[$VALUE_BITS] range[$Range] ops[$($op),*] user_impls[$($user_impls)*]
    //     );
    //     $crate::__bound_int_impl_unsigned!(
    //         attrs[$($attr)*] vis[$vis] name[$Name]
    //         repr[$Repr] carrier[$Carrier] unsigned[$Unsigned] up[$Up]
    //         value_bits[$VALUE_BITS] range[$Range] ops[$($op),*] user_impls[$($user_impls)*]
    //     );
    // };

    // WIP
    (%impl_common
        attrs[$($attr:tt)*] vis[$vis:vis] name[$Name:ident] repr[$Repr:ty]
        signed[$signed:tt] carrier[$Carrier:ty] unsigned[$Unsigned:ty] up[$Up:ty]
        value_bits[$VALUE_BITS:expr] range[$Range:ident] ops[$($op:ident),*]
        user_impls[$($user_impls:tt)*]
    ) => {
        $($attr)*
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug)]
        $vis struct $Name($crate::MaybeNiche<$Repr>);

        // Common public constants and methods
        impl $Name {
            /* constants */

            /// Number of bits in the primitive carrier.
            pub const CARRIER_BITS: u32 = <$Carrier>::BITS;
            /// Number of bits used by the signed payload.
            pub const VALUE_BITS: u32 = ($VALUE_BITS) as u32;
            /// Number of high bits used by boundary metadata.
            pub const META_BITS: u32 = Self::CARRIER_BITS - Self::VALUE_BITS;
            /// Number of metadata bits used by the boundary direction marker.
            pub const DIR_BITS: u32 = 1;
            /// Number of metadata bits used by the saturating boundary-event counter.
            pub const COUNT_BITS: u32 = Self::META_BITS - 1;
            /// Maximum representable boundary-event count.
            pub const MAX_COUNT: $Unsigned = (1 as $Unsigned << Self::COUNT_BITS) - 1;

            /// Whether the payload carrier is signed.
            pub const IS_SIGNED: bool = $crate::cif!(same($signed, true));
            /// Whether the signed payload range is symmetric around zero.
            pub const IS_SYMMETRIC: bool = $crate::cif!(same($Range, symmetric));

            /// Minimum representable payload value.
            pub const MIN_VALUE: $Carrier = Self::_MIN_VALUE;
            /// Maximum representable payload value.
            pub const MAX_VALUE: $Carrier = Self::_MAX_VALUE;

            /// Zero, with empty metadata.
            pub const ZERO: Self = {
                let _ = Self::_CHECK_INVARIANTS;
                Self::from_value_meta(0, 0, false)
            };
            /// Minimum payload value, with empty metadata.
            pub const MIN: Self = Self::from_value_meta(Self::MIN_VALUE, 0, false);
            /// Maximum payload value, with empty metadata.
            pub const MAX: Self = Self::from_value_meta(Self::MAX_VALUE, 0, false);

            /* constructors */

            /// Creates a value, saturating to the payload range.
            ///
            /// If `value` is below [`MIN_VALUE`](#associatedconstant.MIN_VALUE),
            /// the result is clamped to `MIN` and records one lower-boundary event.
            ///
            /// If `value` is above [`MAX_VALUE`](#associatedconstant.MAX_VALUE),
            /// the result is clamped to `MAX` and records one upper-boundary event.
            pub const fn new(value: $Carrier) -> Self {
                if value < Self::MIN_VALUE { Self::from_value_meta(Self::MIN_VALUE, 1, false) }
                else if value > Self::MAX_VALUE { Self::from_value_meta(Self::MAX_VALUE, 1, true) }
                else { Self::from_value_meta(value, 0, false) }
            }
            /// Creates a value if it fits in the payload range.
            pub const fn new_checked(value: $Carrier) -> Option<Self> {
                if value < Self::MIN_VALUE || value > Self::MAX_VALUE { None }
                else { Some(Self::from_value_meta(value, 0, false)) }
            }

            /// Creates a value saturated to the payload range.
            pub const fn new_saturated(value: $Carrier) -> Self {
                let value = $crate::cmp!(clamp value, Self::MIN_VALUE, Self::MAX_VALUE);
                Self::from_value_meta(value, 0, false)
            }
            /// Creates a value saturated to the payload range, from an upcasted carrier.
            pub const fn new_saturated_up(value: $Up) -> Self {
                Self::new_saturated($crate::cast!(saturating value => $Carrier))
            }

            /// Creates a value from a raw encoded carrier.
            ///
            /// Returns `None` if `raw` is reserved, invalid for the underlying representation,
            /// or decodes outside the payload range.
            ///
            /// Non-canonical raw encodings with `count == 0` are canonicalized so
            /// the direction bit is cleared.
            pub const fn from_raw(raw: $Carrier) -> Option<Self> {
                if raw == Self::_RESERVED_RAW { return None; }
                let _ = $crate::unwrap![ok_some? $crate::MaybeNiche::<$Repr>::try_from_prim(raw)];
                let (value, count) = (Self::decode_value(raw), Self::decode_count(raw));
                // Only `symmetric` excludes a value that the signed payload decoder can produce.
                // `full` accepts the complete decoded payload domain.
                // Custom ranges must add their own validation arm.
                if $crate::cif!(same($Range, symmetric)) && !Self::decoded_value_fits(value) {
                    return None }
                let dir_upper = count != 0 && Self::decode_dir_upper(raw);
                let raw = Self::encode_raw(value, count, dir_upper);
                let res = $crate::unwrap![ok_some? $crate::MaybeNiche::<$Repr>::try_from_prim(raw)];
                Some(Self(res))
            }

            /* raw/meta methods */

            /// Returns the raw encoded carrier.
            pub const fn raw(self) -> $Carrier { self.0.prim() }
            /// Returns the decoded payload value.
            pub const fn get(self) -> $Carrier { Self::decode_value(self.raw()) }

            /// Returns whether this value carries at least one boundary event.
            pub const fn is_bounded(self) -> bool { self.bound_count() != 0 }

            /// Returns the saturating boundary-event count.
            ///
            /// A value of `0` means no boundary event is recorded. When this is `0`,
            /// [`bound_dir`](#method.bound_dir) returns `None`.
            pub const fn bound_count(self) -> $Unsigned { Self::decode_count(self.raw()) }

            /// Returns the last recorded boundary direction.
            ///
            /// Returns `None` when [`bound_count`](#method.bound_count) is `0`.
            pub const fn bound_dir(self) -> Option<$crate::Boundary1d> {
                if self.bound_count() == 0 { None }
                else if Self::decode_dir_upper(self.raw()) { Some($crate::Boundary1d::Upper) }
                else { Some($crate::Boundary1d::Lower) }
            }
            /// Returns the same payload value with empty metadata.
            pub const fn clear_meta(self) -> Self {
                Self::from_value_meta(self.get(), 0, false)
            }
            /// Returns the same payload value with explicit boundary metadata.
            ///
            /// `count` saturates to [`MAX_COUNT`](#associatedconstant.MAX_COUNT).
            ///
            /// When `count == 0`, `dir` is ignored
            /// and the raw direction bit is canonicalized to zero.
            pub const fn with_bound_meta(self, count: $Unsigned, dir: Option<$crate::Boundary1d>)
                -> Self {
                let dir_upper = match dir { Some($crate::Boundary1d::Upper) => true, _ => false };
                Self::from_value_meta(self.get(), count, dir_upper)
            }

            /* sign */

            // TODO: allow lints
            /// Whether the value is less than zero.
            pub const fn is_negative(self) -> bool { self.get() < 0 }
            /// Whether the value is greater than zero.
            pub const fn is_positive(self) -> bool { self.get() > 0 }
            /// Whether the value is zero.
            pub const fn is_zero(self) -> bool { self.get() == 0 }
            /// Whether the value is greater than or equal to zero.
            pub const fn is_nonnegative(self) -> bool { self.get() >= 0 }
            /// Whether the value is less than or equal to zero.
            pub const fn is_nonpositive(self) -> bool { self.get() <= 0 }

            /* ordering */

            /// Equality comparison over decoded payload values.
            pub const fn eq(self, other: Self) -> bool { self.get() == other.get() }

            /// Compares decoded payload values.
            pub const fn cmp(self, other: Self) -> $crate::Ordering {
                if self.get() < other.get() { $crate::Ordering::Less }
                else if self.get() > other.get() { $crate::Ordering::Greater }
                else { $crate::Ordering::Equal }
            }

            /// Returns the greater decoded value.
            ///
            /// Metadata is cleared because the result is recomputed.
            pub const fn max(self, other: Self) -> Self {
                Self::from_value_nometa(Self::max_carrier(self.get(), other.get()))
            }
            /// Returns the greater value, preserving the selected operand metadata.
            pub const fn max_meta(self, other: Self) -> Self {
                if self.get() >= other.get() { self } else { other }
            }

            /// Returns the decoded value floored at zero.
            ///
            /// Metadata is cleared because the result is recomputed.
            pub const fn max_zero(self) -> Self {
                Self::from_value_nometa(Self::max_carrier(self.get(), 0))
            }
            /// Returns the decoded value floored at zero, preserving selected metadata.
            pub const fn max_zero_meta(self) -> Self { self.max_meta(Self::ZERO) }

            /// Returns the lesser decoded value.
            ///
            /// Metadata is cleared because the result is recomputed.
            pub const fn min(self, other: Self) -> Self {
                Self::from_value_nometa(Self::min_carrier(self.get(), other.get()))
            }
            /// Returns the lesser value, preserving the selected operand metadata.
            pub const fn min_meta(self, other: Self) -> Self {
                if self.get() <= other.get() { self } else { other }
            }

            /// Returns the decoded value capped at zero.
            ///
            /// Metadata is cleared because the result is recomputed.
            pub const fn min_zero(self) -> Self {
                Self::from_value_nometa(Self::min_carrier(self.get(), 0))
            }
            /// Returns the decoded value capped at zero, preserving selected metadata.
            pub const fn min_zero_meta(self) -> Self { self.min_meta(Self::ZERO) }

            /// Clamps the decoded value between `min` and `max`.
            ///
            /// Metadata is cleared because the result is recomputed.
            ///
            /// If `min > max`, this returns `min`.
            pub const fn clamp(self, min: Self, max: Self) -> Self {
                Self::from_value_nometa(Self::clamp_carrier(self.get(), min.get(), max.get()))
            }
            /// Clamps `self` between `min` and `max`, preserving selected metadata.
            ///
            /// If `min > max`, this returns `min`.
            pub const fn clamp_meta(self, min: Self, max: Self) -> Self {
                if self.get() < min.get() { min }
                else if self.get() > max.get() { max } else { self }
            }
        }

        // Common private helpers
        impl $Name {
            /* constants */

            const _CHECK_INVARIANTS: () = {
                assert!(Self::VALUE_BITS >= 1, "bound_int!: value_bits must be at least 1");
                assert!(Self::VALUE_BITS <= Self::CARRIER_BITS - 2,
                    "bound_int!: value_bits must leave at least 2 metadata bits");
                assert!(Self::META_BITS >= 2,
                    "bound_int!: metadata requires at least 1 count bit and 1 direction bit");
                assert!(Self::COUNT_BITS >= 1,
                    "bound_int!: boundary count requires at least 1 bit");
            };

            const VALUE_MASK: $Unsigned = (1 as $Unsigned << Self::VALUE_BITS) - 1;
            const COUNT_MASK: $Unsigned = (1 as $Unsigned << Self::COUNT_BITS) - 1;
            const DIR_BIT: $Unsigned = 1 as $Unsigned << Self::COUNT_BITS;

            /* constructors */

            /// Creates a value from a canonical raw carrier.
            ///
            /// The caller must only pass a raw pattern emitted by [`encode_raw`].
            const fn from_canonical_raw(raw: $Carrier) -> Self {
                match $crate::MaybeNiche::<$Repr>::try_from_prim(raw) {
                    Ok(repr) => Self(repr),
                    Err(_) => unreachable!(),
                }
            }
            /// Creates a metadata-empty value from an already in-range payload.
            const fn from_value_nometa(value: $Carrier) -> Self {
                Self::from_value_meta(value, 0, false)
            }
            const fn from_value_meta(value: $Carrier, count: $Unsigned, dir_upper: bool) -> Self {
                let raw = Self::encode_raw(value, count, dir_upper);
                Self::from_canonical_raw(raw)
            }

            /* meta */

            const fn encode_raw(value: $Carrier, count: $Unsigned, dir_upper: bool) -> $Carrier {
                let count = if count > Self::MAX_COUNT { Self::MAX_COUNT } else { count };
                // Canonical rule: direction is absent when count == 0.
                let dir = if count == 0 { 0 } else if dir_upper { Self::DIR_BIT } else { 0 };
                let meta = ((count as $Unsigned) & Self::COUNT_MASK) | dir;
                let bits = (meta << Self::VALUE_BITS) | ((value as $Unsigned) & Self::VALUE_MASK);
                bits as $Carrier
            }
            const fn decode_count(raw: $Carrier) -> $Unsigned {
                (((raw as $Unsigned) >> Self::VALUE_BITS) & Self::COUNT_MASK)
            }
            const fn decode_dir_upper(raw: $Carrier) -> bool {
                ((((raw as $Unsigned) >> Self::VALUE_BITS) & Self::DIR_BIT) != 0)
            }
            // less branchy: `value < Self::MIN_VALUE || value > Self::MAX_VALUE`
            const fn decoded_value_fits(value: $Carrier) -> bool {
                ((value - Self::MIN_VALUE) as $Unsigned)
                    <= ((Self::MAX_VALUE - Self::MIN_VALUE) as $Unsigned)
            }

            /* arithmetic ops */

            // Safe from carrier overflow: COUNT_BITS <= CARRIER_BITS - 2, so
            // MAX_COUNT + MAX_COUNT + MAX_COUNT fits in the unsigned carrier.
            const fn count_inc(a: $Unsigned, b: $Unsigned, extra: $Unsigned) -> $Unsigned {
                let sum = (a + b + extra);
                if sum > Self::MAX_COUNT { Self::MAX_COUNT } else { sum }
            }
            const fn merged_dir_upper(a: Self, b: Self) -> bool {
                if b.bound_count() != 0 { Self::decode_dir_upper(b.raw()) }
                else if a.bound_count() != 0 { Self::decode_dir_upper(a.raw()) }
                else { false }
            }

            /* ordering / clamping */

            /// Returns the greater carrier value.
            ///
            /// Safe for the ranges produced by this bounded type.
            const fn max_carrier(x: $Carrier, y: $Carrier) -> $Carrier {
                (x + y + (x - y).abs()) / 2
            }
            /// Returns the lesser carrier value.
            ///
            /// Safe for the ranges produced by this bounded type.
            const fn min_carrier(x: $Carrier, y: $Carrier) -> $Carrier {
                (x + y - (x - y).abs()) / 2
            }
            /// Clamps `x` between `min` and `max`.
            ///
            /// If `min > max`, this returns `min`.
            const fn clamp_carrier(x: $Carrier, min: $Carrier, max: $Carrier) -> $Carrier {
                Self::max_carrier(min, Self::min_carrier(x, max))
            }
            /// Clamps a carrier value to the payload range.
            ///
            /// The input must be in the arithmetic range produced
            /// by valid payload addition, subtraction, or distance.
            const fn clamp_carrier_to_value(x: $Carrier) -> $Carrier {
                Self::clamp_carrier(x, Self::MIN_VALUE, Self::MAX_VALUE)
            }
        }

        /* common traits */

        impl Default for $Name { fn default() -> Self { Self::ZERO } }
        $crate::_impl_init![Self::ZERO => $Name];
        $crate::impl_trait![fmt::Display for $Name |self, f| self.get().fmt(f)];

        impl Eq for $Name {}
        impl PartialEq for $Name {
            fn eq(&self, other: &Self) -> bool { self.get().eq(&other.get()) }
        }
        // Enables symmetric comparisons with the primitive carrier.
        impl PartialEq<$Carrier> for $Name {
            fn eq(&self, other: &$Carrier) -> bool { self.get() == *other }
        }
        // NOTE: this can expose ambiguous inference in tests using untyped literals.
        impl PartialEq<$Name> for $Carrier {
            fn eq(&self, other: &$Name) -> bool { *self == other.get() }
        }
        impl PartialOrd for $Name {
            fn partial_cmp(&self, other: &Self) -> Option<$crate::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for $Name {
            fn cmp(&self, other: &Self) -> $crate::Ordering { self.get().cmp(&other.get()) }
        }
        impl $crate::Hash for $Name {
            fn hash<H: $crate::Hasher>(&self, state: &mut H) { self.get().hash(state); }
        }
    };

    // emit: user impl blocks
    (%emit_user_impls $Name:ident;) => {};
    (%emit_user_impls
        $Name:ident;
        $(#[$impl_attr:meta])*
        impl { $($user_impl:item)* }
        $($rest:tt)*
    ) => {
        $(#[$impl_attr])*
        impl $Name { $($user_impl)* }
        $crate::bound_int!(%emit_user_impls $Name; $($rest)*);
    };
    (%emit_user_impls $Name:ident; $($bad:tt)+) => {
        compile_error!("bound_int!: expected optional `impl { ... }` block after `ops(...)`");
    };
}
#[doc(inline)]
pub use bound_int;
