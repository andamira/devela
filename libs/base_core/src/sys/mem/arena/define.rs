// devela_base_core::sys::mem::arena::bytes
//
//! Defines [`define_arena`].
//

#[cfg(any(doc, test))]
define_arena! {
    #[doc = crate::_TAG_EXAMPLE!()]
    pub ExampleArena
}

/// A custom memory arena generator.
///
/// # Features
/// Uses `unsafe_array` to leverage `MaybeUninit` and avoid initializing the full capacity.
/// And uses `unsafe_slice` for further performance gains.
///
/// # Examples
/// See: [`ExampleArena`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! define_arena {
    // Point of entry, defines the names
    (
    $(#[$arena_attr:meta])*
    $vis:vis $Arena:ident
    ) => {
        define_arena![%arena
            $(#[$arena_attr])* $vis $Arena,
            $crate::ArenaHandle,
            $crate::__Arena::<CAP>
        ];
    };
    () => {};
    // calls the necessary arms in order.
    (%arena $(#[$arena_attr:meta])* $vis:vis $Arena:ident, $Handle:ty, $_:ty) => { $crate::paste! {
        define_arena![%main
            $(#[$arena_attr])* $vis $Arena, // the arena type
            $Handle, // the handle type
            [<$Arena Mark>], // the mark type
            $_ // the internal ops arena namespace
        ];
        define_arena![%prim $Arena, $Handle, ($)];
        define_arena![%mark $Arena, $(#[$arena_attr])* $vis [<$Arena Mark>]];

        #[cfg(test)]
        define_arena![%tests $Arena, [<test_ $Arena>]];
    }};
    // $Arena:   the name of the new generated arena.
    // $Handle:  the handle type used by the arena.
    // $_:       internal arena helper.
    (%main $(#[$arena_attr:meta])* $vis:vis $Arena:ident, $Handle:ty, $Mark:ty, $_:ty) => {
        $(#[$arena_attr])*
        /// An heterogeneous byte arena.
        ///
        /// Generated with [`define_arena!`].
        #[derive(Clone, Debug)]
        $vis struct $Arena<const CAP: usize> {
            data: [$crate::MaybeByte; CAP],
            len: usize,
        }

        /* misc. trait impls */

        impl<const CAP: usize> Eq for $Arena<CAP> {}
        impl<const CAP: usize> PartialEq for $Arena<CAP> {
            fn eq(&self, other: &Self) -> bool { $Arena::eq(self, other) }
        }
        impl<const CAP: usize> Default for $Arena<CAP> {
            fn default() -> Self { Self::new() }
        }

        // Fundamental methods.
        #[allow(dead_code)]
        impl<const CAP: usize> $Arena<CAP> {

            /// Returns a new empty arena.
            #[inline(always)]
            pub const fn new() -> Self {
                Self { data: <$_>::new_array(), len: 0 }
            }

            #[inline(always)]
            /// Returns the total capacity.
            pub const fn capacity(&self) -> usize { CAP }
            #[inline(always)]
            /// Return the occupied length.
            pub const fn len(&self) -> usize { self.len }
            #[inline(always)]
            /// Whether the arena is empty.
            pub const fn is_empty(&self) -> bool { self.len == 0 }
            #[inline(always)]
            /// Returns the remaining byte capacity.
            pub const fn remaining(&self) -> usize { CAP - self.len }
            /// Returns `true` if n bytes fit in the remaining capacity.
            #[inline(always)]
            pub const fn can_write(&self, n: usize) -> bool { self.len + n <= CAP }

            #[inline(always)]
            /// Compares two arenas for equality.
            pub const fn eq(&self, other: &Self) -> bool {
                $crate::Slice::<u8>::eq(self.as_bytes(), other.as_bytes())
            }

            /* snapshot and rollback */

            #[inline(always)]
            /// Snapshot a position in the arena.
            pub const fn mark(&self) -> $Mark { <$Mark>::new(self.len) }
            #[inline(always)]
            /// Rollback to a previous marked position.
            pub const fn rollback(&mut self, m: $Mark) { self.len = m.0; }

            /* byte slices */

            #[inline(always)]
            /// Returns a byte slice over all the written data.
            pub const fn as_bytes(&self) -> &[u8] {
                <$_>::slice_bytes(&self.data, 0, self.len)
            }
            /// Returns an exclusive byte slice over all the written data.
            #[inline(always)]
            pub const fn as_bytes_mut(&mut self) -> &mut [u8] {
                $crate::__Arena::<CAP>::slice_bytes_mut(&mut self.data, 0, self.len)
            }

            /// Write a byte slice into the arena.
            pub const fn push_bytes(&mut self, bytes: &[u8]) -> Option<$Handle> {
                $crate::unwrap!(some_if? self.len.checked_add(bytes.len()), |v| v <= CAP);
                let start = self.len;
                let handle = <$Handle>::new(start, bytes.len());
                $crate::whilst! { i in 0..bytes.len(); {
                    $crate::__Arena::<CAP>::write_byte(&mut self.data, self.len, bytes[i]);
                    self.len += 1;
                }}
                Some(handle)
            }

            /// Read a shared slice over the written bytes.
            pub const fn read_bytes(&self, h: $Handle) -> Option<&[u8]> {
                if h.offset() + h.len() > self.len { return None }
                Some($crate::__Arena::<CAP>::slice_bytes(&self.data,
                    h.offset(), h.offset() + h.len()))
            }
            /// Read an exclusive slice over the written bytes.
            pub const fn read_bytes_mut(&mut self, h: $Handle) -> Option<&mut [u8]> {
                if h.offset() + h.len() > self.len { return None; }
                Some($crate::__Arena::<CAP>::slice_bytes_mut(&mut self.data,
                    h.offset(), h.offset() + h.len()))
            }

            /// Replace the bytes for the handle. Lengths must match. Returns `false` otherwise.
            pub const fn replace_bytes(&mut self, h: $Handle, new: &[u8]) -> bool {
                if let Some(dst) = self.read_bytes_mut(h) {
                    if dst.len() == new.len() {
                        dst.copy_from_slice(new);
                        return true;
                    }
                }
                false
            }

            /* single bytes */

            /// Write a single byte into the arena.
            pub const fn push_byte(&mut self, byte: u8) -> Option<$Handle> {
                if self.len + 1 > CAP { return None; }
                <$_>::write_byte(&mut self.data, self.len, byte);
                let handle = <$Handle>::new(self.len, 1);
                self.len += 1;
                Some(handle)
            }
            /// Read a byte previously written.
            pub const fn read_byte(&self, handle: $Handle) -> Option<u8> {
                if handle.offset() + 1 > self.len { return None }
                Some(<$_>::read_byte(&self.data, handle.offset()))
            }
            /// Read a byte previously written.
            pub const fn read_byte_mut(&mut self, handle: $Handle) -> Option<&mut u8> {
                if handle.offset() + 1 > self.len { return None }
                Some(<$_>::read_byte_mut(&mut self.data, handle.offset()))
            }
            /// Replace the bytes for `handle`. Length must match.
            pub const fn replace_byte(&mut self, handle: $Handle, new: u8) -> bool {
                if handle.len() != 1 { return false; }
                <$_>::write_byte(&mut self.data, handle.offset(), new);
                true
            }

            /* views over multiple values */

            /// Returns a shared slice starting at `handle`,
            /// and spanning `count` items of its length.
            ///
            /// Returns `None` if...
            pub const fn view_bytes(&self, h: $Handle, count: usize) -> Option<&[u8]> {
                let total = h.len() * count;
                if h.offset() + total > self.len { return None; }
                Some(<$_>::slice_bytes(&self.data, h.offset(), h.offset() + total))
            }

            /// Returns an exclusive slice starting at `handle`,
            /// and spanning `count` items of its length.
            ///
            /// Returns `None` if...
            pub const fn view_bytes_mut(&mut self, h: $Handle, count: usize) -> Option<&mut [u8]> {
                let total = h.len() * count;
                if h.offset() + total > self.len { return None; }
                Some(<$_>::slice_bytes_mut(&mut self.data, h.offset(), h.offset() + total))
            }

            /* shrinking the arena */

            /// Truncates the arena if the handle corresponds to the last region.
            pub const fn truncate_last(&mut self, h: $Handle) -> bool {
                let end = h.offset() + h.len();
                if end != self.len { return false; }
                self.len = h.offset();
                true
            }

            /// Copies the last stored value into `dst` and removes it.
            pub const fn pop_into(&mut self, h: $Handle, dst: &mut [u8]) -> bool {
                if h.len() != dst.len() { return false; }
                if let Some(src) = self.read_bytes(h) {
                    $crate::whilst! { i in 0..h.len(); { dst[i] = src[i]; }}
                    self.truncate_last(h)
                } else { false }
            }
        }
    };
    // $_d: the dollar sign passed as a token, as a trick to be able to nest repetitions.
    (%prim $Arena:ident, $Handle:ty, ($_d:tt)) => {
        #[allow(dead_code)]
        /// Implements push, read and replace for primitives.
        impl<const CAP: usize> $Arena<CAP> {
            impl_for_primitives!();

            /* bool */

            /// Pushes a `char`. Returns its handle on success.
            ///
            /// # Errors
            /// Returns `None` if there’s insufficient capacity.
            pub const fn push_bool(&mut self, val: bool) -> Option<$Handle> {
                self.push_byte(val as u8)
            }
            /// Reads a `bool` from the given `handle`.
            ///
            /// # Errors
            /// Returns `None` if the handle is invalid or incomplete.
            pub const fn read_bool(&self, handle: $Handle) -> Option<bool> {
                if let Some(b) = self.read_byte(handle) { Some(b != 0) } else { None }
            }
            /// Replaces a `bool` from the given `handle`. Returns `true` on success.
            pub const fn replace_bool(&mut self, handle: $Handle, val: bool) -> bool {
                self.replace_byte(handle, val as u8)
            }

            /* char */

            /// Pushes a `char`. Returns its handle on success.
            ///
            /// # Errors
            /// Returns `None` if there’s insufficient capacity.
            #[inline(always)]
            pub const fn push_char(&mut self, val: char) -> Option<$Handle> {
                self.push_u32(val as u32)
            }
            /// Reads a `char` from the given `handle`.
            ///
            /// # Errors
            /// Returns `None` if the handle is invalid or incomplete.
            #[inline(always)]
            pub const fn read_char(&self, handle: $Handle) -> Option<char> {
                if let Some(c) = self.read_u32(handle) { char::from_u32(c) } else { None }
            }
            /// Replaces a `char` from the given `handle`. Returns `true` on success.
            #[inline(always)]
            pub const fn replace_char(&mut self, handle: $Handle, val: char) -> bool {
                self.replace_u32(handle, val as u32)
            }
        }

        /// Private helper to implement push, read & replace methods over primitives.
        #[rustfmt::skip] // fixes rustfmt warnings
        macro_rules! impl_for_primitives {
            () => {
                impl_for_primitives!(single-byte: u8, i8);
                impl_for_primitives!(multi-byte:
                    u16, u32, u64, u128, usize,
                    i16, i32, i64, i128, isize,
                    f32, f64,
                );
                impl_for_primitives!(str_len: u8, u16, u32, usize);
            };
            (single-byte: $_d($T:ty),+ $_d(,)?) => {
                $_d( impl_for_primitives!(%single-byte: $T); )+
            };
            (%single-byte: $T:ty) => { $crate::paste! {
                #[doc = "Pushes a `" $T "`. Returns its handle on success."]
                ///
                /// # Errors
                /// Returns `None` if there’s insufficient capacity.
                #[inline(always)]
                pub const fn [<push_ $T>](&mut self, val: $T) -> Option<$Handle> {
                    self.push_byte(val as u8)
                }
                #[doc = "Reads a `" $T "` from the given `handle`."]
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                pub const fn [<read_ $T>](&self, handle: $Handle) -> Option<$T> {
                    if let Some(b) = self.read_byte(handle) { Some(b as $T) } else { None }
                }
                #[doc = "Replaces a `" $T "` from the given `handle`. Returns `true` on success."]
                pub const fn [<replace_ $T>](&mut self, handle: $Handle, val: $T) -> bool {
                    self.replace_byte(handle, val as u8)
                }
            }};
            (multi-byte: $_d($T:ty),+ $_d(,)?) => {
                $_d( impl_for_primitives!(%multi-byte: $T); )+
            };
            (%multi-byte: $T:ty) => { $crate::paste! {
                #[doc = "Pushes a `" $T "` in little-endian order. Returns its handle on success."]
                ///
                /// # Errors
                /// Returns `None` if there’s insufficient capacity.
                #[inline(always)]
                pub const fn [<push_ $T>](&mut self, val: $T) -> Option<$Handle> {
                    self.push_bytes(&val.to_le_bytes())
                }
                #[doc = "Reads a `" $T "` in little-endian order from the given `handle`."]
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                pub const fn [<read_ $T>](&self, handle: $Handle) -> Option<$T> {
                    const T_SIZE: usize = size_of::<$T>();
                    if let Some(bytes) = self.read_bytes(handle) {
                        Some($T::from_le_bytes(
                            *$crate::unwrap![some? bytes.first_chunk::<{T_SIZE}>()]))
                    } else { None }
                }
                #[doc = "Replaces a `" $T "` from the given `handle`. Returns `true` on success."]
                pub const fn [<replace_ $T>](&mut self, handle: $Handle, val: $T) -> bool {
                    if let Some(b) = self.read_bytes_mut(handle) {
                        if let Some(arr) = b.first_chunk_mut::<{size_of::<$T>()}>() {
                            *arr = val.to_le_bytes();
                            return true;
                        }
                    }
                    false
                }
            }};
            (str_len: $_d($T:ty),+ $_d(,)?) => {
                $_d( impl_for_primitives!(%str_len: $T); )+
            };
            (%str_len: $T:ty) => { $crate::paste! {
                #[doc = "Pushes a `&str` with a prefixed len of up to [`" $T "::MAX`] bytes."]
                /// Returns its handle on success.
                ///
                /// # Errors
                /// Returns `None` if there’s insufficient capacity or the string is too long.
                pub const fn [<push_str_ $T>](&mut self, val: &str) -> Option<$Handle> {
                    let len = val.len();
                    if len <= <$T>::MAX as usize {
                        let prefix = $crate::unwrap![some? self.[<push_ $T>](len as $T)];
                        let data = $crate::unwrap![some? self.push_bytes(&val.as_bytes())];
                        Some($Handle::new(prefix.offset(), prefix.len() + data.len()))
                    } else { None }
                }

                #[doc = "Reads a `&str` with a prefixed len of up to [`" $T "::MAX`] bytes"]
                /// from the given `handle`.
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                pub const fn [<read_str_ $T>](&self, handle: $Handle) -> Option<&str> {
                    let len_size = size_of::<$T>();
                    let handle = $Handle::new(handle.offset() + len_size, handle.len() - len_size);
                    let s = $crate::unwrap![some? self.read_bytes(handle)];
                    if let Ok(s) = $crate::Str::from_utf8(s) { Some(s) } else { None }
                }

                #[doc = "Replaces a `&str` with a prefixed len of up to [`" $T "::MAX`] bytes"]
                /// from the given `handle`. Returns `true` on success.
                ///
                /// Both strings have to have the same byte length.
                pub const fn [<replace_str_ $T>](&mut self, handle: $Handle, val: &str) -> bool {
                    let len_size = size_of::<$T>();
                    let handle = $Handle::new(handle.offset() + len_size, handle.len() - len_size);
                    if handle.len() != val.len() { return false }
                    if let Some(dst) = self.read_bytes_mut(handle) {
                        dst.copy_from_slice(val.as_bytes());
                        return true;
                    }
                    false
                }
            }};
        }
        use impl_for_primitives;
    };
    (%mark $Arena:ident, $(#[$mark_attr:meta])* $vis:vis $Mark:ident) => {
        $(#[$mark_attr])*
        /// Append-only mark for snapshots and rollback in an arena.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $Mark(usize);
        #[allow(dead_code)]
        impl $Mark {
            const fn new(mark: usize) -> Self { $Mark(mark) }
        }
    };
    (%tests $Arena:ident, $mod:ident) => {
        #[allow(non_snake_case)]
        mod $mod {
            use super::$Arena;
            #[test]
            fn push_and_read_bytes() {
                let mut a = $Arena::<16>::new();
                let handle = a.push_bytes(&[1, 2, 3, 4]).unwrap();
                assert_eq!(handle.offset(), 0);
                assert_eq!(handle.len(), 4);
                assert_eq!(a.read_bytes(handle).unwrap(), &[1, 2, 3, 4]);
            }

            #[test]
            fn replace_and_mutate_bytes() {
                let mut a = $Arena::<8>::new();
                let h = a.push_bytes(&[9, 9]).unwrap();
                assert!(a.replace_bytes(h, &[7, 8]));
                assert_eq!(a.read_bytes(h).unwrap(), &[7, 8]);
                let dst = a.read_bytes_mut(h).unwrap();
                dst.copy_from_slice(&[5, 6]);
                assert_eq!(a.read_bytes(h).unwrap(), &[5, 6]);
            }

            #[test]
            fn push_and_read_primitives() {
                let mut a = $Arena::<32>::new();
                let h = a.push_u32(0x11223344).unwrap();
                assert_eq!(a.read_u32(h), Some(0x11223344));
                assert!(a.replace_u32(h, 0x55667788));
                assert_eq!(a.read_u32(h), Some(0x55667788));
            }

            #[test]
            fn push_and_read_str() {
                let mut a = $Arena::<32>::new();
                let h = a.push_str_u8("hi").unwrap();
                assert_eq!(a.read_str_u8(h), Some("hi"));
            }

            #[test]
            fn bool_and_char() {
                let mut a = $Arena::<16>::new();
                let hb = a.push_bool(true).unwrap();
                let hc = a.push_char('Z').unwrap();
                assert_eq!(a.read_bool(hb), Some(true));
                assert_eq!(a.read_char(hc), Some('Z'));
            }

            #[test]
            fn pop_and_truncate() {
                let mut a = $Arena::<8>::new();
                let h1 = a.push_bytes(&[1, 2]).unwrap();
                let h2 = a.push_bytes(&[3, 4]).unwrap();
                assert!(!a.truncate_last(h1));
                assert!(a.truncate_last(h2));
                assert_eq!(a.len(), h1.offset() + h1.len());
            }

            #[test]
            fn capacity_and_remaining() {
                let a = $Arena::<8>::new();
                assert_eq!(a.capacity(), 8);
                assert_eq!(a.remaining(), 8);
            }

            #[test]
            fn handle_bounds_checks() {
                let mut a = $Arena::<4>::new();
                assert!(a.push_bytes(&[1, 2, 3, 4]).is_some());
                assert!(a.push_byte(5).is_none()); // capacity overflow
            }

            #[test]
            fn eq_bytes_and_replace_str() {
                let mut a = $Arena::<32>::new();
                let h = a.push_str_u8("hi").unwrap();
                assert_eq!(a.read_str_u8(h), Some("hi"));
                assert!(a.replace_str_u8(h, "hi"));
                assert_eq!(a.read_str_u8(h), Some("hi"));

                let mut b = $Arena::<32>::new();
                let _ = b.push_str_u8("hi");
                assert!(a == b);
            }
        }
    };
}
#[doc(inline)]
pub use define_arena;
