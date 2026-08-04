// devela/src/data/id/handle/generation.rs
//
//! Defines [`handle_gen!`] macro.
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
/// # use devela::{NonMaxU32, NonMaxU16, handle_gen};
/// handle_gen! {
///     [
///       index: u32 + NonMaxU32;
///       generation: u16 + u16;
///     ]
///     /// A custom handle.
///     pub MyHandle;
/// }
/// ```
/// See also [`HandleGenExample`][crate::HandleGenExample].
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! handle_gen {
    // point of entry
    (
     [
      index: $iprim:ident + $Index:ty;
      generation: $gprim:ident + $Generation:ty;
     ]

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?

     ) => {
        $crate::handle_gen! { %define
            [
                index: $iprim + $Index;
                generation: $gprim + $Generation;
            ]
            $(#[$handle_attr])*
            $hvis $Handle;
        }
    };
    (
     %define
     [
      index: $iprim:ident + $Index:ty;
      generation: $gprim:ident + $Generation:ty;
     ]
     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident;) => {

        $(#[$handle_attr])*
        #[must_use]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $hvis struct $Handle {
            index: $crate::MaybeNiche<$Index>,
            generation: $crate::MaybeNiche<$Generation>,
        }

        #[allow(dead_code)]
        impl $Handle {
            /// Creates a handle from validated internal parts.
            const fn __new(
                index: $crate::MaybeNiche<$Index>,
                generation: $crate::MaybeNiche<$Generation>,
            ) -> Self {
                Self { index, generation }
            }

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
