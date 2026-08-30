// devela/src/data/id/handle/define.rs
//
//! Defines [`handle!`] macro.
//

#[doc = crate::_tags!(construction uid)]
/// Defines a compact handle from arbitrary numeric components.
#[doc = crate::_doc_meta!{
    location("data/id/handle", macro handle),
}]
/// Each declared component is stored through [`MaybeNiche`], allowing primitive
/// and niche-aware integer representations to share the same generated API.
/// Component names have no built-in semantics: the receiving store or resolver
/// decides what fields such as `index`, `generation`, `revision`, or `scope` mean.
///
/// Constructors validate only the numeric representation of each component.
/// They do not establish whether the resulting handle
/// resolves to live or valid external state.
///
/// The generated handle is copyable, contains no references or ownership, and
/// exposes checked conversion to and from each component's primitive carrier.
///
/// # Syntax
/// Each component declares its primitive carrier and may optionally declare
/// a distinct storage representation:
///
/// - `name: Prim;` uses `Prim` as both carrier and representation.
/// - `name: Prim + Repr;` uses `Prim` as the carrier and `Repr` as the representation.
///
/// Primitive carriers are used by `from_prim`, `*_prim`, and `into_prim`.
/// Representations are used by `new`, direct component accessors, and `into_parts`.
///
/// Each representation must be supported by [`MaybeNiche`]
/// with the declared primitive carrier.
///
/// # Example
/// ```
/// # use devela::{NonMaxU16, handle};
/// handle! {
///     [
///         index: u16 + NonMaxU16;
///         revision: u16;
///     ]
///     /// A compact versioned reference.
///     pub EntityRef;
/// }
///
/// let handle = EntityRef::from_prim(7, 3).unwrap();
/// assert_eq!(handle.get_index_prim(), 7);
/// assert_eq!(handle.get_revision_prim(), 3);
/// assert_eq!(handle.into_prim(), (7, 3));
/// ```
///
/// See also [`HandleExample`].
///
/// [`MaybeNiche`]: crate::MaybeNiche
/// [`HandleExample`]: crate::HandleExample
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! handle {
    (
        [ $($fields:tt)* ]
        $(#[$attr:meta])*
        $vis:vis $Handle:ident $(;)?
    ) => {
        $crate::handle! { %parse_fields
            [] [$($fields)*] [$(#[$attr])* $vis $Handle]
        }
    };
    // explicit representation
    (%parse_fields
        [$($done:tt)*]
        [$field:ident : $Prim:ident + $Repr:ty; $($rest:tt)*]
        [$($tail:tt)*]
    ) => {
        $crate::handle! { %parse_fields
            [$($done)* $field: $Prim, $Repr;] [$($rest)*] [$($tail)*]
        }
    };
    // primitive == representation
    (%parse_fields
        [$($done:tt)*]
        [$field:ident : $Prim:ident; $($rest:tt)*]
        [$($tail:tt)*]
    ) => {
        $crate::handle! { %parse_fields
            [$($done)* $field: $Prim, $Prim;] [$($rest)*] [$($tail)*]
        }
    };
    // done
    (%parse_fields [$($done:tt)*] [] [$($tail:tt)*]) => {
        $crate::handle! { %define [$($done)*] $($tail)* }
    };
    // catch-all
    (%parse_fields [$($done:tt)*] [$($bad:tt)+] [$($tail:tt)*]) => {
        compile_error!("invalid handle field; expected `name: Prim;` or `name: Prim + Repr;`");
    };
    // normalized kernel
    (%define
        [ $( $field:ident : $Prim:ident, $Repr:ty; )+ ]
        $(#[$attr:meta])*
        $vis:vis $Handle:ident $(;)?
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $Handle {
            $( $field: $crate::MaybeNiche<$Repr>, )+
        }

        impl $crate::Debug for $Handle {
            fn fmt(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                f.debug_struct(stringify!($Handle))
                    $( .field(stringify!($field), &self.$field.get_prim()) )+
                    .finish()
            }
        }

        /// # Core handle methods
        ///
        /// Construction, decomposition and const-compatible comparison.
        #[allow(dead_code)]
        impl $Handle {
            /* constructors */

            /// Creates a new handle from its representation components.
            #[must_use]
            $vis const fn new( $($field: $Repr),+ ) -> Self {
                $( let $field = $crate::MaybeNiche::<$Repr>::new($field); )+
                Self { $($field),+ }
            }
            /// Creates a new handle from primitive carrier components.
            ///
            /// Returns an error if any component violates its representation invariant.
            $vis const fn from_prim( $( $field: $Prim, )+ ) -> Result<Self, $crate::InvalidValue> {
                $(
                    let $field = $crate::unwrap![ok?
                        $crate::MaybeNiche::<$Repr>::try_from_prim($field)];
                )+
                Ok(Self { $($field),+ })
            }
            /// Creates a new handle from `usize` components.
            ///
            /// Returns an error if any value does not fit its primitive carrier or
            /// violates its representation invariant.
            $vis const fn try_from_usize($($field: usize),+)
                -> Result<Self, $crate::NicheValueError> {
                $(
                    let $field = $crate::unwrap![ok?
                        $crate::MaybeNiche::<$Repr>::try_from_usize($field)];
                )+
                Ok(Self { $($field),+ })
            }

            /* decomposition */

            /// Returns the representation components in declaration order.
            #[must_use]
            $vis const fn into_parts(self) -> ($($Repr,)+) {
                ($(self.$field.get(),)+)
            }
            /// Returns the primitive carrier components in declaration order.
            #[must_use]
            $vis const fn into_prim(self) -> ($($Prim,)+) {
                ($(self.$field.get_prim(),)+)
            }

            /* comparison */

            /// Returns whether this handle has the same component values as `other`.
            #[must_use]
            $vis const fn eq(self, other: Self) -> bool {
                true $(&& self.$field.get_prim() == other.$field.get_prim())+
            }
        }
        /* component accessors */
        $( $crate::handle!(%field_impl $vis $Handle, $field: $Prim, $Repr;); )+

        /* scalar packing */
        $crate::handle!(%pack_impl [$($field: $Prim, $Repr;)+] $vis $Handle);
    };
    (%field_impl $vis:vis $Handle:ident, $field:ident : $Prim:ty, $Repr:ty;) => { $crate::paste! {
        #[doc = "# Methods for `" $field "`"]
        #[allow(dead_code)]
        impl $Handle {
            #[doc = "Returns the `" $field "` component."]
            #[must_use]
            $vis const fn [<get_ $field>](self) -> $Repr { self.$field.get() }

            #[doc = "Returns the `" $field "` component as its primitive carrier."]
            #[must_use]
            $vis const fn [<get_ $field _prim>](self) -> $Prim {
                self.$field.get_prim()
            }
            #[doc = "Returns the `" $field "` component as a `usize`."]
            ///
            /// # Errors
            /// Returns an error if the component cannot fit in a `usize`.
            $vis const fn [<get_ $field _usize>](self) -> Result<usize, $crate::Overflow> {
                self.$field.try_to_usize()
            }
        }
    }};
    // Implements scalar packing over all primitive handle components.
    (%pack_impl
        [ $( $field:ident : $Prim:ident, $Repr:ty; )+ ]
        $vis:vis $Handle:ident
    ) => {
        // Canonical Word representation
        $crate::word! {
            impl $Handle => ($($Repr,)+) {
                raw(handle) { handle.into_parts() }
                from_raw(raw) {
                    let ($($field,)+) = raw;
                    Self::new($($field),+)
                }
            }
        }

        /// # Scalar packing
        ///
        /// Packs the primitive component bits into unsigned scalar carriers.
        #[allow(dead_code)]
        impl $Handle {
            /// The total bit width of all primitive components.
            ///
            /// Components occupy bits from low to high in declaration order.
            $vis const PACK_BITS: u32 = 0 $(+ <$Prim>::BITS)+;

            $crate::handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u8);
            $crate::handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u16);
            $crate::handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u32);
            $crate::handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u64);
            $crate::handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u128);
        }
    };
    // Implements packing to and unpacking from one unsigned scalar carrier.
    (%pack_methods
        [ $( $field:ident : $Prim:ident, $Repr:ty; )+ ]
        $vis:vis $Handle:ident;
        $Pack:ident
    ) => { $crate::paste! {
        #[doc = "Tries to pack this handle into a `" $Pack "`."]
        ///
        /// Primitive component bits are placed from low to high
        /// in declaration order.
        ///
        /// Returns `None` if the complete value cannot be represented
        /// by the target scalar.
        $vis const fn [<try_pack_ $Pack>](self) -> Option<$Pack> {
            let mut packed: $Pack = 0;
            let mut shift = 0u32;
            $(
                let value: $Prim = self.$field.get_prim();
                let available = <$Pack>::BITS.saturating_sub(shift);
                if available == 0 {
                    if value != 0 { return None; }
                } else {
                    // If this component extends beyond the target,
                    // all omitted high bits must be zero.
                    if available < <$Prim>::BITS && (value >> available) != 0 { return None; }
                    // Keep exactly this primitive component's low bits.
                    //
                    // saturating_sub avoids an invalid constant subtraction
                    // when Prim is wider than Pack.
                    let mask = <$Pack>::MAX >> <$Pack>::BITS.saturating_sub(<$Prim>::BITS);
                    packed |= ((value as $Pack) & mask) << shift;
                }
                shift = shift.saturating_add(<$Prim>::BITS);
            )+
            let _ = shift;
            Some(packed)
        }
        #[doc = "Tries to reconstruct this handle from a packed `" $Pack "`."]
        ///
        /// Primitive component bits are read from low to high in declaration order.
        ///
        /// # Errors
        /// Returns an error if extra bits are set or a component violates its invariant.
        $vis const fn [<try_unpack_ $Pack>](packed: $Pack) -> Result<Self, $crate::InvalidValue> {
            // A wider scalar must not carry information beyond this handle.
            if Self::PACK_BITS < <$Pack>::BITS && (packed >> Self::PACK_BITS) != 0 {
                return Err($crate::InvalidValue);
            }
            let mut shift = 0u32;
            $(
                let $field: $Prim = if shift >= <$Pack>::BITS {
                    0
                } else {
                    let available = <$Pack>::BITS - shift;
                    let width = if <$Prim>::BITS < available { <$Prim>::BITS } else { available };
                    let mask = <$Pack>::MAX >> (<$Pack>::BITS - width);
                    ((packed >> shift) & mask) as $Prim
                };
                shift = shift.saturating_add(<$Prim>::BITS);
            )+
            let _ = shift;
            Self::from_prim($($field),+)
        }
    }};
}
#[doc(inline)]
pub use handle;

#[cfg(test)]
mod _test {
    use crate::{HandleExample, PoolHandleExample, Value32, ValueKind4, const_assert, unwrap};

    #[test]
    fn components() {
        let handle = HandleExample::from_prim(7, 15, 3).unwrap();
        assert_eq![handle.get_index_prim(), 7];
        assert_eq![handle.get_kind_prim(), 15];
        assert_eq![handle.get_revision_prim(), 3];
        assert_eq![handle.get_index_usize(), Ok(7)];
        assert_eq![handle.get_kind_usize(), Ok(15)];
        assert_eq![handle.get_revision_usize(), Ok(3)];
        assert_eq![handle.into_prim(), (7, 15, 3)];
        let (index, kind, revision) = handle.into_parts();
        assert_eq![index.get(), 7];
        assert_eq![kind, 15];
        assert_eq![revision, 3];
        let raw = handle.raw();
        assert_eq!(HandleExample::from_raw(raw), handle);
        assert_eq!(HandleExample::try_from_raw(raw), Ok(handle));
    }
    #[test]
    fn rejects_invalid_niche_value() {
        assert![HandleExample::from_prim(u8::MAX, 0, 0).is_err()];
    }
    #[test]
    fn scalar_pack_roundtrip() {
        let handle = HandleExample::from_prim(7, 15, 3).unwrap();
        assert_eq!(HandleExample::PACK_BITS, 32);
        let packed = handle.try_pack_u32().unwrap();
        assert_eq!(packed, 0x0003_0F07);
        assert_eq!(HandleExample::try_unpack_u32(packed), Ok(handle));
    }
    #[test]
    fn packed_can_use_a_smaller_carrier() {
        let handle = HandleExample::from_prim(7, 3, 0).unwrap();
        let packed = handle.try_pack_u16().unwrap();
        assert_eq!(packed, 0x0307);
        assert_eq!(HandleExample::try_unpack_u16(packed), Ok(handle));
    }
    #[test]
    fn packed_bridges_to_value_payload() {
        let handle = PoolHandleExample::from_prim(7, 3).unwrap();
        // u8 index + u16 generation.
        assert_eq!(PoolHandleExample::PACK_BITS, 24);
        assert!(PoolHandleExample::PACK_BITS <= Value32::PAYLOAD_BITS);
        let payload = handle.try_pack_u32().unwrap();
        let value = Value32::try_from_parts(ValueKind4::Ref, payload).unwrap();
        assert_eq!(value.kind(), ValueKind4::Ref);
        let decoded = PoolHandleExample::try_unpack_u32(value.payload()).unwrap();
        assert_eq!(decoded, handle);
    }
    #[test]
    fn unpack_rejects_noncanonical_pack() {
        // HandleExample owns only 32 packed bits.
        assert!(HandleExample::try_unpack_u64(1_u64 << 40).is_err());
        // index is NonMaxU8.
        assert!(HandleExample::try_unpack_u32(0x0000_00FF).is_err());
    }
    #[test]
    const fn scalar_pack_is_const() {
        const H: HandleExample = unwrap![ok HandleExample::from_prim(7, 15, 3)];
        const P: u32 = unwrap![some H.try_pack_u32()];
        const H2: HandleExample = unwrap![ok HandleExample::try_unpack_u32(P)];
        const_assert!(eq P, 0x0003_0F07);
        const_assert!(H.eq(H2));
    }
}
