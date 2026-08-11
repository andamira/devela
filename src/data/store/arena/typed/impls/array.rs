// devela/src/data/store/arena/typed/impls/array.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __arena_impl_array {
    (
     [index: $iprim:ident + $Index:ty;]
     $(#[$arena_attr:meta])* $vis:vis $Arena:ident;
     $hvis:vis $Handle:ident;
     [mark: $($mvis:vis $Mark:ident)?]
    ) => {
        $(#[$arena_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Arena<T, const CAP: usize> {
            values: [$crate::Option<T>; CAP],
            len: usize,
        }

        impl<T, const CAP: usize> $crate::ConstInit for $Arena<T, CAP> {
            const INIT: Self = Self::new();
        }
        impl<T, const CAP: usize> Default for $Arena<T, CAP> {
            fn default() -> Self { Self::new() }
        }

        #[allow(dead_code)]
        impl<T, const CAP: usize> $Arena<T, CAP> {
            /// Verifies the representation laws required by this arena.
            const __VALID_CONFIG: () = {
                const fn __index_primitive<P: $crate::PrimIndex>() {}
                __index_primitive::<$iprim>();
                assert!(CAP <= Self::__index_capacity(),
                    "the arena capacity exceeds its index representation");
                assert!(!$crate::MaybeNiche::<$Index>::HAS_NEGATIVE,
                    "the arena index representation must be unsigned");
                assert!($crate::MaybeNiche::<$Index>::IS_CONTIGUOUS,
                    "the arena index representation must be contiguous");
                assert!($crate::MaybeNiche::<$Index>::ZERO.is_some(),
                    "the arena index representation must contain zero");
            };

            /* construction */

            /// Returns a new empty arena.
            #[must_use]
            $vis const fn new() -> Self {
                let () = Self::__VALID_CONFIG;
                Self { values: [const { None }; CAP], len: 0 }
            }

            /* capacity */

            /// Returns the total number of usable value slots.
            #[must_use]
            $vis const fn capacity(&self) -> usize { CAP }

            /// Returns the number of retained values.
            #[must_use]
            $vis const fn len(&self) -> usize { self.len }

            /// Returns whether the arena contains no values.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.len == 0 }

            /// Returns how many additional values fit in the arena.
            #[must_use]
            $vis const fn remaining(&self) -> usize { CAP - self.len }

            /// Returns whether no further value can be inserted.
            #[must_use]
            $vis const fn is_full(&self) -> bool { self.len == CAP }

            /* access */

            /// Returns whether `handle` currently resolves to a retained value.
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
            /// or both handles resolve to the same value.
            #[must_use]
            $hvis const fn get2_mut(&mut self, a: $Handle, b: $Handle)
                -> Option<(&mut T, &mut T)> {
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

            /// Inserts `value` at the current frontier, returning its handle.
            ///
            /// # Errors
            /// Returns `value` unchanged when the arena is full.
            $hvis fn insert(&mut self, value: T) -> Result<$Handle, T> {
                if self.is_full() { return Err(value); }
                let index = match $crate::MaybeNiche::<$Index>::try_from_usize(self.len) {
                    Ok(index) => index,
                    Err(_) => return Err(value),
                };
                self.values[self.len] = Some(value);
                self.len += 1;
                Ok($Handle::new(index.get()))
            }

            /// Inserts a copyable `value` at the current frontier, returning its handle.
            ///
            /// This is the const-capable variant of [`insert`][Self::insert].
            ///
            /// # Errors
            /// Returns `value` unchanged when the arena is full.
            $hvis const fn insert_copy(&mut self, value: T) -> Result<$Handle, T> where T: Copy {
                if self.is_full() { return Err(value); }
                let index = match $crate::MaybeNiche::<$Index>::try_from_usize(self.len) {
                    Ok(index) => index,
                    Err(_) => return Err(value),
                };
                self.values[self.len] = Some(value);
                self.len += 1;
                Ok($Handle::new(index.get()))
            }

            /* marks and reclamation */

            $(
                /// Returns a mark at the current insertion frontier.
                #[must_use]
                $mvis const fn mark(&self) -> $Mark {
                    <$Mark>::new(self.len)
                }

                /// Removes every value inserted after `mark`.
                ///
                /// Returns `false` without changing the arena if `mark` lies ahead
                /// of the current frontier.
                ///
                /// Reclaimed handles are not permanently invalidated: later insertion
                /// may reuse their indices.
                $mvis fn rollback(&mut self, mark: $Mark) -> bool {
                    if mark.0 > self.len { return false; }
                    while self.len > mark.0 {
                        self.len -= 1;
                        let _ = self.values[self.len].take();
                    }
                    true
                }

                /// Removes every copyable value inserted after `mark`.
                ///
                /// This is the const-capable variant of [`rollback`][Self::rollback].
                /// Returns `false` without changing the arena if `mark` lies ahead
                /// of the current frontier.
                $mvis const fn rollback_copy(&mut self, mark: $Mark) -> bool where T: Copy {
                    if mark.0 > self.len { return false; }
                    while self.len > mark.0 {
                        self.len -= 1;
                        self.values[self.len] = None;
                    }
                    true
                }
            )?

            /// Removes every retained value.
            ///
            /// Later insertion begins again at index zero, so previously issued
            /// handles may resolve again.
            $vis fn clear(&mut self) {
                while self.len != 0 {
                    self.len -= 1;
                    let _ = self.values[self.len].take();
                }
            }
            /// Removes every retained copyable value.
            ///
            /// This is the const-capable variant of [`clear`][Self::clear].
            $vis const fn clear_copy(&mut self) where T: Copy {
                while self.len != 0 {
                    self.len -= 1;
                    self.values[self.len] = None;
                }
            }

            /* iteration */

            /// Iterates over retained values in insertion order.
            $vis fn iter(&self) -> impl Iterator<Item = &T> + '_ {
                let len = self.len;
                self.values[..len].iter().filter_map(|slot| slot.as_ref())
            }
            /// Iterates mutably over retained values in insertion order.
            $vis fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
                let len = self.len;
                self.values[..len].iter_mut().filter_map(|slot| slot.as_mut())
            }

            /* private */

            const fn __resolve_index(&self, handle: $Handle) -> Option<usize> {
                let index = match handle.index_usize() {
                    Ok(index) => index,
                    Err(_) => return None,
                };
                if index >= self.len || self.values[index].is_none() { return None; }
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
