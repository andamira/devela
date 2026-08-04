// devela/src/data/id/handle/span.rs
//
//! Defines [`handle_span!`] macro.
//

#[doc = crate::_tags!(construction uid rework)]
/// Defines a lightweight handle type.
#[doc = crate::_doc_meta!{location("data/id")}]
///
/// A *handle* is a lightweight, copyable semantic reference that identifies
/// an entry within a managed collection, such as an arena, list, or graph.
///
/// Handles are plain data values. They contain only small scalar fields
/// (like offsets, lengths, or indices) and no lifetimes or ownership.
///
/// Handles form the connective tissue of the data layer,
/// bridging raw storage with higher-level structure.
///
/// # Examples
/// A simple handle for an arena.
/// ```
/// # use devela::{NonMaxUsize, handle_span};
/// handle_span! {
///     [offset: usize+NonMaxUsize; ]
///     /// A custom handle.
///     pub MyHandle;
/// }
/// ```
/// See also [`HandleSpanExample`][crate::HandleSpanExample].
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! handle_span {
    // point of entry
    (
     [
      offset: $prim:ident + $T:ty;
     ]

     $(#[$handle_attr:meta])*
     $vis:vis $Handle:ident $(;)?
     ) => {
         $crate::handle_span![%handle
             [offset:$prim+$T;]
             $(#[$handle_attr])* $vis $Handle ];
    };
    // calls the necessary arms in order.
    (
     %handle
     [offset:$prim:ident+$T:ty;]
     $(#[$handle_attr:meta])* $vis:vis $Handle:ident) => { $crate::paste! {

        $crate::handle_span![%main
            [offset:$prim+$T;]
            $(#[$handle_attr])* $vis $Handle ];

        // #[cfg(test)]
        // $crate::handle_span![%tests $Handle, [<test_ $Handle>]];
    }};
    (
     %main
     [offset:$prim:ident+$T:ty;]
     $(#[$handle_attr:meta])* $vis:vis $Handle:ident) => {

        $(#[$handle_attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis struct $Handle {
            offset: $crate::MaybeNiche::<$T>,
            len: $crate::MaybeNiche::<$T>,
        }

        impl $crate::ConstInit for $Handle {
            const INIT: Self = Self::new(<$T>::INIT, <$T>::INIT);
        }

        /// Fundamental const methods for creation and access.
        #[allow(dead_code)]
        impl $Handle {
            /* constructors */

            /// Creates a new handle from an `offset` and `len`.
            #[must_use]
            $vis const fn new(offset: $T, len: $T) -> Self {
                let offset = $crate::MaybeNiche::<$T>::new(offset);
                let len = $crate::MaybeNiche::<$T>::new(len);
                Self { offset, len }
            }

            /// Creates a new handle from a primitive `offset` and `len`.
            ///
            /// Returns `None` if any of the values are invalid.
            $vis const fn from_prim(offset: $prim, len: $prim)
                -> Result<Self, $crate::InvalidValue> {
                let offset = $crate::unwrap![ok? $crate::MaybeNiche::<$T>::try_from_prim(offset)];
                let len = $crate::unwrap![ok? $crate::MaybeNiche::<$T>::try_from_prim(len)];
                Ok(Self { offset, len })
            }

            // MAYBE: if we gate the unsafe with a macro argument
            // /// Creates a new handle from a primitive `offset` and `len`, without any checks.
            // /// # Safety
            // /// Callers must ensure that the values satisfies the validity constraints.
            // #[must_use]
            // $vis const fn from_prim_unchecked(offset: $prim, len: $prim) -> Self {
            //     unimplemented![]
            // }

            /// Creates a new handle from a *lossy* primitive `offset` and `len`.
            ///
            /// Converting invalid inputs into a valid but *approximate* representation.
            #[must_use]
            $vis const fn from_prim_lossy(offset: $prim, len: $prim) -> Self {
                let offset = $crate::MaybeNiche::<$T>::from_prim_lossy(offset);
                let len = $crate::MaybeNiche::<$T>::from_prim_lossy(len);
                Self { offset, len }
            }

            /// Creates a new handle from a primitive `offset` and `len`.
            ///
            /// Returns `None` if any of the values can't fit in the primitive representation,
            /// or if it's not valid for the current niche.
            $vis const fn try_from_usize(offset: usize, len: usize)
                -> Result<Self, $crate::NicheValueError> {
                let o = $crate::unwrap![ok? $crate::MaybeNiche::<$T>::try_from_usize(offset)];
                let len = $crate::unwrap![ok? $crate::MaybeNiche::<$T>::try_from_usize(len)];
                Ok(Self { offset: o, len })
            }

            /* accessors */

            /// Returns the length of the stored data.
            #[must_use]
            #[allow(clippy::len_without_is_empty)]
            $vis const fn len(self) -> $T { self.len.get() }
            /// Returns the length of the stored data as the corresponding primitive.
            #[must_use]
            $vis const fn len_prim(self) -> $prim { self.len.get_prim() }

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
            $vis const fn offset(self) -> $T { self.offset.get() }
            /// Returns the offset of the stored data as the corresponding primitive.
            #[must_use]
            $vis const fn offset_prim(self) -> $prim { self.offset.get_prim() }

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
}
#[doc(inline)]
pub use handle_span;
