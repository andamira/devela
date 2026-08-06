// devela/src/data/id/handle/span.rs
//
//! Defines [`handle_span!`] macro.
//

#[doc = crate::_tags!(construction uid)]
/// Defines a compact handle for a contiguous span.
#[doc = crate::_doc_meta!{location("data/id")}]
///
/// The generated handle stores an offset and a length
/// using the configured numeric representation.
///
/// A span handle describes coordinates only. It does not prove
/// that the span belongs to, or fits within, any particular store.
/// The receiving store is responsible for validating its bounds.
///
/// The generated type is copyable, contains no references or ownership,
/// and may use niche-aware representations.
///
/// # Examples
/// A simple handle for an arena.
/// ```
/// # use devela::{NonMaxUsize, handle_span};
/// handle_span! {
///     [
///       offset: usize+NonMaxUsize;
///     ]
///     /// A custom handle.
///     pub MyHandle;
/// }
/// ```
/// See also [`HandleSpanExample`].
///
/// [`HandleSpanExample`]: crate::HandleSpanExample
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! handle_span {
    (

     [
      offset: $oprim:ident;
     ]

     $(#[$handle_attr:meta])*
     $vis:vis $Handle:ident $(;)?

     ) => {
         $crate::handle_span![
             [offset:$oprim + $oprim;]
             $(#[$handle_attr])* $vis $Handle
         ];
    };
    (

     [
      offset: $oprim:ident + $Offset:ty;
     ]

     $(#[$handle_attr:meta])*
     $vis:vis $Handle:ident $(;)?

     ) => {
        $(#[$handle_attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis struct $Handle {
            offset: $crate::MaybeNiche::<$Offset>,
            len: $crate::MaybeNiche::<$Offset>,
        }
        impl $crate::ConstInit for $Handle {
            const INIT: Self = Self::new(
                <$Offset as $crate::ConstInit>::INIT,
                <$Offset as $crate::ConstInit>::INIT,
            );
        }
        impl Default for $Handle {
            fn default() -> Self { <Self as $crate::ConstInit>::INIT }
        }

        /// Fundamental const methods for creation and access.
        #[allow(dead_code)]
        impl $Handle {
            $crate::handle_span!(%guard_offset_repr $oprim, $Offset);

            /* constructors */

            /// Creates a new handle from an `offset` and `len`.
            #[must_use]
            $vis const fn new(offset: $Offset, len: $Offset) -> Self {
                let offset = $crate::MaybeNiche::<$Offset>::new(offset);
                let len = $crate::MaybeNiche::<$Offset>::new(len);
                Self { offset, len }
            }

            /// Creates a new handle from a primitive `offset` and `len`.
            ///
            /// Returns an error if either value is invalid.
            $vis const fn from_prim(offset: $oprim, len: $oprim)
                -> Result<Self, $crate::InvalidValue> {
                let offset = $crate::unwrap![ok? $crate::MaybeNiche::<$Offset>::try_from_prim(offset)];
                let len = $crate::unwrap![ok? $crate::MaybeNiche::<$Offset>::try_from_prim(len)];
                Ok(Self { offset, len })
            }

            /// Creates a new handle from a primitive `offset` and `len`.
            ///
            /// Returns an error if either value is invalid.
            $vis const fn try_from_usize(offset: usize, len: usize)
                -> Result<Self, $crate::NicheValueError> {
                let o = $crate::unwrap![ok? $crate::MaybeNiche::<$Offset>::try_from_usize(offset)];
                let len = $crate::unwrap![ok? $crate::MaybeNiche::<$Offset>::try_from_usize(len)];
                Ok(Self { offset: o, len })
            }

            /* accessors */

            /// Returns the length of the stored data.
            #[must_use]
            $vis const fn len(self) -> $Offset { self.len.get() }
            /// Returns the length of the stored data as the corresponding primitive.
            #[must_use]
            $vis const fn len_prim(self) -> $oprim { self.len.get_prim() }
            /// Returns whether this span has zero length.
            #[must_use]
            $vis const fn is_empty(self) -> bool { self.len_prim() == 0 }

            /// Returns the length of the stored data as a usize.
            $vis const fn len_usize(self) -> Result<usize, $crate::Overflow> {
                self.len.try_to_usize()
            }
            /// Returns the length of the stored data as a usize, saturating at the numeric bounds.
            #[must_use]
            $vis const fn len_usize_saturating(self) -> usize {
                self.len.to_usize_saturating()
            }

            /// Returns the offset of the stored data.
            #[must_use]
            $vis const fn offset(self) -> $Offset { self.offset.get() }
            /// Returns the offset of the stored data as the corresponding primitive.
            #[must_use]
            $vis const fn offset_prim(self) -> $oprim { self.offset.get_prim() }

            /// Returns the offset of the stored data as a usize.
            $vis const fn offset_usize(self) -> Result<usize, $crate::Overflow> {
                self.offset.try_to_usize()
            }
            /// Returns the offset of the stored data as a usize, saturating at the numeric bounds.
            #[must_use]
            $vis const fn offset_usize_saturating(self) -> usize {
                self.offset.to_usize_saturating()
            }
        }
    };
    (%guard_offset_repr $P:ty, $I:ty) => {
        const __GUARD_OFFSET_REPR: () = {
            const fn __allowed<P, I>()
            where
                P: $crate::PrimIndex,
                I: $crate::IndexRepr<Prim = P>,
            {}
            __allowed::<$P, $I>();
        };
    };
}
#[doc(inline)]
pub use handle_span;
