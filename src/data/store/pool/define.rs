// devela/src/data/store/pool/define.rs
//
//! Defines [`pool!`].
//

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
///
/// See: [`PoolExample`], [`PoolHandleExample`].
///
/// [`PoolExample`]: crate::PoolExample
/// [`PoolHandleExample`]: crate::PoolHandleExample
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

        $crate::pool! {%define
            [
                index: $iprim + $Index;
                generation: $gprim + $Generation;
            ]
            $(#[$pool_attr])* $vis $Pool; // the pool name
            $hvis $Handle; // the handle name
        }
    };
    (%define
     [
      index: $iprim:ident + $Index:ty;
      generation: $gprim:ident + $Generation:ty;
     ]
     $(#[$pool_attr:meta])* $vis:vis $Pool:ident;
     $hvis:vis $Handle:ident;
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
            $hvis const fn contains(&self, handle: $Handle) -> bool {
                self.__resolve_index(handle).is_some()
            }
            /// Returns a shared reference to the value resolved by `handle`.
            #[must_use]
            $hvis const fn get(&self, handle: $Handle) -> Option<&T> {
                let index = $crate::unwrap![some? self.__resolve_index(handle)];
                self.values[index].as_ref()
            }
            /// Returns an exclusive reference to the value resolved by `handle`.
            #[must_use]
            $hvis const fn get_mut(&mut self, handle: $Handle) -> Option<&mut T> {
                let index = $crate::unwrap![some? self.__resolve_index(handle)];
                self.values[index].as_mut()
            }

            /// Returns exclusive references to the values resolved by `a` and `b`.
            ///
            /// The references follow the order of the supplied handles.
            ///
            /// Returns `None` if either handle is invalid
            /// or both handles resolve to the same slot.
            #[must_use]
            $hvis const fn get2_mut(&mut self, a: $Handle, b: $Handle) -> Option<(&mut T, &mut T)> {
                let a_index = $crate::unwrap![some? self.__resolve_index(a)];
                let b_index = $crate::unwrap![some? self.__resolve_index(b)];
                if a_index == b_index { return None; }
                if a_index < b_index {
                    let (left, right) = self.values.split_at_mut(b_index);
                    Some((
                        $crate::unwrap![some? left[a_index].as_mut()],
                        $crate::unwrap![some? right[0].as_mut()],
                    ))
                } else {
                    let (left, right) = self.values.split_at_mut(a_index);
                    Some((
                        $crate::unwrap![some? right[0].as_mut()],
                        $crate::unwrap![some? left[b_index].as_mut()],
                    ))
                }
            }

            /* mutation */

            /// Inserts `value`, returning its handle.
            ///
            /// # Errors
            /// Returns `value` unchanged when the pool is full.
            $hvis fn insert(&mut self, value: T) -> Result<$Handle, T> {
                let Some((index_usize, index)) = self.__acquire_slot() else { return Err(value); };
                let generation = self.generations[index_usize];
                self.values[index_usize] = Some(value);
                self.len += 1;
                Ok($Handle::new(index.get(), generation.get()))
            }
            /// Inserts a copyable `value`, returning its handle.
            ///
            /// This is the const-capable variant of [`insert`][Self::insert].
            ///
            /// # Errors
            /// Returns `value` unchanged when the pool is full.
            $hvis const fn insert_copy(&mut self, value: T) -> Result<$Handle, T> where T: Copy {
                let Some((index_usize, index)) = self.__acquire_slot() else { return Err(value); };
                let generation = self.generations[index_usize];
                self.values[index_usize] = Some(value);
                self.len += 1;
                Ok($Handle::new(index.get(), generation.get()))
            }

            /// Replaces the value resolved by `handle`, returning the previous value.
            ///
            /// # Errors
            /// Returns `value` unchanged if `handle` does not currently resolve.
            $hvis const fn replace(&mut self, handle: $Handle, value: T) -> Result<T, T> {
                let index = $crate::unwrap![some_ok_or? self.__resolve_index(handle), value];
                match self.values[index].as_mut() {
                    Some(slot) => Ok($crate::Mem::replace(slot, value)),
                    None => Err(value), // unreachable while pool invariants hold
                }
            }
            /// Removes and returns the value resolved by `handle`.
            ///
            /// The vacated slot advances its generation before it can be reused.
            $hvis const fn remove(&mut self, handle: $Handle) -> Option<T> {
                let index = $crate::unwrap![some? self.__resolve_index(handle)];
                let next_generation = Self::__next_generation(self.generations[index]);
                let free_pos = self.free_len;
                self.generations[index] = next_generation;
                self.free[free_pos] = $crate::MaybeNiche(handle.index());
                self.free_len = free_pos + 1;
                self.len -= 1;
                $crate::Mem::replace(&mut self.values[index], None)
            }
            /// Removes every value and invalidates every live handle.
            $vis fn clear(&mut self) {
                let frontier = self.len + self.free_len;
                $crate::whilst! { index in 0..frontier; {
                    if self.values[index].take().is_some() { // drop
                        self.generations[index] = Self::__next_generation(self.generations[index]);
                    }
                }}
                self.len = 0;
                self.free_len = 0;
            }
            /// Removes every copyable value and invalidates every live handle.
            ///
            /// This is the const-capable variant of [`clear`][Self::clear].
            $vis const fn clear_copy(&mut self) where T: Copy {
                let frontier = self.len + self.free_len;
                $crate::whilst! { index in 0..frontier; {
                    if self.values[index].is_some() {
                        self.values[index] = None;
                        self.generations[index] = Self::__next_generation(self.generations[index]);
                    }
                }}
                self.len = 0;
                self.free_len = 0;
            }

            /* iteration */

            /// Iterates over occupied values in ascending slot order.
            $vis const fn iter(&self) -> $crate::PoolIter<&[Option<T>]> {
                $crate::PoolIter::_new(&self.values, self.len)
            }
            /// Iterates mutably over occupied values in ascending slot order.
            $vis const fn iter_mut(&mut self) -> $crate::PoolIter<&mut [Option<T>]> {
                $crate::PoolIter::_new(&mut self.values, self.len)
            }

            // MAYBE: concrete const-capable handle/entry iterator when needed.
            /// Iterates over current handles in ascending slot order.
            $hvis fn handles(&self) -> impl Iterator<Item = $Handle> + '_ {
                self.entries().map(|(handle, _)| handle)
            }
            /// Iterates over current handles and shared values in ascending slot order.
            $hvis fn entries(&self) -> impl Iterator<Item = ($Handle, &T)> + '_ {
                self.values.iter().enumerate()
                    .filter_map(|(index, value)| {
                        let value = value.as_ref()?;
                        let handle = self.__handle_at(index)?;
                        Some((handle, value))
                    })
            }

            /* private */

            const fn __acquire_slot(&mut self) -> Option<(usize, $crate::MaybeNiche<$Index>)> {
                if self.is_full() { return None; }
                if self.free_len != 0 {
                    let free_pos = self.free_len - 1;
                    let index = self.free[free_pos];
                    let index_usize = $crate::unwrap![ok_some? index.try_to_usize()];
                    self.free_len = free_pos;
                    Some((index_usize, index))
                } else {
                    let index_usize = self.len;
                    let index = $crate::unwrap![ok_some?
                        $crate::MaybeNiche::<$Index>::try_from_usize(index_usize)];
                    Some((index_usize, index))
                }
            }
            const fn __handle_at(&self, index_usize: usize) -> Option<$Handle> {
                if index_usize >= CAP || self.values[index_usize].is_none() { return None; }
                let index = $crate::unwrap![ok_some?
                    $crate::MaybeNiche::<$Index>::try_from_usize(index_usize)];
                let generation = self.generations[index_usize];
                Some($Handle::new(index.get(), generation.get()))
            }
            const fn __resolve_index(&self, handle: $Handle) -> Option<usize> {
                let index = $crate::unwrap![ok_some?
                    $crate::MaybeNiche(handle.index()).try_to_usize()];
                if index >= CAP { return None; }
                if self.generations[index].ne($crate::MaybeNiche(handle.generation())) {
                    return None;
                }
                if self.values[index].is_none() { return None; }
                Some(index)
            }
            const fn __next_generation(current: $crate::MaybeNiche<$Generation>)
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

        impl<'a, T, const CAP: usize> $crate::IteratorInto for &'a $Pool<T, CAP> {
            type Item = &'a T;
            type IntoIter = $crate::PoolIter<&'a [$crate::Option<T>]>;
            fn into_iter(self) -> Self::IntoIter { self.iter() }
        }
        impl<'a, T, const CAP: usize> $crate::IteratorInto for &'a mut $Pool<T, CAP> {
            type Item = &'a mut T;
            type IntoIter = $crate::PoolIter<&'a mut [$crate::Option<T>]>;
            fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
        }
    };
}
#[doc(inline)]
pub use pool;
