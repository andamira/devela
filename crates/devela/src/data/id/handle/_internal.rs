//
//! Defines [`__handle!`].
//

#[doc(hidden)]
#[macro_export]
macro_rules! __handle· {
    // explicit representation
    (%parse_fields
        [$($done:tt)*]
        [$field:ident : $Prim:ident + $Repr:ty; $($rest:tt)*]
        [$($tail:tt)*]
    ) => {
        $crate::__handle! { %parse_fields
            [$($done)* $field: $Prim, $Repr;] [$($rest)*] [$($tail)*]
        }
    };
    // primitive == representation
    (%parse_fields
        [$($done:tt)*]
        [$field:ident : $Prim:ident; $($rest:tt)*]
        [$($tail:tt)*]
    ) => {
        $crate::__handle! { %parse_fields
            [$($done)* $field: $Prim, $Prim;] [$($rest)*] [$($tail)*]
        }
    };
    // done
    (%parse_fields [$($done:tt)*] [] [$($tail:tt)*]) => {
        $crate::__handle! { %define [$($done)*] $($tail)* }
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
        $( $crate::__handle!(%field_impl $vis $Handle, $field: $Prim, $Repr;); )+

        /* scalar packing */
        $crate::__handle!(%pack_impl [$($field: $Prim, $Repr;)+] $vis $Handle);
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

            $crate::__handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u8);
            $crate::__handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u16);
            $crate::__handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u32);
            $crate::__handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u64);
            $crate::__handle!(%pack_methods [$($field: $Prim, $Repr;)+] $vis $Handle; u128);
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
pub use __handle· as __handle;
