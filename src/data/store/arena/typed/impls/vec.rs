// devela/src/data/store/arena/typed/impls/vec.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __arena_impl_vec· {
    (
     [index: $iprim:ident + $Index:ty;]
     $(#[$arena_attr:meta])* $vis:vis $Arena:ident;
     $hvis:vis $Handle:ident;
     [mark: $($mvis:vis $Mark:ident)?]
    ) => {
        $(#[$arena_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Arena<T> {
            values: $crate::Vec<T>,
        }

        impl<T> Default for $Arena<T> {
            fn default() -> Self { Self::new() }
        }

        #[allow(dead_code)]
        impl<T> $Arena<T> {
            /// Verifies the representation laws required by this arena.
            const __VALID_CONFIG: () = {
                const fn __index_primitive<P: $crate::PrimIndex>() {}
                __index_primitive::<$iprim>();
                assert!(!$crate::MaybeNiche::<$Index>::HAS_NEGATIVE,
                    "the arena index representation must be unsigned");
                assert!($crate::MaybeNiche::<$Index>::IS_CONTIGUOUS,
                    "the arena index representation must be contiguous");
                assert!($crate::MaybeNiche::<$Index>::ZERO.is_some(),
                    "the arena index representation must contain zero");
            };

            /* construction */

            /// Returns a new allocating empty arena.
            #[must_use]
            $vis const fn new() -> Self {
                let () = Self::__VALID_CONFIG;
                Self { values: $crate::Vec::<T>::new() }
            }
            /// Returns a new empty arena with space for at least `capacity` values
            /// without reallocating.
            ///
            /// # Panics
            /// Panics if `capacity` exceeds the configured index representation
            /// or if the allocation cannot be created.
            #[must_use]
            $vis fn with_capacity(capacity: usize) -> Self {
                let () = Self::__VALID_CONFIG;
                assert!(capacity <= Self::__index_capacity(),
                    "the requested arena capacity exceeds its index representation");
                Self { values: $crate::Vec::<T>::with_capacity(capacity) }
            }

            /* capacity */

            /// Returns the usable value capacity available without reallocating.
            #[must_use]
            $vis const fn capacity(&self) -> usize {
                $crate::Cmp(self.values.capacity()).min(Self::__index_capacity())
            }
            /// Returns the number of retained values.
            #[must_use]
            $vis const fn len(&self) -> usize { self.values.len() }

            /// Returns whether the arena contains no values.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.values.is_empty() }

            /// Returns how many additional values fit without reallocating.
            ///
            /// The vector may grow when this reaches zero unless [`is_full`][Self::is_full].
            #[must_use]
            $vis const fn remaining(&self) -> usize { self.capacity() - self.len() }

            /// Returns whether no further value can be inserted.
            #[must_use]
            $vis const fn is_full(&self) -> bool {
                $crate::MaybeNiche::<$Index>::try_from_usize(self.values.len()).is_err()
            }

            /* access */

            /// Returns whether `handle` currently resolves to a retained value.
            #[must_use]
            $hvis fn contains(&self, handle: $Handle) -> bool {
                self.__resolve_index(handle).is_some()
            }

            /// Returns a shared reference to the value resolved by `handle`.
            #[must_use]
            $hvis fn get(&self, handle: $Handle) -> Option<&T> {
                let index = self.__resolve_index(handle)?;
                self.values.get(index)
            }
            /// Returns an exclusive reference to the value resolved by `handle`.
            #[must_use]
            $hvis fn get_mut(&mut self, handle: $Handle) -> Option<&mut T> {
                let index = self.__resolve_index(handle)?;
                self.values.get_mut(index)
            }
            /// Returns exclusive references to the values resolved by `a` and `b`.
            ///
            /// The references follow the order of the supplied handles.
            ///
            /// Returns `None` if either handle is invalid
            /// or both handles resolve to the same value.
            #[must_use]
            $hvis fn get2_mut(&mut self, a: $Handle, b: $Handle)
                -> Option<(&mut T, &mut T)> {
                let a_index = self.__resolve_index(a)?;
                let b_index = self.__resolve_index(b)?;
                if a_index == b_index { return None; }
                if a_index < b_index {
                    let (left, right) = self.values.split_at_mut(b_index);
                    Some((&mut left[a_index], &mut right[0]))
                } else {
                    let (left, right) = self.values.split_at_mut(a_index);
                    Some((&mut right[0], &mut left[b_index]))
                }
            }

            /* mutation */

            /// Inserts `value` at the current frontier, returning its handle.
            ///
            /// # Errors
            /// Returns `value` unchanged when no further index can be represented.
            $hvis fn insert(&mut self, value: T) -> Result<$Handle, T> {
                let Ok(index) = $crate::MaybeNiche::<$Index>::try_from_usize(self.values.len())
                    else { return Err(value) };
                self.values.push(value);
                Ok($Handle::new(index.get()))
            }

            /* marks and reclamation */

            $(
                /// Returns a mark at the current insertion frontier.
                #[must_use]
                $mvis const fn mark(&self) -> $Mark { <$Mark>::new(self.values.len()) }

                /// Retracts the insertion frontier to `mark`, reclaiming its suffix.
                ///
                /// Returns `false` without changing the arena
                /// if `mark` lies ahead of the current frontier.
                ///
                /// Reclaimed handles are not permanently invalidated:
                /// later insertion may reuse their indices.
                $mvis fn rollback(&mut self, mark: $Mark) -> bool {
                    if mark.0 > self.values.len() { return false; }
                    self.values.truncate(mark.0);
                    true
                }
            )?

            /// Removes every retained value.
            ///
            /// Later insertion begins again at index zero, so previously issued
            /// handles may resolve again.
            $vis fn clear(&mut self) { self.values.clear(); }

            /* iteration */

            /// Iterates over retained values in insertion order.
            $vis fn iter(&self) -> impl Iterator<Item = &T> + '_ { self.values.iter() }
            /// Iterates mutably over retained values in insertion order.
            $vis fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
                self.values.iter_mut()
            }

            /* private */

            fn __resolve_index(&self, handle: $Handle) -> Option<usize> {
                let index = handle.get_index_usize().ok()?;
                if index >= self.values.len() { return None; }
                Some(index)
            }
            const fn __index_capacity() -> usize {
                match $crate::MaybeNiche::<$Index>::MAX.try_to_usize() {
                    Ok(max) => max.saturating_add(1),
                    Err(_) => usize::MAX,
                }
            }
        }
    };
}
pub use __arena_impl_vec· as __arena_impl_vec;
