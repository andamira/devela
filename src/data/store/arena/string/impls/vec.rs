// devela/src/data/store/arena/string/impls/vec.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __arena_string_impl_vec {
    (
        [index: $iprim:ident + $Index:ty;]
        [cursor: $cprim:ident + $Cursor:ty;]
        $(#[$arena_attr:meta])* $vis:vis $Arena:ident;
        $hvis:vis $Handle:ident;
        [mark: $($mvis:vis $Mark:ident)?]
    ) => {
        $(#[$arena_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Arena {
            /// Cumulative UTF-8 byte end of every retained string.
            ends: $crate::Vec<$crate::MaybeNiche<$Cursor>>,

            /// Packed UTF-8 bytes.
            data: $crate::Vec<u8>,
        }
        impl Default for $Arena {
            fn default() -> Self { Self::new() }
        }

        #[allow(dead_code)]
        impl $Arena {
            const _VALID_CONFIG: () = {
                const fn require_index<P: $crate::PrimIndex>() {}
                require_index::<$iprim>();
                require_index::<$cprim>();
                assert!(!$crate::MaybeNiche::<$Index>::HAS_NEGATIVE
                        && $crate::MaybeNiche::<$Index>::IS_CONTIGUOUS
                        && $crate::MaybeNiche::<$Index>::ZERO.is_some(),
                    "arena_string! index representation must be unsigned, contiguous, and contain zero",
                );
                assert!(!$crate::MaybeNiche::<$Cursor>::HAS_NEGATIVE
                        && $crate::MaybeNiche::<$Cursor>::IS_CONTIGUOUS
                        && $crate::MaybeNiche::<$Cursor>::ZERO.is_some(),
                    "arena_string! cursor representation must be unsigned, contiguous, and contain zero",
                );
                // Also forces `$Cursor` to use `$cprim` as its primitive carrier.
                let _ = $crate::MaybeNiche::<$Cursor>::try_from_prim(0 as $cprim);
            };
            /// Maximum string-entry capacity supported by the index representation.
            $vis const MAX_CAPACITY: usize = Self::__index_capacity();

            /// Maximum packed-byte length supported by the cursor representation.
            $vis const MAX_BYTE_CAPACITY: usize = {
                $crate::unwrap![ok_or $crate::MaybeNiche::<$Cursor>::MAX.try_to_usize(), usize::MAX]
            };

            /* construction */

            /// Creates an empty allocating string arena.
            #[must_use]
            $vis const fn new() -> Self {
                let () = Self::_VALID_CONFIG;
                Self {
                    ends: $crate::Vec::new(),
                    data: $crate::Vec::new(),
                }
            }
            /// Creates an empty arena with initial string and byte capacities.
            ///
            /// # Panics
            /// Panics if either requested capacity exceeds its configured representation.
            #[must_use]
            $vis fn with_capacity(string_capacity: usize, byte_capacity: usize) -> Self {
                let () = Self::_VALID_CONFIG;
                assert!(string_capacity <= Self::MAX_CAPACITY,
                    "arena_string! string capacity exceeds its index representation");
                assert!(byte_capacity <= Self::MAX_BYTE_CAPACITY,
                    "arena_string! byte capacity exceeds its cursor representation");
                Self {
                    ends: $crate::Vec::with_capacity(string_capacity),
                    data: $crate::Vec::with_capacity(byte_capacity),
                }
            }

            /* string capacity */

            /// Returns the string capacity available without reallocating.
            #[must_use]
            $vis fn capacity(&self) -> usize {
                ::core::cmp::min(self.ends.capacity(), Self::MAX_CAPACITY)
            }
            /// Returns the number of retained strings.
            #[must_use]
            $vis const fn len(&self) -> usize { self.ends.len() }

            /// Returns whether no strings are retained.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.ends.is_empty() }

            /// Returns whether no further string identity can be represented.
            #[must_use]
            $vis const fn is_full(&self) -> bool {
                $crate::MaybeNiche::<$Index>::try_from_usize(self.ends.len()).is_err()
            }
            /// Returns how many strings fit without reallocating.
            #[must_use]
            $vis fn remaining(&self) -> usize { self.capacity() - self.len() }

            /* byte capacity */

            /// Returns the packed-byte capacity available without reallocating.
            #[must_use]
            $vis fn byte_capacity(&self) -> usize {
                ::core::cmp::min(self.data.capacity(), Self::MAX_BYTE_CAPACITY)
            }
            /// Returns the packed UTF-8 byte length.
            #[must_use]
            $vis const fn byte_len(&self) -> usize { self.data.len() }

            /// Returns how many bytes fit without reallocating.
            #[must_use]
            $vis fn byte_remaining(&self) -> usize {
                self.byte_capacity() - self.byte_len()
            }

            /// Returns whether `string` can be represented by this arena.
            ///
            /// This concerns the configured index and cursor representations;
            /// insertion may still reallocate.
            #[must_use]
            $vis const fn can_insert(&self, string: &str) -> bool {
                if self.is_full() { return false; }
                let Some(end) = self.data.len().checked_add(string.len()) else { return false; };
                $crate::MaybeNiche::<$Cursor>::try_from_usize(end).is_ok()
            }

            /* access */

            /// Returns whether `handle` currently resolves to a retained string.
            #[must_use]
            $hvis const fn contains(&self, handle: $Handle) -> bool {
                self.__resolve_index(handle).is_some()
            }
            /// Returns the string resolved by `handle`.
            #[must_use]
            $hvis fn get(&self, handle: $Handle) -> Option<&str> {
                let index = $crate::unwrap![some? self.__resolve_index(handle)];
                Some(self.__str_at(index))
            }
            /// Returns all packed UTF-8 bytes, without preserving entry boundaries.
            #[must_use]
            $vis fn as_bytes(&self) -> &[u8] {
                &self.data
            }

            /* mutation */

            /// Inserts a string at the current frontier.
            ///
            /// Equal strings are retained independently and receive distinct handles.
            ///
            /// Returns `None` if the next string identity or resulting byte
            /// frontier cannot be represented.
            $hvis fn insert(&mut self, string: &str) -> Option<$Handle> {
                // Validate every representational constraint before mutation.
                let index = $crate::unwrap![ok_some?
                    $crate::MaybeNiche::<$Index>::try_from_usize(self.ends.len())];
                let end = $crate::unwrap![some? self.data.len().checked_add(string.len())];
                let end = $crate::unwrap![ok_some?
                    $crate::MaybeNiche::<$Cursor>::try_from_usize(end)];
                self.data.extend_from_slice(string.as_bytes());
                self.ends.push(end);
                Some($Handle::new(index.get()))
            }

            /* marks and reclamation */

            $(
                /// Returns a mark at the current insertion frontier.
                #[must_use]
                $mvis const fn mark(&self) -> $Mark {
                    <$Mark>::new(self.ends.len())
                }
                /// Reclaims every string inserted after `mark`.
                ///
                /// The packed byte frontier follows automatically
                /// from the restored string frontier.
                ///
                /// Returns `false` without changing the arena
                /// if `mark` lies ahead of the current frontier.
                $mvis fn rollback(&mut self, mark: $Mark) -> bool {
                    if mark.0 > self.ends.len() { return false; }
                    let byte_len = if mark.0 == 0 { 0 } else { self.__end_usize(mark.0 - 1) };
                    self.ends.truncate(mark.0);
                    self.data.truncate(byte_len);
                    true
                }
            )?
            /// Reclaims every retained string.
            ///
            /// Previously issued handles may resolve again after later insertion.
            $vis fn clear(&mut self) {
                self.ends.clear();
                self.data.clear();
            }

            /* iteration */

            /// Iterates over retained strings in insertion order.
            $vis fn iter(&self) -> impl Iterator<Item = &str> + '_ {
                (0..self.len()).map(|index| self.__str_at(index))
            }
            /// Iterates over retained string handles in insertion order.
            $vis fn ids(&self) -> impl Iterator<Item = $Handle> + '_ {
                (0..self.len())
                    .filter_map(|index| <$Handle>::try_from_usize(index).ok())
            }
            /// Iterates over retained handles and strings in insertion order.
            $vis fn entries(&self) -> impl Iterator<Item = ($Handle, &str)> + '_ {
                (0..self.len()).filter_map(move |index| {
                    let handle = <$Handle>::try_from_usize(index).ok()?;
                    Some((handle, self.__str_at(index)))
                })
            }

            /* private */

            const fn __index_capacity() -> usize {
                match $crate::MaybeNiche::<$Index>::MAX.try_to_usize() {
                    Ok(max) => max.saturating_add(1),
                    Err(_) => usize::MAX,
                }
            }
            fn __end_usize(&self, index: usize) -> usize {
                $crate::unwrap![ok_guaranteed_or_ub self.ends[index].try_to_usize()]
            }
            fn __range_at(&self, index: usize) -> (usize, usize) {
                let start = if index == 0 { 0 } else { self.__end_usize(index - 1) };
                let end = self.__end_usize(index);
                (start, end)
            }
            fn __str_at(&self, index: usize) -> &str {
                let (start, end) = self.__range_at(index);
                // SAFETY invariant: every stored byte entered through `&str`,
                // and rollback only truncates at recorded string boundaries.
                $crate::Str::__utf8_bytes_to_str(&self.data[start..end])
            }
            const fn __resolve_index(&self, handle: $Handle) -> Option<usize> {
                let index = $crate::unwrap![ok_some? handle.get_index_usize()];
                if index >= self.ends.len() { return None; }
                Some(index)
            }
        }
    };
}
