// devela/src/data/store/arena/bytes/impls/vec.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __arena_bytes_impl_vec {
    (
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $($mvis:vis $Mark:ident)?]
        [module: $module:ident]
        ($_d:tt)
    ) => {
        $(#[$arena_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Arena {
            data: $crate::Vec<u8>,
        }

        impl Eq for $Arena {}
        impl PartialEq for $Arena {
            fn eq(&self, other: &Self) -> bool {
                $Arena::eq(self, other)
            }
        }
        impl Default for $Arena {
            fn default() -> Self {
                Self::new()
            }
        }

        #[allow(dead_code, private_interfaces)]
        impl $Arena {
            /* private helpers */

            /// Returns the maximum written byte length representable
            /// by both the cursor primitive and handle representation.
            const fn _max_capacity() -> usize {
                let pmax = $crate::unwrap![ok_or
                    $crate::MaybeNiche::<$cprim>::MAX.try_to_usize(), usize::MAX];
                let rmax = $crate::unwrap![ok_or
                    $crate::MaybeNiche::<$Cursor>::MAX.try_to_usize(), usize::MAX];
                if pmax < rmax { pmax } else { rmax }
            }
            /// Returns the written byte length in the machine indexing domain.
            const fn _len_usize(&self) -> usize { self.data.len() }

            /// Resolves a handle into a validated half-open byte range.
            const fn _span_usize(&self, h: $Handle) -> Option<(usize, usize)> {
                let start = $crate::unwrap![ok_some? h.get_offset_usize()];
                let len = $crate::unwrap![ok_some? h.get_len_usize()];
                let end = $crate::unwrap![some? start.checked_add(len)];
                if end > self._len_usize() { return None; }
                Some((start, end))
            }
            /// Resolves a repeated handle span into a validated byte range.
            const fn _span_repeat_usize(&self, h: $Handle, count: $cprim)
                -> Option<(usize, usize)> {
                let (start, first_end) = $crate::unwrap![some? self._span_usize(h)];
                let len = first_end - start;
                let total = $crate::unwrap![some? len.checked_mul(count as usize)];
                let end = $crate::unwrap![some? start.checked_add(total)];
                if end > self._len_usize() { return None; }
                Some((start, end))
            }

            /* construction */

            /// Returns a new empty arena.
            #[must_use]
            $vis const fn new() -> Self { Self { data: $crate::Vec::new() } }

            /// Returns a new empty arena with space for at least `capacity`
            /// bytes without reallocating.
            ///
            /// # Panics
            /// Panics if `capacity` exceeds the configured cursor or handle
            /// representation, or if the allocation cannot be created.
            #[must_use]
            $vis fn with_capacity(capacity: usize) -> Self {
                assert!(capacity <= Self::_max_capacity(),
                    "the requested arena capacity exceeds its cursor or handle representation");
                Self { data: $crate::Vec::with_capacity(capacity) }
            }

            /* capacity */

            /// Returns the usable byte capacity available without reallocating.
            #[must_use]
            $vis const fn capacity(&self) -> $cprim {
                let physical = self.data.capacity();
                let max = Self::_max_capacity();
                if physical < max { physical as $cprim } else { max as $cprim }
            }
            /// Returns the occupied byte length.
            #[must_use]
            $vis const fn len(&self) -> $cprim { self.data.len() as $cprim }

            /// Returns whether the arena contains no bytes.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.data.is_empty() }

            /// Returns whether no additional byte can be written.
            #[must_use]
            $vis const fn is_full(&self) -> bool { self.data.len() == Self::_max_capacity() }

            /// Returns the byte capacity remaining without reallocating.
            ///
            /// The vector may grow when this reaches zero unless
            /// [`is_full`][Self::is_full].
            #[must_use]
            $vis const fn remaining(&self) -> $cprim { self.capacity() - self.len() }

            /// Returns whether `n` additional bytes can be written.
            ///
            /// The operation may reallocate.
            #[must_use]
            $vis const fn can_write(&self, n: $cprim) -> bool {
                let start = self.data.len();
                let len = n as usize;
                let end = $crate::unwrap![some_or? start.checked_add(len), false];
                end <= Self::_max_capacity() && <$Handle>::try_from_usize(start, len).is_ok()
            }

            /* snapshot and rollback */

            $(
                /// Creates a rollback mark at the current byte length.
                #[must_use]
                $mvis const fn mark(&self) -> $Mark {
                    <$Mark>::new(self.len())
                }

                /// Rolls back to `mark`, returning whether the mark was valid.
                $mvis fn rollback(&mut self, mark: $Mark) -> bool {
                    let mark = mark.0 as usize;
                    if mark > self.data.len() {
                        return false;
                    }
                    self.data.truncate(mark);
                    true
                }
            )?

            /* whole storage */

            /// Returns a byte slice over all the written data.
            #[must_use]
            $vis fn as_bytes(&self) -> &[u8] { self.data.as_slice() }

            /// Returns an exclusive byte slice over all the written data.
            #[must_use]
            $vis fn as_bytes_mut(&mut self) -> &mut [u8] { self.data.as_mut_slice() }

            /// Compares two arenas for equality.
            #[must_use]
            $vis fn eq(&self, other: &Self) -> bool {
                $crate::Slice::<u8>::eq(self.as_bytes(), other.as_bytes())
            }

            /* byte spans */

            /// Appends `len` bytes initialized to `byte`.
            ///
            /// The underlying vector may reallocate.
            ///
            /// # Errors
            /// Returns `None` if the complete span would exceed the arena's
            /// representable byte-coordinate range.
            $hvis fn push_filled(&mut self, len: usize, byte: u8) -> Option<$Handle> {
                let start = self._len_usize();
                let end = $crate::unwrap![some? start.checked_add(len)];
                if end > Self::_max_capacity() { return None; }
                let handle = $crate::unwrap![ok_some? <$Handle>::try_from_usize(start, len)];
                self.data.resize(end, byte);
                Some(handle)
            }
            /// Appends `len` zero-initialized bytes.
            ///
            /// The underlying vector may reallocate.
            ///
            /// # Errors
            /// Returns `None` if the complete span would exceed the arena's
            /// representable byte-coordinate range.
            $hvis fn push_zeroed(&mut self, len: usize) -> Option<$Handle> {
                self.push_filled(len, 0)
            }
            /// Writes a byte slice into the arena.
            ///
            /// The underlying vector may reallocate.
            ///
            /// # Errors
            /// Returns `None` if the complete byte slice would exceed
            /// the arena's representable byte-coordinate range.
            $hvis fn push_bytes(&mut self, bytes: &[u8]) -> Option<$Handle> {
                let start = self._len_usize();
                let end = $crate::unwrap![some? start.checked_add(bytes.len())];
                if end > Self::_max_capacity() { return None; }
                let handle = // Validate everything before mutating the Vec.
                    $crate::unwrap![ok_some? <$Handle>::try_from_usize(start, bytes.len())];
                self.data.extend_from_slice(bytes);
                Some(handle)
            }
            /// Returns the bytes described by `handle`.
            $hvis fn read_bytes(&self, handle: $Handle) -> Option<&[u8]> {
                let (start, end) = $crate::unwrap![some? self._span_usize(handle)];
                Some(&self.data[start..end])
            }
            /// Returns the bytes described by `handle` exclusively.
            $hvis fn read_bytes_mut(&mut self, handle: $Handle) -> Option<&mut [u8]> {
                let (start, end) = $crate::unwrap![some? self._span_usize(handle)];
                Some(&mut self.data[start..end])
            }
            /// Replaces the bytes described by `handle`.
            $hvis fn replace_bytes(&mut self, handle: $Handle, new: &[u8]) -> bool {
                let dst = $crate::unwrap![some_or? self.read_bytes_mut(handle), false];
                if dst.len() != new.len() { return false; }
                dst.copy_from_slice(new);
                true
            }

            /* string spans */

            /// Writes a UTF-8 string into the arena.
            ///
            /// The returned handle describes exactly the string's UTF-8 bytes.
            /// No length prefix or terminator is stored.
            ///
            /// # Errors
            /// Returns `None` if the complete UTF-8 byte span would exceed
            /// the arena's representable byte-coordinate range.
            $hvis fn push_str(&mut self, val: &str) -> Option<$Handle> {
                self.push_bytes(val.as_bytes())
            }
            /// Returns the UTF-8 string described by `handle`.
            ///
            /// # Errors
            /// Returns `None` if the handle does not describe a span within the
            /// written prefix or if its bytes are not valid UTF-8.
            $hvis fn read_str(&self, handle: $Handle) -> Option<&str> {
                let bytes = $crate::unwrap![some? self.read_bytes(handle)];
                $crate::unwrap![ok_some $crate::Str::from_utf8(bytes)]
            }
            /// Returns the UTF-8 string described by `handle` exclusively.
            ///
            /// # Errors
            /// Returns `None` if the handle does not describe a span within the
            /// written prefix or if its bytes are not valid UTF-8.
            $hvis fn read_str_mut(&mut self, handle: $Handle) -> Option<&mut str> {
                let bytes = $crate::unwrap![some? self.read_bytes_mut(handle)];
                $crate::unwrap![ok_some $crate::Str::from_utf8_mut(bytes)]
            }
            /// Replaces the UTF-8 string described by `handle`.
            ///
            /// Returns `false` if the handle is invalid or `val` has a different
            /// byte length from the described span.
            $hvis fn replace_str(&mut self, handle: $Handle, val: &str) -> bool {
                self.replace_bytes(handle, val.as_bytes())
            }

            /* single bytes */

            /// Writes a single byte into the arena.
            ///
            /// # Errors
            /// Returns `None` if the byte would exceed
            /// the arena's representable byte-coordinate range.
            $hvis fn push_byte(&mut self, byte: u8) -> Option<$Handle> {
                self.push_bytes(&[byte])
            }
            /// Reads the single byte described by `h`.
            $hvis fn read_byte(&self, h: $Handle) -> Option<u8> {
                let (start, end) = $crate::unwrap![some? self._span_usize(h)];
                if end - start != 1 { return None; }
                Some(self.data[start])
            }
            /// Returns the single byte described by `h` exclusively.
            $hvis fn read_byte_mut(&mut self, h: $Handle) -> Option<&mut u8> {
                let (start, end) = $crate::unwrap![some? self._span_usize(h)];
                if end - start != 1 { return None; }
                Some(&mut self.data[start])
            }
            /// Replaces the single byte described by `h`.
            $hvis fn replace_byte(&mut self, h: $Handle, new: u8) -> bool {
                let dst = $crate::unwrap![some_or? self.read_byte_mut(h), false];
                *dst = new;
                true
            }

            /* views over multiple values */

            /// Returns a shared slice starting at `handle`,
            /// and spanning `count` items of its length.
            ///
            /// Returns `None` if the repeated span overflows
            /// or extends beyond the written prefix.
            $hvis fn view_bytes(&self, h: $Handle, count: $cprim) -> Option<&[u8]> {
                let (start, end) = $crate::unwrap![some? self._span_repeat_usize(h, count)];
                Some(&self.data[start..end])
            }
            /// Returns an exclusive slice starting at `handle`,
            /// and spanning `count` items of its length.
            ///
            /// Returns `None` if the repeated span overflows
            /// or extends beyond the written prefix.
            $hvis fn view_bytes_mut(&mut self, h: $Handle, count: $cprim) -> Option<&mut [u8]> {
                let (start, end) = $crate::unwrap![some? self._span_repeat_usize(h, count)];
                Some(&mut self.data[start..end])
            }

            /* shrinking the arena */

            /// Truncates the arena if `handle` describes its final region.
            $hvis fn truncate_last(&mut self, h: $Handle) -> bool {
                let (start, end) = $crate::unwrap![some_or? self._span_usize(h), false];
                if end != self._len_usize() { return false; }
                self.data.truncate(start);
                true
            }
            /// Copies the final stored span into `dst` and removes it.
            $hvis fn pop_into(&mut self, h: $Handle, dst: &mut [u8]) -> bool {
                let src = $crate::unwrap![some_or? self.read_bytes(h), false];
                if src.len() != dst.len() { return false; }
                dst.copy_from_slice(src);
                self.truncate_last(h)
            }
            /// Removes all written bytes.
            $vis fn clear(&mut self) { self.data.clear(); }

        }

        /* primitives */

        #[allow(dead_code, private_interfaces)]
        impl $Arena {
            $module::_impl_arena_methods_for_prims!();

            /* bool */

            /// Pushes a `bool`. Returns its handle on success.
            ///
            /// # Errors
            /// Returns `None` if the value would exceed
            /// the arena's representable byte-coordinate range.
            $hvis fn push_bool(&mut self, val: bool) -> Option<$Handle> {
                self.push_byte(val as u8)
            }
            /// Reads a `bool` from the given `handle`.
            $hvis fn read_bool(&self, handle: $Handle) -> Option<bool> {
                match self.read_byte(handle) {
                    Some(0) => Some(false),
                    Some(1) => Some(true),
                    _ => None,
                }
            }
            /// Replaces a `bool` from the given `handle`.
            $hvis fn replace_bool(&mut self, handle: $Handle, val: bool) -> bool {
                self.replace_byte(handle, val as u8)
            }

            /* char */

            /// Pushes a `char`. Returns its handle on success.
            ///
            /// # Errors
            /// Returns `None` if the value would exceed
            /// the arena's representable byte-coordinate range.
            $hvis fn push_char(&mut self, val: char) -> Option<$Handle> {
                self.push_u32(val as u32)
            }
            /// Reads a `char` from the given `handle`.
            $hvis fn read_char(&self, handle: $Handle) -> Option<char> {
                $crate::unwrap![some_map_into self.read_u32(handle), |c| char::from_u32(c)]
            }
            /// Replaces a `char` from the given `handle`.
            $hvis fn replace_char(&mut self, handle: $Handle, val: char) -> bool {
                self.replace_u32(handle, val as u32)
            }
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        mod $module {
            /// Private helper to implement push, read & replace methods over primitives.
            #[rustfmt::skip]
            macro_rules! _impl_arena_methods_for_prims {
                () => {
                    $module::_impl_arena_methods_for_prims!(single-byte: u8, i8);
                    $module::_impl_arena_methods_for_prims!(multi-byte:
                        u16, u32, u64, u128, usize,
                        i16, i32, i64, i128, isize,
                        f32, f64,
                    );
                };
                (single-byte: $_d($oprim:ty),+ $_d(,)?) => {
                    $_d( $module::_impl_arena_methods_for_prims!(%single-byte: $oprim); )+
                };
                (%single-byte: $oprim:ty) => { $crate::paste! {
                    #[doc = "Pushes a `" $oprim "`. Returns its handle on success."]
                    /// # Errors
                    /// Returns `None` if the value would exceed
                    /// the arena's representable byte-coordinate range.
                    $hvis fn [<push_ $oprim>](&mut self, val: $oprim) -> Option<$Handle> {
                        self.push_byte(val as u8)
                    }
                    #[doc = "Reads a `" $oprim "` from the given `handle`."]
                    $hvis fn [<read_ $oprim>](&self, handle: $Handle) -> Option<$oprim> {
                        if let Some(b) = self.read_byte(handle) { Some(b as $oprim) } else { None }
                    }
                    #[doc = "Replaces a `" $oprim
                    "` from the given `handle`. Returns `true` on success."]
                    $hvis fn [<replace_ $oprim>](&mut self, handle: $Handle, val: $oprim) -> bool {
                        self.replace_byte(handle, val as u8)
                    }
                }};
                (multi-byte: $_d($oprim:ty),+ $_d(,)?) => {
                    $_d( $module::_impl_arena_methods_for_prims!(%multi-byte: $oprim); )+
                };
                (%multi-byte: $oprim:ty) => { $crate::paste! {
                    #[doc = "Pushes a `" $oprim
                    "` in little-endian order. Returns its handle on success."]
                    /// # Errors
                    /// Returns `None` if the value would exceed
                    /// the arena's representable byte-coordinate range.
                    $hvis fn [<push_ $oprim>](&mut self, val: $oprim) -> Option<$Handle> {
                        self.push_bytes(&val.to_le_bytes())
                    }
                    #[doc = "Reads a `" $oprim
                    "` in little-endian order from the given `handle`."]
                    $hvis fn [<read_ $oprim>](&self, handle: $Handle) -> Option<$oprim> {
                        const T_SIZE: usize = core::mem::size_of::<$oprim>();
                        let bytes = $crate::unwrap![some? self.read_bytes(handle)];
                        if bytes.len() != T_SIZE { return None; }
                        Some($oprim::from_le_bytes(
                            *$crate::unwrap![some? bytes.first_chunk::<T_SIZE>()]))
                    }
                    #[doc = "Replaces a `" $oprim
                    "` from the given `handle`. Returns `true` on success."]
                    $hvis fn [<replace_ $oprim>](&mut self, handle: $Handle, val: $oprim) -> bool {
                        const T_SIZE: usize = core::mem::size_of::<$oprim>();
                        let bytes = $crate::unwrap![some_or? self.read_bytes_mut(handle), false];
                        if bytes.len() != T_SIZE { return false; }
                        let arr = $crate::unwrap![some_or? bytes.first_chunk_mut::<T_SIZE>(), false];
                        *arr = val.to_le_bytes();
                        true
                    }
                }};
            }
            pub(super) use _impl_arena_methods_for_prims;
        }
    };
}
