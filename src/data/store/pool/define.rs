// devela/src/data/store/pool/define.rs
//
//! Defines [`pool!`].
//

#[cfg(any(test, feature = "_docs_examples"))]
pool! {
    [
        index: u8;
        generation: u16 + crate::NonMaxU16;
    ]

    #[doc = crate::_tags!(example data_structure)]
    /// An example fixed-capacity generational pool.
    ///
    /// Generated with [`pool!`].
    pub PoolExample;

    #[doc = crate::_tags!(example uid)]
    /// A handle into [`PoolExample`].
    ///
    /// Generated with [`pool!`].
    pub PoolHandleExample;
}

#[doc = crate::_tags!(construction data_structure)]
/// Defines an owning fixed-capacity generational pool.
#[doc = crate::_doc_meta!{location("data/store")}]
///
/// The generated pool stores values in stable slots. Removing a value advances
/// that slot's generation before the slot can be reused, so its previous handle
/// no longer resolves.
///
/// Handles are relative to the pool instance that produced them.
///
/// They do not contain a pool identity. Resolving a handle against another
/// instance of the same generated pool type may coincidentally succeed.
///
/// Generations wrap through the valid values of their configured representation.
/// A stale handle can therefore become valid again after a complete generation
/// cycle for the same slot.
///
/// # Configuration
/// The index representation must be unsigned, contiguous from zero, and able to
/// represent every index in `0..CAP`. The generation representation must contain
/// at least two distinct values.
///
/// # Examples
/// ```
/// # use devela::{NonMaxU8, NonMaxU16, pool};
/// pool! {
///     [
///         index: u8;
///         generation: u16 + NonMaxU16;
///     ]
///     pub Entities;
///     pub EntityId;
/// }
/// let mut entities = Entities::<&str, 8>::new();
/// let id = entities.insert("tree").unwrap();
/// assert_eq!(entities.get(id), Some(&"tree"));
/// ```
/// See: [`PoolExample`], [`PoolHandleExample`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! pool {
    (

     [
      index: $iprim:ident;
      generation: $gprim:ident;
     ]

     $(#[$pool_attr:meta])*
     $vis:vis $Pool:ident;

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?

    ) => {
        $crate::pool! {
            [
                index: $iprim + $iprim;
                generation: $gprim + $gprim;
            ]
            $(#[$pool_attr])* $vis $Pool;
            $(#[$handle_attr])* $hvis $Handle;
        }
    };
    (

     [
      index: $iprim:ident + $Index:ty;
      generation: $gprim:ident;
     ]

     $(#[$pool_attr:meta])*
     $vis:vis $Pool:ident;

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?

    ) => {
        $crate::pool! {
            [
                index: $iprim + $Index;
                generation: $gprim + $gprim;
            ]
            $(#[$pool_attr])* $vis $Pool;
            $(#[$handle_attr])* $hvis $Handle;
        }
    };
    (

     [
      index: $iprim:ident;
      generation: $gprim:ident + $Generation:ty;
     ]

     $(#[$pool_attr:meta])*
     $vis:vis $Pool:ident;

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?

    ) => {
        $crate::pool! {
            [
                index: $iprim + $iprim;
                generation: $gprim + $Generation;
            ]
            $(#[$pool_attr])* $vis $Pool;
            $(#[$handle_attr])* $hvis $Handle;
        }
    };
    (

     [
      index: $iprim:ident + $Index:ty;
      generation: $gprim:ident + $Generation:ty;
     ]

     $(#[$pool_attr:meta])*
     $vis:vis $Pool:ident;

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?

    ) => {
        /* handle */

        $crate::handle_gen! {
            [
                index: $iprim + $Index;
                generation: $gprim + $Generation;
            ]
            $(#[$handle_attr])* $hvis $Handle
        }

        /* pool */

        $crate::paste! {
            $crate::pool! {%define
                [
                    index: $iprim + $Index;
                    generation: $gprim + $Generation;
                ]
                $(#[$pool_attr])* $vis $Pool; // the pool name
                $Handle; // the handle name
                [<_test_ $Pool>]; // the test module name
            }
        }
    };
    (%define
     [
      index: $iprim:ident + $Index:ty;
      generation: $gprim:ident + $Generation:ty;
     ]
     $(#[$pool_attr:meta])* $vis:vis $Pool:ident;
     $Handle:ident;
     $test_mod:ident;
    ) => {
        $(#[$pool_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Pool<T, const CAP: usize> {
            values: [Option<T>; CAP],
            generations: [$crate::MaybeNiche<$Generation>; CAP],
            free: [$crate::MaybeNiche<$Index>; CAP],
            len: usize,
            free_len: usize,
        }

        /* misc. trait impls */

        impl<T, const CAP: usize> $crate::ConstInit for $Pool<T, CAP> {
            const INIT: Self = Self::new();
        }
        impl<T, const CAP: usize> Default for $Pool<T, CAP> {
            fn default() -> Self { Self::new() }
        }

        // Fundamental methods
        #[allow(dead_code)]
        impl<T, const CAP: usize> $Pool<T, CAP> {
            /// Verifies the representation laws required by this pool.
            const __VALID_CONFIG: () = {
                assert!(!$crate::MaybeNiche::<$Index>::HAS_NEGATIVE,
                    "the pool index representation must be unsigned");
                assert!($crate::MaybeNiche::<$Index>::IS_CONTIGUOUS,
                    "the pool index representation must be contiguous");
                assert!($crate::MaybeNiche::<$Index>::ZERO.is_some(),
                    "the pool index representation must contain zero");
                assert!($crate::MaybeNiche::<$Generation>::MIN.get_prim()
                    != $crate::MaybeNiche::<$Generation>::MAX.get_prim(),
                    "the pool generation representation needs at least two values");
                if CAP != 0 {
                    assert!($crate::MaybeNiche::<$Index>::try_from_usize(CAP - 1).is_ok(),
                        "the pool capacity exceeds its index representation");
                }
            };

            /// Returns a new empty pool.
            #[must_use]
            $vis const fn new() -> Self {
                let () = Self::__VALID_CONFIG;
                Self {
                    values: [const { None }; CAP],
                    generations: [
                        <$crate::MaybeNiche<$Generation> as $crate::ConstInit>::INIT; CAP ],
                    free: [ <$crate::MaybeNiche<$Index> as $crate::ConstInit>::INIT; CAP ],
                    len: 0,
                    free_len: 0,
                }
            }

            /* capacity */

            /// Returns the total number of slots.
            #[must_use]
            $vis const fn capacity(&self) -> usize { CAP }

            /// Returns the number of occupied slots.
            #[must_use]
            $vis const fn len(&self) -> usize { self.len }

            /// Returns the number of vacant slots.
            #[must_use]
            $vis const fn remaining(&self) -> usize { CAP - self.len }

            /// Returns whether the pool contains no values.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.len == 0 }

            /// Returns whether every slot is occupied.
            #[must_use]
            $vis const fn is_full(&self) -> bool { self.len == CAP }

            /* access */

            /// Returns whether `handle` currently resolves to a value.
            #[must_use]
            $vis fn contains(&self, handle: $Handle) -> bool {
                self.__resolve_index(handle).is_some()
            }
            /// Returns a shared reference to the value resolved by `handle`.
            #[must_use]
            $vis fn get(&self, handle: $Handle) -> Option<&T> {
                let index = self.__resolve_index(handle)?;
                self.values[index].as_ref()
            }
            /// Returns an exclusive reference to the value resolved by `handle`.
            #[must_use]
            $vis fn get_mut(&mut self, handle: $Handle) -> Option<&mut T> {
                let index = self.__resolve_index(handle)?;
                self.values[index].as_mut()
            }

            /* mutation */

            /// Inserts `value`, returning its handle.
            ///
            /// # Errors
            /// Returns `value` unchanged when the pool is full.
            $vis fn insert(&mut self, value: T) -> Result<$Handle, T> {
                if self.is_full() { return Err(value); }
                let (index_usize, index) = if self.free_len != 0 {
                    let free_pos = self.free_len - 1;
                    let index = self.free[free_pos];
                    let index_usize = match index.try_to_usize() {
                        Ok(index) => index,
                        Err(_) => return Err(value), // impossible for internally stored indices
                    };
                    self.free_len = free_pos;
                    (index_usize, index)
                } else {
                    // With no vacant introduced slots, `len` is the frontier.
                    let index_usize = self.len;
                    let index = match $crate::MaybeNiche::<$Index>::try_from_usize(index_usize) {
                        Ok(index) => index,
                        Err(_) => return Err(value), // guarded by `__VALID_CONFIG`
                    };
                    (index_usize, index)
                };
                debug_assert!(index_usize < CAP);
                debug_assert!(self.values[index_usize].is_none());
                let generation = self.generations[index_usize];
                self.values[index_usize] = Some(value);
                self.len += 1;
                Ok($Handle::new(index.get(), generation.get()))
            }
            /// Removes and returns the value resolved by `handle`.
            ///
            /// The vacated slot advances its generation before it can be reused.
            $vis fn remove(&mut self, handle: $Handle) -> Option<T> {
                let index = self.__resolve_index(handle)?;
                let value = self.values[index].take()?;
                self.generations[index] = Self::__next_generation(self.generations[index]);
                self.free[self.free_len] = $crate::MaybeNiche(handle.index());
                self.free_len += 1;
                self.len -= 1;
                Some(value)
            }
            /// Removes every value and invalidates every live handle.
            $vis fn clear(&mut self) {
                let frontier = self.len + self.free_len;
                $crate::whilst! { index in 0..frontier; {
                    if self.values[index].take().is_some() {
                        self.generations[index] = Self::__next_generation(self.generations[index]);
                    }
                }}
                self.len = 0;
                self.free_len = 0;
            }

            /* private */

            fn __resolve_index(&self, handle: $Handle) -> Option<usize> {
                let index = $crate::MaybeNiche(handle.index()).try_to_usize().ok()?;
                if index >= CAP { return None; }
                if self.generations[index].ne($crate::MaybeNiche(handle.generation())) {
                    return None;
                }
                if self.values[index].is_none() { return None; }
                Some(index)
            }
            fn __next_generation(current: $crate::MaybeNiche<$Generation>)
                -> $crate::MaybeNiche<$Generation> {
                let mut candidate = current.get_prim();
                loop {
                    candidate = candidate.wrapping_add(1);
                    if let Ok(next) = $crate::MaybeNiche::<$Generation>::try_from_prim(candidate) {
                        return next;
                    }
                }
            }
        }

        /* tests */

        #[cfg(test)]
        #[allow(non_snake_case)]
        mod $test_mod {
            use super::$Pool;

            #[test]
            fn empty_and_capacity() {
                let pool = $Pool::<u8, 3>::new();
                assert_eq!(pool.capacity(), 3);
                assert_eq!(pool.len(), 0);
                assert_eq!(pool.remaining(), 3);
                assert!(pool.is_empty());
                assert!(!pool.is_full());
            }
            #[test]
            fn insertion_and_access() {
                let mut pool = $Pool::<&str, 2>::new();
                let a = pool.insert("a").unwrap();
                let b = pool.insert("b").unwrap();
                assert_eq!(pool.get(a), Some(&"a"));
                assert_eq!(pool.get(b), Some(&"b"));
                assert!(pool.contains(a));
                assert!(pool.is_full());
                assert_eq!(pool.insert("c"), Err("c"));
            }
            #[test]
            fn removal_preserves_unrelated_handles() {
                let mut pool = $Pool::<&str, 3>::new();
                let a = pool.insert("a").unwrap();
                let b = pool.insert("b").unwrap();
                assert_eq!(pool.remove(a), Some("a"));
                assert_eq!(pool.get(a), None);
                assert_eq!(pool.get(b), Some(&"b"));
                assert_eq!(pool.remove(a), None);
            }
            #[test]
            fn reuse_invalidates_the_previous_handle() {
                let mut pool = $Pool::<&str, 1>::new();
                let old = pool.insert("old").unwrap();
                assert_eq!(pool.remove(old), Some("old"));
                let new = pool.insert("new").unwrap();
                assert_eq!(old.index_prim(), new.index_prim());
                assert_ne!(old.generation_prim(), new.generation_prim());
                assert_eq!(pool.get(old), None);
                assert_eq!(pool.remove(old), None);
                assert_eq!(pool.get(new), Some(&"new"));
            }
            #[test]
            fn zero_capacity() {
                let mut pool = $Pool::<u8, 0>::new();
                assert!(pool.is_empty());
                assert!(pool.is_full());
                assert_eq!(pool.insert(7), Err(7));
            }
            #[test]
            fn clear_invalidates_live_handles() {
                let mut pool = PoolExample::<&str, 3>::new();
                let a = pool.insert("a").unwrap();
                let b = pool.insert("b").unwrap();
                pool.clear();
                assert!(pool.is_empty());
                assert_eq!(pool.get(a), None);
                assert_eq!(pool.get(b), None);
                let c = pool.insert("c").unwrap();
                assert_eq!(pool.get(c), Some(&"c"));
                assert_eq!(pool.get(a), None);
            }
        }
    };
}
#[doc(inline)]
pub use pool;
