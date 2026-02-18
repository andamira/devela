// devela_base_core::sys::mem::arena::arena::bytes
//
//! Defines [`define_arena`].
//

#[cfg(any(test, feature = "_docs_examples"))]
define_arena! {
    // [ offset: u8+crate::NonExtremeU8; ] // WIP TODO
    [ offset: u8+u8; ]

    #[doc = crate::_tags!(example allocation)]
    /// An example memory arena.
    ///
    /// Generated with [`define_arena!`].
    pub ArenaExample;
    #[doc = crate::_tags!(example allocation)]
    /// An example memory arena handle.
    ///
    /// Generated with [`define_arena!`].
    pub ArenaHandleExample;
    #[doc = crate::_tags!(example allocation)]
    /// An example memory arena mark.
    ///
    /// Generated with [`define_arena!`].
    pub ArenaMarkExample;
}

#[doc = crate::_tags!(construction allocation)]
/// A custom memory arena generator.
#[doc = crate::_doc_location!("sys/mem")]
///
/// # Features
/// Uses `unsafe_array` to leverage `MaybeUninit` and avoid initializing the full capacity.
/// And uses `unsafe_slice` for further performance gains.
///
/// # Examples
/// See: [`ArenaExample`], [`ArenaHandleExample`], [`ArenaMarkExample`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! define_arena {
    // point of entry
    (
     [ offset: $prim:ident+$T:ty; ]

     $(#[$arena_attr:meta])*
     $vis:vis $Arena:ident;
     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?
     $(#[$mark_attr:meta])*
     $mvis:vis $Mark:ident $(;)?
    ) => {
        $crate::define_handle! {
            [offset:$prim+$T;] $(#[$handle_attr])* $hvis $Handle
        }
        $crate::define_arena![%arena
            $(#[$arena_attr])* $vis $Arena<$T>,
            $Handle,
            $(#[$mark_attr])* $Mark,
            $crate::__Arena::<CAP>
        ];
    };
    // calls the necessary arms in order.
    (%arena
     $(#[$arena_attr:meta])*
     $vis:vis $Arena:ident<$T:ty>,
     $Handle:ident,
     $(#[$mark_attr:meta])*
     $Mark:ident,
     $_:ty
    ) => {
        $crate::define_arena![%main
            $(#[$arena_attr])* $vis $Arena<$T>, // the arena type
            $Handle, // the handle type
            $Mark, // the mark type
            $_ // the internal ops arena namespace
        ];
        $crate::define_arena![%prim $vis $Arena<$T>, $Handle, ($)];
        $crate::define_arena![%mark $Arena<$T>, $(#[$mark_attr])* $vis $Mark];
        $crate::paste! {
            #[cfg(test)]
            $crate::define_arena![%tests $Arena, [<test_ $Arena>]];
        }
    };
    // $Arena:   the name of the new generated arena.
    // $Handle:  the handle type used by the arena.
    // $_:       internal arena helper.
    (%main
     $(#[$arena_attr:meta])*
     $vis:vis $Arena:ident<$T:ty>,
     $Handle:ident,
     $Mark:ident,
     $_:ty
    ) => {
        $(#[$arena_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Arena<const CAP: usize> {
            data: [$crate::MaybeByte; CAP],
            len: $T,
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
        #[allow(dead_code, private_interfaces)]
        impl<const CAP: usize> $Arena<CAP> {

            /// Returns a new empty arena.
            #[inline(always)]
            $vis const fn new() -> Self {
                Self { data: <$_>::new_array(), len: 0 }
            }

            #[inline(always)]
            /// Returns the total capacity.
            $vis const fn capacity(&self) -> $T { CAP as $T }
            #[inline(always)]
            /// Return the occupied length.
            $vis const fn len(&self) -> $T { self.len }
            #[inline(always)]
            /// Whether the arena is empty.
            $vis const fn is_empty(&self) -> bool { self.len == 0 }
            #[inline(always)]
            /// Returns the remaining byte capacity.
            $vis const fn remaining(&self) -> $T { CAP as $T - self.len }
            /// Returns `true` if n bytes fit in the remaining capacity.
            #[inline(always)]
            $vis const fn can_write(&self, n: $T) -> bool { self.len + n <= CAP as $T }

            #[inline(always)]
            /// Compares two arenas for equality.
            $vis const fn eq(&self, other: &Self) -> bool {
                $crate::Slice::<u8>::eq(self.as_bytes(), other.as_bytes())
            }

            /* snapshot and rollback */

            #[inline(always)]
            /// Snapshot a position in the arena.
            $vis const fn mark(&self) -> $Mark { <$Mark>::new(self.len) }
            #[inline(always)]
            /// Rollback to a previous marked position.
            $vis const fn rollback(&mut self, m: $Mark) { self.len = m.0; }

            /* byte slices */

            #[inline(always)]
            /// Returns a byte slice over all the written data.
            $vis const fn as_bytes(&self) -> &[u8] {
                <$_>::slice_bytes(&self.data, 0, self.len as usize)
            }
            /// Returns an exclusive byte slice over all the written data.
            #[inline(always)]
            $vis const fn as_bytes_mut(&mut self) -> &mut [u8] {
                $crate::__Arena::<CAP>::slice_bytes_mut(&mut self.data, 0, self.len as usize)
            }

            /// Write a byte slice into the arena.
            $vis const fn push_bytes(&mut self, bytes: &[u8]) -> Option<$Handle> {
                $crate::unwrap!(some_if?
                    self.len.checked_add(bytes.len() as $T), |v| v <= CAP as $T);
                let start = self.len;
                let handle = <$Handle>::new(start as $T, bytes.len() as $T);
                $crate::whilst! { i in 0..bytes.len(); {
                    $crate::__Arena::<CAP>::write_byte(&mut self.data, self.len as usize, bytes[i]);
                    self.len += 1;
                }}
                Some(handle)
            }

            /// Read a shared slice over the written bytes.
            $vis const fn read_bytes(&self, h: $Handle) -> Option<&[u8]> {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                if h.len() + h.offset() > self.len { return None }
                Some($crate::__Arena::<CAP>::slice_bytes(&self.data, hoff, hlen + hoff))
            }
            /// Read an exclusive slice over the written bytes.
            $vis const fn read_bytes_mut(&mut self, h: $Handle) -> Option<&mut [u8]> {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                if h.len() + h.offset() > self.len { return None }
                Some($crate::__Arena::<CAP>::slice_bytes_mut(&mut self.data, hoff, hlen + hoff))
            }

            /// Replace the bytes for the handle. Lengths must match. Returns `false` otherwise.
            $vis const fn replace_bytes(&mut self, h: $Handle, new: &[u8]) -> bool {
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
            $vis const fn push_byte(&mut self, byte: u8) -> Option<$Handle> {
                if self.len as usize + 1 > CAP { return None; }
                <$_>::write_byte(&mut self.data, self.len as usize, byte);
                let handle = <$Handle>::new(self.len as $T, 1);
                self.len += 1;
                Some(handle)
            }
            /// Read a byte previously written.
            $vis const fn read_byte(&self, h: $Handle) -> Option<u8> {
                if h.offset() + 1 > self.len { return None }
                Some(<$_>::read_byte(&self.data, h.offset() as usize))
            }
            /// Read a byte previously written.
            $vis const fn read_byte_mut(&mut self, h: $Handle) -> Option<&mut u8> {
                if h.offset() + 1 > self.len { return None }
                Some(<$_>::read_byte_mut(&mut self.data, h.offset() as usize))
            }
            /// Replace the bytes for `handle`. Length must match.
            $vis const fn replace_byte(&mut self, handle: $Handle, new: u8) -> bool {
                if handle.len() != 1 { return false; }
                <$_>::write_byte(&mut self.data, handle.offset() as usize, new);
                true
            }

            /* views over multiple values */

            /// Returns a shared slice starting at `handle`,
            /// and spanning `count` items of its length.
            ///
            /// Returns `None` if...
            $vis const fn view_bytes(&self, h: $Handle, count: $T) -> Option<&[u8]> {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                let total = hlen * count as usize;
                if hoff + total > self.len as usize { return None; }
                Some(<$_>::slice_bytes(&self.data, hoff, hoff + total))
            }

            /// Returns an exclusive slice starting at `handle`,
            /// and spanning `count` items of its length.
            ///
            /// Returns `None` if...
            $vis const fn view_bytes_mut(&mut self, h: $Handle, count: $T) -> Option<&mut [u8]> {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                let total = hlen * count as usize;
                if hoff + total > self.len as usize { return None; }
                Some(<$_>::slice_bytes_mut(&mut self.data, hoff, hoff + total))
            }

            /* shrinking the arena */

            /// Truncates the arena if the handle corresponds to the last region.
            $vis const fn truncate_last(&mut self, h: $Handle) -> bool {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                if hoff + hlen != self.len as usize { return false; }
                self.len = h.offset();
                true
            }

            /// Copies the last stored value into `dst` and removes it.
            $vis const fn pop_into(&mut self, h: $Handle, dst: &mut [u8]) -> bool {
                let hlen = h.len() as usize;
                if hlen != dst.len() { return false; }
                if let Some(src) = self.read_bytes(h) {
                    $crate::whilst! { i in 0..hlen; { dst[i] = src[i]; }}
                    self.truncate_last(h)
                } else { false }
            }
        }
    };
    // $_d: the dollar sign passed as a token, as a trick to be able to nest repetitions.
    (%prim $vis:vis $Arena:ident<$T:ty>, $Handle:ty, ($_d:tt)) => {
        #[allow(dead_code, private_interfaces)]
        /// Implements push, read and replace for primitives.
        impl<const CAP: usize> $Arena<CAP> {
            impl_for_primitives!();

            /* bool */

            /// Pushes a `char`. Returns its handle on success.
            ///
            /// # Errors
            /// Returns `None` if there's insufficient capacity.
            $vis const fn push_bool(&mut self, val: bool) -> Option<$Handle> {
                self.push_byte(val as u8)
            }
            /// Reads a `bool` from the given `handle`.
            ///
            /// # Errors
            /// Returns `None` if the handle is invalid or incomplete.
            $vis const fn read_bool(&self, handle: $Handle) -> Option<bool> {
                if let Some(b) = self.read_byte(handle) { Some(b != 0) } else { None }
            }
            /// Replaces a `bool` from the given `handle`. Returns `true` on success.
            $vis const fn replace_bool(&mut self, handle: $Handle, val: bool) -> bool {
                self.replace_byte(handle, val as u8)
            }

            /* char */

            /// Pushes a `char`. Returns its handle on success.
            ///
            /// # Errors
            /// Returns `None` if there's insufficient capacity.
            #[inline(always)]
            $vis const fn push_char(&mut self, val: char) -> Option<$Handle> {
                self.push_u32(val as u32)
            }
            /// Reads a `char` from the given `handle`.
            ///
            /// # Errors
            /// Returns `None` if the handle is invalid or incomplete.
            #[inline(always)]
            $vis const fn read_char(&self, handle: $Handle) -> Option<char> {
                if let Some(c) = self.read_u32(handle) { char::from_u32(c) } else { None }
            }
            /// Replaces a `char` from the given `handle`. Returns `true` on success.
            #[inline(always)]
            $vis const fn replace_char(&mut self, handle: $Handle, val: char) -> bool {
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
            (single-byte: $_d($P:ty),+ $_d(,)?) => {
                $_d( impl_for_primitives!(%single-byte: $P); )+
            };
            (%single-byte: $P:ty) => { $crate::paste! {
                #[doc = "Pushes a `" $P "`. Returns its handle on success."]
                ///
                /// # Errors
                /// Returns `None` if there's insufficient capacity.
                #[inline(always)]
                $vis const fn [<push_ $P>](&mut self, val: $P) -> Option<$Handle> {
                    self.push_byte(val as u8)
                }
                #[doc = "Reads a `" $P "` from the given `handle`."]
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                $vis const fn [<read_ $P>](&self, handle: $Handle) -> Option<$P> {
                    if let Some(b) = self.read_byte(handle) { Some(b as $P) } else { None }
                }
                #[doc = "Replaces a `" $P "` from the given `handle`. Returns `true` on success."]
                $vis const fn [<replace_ $P>](&mut self, handle: $Handle, val: $P) -> bool {
                    self.replace_byte(handle, val as u8)
                }
            }};
            (multi-byte: $_d($P:ty),+ $_d(,)?) => {
                $_d( impl_for_primitives!(%multi-byte: $P); )+
            };
            (%multi-byte: $P:ty) => { $crate::paste! {
                #[doc = "Pushes a `" $P "` in little-endian order. Returns its handle on success."]
                ///
                /// # Errors
                /// Returns `None` if there's insufficient capacity.
                #[inline(always)]
                $vis const fn [<push_ $P>](&mut self, val: $P) -> Option<$Handle> {
                    self.push_bytes(&val.to_le_bytes())
                }
                #[doc = "Reads a `" $P "` in little-endian order from the given `handle`."]
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                $vis const fn [<read_ $P>](&self, handle: $Handle) -> Option<$P> {
                    const T_SIZE: usize = size_of::<$P>();
                    if let Some(bytes) = self.read_bytes(handle) {
                        Some($P::from_le_bytes(
                            *$crate::unwrap![some? bytes.first_chunk::<{T_SIZE}>()]))
                    } else { None }
                }
                #[doc = "Replaces a `" $P "` from the given `handle`. Returns `true` on success."]
                $vis const fn [<replace_ $P>](&mut self, handle: $Handle, val: $P) -> bool {
                    if let Some(b) = self.read_bytes_mut(handle) {
                        if let Some(arr) = b.first_chunk_mut::<{size_of::<$P>()}>() {
                            *arr = val.to_le_bytes();
                            return true;
                        }
                    }
                    false
                }
            }};
            (str_len: $_d($P:ty),+ $_d(,)?) => {
                $_d( impl_for_primitives!(%str_len: $P); )+
            };
            (%str_len: $P:ty) => { $crate::paste! {
                #[doc = "Pushes a `&str` with a prefixed len of up to [`" $P "::MAX`] bytes."]
                /// Returns its handle on success.
                ///
                /// # Errors
                /// Returns `None` if there's insufficient capacity or the string is too long.
                $vis const fn [<push_str_ $P>](&mut self, val: &str) -> Option<$Handle> {
                    let len = val.len();
                    if len <= <$P>::MAX as usize {
                        let prefix = $crate::unwrap![some? self.[<push_ $P>](len as $P)];
                        let data = $crate::unwrap![some? self.push_bytes(&val.as_bytes())];
                        Some($Handle::new(prefix.offset(), prefix.len() + data.len()))
                    } else { None }
                }

                #[doc = "Reads a `&str` with a prefixed len of up to [`" $P "::MAX`] bytes"]
                /// from the given handle.
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                $vis const fn [<read_str_ $P>](&self, h: $Handle) -> Option<&str> {
                    let len_size = size_of::<$P>() as $T;
                    // $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                    let h = $Handle::new(h.offset() + len_size, h.len() - len_size);
                    let s = $crate::unwrap![some? self.read_bytes(h)];
                    if let Ok(s) = $crate::Str::from_utf8(s) { Some(s) } else { None }
                }

                #[doc = "Replaces a `&str` with a prefixed len of up to [`" $P "::MAX`] bytes"]
                /// from the given handle. Returns `true` on success.
                ///
                /// Both strings have to have the same byte length.
                $vis const fn [<replace_str_ $P>](&mut self, h: $Handle, val: &str) -> bool {
                    let len_size = size_of::<$P>() as $T;
                    let h = $Handle::new(h.offset() + len_size, h.len() - len_size);
                    if h.len() as usize != val.len() { return false }
                    if let Some(dst) = self.read_bytes_mut(h) {
                        dst.copy_from_slice(val.as_bytes());
                        return true;
                    }
                    false
                }
            }};
        }
        use impl_for_primitives;
    };
    (%mark $Arena:ident<$T:ty>, $(#[$mark_attr:meta])* $vis:vis $Mark:ident) => {
        $(#[$mark_attr])*
        // Append-only mark for snapshots and rollback in an arena.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $Mark($T);
        #[allow(dead_code)]
        impl $Mark {
            // Constructor always private.
            const fn new(mark: $T) -> Self { $Mark(mark) }
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
