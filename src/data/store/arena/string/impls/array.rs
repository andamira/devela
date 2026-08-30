// devela/src/data/store/arena/string/impls/array.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __arena_string_impl_array {
    (
        [index: $iprim:ident + $Index:ty;]
        [cursor: $cprim:ident + $Cursor:ty;]
        $(#[$arena_attr:meta])* $vis:vis $Arena:ident;
        $hvis:vis $Handle:ident;
        [mark: $($mvis:vis $Mark:ident)?]
    ) => {
        $(#[$arena_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Arena<const STRINGS: usize, const BYTES: usize> {
            /// Cumulative UTF-8 byte end of every retained string.
            ends: [$crate::MaybeNiche<$Cursor>; STRINGS],

            /// Packed UTF-8 bytes.
            data: [$crate::MaybeByte; BYTES],

            /// Current string insertion frontier.
            len: $crate::MaybeNiche<$Index>,
        }

        impl<const STRINGS: usize, const BYTES: usize> $crate::ConstInit
            for $Arena<STRINGS, BYTES> {
            const INIT: Self = Self::new();
        }
        impl<const STRINGS: usize, const BYTES: usize> Default for $Arena<STRINGS, BYTES> {
            fn default() -> Self { Self::new() }
        }

        #[allow(dead_code)]
        impl<const STRINGS: usize, const BYTES: usize> $Arena<STRINGS, BYTES> {
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
                assert!(STRINGS <= Self::MAX_CAPACITY,
                    "arena_string! string capacity exceeds its index representation");
                assert!(BYTES <= Self::MAX_BYTE_CAPACITY,
                    "arena_string! byte capacity exceeds its cursor representation");
            };

            /// Maximum fixed string-entry capacity supported by the index representation.
            $vis const MAX_CAPACITY: usize = {
                $crate::unwrap![ok_or $crate::MaybeNiche::<$Index>::MAX.try_to_usize(), usize::MAX]
            };
            /// Maximum fixed packed-byte capacity supported by the cursor representation.
            $vis const MAX_BYTE_CAPACITY: usize = {
                $crate::unwrap![ok_or $crate::MaybeNiche::<$Cursor>::MAX.try_to_usize(), usize::MAX]
            };

            /* construction */

            /// Creates an empty fixed-capacity string arena.
            #[must_use]
            $vis const fn new() -> Self {
                let () = Self::_VALID_CONFIG;
                let zero =
                    $crate::unwrap![some_guaranteed_or_ub $crate::MaybeNiche::<$Cursor>::ZERO];
                Self {
                    ends: [zero; STRINGS],
                    data: $crate::__ArenaBytesArray::<BYTES>::new_array(),
                    len: Self::_len_zero(),
                }
            }

            /* string capacity */

            /// Returns the fixed string-entry capacity.
            #[must_use]
            $vis const fn capacity(&self) -> usize { STRINGS }

            /// Returns the number of retained strings.
            #[must_use]
            $vis const fn len(&self) -> usize { self._len_usize() }

            /// Returns whether no strings are retained.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.len() == 0 }

            /// Returns whether the string-entry capacity is exhausted.
            #[must_use]
            $vis const fn remaining(&self) -> usize { STRINGS - self.len() }

            /// Returns the remaining string-entry capacity.
            #[must_use]
            $vis const fn is_full(&self) -> bool { self.len() == STRINGS }

            /* byte capacity */

            /// Returns the fixed packed-byte capacity.
            #[must_use]
            $vis const fn byte_capacity(&self) -> usize { BYTES }

            /// Returns the number of packed bytes occupied by retained strings.
            #[must_use]
            $vis const fn byte_len(&self) -> usize {
                let len = self.len();
                if len == 0 { 0 } else { self._end_usize(len - 1) }
            }
            /// Returns the remaining packed-byte capacity.
            #[must_use]
            $vis const fn byte_remaining(&self) -> usize {
                BYTES - self.byte_len()
            }
            /// Returns whether `string` fits the remaining entry and byte capacities.
            #[must_use]
            $vis const fn can_insert(&self, string: &str) -> bool {
                !self.is_full() && string.len() <= self.byte_remaining()
            }

            /* access */

            /// Returns whether `handle` currently resolves to a retained string.
            #[must_use]
            $hvis const fn contains(&self, handle: $Handle) -> bool {
                self._resolve_index(handle).is_some()
            }
            /// Returns the retained string currently resolved by `handle`.
            #[must_use]
            $hvis const fn get(&self, handle: $Handle) -> Option<&str> {
                let index = $crate::unwrap![some? self._resolve_index(handle)];
                Some(self._str_at(index))
            }
            /// Returns all packed UTF-8 bytes, without preserving entry boundaries.
            #[must_use]
            $vis const fn as_bytes(&self) -> &[u8] {
                $crate::__ArenaBytesArray::<BYTES>::slice_bytes(&self.data, 0, self.byte_len())
            }

            /* mutation */

            /// Inserts a string at the current frontier.
            ///
            /// Equal strings are retained independently and receive distinct handles.
            ///
            /// Returns `None` if either the string-entry capacity or byte capacity
            /// is insufficient.
            $hvis const fn insert(&mut self, string: &str) -> Option<$Handle> {
                if self.is_full() { return None; }
                let index_usize = self.len();
                let start = self.byte_len();
                let end = $crate::unwrap![some? start.checked_add(string.len())];
                if end > BYTES { return None; }
                // Check every representational constraint before mutating storage.
                let end_repr =
                    $crate::unwrap![ok_some? $crate::MaybeNiche::<$Cursor>::try_from_usize(end)];
                let index = self.len;
                let handle = $Handle::new(index.get());
                let bytes = string.as_bytes();
                $crate::whilst! { i in 0..bytes.len(); {
                    $crate::__ArenaBytesArray::<BYTES>
                        ::write_byte(&mut self.data, start + i, bytes[i]);
                }}
                self.ends[index_usize] = end_repr;
                self.len = Self::_len_from_usize(index_usize + 1);
                Some(handle)
            }

            /* marks and reclamation */

            $(
                /// Returns a mark at the current string insertion frontier.
                #[must_use]
                $mvis const fn mark(&self) -> $Mark { <$Mark>::new(self.len) }

                /// Reclaims every string inserted after `mark`.
                ///
                /// The packed byte frontier follows automatically
                /// from the restored string frontier.
                ///
                /// Returns `false` without changing the arena
                /// if `mark` lies ahead of the current frontier.
                $mvis const fn rollback(&mut self, mark: $Mark) -> bool {
                    if mark.0.gt(self.len) { return false; }
                    self.len = mark.0;
                    true
                }
            )?
            /// Reclaims every retained string.
            ///
            /// Previously issued handles may resolve again after later insertion.
            $vis const fn clear(&mut self) {
                self.len = Self::_len_zero();
            }

            /* iteration */

            /// Iterates over retained strings in insertion order.
            $vis fn iter(&self) -> impl Iterator<Item = &str> + '_ {
                (0..self.len()).map(|index| self._str_at(index))
            }
            /// Iterates over retained string handles in insertion order.
            $vis fn ids(&self) -> impl Iterator<Item = $Handle> + '_ {
                (0..self.len()).filter_map(|index| <$Handle>::try_from_usize(index).ok())
            }
            /// Iterates over retained handles and strings in insertion order.
            $vis fn entries(&self) -> impl Iterator<Item = ($Handle, &str)> + '_ {
                (0..self.len()).filter_map(move |index| {
                    let handle = <$Handle>::try_from_usize(index).ok()?;
                    Some((handle, self._str_at(index)))
                })
            }

            /* private */

            const fn _len_zero() -> $crate::MaybeNiche<$Index> {
                $crate::unwrap![some_guaranteed_or_ub $crate::MaybeNiche::<$Index>::ZERO]
            }
            const fn _len_from_usize(len: usize) -> $crate::MaybeNiche<$Index> {
                $crate::unwrap![ok_guaranteed_or_ub
                    $crate::MaybeNiche::<$Index>::try_from_usize(len)]
            }
            const fn _len_usize(&self) -> usize {
                $crate::unwrap![ok_guaranteed_or_ub self.len.try_to_usize()]
            }
            const fn _end_usize(&self, index: usize) -> usize {
                $crate::unwrap![ok_guaranteed_or_ub self.ends[index].try_to_usize()]
            }
            const fn _range_at(&self, index: usize) -> (usize, usize) {
                let start = if index == 0 { 0 } else { self._end_usize(index - 1) };
                let end = self._end_usize(index);
                (start, end)
            }
            const fn _str_at(&self, index: usize) -> &str {
                let (start, end) = self._range_at(index);
                let bytes = $crate::__ArenaBytesArray::<BYTES>::slice_bytes(&self.data, start, end);
                // Every stored span entered through `&str`.
                $crate::Str::__utf8_bytes_to_str(bytes)
            }
            const fn _resolve_index(&self, handle: $Handle) -> Option<usize> {
                let index = $crate::unwrap![ok_some? handle.get_index_usize()];
                if index >= self.len() { return None; }
                Some(index)
            }
        }
    };
}
