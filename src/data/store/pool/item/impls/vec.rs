#[doc(hidden)]
#[macro_export]
macro_rules! __pool_impl_vec· {
    (
     [
      index: $iprim:ident + $Index:ty;
      generation: $gprim:ident + $Generation:ty;
     ]
     $(#[$pool_attr:meta])* $vis:vis $Pool:ident;
     $hvis:vis $Handle:ident;
    ) => {
        $(#[$pool_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Pool<T> {
            values: $crate::Vec<Option<T>>,
            generations: $crate::Vec<$crate::MaybeNiche<$Generation>>,
            free: $crate::Vec<$crate::MaybeNiche<$Index>>,
            len: usize,
        }

        /* misc. trait impls */

        impl<T> Default for $Pool<T> {
            fn default() -> Self { Self::new() }
        }

        // Fundamental methods
        #[allow(dead_code)]
        impl<T> $Pool<T> {
            /// Verifies the representation laws required by this pool.
            const _VALID_CONFIG: () = {
                assert!(!$crate::MaybeNiche::<$Index>::HAS_NEGATIVE,
                    "the pool index representation must be unsigned");
                assert!($crate::MaybeNiche::<$Index>::IS_CONTIGUOUS,
                    "the pool index representation must be contiguous");
                assert!($crate::MaybeNiche::<$Index>::ZERO.is_some(),
                    "the pool index representation must contain zero");
                assert!($crate::MaybeNiche::<$Generation>::MIN.get_prim()
                    != $crate::MaybeNiche::<$Generation>::MAX.get_prim(),
                        "the pool generation representation needs at least two values");
            };

            /// Returns a new empty pool.
            #[must_use]
            $vis const fn new() -> Self {
                let () = Self::_VALID_CONFIG;
                Self {
                    values: $crate::Vec::<$crate::Option<T>>::new(),
                    generations: $crate::Vec::<$crate::MaybeNiche<$Generation>>::new(),
                    free: $crate::Vec::<$crate::MaybeNiche<$Index>>::new(),
                    len: 0,
                }
            }
            /// Returns a new empty pool with space for at least `capacity` values
            /// without reallocating.
            ///
            /// # Panics
            /// Panics if `capacity` exceeds the configured index representation
            /// or if the allocation cannot be created.
            #[must_use]
            $vis fn with_capacity(capacity: usize) -> Self {
                let () = Self::_VALID_CONFIG;
                assert!(capacity <= Self::MAX_CAPACITY,
                    "the requested pool capacity exceeds its index representation");
                Self {
                    values: $crate::Vec::<$crate::Option<T>>::with_capacity(capacity),
                    generations:
                        $crate::Vec::<$crate::MaybeNiche<$Generation>>::with_capacity(capacity),
                    free: $crate::Vec::<$crate::MaybeNiche<$Index>>::with_capacity(capacity),
                    len: 0,
                }
            }

            /* capacity */

            /// Returns the usable slot capacity available without reallocating.
            #[must_use]
            $vis const fn capacity(&self) -> usize {
                let values = self.values.capacity();
                let generations = self.generations.capacity();
                let free = self.free.capacity();
                let storage = $crate::Cmp($crate::Cmp(values).min(generations)).min(free);
                $crate::Cmp(storage).min(Self::MAX_CAPACITY)
            }
            /// Returns the number of introduced slots.
            #[must_use]
            $vis const fn slot_count(&self) -> usize { self.values.len() }

            // len, is_empty, remaining
            $crate::pool! {%impl_common_core
                [ index: $iprim + $Index; generation: $gprim + $Generation; ]
                $vis $Pool; $hvis $Handle;
            }

            /// Returns whether no further value can be inserted.
            ///
            /// This occurs when there is no vacant slot
            /// and no new slot index can be represented.
            #[must_use]
            $vis const fn is_full(&self) -> bool {
                self.free.is_empty()
                    && $crate::MaybeNiche::<$Index>::try_from_usize(self.values.len()).is_err()
            }

            /* access */

            /// Returns whether `handle` currently resolves to a value.
            #[must_use]
            $hvis fn contains(&self, handle: $Handle) -> bool {
                self._resolve_index(handle).is_some()
            }
            /// Returns a shared reference to the value resolved by `handle`.
            #[must_use]
            $hvis fn get(&self, handle: $Handle) -> Option<&T> {
                let index = $crate::unwrap![some? self._resolve_index(handle)];
                self.values[index].as_ref()
            }
            /// Returns an exclusive reference to the value resolved by `handle`.
            #[must_use]
            $hvis fn get_mut(&mut self, handle: $Handle) -> Option<&mut T> {
                let index = $crate::unwrap![some? self._resolve_index(handle)];
                self.values[index].as_mut()
            }

            /// Returns exclusive references to the values resolved by `a` and `b`.
            ///
            /// The references follow the order of the supplied handles.
            ///
            /// Returns `None` if either handle is invalid
            /// or both handles resolve to the same slot.
            #[must_use]
            $hvis fn get2_mut(&mut self, a: $Handle, b: $Handle) -> Option<(&mut T, &mut T)> {
                let a_index = $crate::unwrap![some? self._resolve_index(a)];
                let b_index = $crate::unwrap![some? self._resolve_index(b)];
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
                let Some((index_usize, index)) = self._acquire_slot() else { return Err(value); };
                let generation = self.generations[index_usize];
                self.values[index_usize] = Some(value);
                self.len += 1;
                Ok($Handle::new(index.get(), generation.get()))
            }

            /// Replaces the value resolved by `handle`, returning the previous value.
            ///
            /// # Errors
            /// Returns `value` unchanged if `handle` does not currently resolve.
            $hvis fn replace(&mut self, handle: $Handle, value: T) -> Result<T, T> {
                let index = $crate::unwrap![some_ok_or? self._resolve_index(handle), value];
                match self.values[index].as_mut() {
                    Some(slot) => Ok($crate::Mem::replace(slot, value)),
                    None => Err(value), // unreachable while pool invariants hold (IMPROVE?)
                }
            }

            /// Removes and returns the value resolved by `handle`.
            ///
            /// The vacated slot advances its generation before it can be reused.
            $hvis fn remove(&mut self, handle: $Handle) -> Option<T> {
                let index = self._resolve_index(handle)?;
                let value = self.values[index].take()?;
                self.generations[index] = Self::_next_generation(self.generations[index]);
                self.free.push($crate::MaybeNiche(handle.get_index()));
                self.len -= 1;
                Some(value)
            }
            /// Removes every value and invalidates every live handle.
            $vis fn clear(&mut self) {
                self.free.clear();
                for index in (0..self.values.len()).rev() {
                    if self.values[index].take().is_some() {
                        self.generations[index] = Self::_next_generation(self.generations[index]);
                    }
                    let encoded = $crate::MaybeNiche::<$Index>::try_from_usize(index).unwrap();
                    self.free.push(encoded);
                }
                self.len = 0;
            }

            /* iteration */

            /// Iterates over occupied values in ascending slot order.
            $vis fn iter(&self) -> $crate::PoolIter<&[$crate::Option<T>]> {
                $crate::PoolIter::_new(self.values.as_slice(), self.len)
            }
            /// Iterates mutably over occupied values in ascending slot order.
            $vis fn iter_mut(&mut self) -> $crate::PoolIter<&mut [$crate::Option<T>]> {
                $crate::PoolIter::_new(self.values.as_mut_slice(), self.len)
            }

            $crate::pool! {%impl_common_iter
                [ index: $iprim + $Index; generation: $gprim + $Generation; ]
                $vis $Pool; $hvis $Handle;
            }

            /* private */

            fn _acquire_slot(&mut self) -> Option<(usize, $crate::MaybeNiche<$Index>)> {
                if let Some(index) = self.free.pop() {
                    let index_usize = index.try_to_usize().ok()?;
                    return Some((index_usize, index));
                }
                let index_usize = self.values.len();
                let index = $crate::MaybeNiche::<$Index>::try_from_usize(index_usize).ok()?;
                self._reserve_one_slot();
                self.values.push(None);
                self.generations
                    .push(<$crate::MaybeNiche::<$Generation> as $crate::ConstInit>::INIT);
                Some((index_usize, index))
            }
            fn _reserve_one_slot(&mut self) {
                self.values.reserve(1);
                self.generations.reserve(1);
                let slot_capacity = self.values.capacity().min(self.generations.capacity());
                self.free .reserve(slot_capacity.saturating_sub(self.free.len()));
            }
        }

        $crate::pool! {%impl_common_iter_traits
            [ index: $iprim + $Index; generation: $gprim + $Generation; ]
            $vis $Pool; $hvis $Handle;
        }
    };
}
pub use __pool_impl_vec· as __pool_impl_vec;
