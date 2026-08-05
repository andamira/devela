// devela/src/data/id/handle/generation.rs
//
//! Defines [`handle_gen!`] macro.
//

#[doc = crate::_tags!(construction uid rework)]
/// Defines a compact generational handle.
#[doc = crate::_doc_meta!{location("data/id")}]
///
/// The generated handle stores a slot index and generation.
///
/// A store can advance a slot's generation when reclaiming it, allowing old
/// handles to be rejected after that slot is reused. Generation values may
/// eventually wrap, so stale-handle rejection is bounded by the configured
/// generation domain.
///
/// The handle contains no store-instance identity. A handle used with another
/// compatible store may therefore coincidentally resolve.
///
/// Constructors validate only numeric representation.
/// They do not validate whether the handle resolves to a live value.
///
/// # Examples
/// A simple handle for a pool.
/// ```
/// # use devela::{NonMaxU32, handle_gen};
/// handle_gen! {
///     [
///       index: u32 + NonMaxU32;
///       generation: u16;
///     ]
///     /// A custom handle.
///     pub MyHandle;
/// }
/// ```
/// See also [`HandleGenExample`][crate::HandleGenExample].
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! handle_gen {
    (

     [
      index: $iprim:ident;
      generation: $gprim:ident;
     ]

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?

    ) => {
        $crate::handle_gen! {
            [ index: $iprim + $iprim; generation: $gprim + $gprim; ]
            $(#[$handle_attr])* $hvis $Handle;
        }
    };
    (

     [
      index: $iprim:ident + $Index:ty;
      generation: $gprim:ident;
     ]

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?

    ) => {
        $crate::handle_gen! {
            [ index: $iprim + $Index; generation: $gprim + $gprim; ]
            $(#[$handle_attr])* $hvis $Handle;
        }
    };
    (

     [
      index: $iprim:ident;
      generation: $gprim:ident + $Generation:ty;
     ]

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?

    ) => {
        $crate::handle_gen! {
            [ index: $iprim + $iprim; generation: $gprim + $Generation; ]
            $(#[$handle_attr])* $hvis $Handle;
        }
    };
    (

     [
      index: $iprim:ident + $Index:ty;
      generation: $gprim:ident + $Generation:ty;
     ]
     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?
    ) => {
        $(#[$handle_attr])*
        #[must_use]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $hvis struct $Handle {
            index: $crate::MaybeNiche<$Index>,
            generation: $crate::MaybeNiche<$Generation>,
        }

        #[allow(dead_code)]
        impl $Handle {
            /* constructors */

            /// Creates a new handle from an `index` and `generation`.
            $hvis const fn new(index: $Index, generation: $Generation) -> Self {
                let index = $crate::MaybeNiche::<$Index>::new(index);
                let generation = $crate::MaybeNiche::<$Generation>::new(generation);
                Self { index, generation }
            }
            /// Creates a new handle from a primitive `index` and `generation`.
            ///
            /// Returns an error if either value is invalid.
            $hvis const fn from_prim(index: $iprim, generation: $gprim)
                -> Result<Self, $crate::InvalidValue> {
                let i = $crate::unwrap![ok? $crate::MaybeNiche::<$Index>::try_from_prim(index)];
                let generation = $crate::unwrap![ok?
                    $crate::MaybeNiche::<$Generation>::try_from_prim(generation)];
                Ok(Self { index: i, generation })
            }
            /// Creates a new handle from a primitive `index` and `generation`.
            ///
            /// Returns an error if either value is invalid.
            $hvis const fn try_from_usize(index: usize, generation: usize)
                -> Result<Self, $crate::NicheValueError> {
                let i = $crate::unwrap![ok? $crate::MaybeNiche::<$Index>::try_from_usize(index)];
                let generation = $crate::unwrap![ok?
                    $crate::MaybeNiche::<$Generation>::try_from_usize(generation)];
                Ok(Self { index: i, generation })
            }

            /* accessors */

            /// Returns the slot index.
            #[must_use]
            $hvis const fn index(self) -> $Index { self.index.get() }

            /// Returns the slot index as its primitive carrier.
            #[must_use]
            $hvis const fn index_prim(self) -> $iprim { self.index.get_prim() }

            /// Returns the slot index as a `usize`.
            ///
            /// # Errors
            /// Returns an error if it cannot fit in a `usize`.
            $hvis const fn index_usize(self) -> Result<usize, $crate::Overflow> {
                self.index.try_to_usize()
            }

            /// Returns the slot generation.
            #[must_use]
            $hvis const fn generation(self) -> $Generation { self.generation.get() }

            /// Returns the slot generation as its primitive carrier.
            #[must_use]
            $hvis const fn generation_prim(self) -> $gprim { self.generation.get_prim() }
        }
    };
}
#[doc(inline)]
pub use handle_gen;
