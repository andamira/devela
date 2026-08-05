// devela/src/data/store/arena/byte/define.rs
//
//! Defines [`arena_bytes!`].
//

#[cfg(any(test, feature = "_docs_examples"))]
arena_bytes! {
    [
        offset: u8;
    ]

    #[doc = crate::_tags!(example data_structure)]
    /// An example memory arena.
    ///
    /// Generated with [`arena_bytes!`].
    pub ArenaBytesExample;

    #[doc = crate::_tags!(example uid)]
    /// An example handle into [`ArenaBytesExample`].
    ///
    /// Generated with [`arena_bytes!`] and [`handle_span!`][crate::handle_span].
    pub ArenaBytesHandleExample;

    #[doc = crate::_tags!(example state)]
    /// An example memory arena mark.
    ///
    /// Generated with [`arena_bytes!`].
    pub ArenaBytesMarkExample;
}

#[doc = crate::_tags!(construction data_structure)]
/// A custom byte store arena generator.
#[doc = crate::_doc_meta!{location("data/store")}]
///
/// # Features
/// Uses `unsafe_array` to leverage `MaybeUninit` and avoid initializing the full capacity.
/// And uses `unsafe_slice` for further performance gains.
///
/// # Examples
/// See: [`ArenaBytesExample`], [`ArenaBytesHandleExample`], [`ArenaBytesMarkExample`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! arena_bytes {
    (
     [
      offset: $oprim:ident;
     ]

     $(#[$arena_attr:meta])*
     $vis:vis $Arena:ident;

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident $(;)?

     $(#[$mark_attr:meta])*
     $mvis:vis $Mark:ident $(;)?
    ) => {
        $crate::arena_bytes![
            [
                offset: $oprim + $oprim;
            ]
            $(#[$arena_attr])* $vis $Arena;
            $(#[$handle_attr])* $hvis $Handle;
            $(#[$mark_attr])* $mvis $Mark;
        ];
    };
    (

     [
      offset: $oprim:ident + $Offset:ty;
     ]

     $(#[$arena_attr:meta])*
     $vis:vis $Arena:ident;

     $(#[$handle_attr:meta])*
     $hvis:vis $Handle:ident;

     $(#[$mark_attr:meta])*
     $mvis:vis $Mark:ident $(;)?

    ) => {
        /* handle */

        $crate::handle_span! {
            [
                offset:$oprim + $Offset;
            ]
            $(#[$handle_attr])* $hvis $Handle
        }

        /* mark */

        $(#[$mark_attr])*
        // Append-only mark for snapshots and rollback in an arena.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $mvis struct $Mark($Offset);
        #[allow(dead_code)]
        impl $Mark {
            // Constructor always private.
            const fn new(mark: $Offset) -> Self { $Mark(mark) }
        }

        /* arena */

        $crate::paste! {
            $crate::arena_bytes![%define
                $(#[$arena_attr])* $vis $Arena<$Offset>; // the arena type
                $hvis $Handle; // the handle name
                $mvis $Mark; // the mark name
                [<_test_ $Arena>]; // the test module name
                $crate::__ArenaBytes::<CAP>; // the internal ops arena namespace
                ($); // the dollar sign passed as a token
            ];
        }
    };
    (%define
     $(#[$arena_attr:meta])*
     $vis:vis $Arena:ident<$Offset:ty>;
     $hvis:vis $Handle:ident;
     $mvis:vis $Mark:ident;
     $test_mod:ident;
     $_:ty;
     ($_d:tt);
    ) => {
        $(#[$arena_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Arena<const CAP: usize> {
            data: [$crate::MaybeByte; CAP],
            len: $Offset,
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
            $vis const fn new() -> Self {
                Self { data: <$_>::new_array(), len: 0 }
            }

            /// Returns the total capacity.
            $vis const fn capacity(&self) -> $Offset { CAP as $Offset }
            /// Return the occupied length.
            $vis const fn len(&self) -> $Offset { self.len }
            /// Whether the arena is empty.
            $vis const fn is_empty(&self) -> bool { self.len == 0 }
            /// Returns the remaining byte capacity.
            $vis const fn remaining(&self) -> $Offset { CAP as $Offset - self.len }
            /// Returns `true` if n bytes fit in the remaining capacity.
            $vis const fn can_write(&self, n: $Offset) -> bool { self.len + n <= CAP as $Offset }

            /// Compares two arenas for equality.
            $vis const fn eq(&self, other: &Self) -> bool {
                $crate::Slice::<u8>::eq(self.as_bytes(), other.as_bytes())
            }

            /* snapshot and rollback */

            /// Snapshot a position in the arena.
            $mvis const fn mark(&self) -> $Mark { <$Mark>::new(self.len) }
            /// Rolls back to `mark`, returning whether it was valid.
            $mvis const fn rollback(&mut self, mark: $Mark) -> bool {
                if mark.0 <= self.len { self.len = mark.0; true } else { false }
            }

            /* byte slices */

            /// Returns a byte slice over all the written data.
            $vis const fn as_bytes(&self) -> &[u8] {
                <$_>::slice_bytes(&self.data, 0, self.len as usize)
            }
            /// Returns an exclusive byte slice over all the written data.
            $vis const fn as_bytes_mut(&mut self) -> &mut [u8] {
                $crate::__ArenaBytes::<CAP>::slice_bytes_mut(&mut self.data, 0, self.len as usize)
            }

            /// Write a byte slice into the arena.
            $hvis const fn push_bytes(&mut self, bytes: &[u8]) -> Option<$Handle> {
                $crate::unwrap!(some_if?
                    self.len.checked_add(bytes.len() as $Offset), |v| v <= CAP as $Offset);
                let start = self.len;
                let handle = <$Handle>::new(start as $Offset, bytes.len() as $Offset);
                $crate::whilst! { i in 0..bytes.len(); {
                    $crate::__ArenaBytes::<CAP>::write_byte(&mut self.data,
                        self.len as usize, bytes[i]);
                    self.len += 1;
                }}
                Some(handle)
            }

            /// Read a shared slice over the written bytes.
            $hvis const fn read_bytes(&self, h: $Handle) -> Option<&[u8]> {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                if h.len() + h.offset() > self.len { return None }
                Some($crate::__ArenaBytes::<CAP>::slice_bytes(&self.data, hoff, hlen + hoff))
            }
            /// Read an exclusive slice over the written bytes.
            $hvis const fn read_bytes_mut(&mut self, h: $Handle) -> Option<&mut [u8]> {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                if h.len() + h.offset() > self.len { return None }
                Some($crate::__ArenaBytes::<CAP>::slice_bytes_mut(&mut self.data,
                    hoff, hlen + hoff))
            }

            /// Replace the bytes for the handle. Lengths must match. Returns `false` otherwise.
            $hvis const fn replace_bytes(&mut self, h: $Handle, new: &[u8]) -> bool {
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
            $hvis const fn push_byte(&mut self, byte: u8) -> Option<$Handle> {
                if self.len as usize + 1 > CAP { return None; }
                <$_>::write_byte(&mut self.data, self.len as usize, byte);
                let handle = <$Handle>::new(self.len as $Offset, 1);
                self.len += 1;
                Some(handle)
            }
            /// Read a byte previously written.
            $hvis const fn read_byte(&self, h: $Handle) -> Option<u8> {
                if h.offset() + 1 > self.len { return None }
                Some(<$_>::read_byte(&self.data, h.offset() as usize))
            }
            /// Read a byte previously written.
            $hvis const fn read_byte_mut(&mut self, h: $Handle) -> Option<&mut u8> {
                if h.offset() + 1 > self.len { return None }
                Some(<$_>::read_byte_mut(&mut self.data, h.offset() as usize))
            }
            /// Replace the bytes for `handle`. Length must match.
            $hvis const fn replace_byte(&mut self, handle: $Handle, new: u8) -> bool {
                if handle.len() != 1 { return false; }
                <$_>::write_byte(&mut self.data, handle.offset() as usize, new);
                true
            }

            /* views over multiple values */

            /// Returns a shared slice starting at `handle`,
            /// and spanning `count` items of its length.
            ///
            /// Returns `None` if...
            $hvis const fn view_bytes(&self, h: $Handle, count: $Offset) -> Option<&[u8]> {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                let total = hlen * count as usize;
                if hoff + total > self.len as usize { return None; }
                Some(<$_>::slice_bytes(&self.data, hoff, hoff + total))
            }

            /// Returns an exclusive slice starting at `handle`,
            /// and spanning `count` items of its length.
            ///
            /// Returns `None` if...
            $hvis const fn view_bytes_mut(&mut self, h: $Handle, count: $Offset)
                -> Option<&mut [u8]> {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                let total = hlen * count as usize;
                if hoff + total > self.len as usize { return None; }
                Some(<$_>::slice_bytes_mut(&mut self.data, hoff, hoff + total))
            }

            /* shrinking the arena */

            /// Truncates the arena if the handle corresponds to the last region.
            $hvis const fn truncate_last(&mut self, h: $Handle) -> bool {
                $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                if hoff + hlen != self.len as usize { return false; }
                self.len = h.offset();
                true
            }

            /// Copies the last stored value into `dst` and removes it.
            $hvis const fn pop_into(&mut self, h: $Handle, dst: &mut [u8]) -> bool {
                let hlen = h.len() as usize;
                if hlen != dst.len() { return false; }
                if let Some(src) = self.read_bytes(h) {
                    $crate::whilst! { i in 0..hlen; { dst[i] = src[i]; }}
                    self.truncate_last(h)
                } else { false }
            }
        }

        /* primitives */

        #[allow(dead_code, private_interfaces)]
        /// Implements push, read and replace for primitives.
        impl<const CAP: usize> $Arena<CAP> {
            _impl_arena_methods_for_prims!();

            /* bool */

            /// Pushes a `char`. Returns its handle on success.
            ///
            /// # Errors
            /// Returns `None` if there's insufficient capacity.
            $hvis const fn push_bool(&mut self, val: bool) -> Option<$Handle> {
                self.push_byte(val as u8)
            }
            /// Reads a `bool` from the given `handle`.
            ///
            /// # Errors
            /// Returns `None` if the handle is invalid or incomplete.
            $hvis const fn read_bool(&self, handle: $Handle) -> Option<bool> {
                if let Some(b) = self.read_byte(handle) { Some(b != 0) } else { None }
            }
            /// Replaces a `bool` from the given `handle`. Returns `true` on success.
            $hvis const fn replace_bool(&mut self, handle: $Handle, val: bool) -> bool {
                self.replace_byte(handle, val as u8)
            }

            /* char */

            /// Pushes a `char`. Returns its handle on success.
            ///
            /// # Errors
            /// Returns `None` if there's insufficient capacity.
            $hvis const fn push_char(&mut self, val: char) -> Option<$Handle> {
                self.push_u32(val as u32)
            }
            /// Reads a `char` from the given `handle`.
            ///
            /// # Errors
            /// Returns `None` if the handle is invalid or incomplete.
            $hvis const fn read_char(&self, handle: $Handle) -> Option<char> {
                if let Some(c) = self.read_u32(handle) { char::from_u32(c) } else { None }
            }
            /// Replaces a `char` from the given `handle`. Returns `true` on success.
            $hvis const fn replace_char(&mut self, handle: $Handle, val: char) -> bool {
                self.replace_u32(handle, val as u32)
            }
        }

        /// Private helper to implement push, read & replace methods over primitives.
        #[rustfmt::skip] // fixes rustfmt warnings
        macro_rules! _impl_arena_methods_for_prims {
            () => {
                _impl_arena_methods_for_prims!(single-byte: u8, i8);
                _impl_arena_methods_for_prims!(multi-byte:
                    u16, u32, u64, u128, usize,
                    i16, i32, i64, i128, isize,
                    f32, f64,
                );
                _impl_arena_methods_for_prims!(str_len: u8, u16, u32, usize);
            };
            (single-byte: $_d($oprim:ty),+ $_d(,)?) => {
                $_d( _impl_arena_methods_for_prims!(%single-byte: $oprim); )+
            };
            (%single-byte: $oprim:ty) => { $crate::paste! {
                #[doc = "Pushes a `" $oprim "`. Returns its handle on success."]
                ///
                /// # Errors
                /// Returns `None` if there's insufficient capacity.
                $hvis const fn [<push_ $oprim>](&mut self, val: $oprim) -> Option<$Handle> {
                    self.push_byte(val as u8)
                }
                #[doc = "Reads a `" $oprim "` from the given `handle`."]
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                $hvis const fn [<read_ $oprim>](&self, handle: $Handle) -> Option<$oprim> {
                    if let Some(b) = self.read_byte(handle) { Some(b as $oprim) } else { None }
                }
                #[doc = "Replaces a `" $oprim "`
                from the given `handle`. Returns `true` on success."]
                $hvis const fn [<replace_ $oprim>](&mut self, handle: $Handle, val: $oprim)
                -> bool {
                    self.replace_byte(handle, val as u8)
                }
            }};
            (multi-byte: $_d($oprim:ty),+ $_d(,)?) => {
                $_d( _impl_arena_methods_for_prims!(%multi-byte: $oprim); )+
            };
            (%multi-byte: $oprim:ty) => { $crate::paste! {
                #[doc = "Pushes a `" $oprim
                "` in little-endian order. Returns its handle on success."]
                ///
                /// # Errors
                /// Returns `None` if there's insufficient capacity.
                $hvis const fn [<push_ $oprim>](&mut self, val: $oprim) -> Option<$Handle> {
                    self.push_bytes(&val.to_le_bytes())
                }
                #[doc = "Reads a `" $oprim "` in little-endian order from the given `handle`."]
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                $hvis const fn [<read_ $oprim>](&self, handle: $Handle) -> Option<$oprim> {
                    const T_SIZE: usize = size_of::<$oprim>();
                    if let Some(bytes) = self.read_bytes(handle) {
                        Some($oprim::from_le_bytes(
                            *$crate::unwrap![some? bytes.first_chunk::<{T_SIZE}>()]))
                    } else { None }
                }
                #[doc = "Replaces a `" $oprim
                "` from the given `handle`. Returns `true` on success."]
                $hvis const fn [<replace_ $oprim>](&mut self, handle: $Handle, val: $oprim)
                    -> bool {
                    if let Some(b) = self.read_bytes_mut(handle) {
                        if let Some(arr) = b.first_chunk_mut::<{size_of::<$oprim>()}>() {
                            *arr = val.to_le_bytes();
                            return true;
                        }
                    }
                    false
                }
            }};
            (str_len: $_d($oprim:ty),+ $_d(,)?) => {
                $_d( _impl_arena_methods_for_prims!(%str_len: $oprim); )+
            };
            (%str_len: $oprim:ty) => { $crate::paste! {
                #[doc = "Pushes a `&str` with a prefixed len of up to [`" $oprim "::MAX`] bytes."]
                /// Returns its handle on success.
                ///
                /// # Errors
                /// Returns `None` if there's insufficient capacity or the string is too long.
                $hvis const fn [<push_str_ $oprim>](&mut self, val: &str) -> Option<$Handle> {
                    let len = val.len();
                    if len <= <$oprim>::MAX as usize {
                        let prefix = $crate::unwrap![some? self.[<push_ $oprim>](len as $oprim)];
                        let data = $crate::unwrap![some? self.push_bytes(&val.as_bytes())];
                        Some($Handle::new(prefix.offset(), prefix.len() + data.len()))
                    } else { None }
                }

                #[doc = "Reads a `&str` with a prefixed len of up to [`" $oprim "::MAX`] bytes"]
                /// from the given handle.
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                $hvis const fn [<read_str_ $oprim>](&self, h: $Handle) -> Option<&str> {
                    let len_size = size_of::<$oprim>() as $Offset;
                    // $crate::lets![hlen=h.len() as usize, hoff=h.offset() as usize];
                    let h = $Handle::new(h.offset() + len_size, h.len() - len_size);
                    let s = $crate::unwrap![some? self.read_bytes(h)];
                    if let Ok(s) = $crate::Str::from_utf8(s) { Some(s) } else { None }
                }

                #[doc = "Replaces a `&str` with a prefixed len of up to [`" $oprim "::MAX`] bytes"]
                /// from the given handle. Returns `true` on success.
                ///
                /// Both strings have to have the same byte length.
                $hvis const fn [<replace_str_ $oprim>](&mut self, h: $Handle, val: &str) -> bool {
                    let len_size = size_of::<$oprim>() as $Offset;
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
        use _impl_arena_methods_for_prims;

        /* tests */

        #[cfg(test)]
        #[allow(non_snake_case)]
        mod $test_mod {
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
pub use arena_bytes;
