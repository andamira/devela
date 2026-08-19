// devela/src/data/store/arena/byte/define.rs
//
//! Defines [`arena_bytes!`].
//

#[doc = crate::_tags!(construction data_structure)]
/// Defines a fixed-capacity byte arena with compact span handles.
#[doc = crate::_doc_meta!{location("data/store")}]
///
/// The generated arena stores bytes in an append-only initialized prefix.
///
/// Arena positions use the primitive part of the configured `cursor`.
/// The arena's current length and rollback marks use that primitive directly,
/// while generated handles may use a different representation for their
/// byte offsets and lengths.
///
/// Handles describe coordinates only; they do not identify a particular arena.
/// The receiving arena validates that their spans lie within its written prefix.
///
/// # Configuration
/// `cursor` selects the primitive byte-coordinate type and, optionally,
/// the representation used by generated handle fields.
///
/// - `cursor: u16;` uses `u16` for both.
/// - `cursor: u16 + NonMaxU16;` keeps arena cursor state as `u16`
///   while storing handle coordinates as `NonMaxU16`.
///
/// # Features
/// Uses `unsafe_array` to avoid initializing the full byte capacity,
/// and `unsafe_slice` for additional slice-access optimizations.
///
/// # Example
/// ```
/// # use devela::{NonMaxU16, arena_bytes};
/// arena_bytes! {
///     [cursor: u16 + NonMaxU16;]
///
///     /// A byte arena.
///     pub Bytes;
///     /// A byte span within `Bytes`.
///     pub BytesHandle;
///     /// A rollback position within `Bytes`.
///     pub BytesMark;
/// }
///
/// let mut arena = Bytes::<64>::new();
/// let handle = arena.push_bytes(b"devela").unwrap();
/// assert_eq!(arena.read_bytes(handle), Some(&b"devela"[..]));
/// ```
/// See:
/// [`ArenaBytesExample`],
/// [`ArenaBytesHandleExample`],
/// [`ArenaBytesMarkExample`].
///
/// [`ArenaBytesExample`]: crate::ArenaBytesExample
/// [`ArenaBytesHandleExample`]: crate::ArenaBytesHandleExample
/// [`ArenaBytesMarkExample`]: crate::ArenaBytesMarkExample
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! arena_bytes {
    (
        [cursor: $cprim:ident $(+ $Cursor:ty)?;]

        $(#[$arena_attr:meta])*
        $vis:vis $Arena:ident;

        $(#[$handle_attr:meta])*
        $hvis:vis $Handle:ident;

        $(#[$mark_attr:meta])*
        $mvis:vis $Mark:ident $(;)?
    ) => {
        $crate::arena_bytes! { %normalize_cursor
            [cursor: $cprim $(+ $Cursor)?]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $(#[$handle_attr])* $hvis $Handle]
            [mark: $(#[$mark_attr])* $mvis $Mark]
        }
    };
    (%normalize_cursor
        [cursor: $cprim:ident]
        $($rest:tt)*
    ) => {
        $crate::arena_bytes! { %generate [cursor: $cprim + $cprim] $($rest)* }
    };
    (%normalize_cursor
        [cursor: $cprim:ident + $Cursor:ty]
        $($rest:tt)*
    ) => {
        $crate::arena_bytes! { %generate [cursor: $cprim + $Cursor] $($rest)* }
    };
    (%generate
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $(#[$handle_attr:meta])* $hvis:vis $Handle:ident]
        [mark: $(#[$mark_attr:meta])* $mvis:vis $Mark:ident]
    ) => {
        $crate::handle_span! {
            [offset: $cprim + $Cursor;]
            $(#[$handle_attr])*
            $hvis $Handle;
        }

        $(#[$mark_attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $mvis struct $Mark($cprim);

        #[allow(dead_code)]
        impl $Mark {
            const fn new(cursor: $cprim) -> Self {
                Self(cursor)
            }
        }

        $crate::arena_bytes! { %define
            [cursor: $cprim]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $mvis $Mark]
            [internal: $crate::__ArenaBytes::<CAP>]
            ($)
        }
    };
    (%define
        [cursor: $cprim:ident]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $mvis:vis $Mark:ident]
        [internal: $_:ty]
        ($_d:tt)
    ) => {
        $(#[$arena_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Arena<const CAP: usize> {
            data: [$crate::MaybeByte; CAP],

            /// End of the initialized byte prefix.
            len: $cprim,
        }

        impl<const CAP: usize> Eq for $Arena<CAP> {}
        impl<const CAP: usize> PartialEq for $Arena<CAP> {
            fn eq(&self, other: &Self) -> bool { $Arena::eq(self, other) }
        }
        impl<const CAP: usize> Default for $Arena<CAP> {
            fn default() -> Self { Self::new() }
        }

        #[allow(dead_code, private_interfaces)]
        impl<const CAP: usize> $Arena<CAP> {
            /* private helpers */

            /// Ensures every arena position can also be represented by its handle.
            ///
            /// `CAP` itself must be representable because a full arena has
            /// `len == CAP`, and a span may cover the entire arena.
            const _GUARD_CAPACITY: () = assert!(
                <$Handle>::try_from_usize(CAP, CAP).is_ok(),
                "arena_bytes! capacity exceeds its cursor or handle representation",
            );

            /// Returns the written byte length in the machine indexing domain.
            const fn _len_usize(&self) -> usize { self.len as usize }

            /// Resolves a handle into a validated half-open byte range.
            const fn _span_usize(&self, h: $Handle) -> Option<(usize, usize)> {
                let start = $crate::unwrap![ok_some? h.offset_usize()];
                let len = $crate::unwrap![ok_some? h.len_usize()];
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

            /* constructor */

            /// Returns a new empty arena.
            $vis const fn new() -> Self {
                let () = Self::_GUARD_CAPACITY;
                Self { data: <$_>::new_array(), len: 0 }
            }

            /* capacity */

            /// Returns the total byte capacity.
            $vis const fn capacity(&self) -> $cprim { CAP as $cprim }
            /// Returns the occupied byte length.
            $vis const fn len(&self) -> $cprim { self.len }
            /// Returns whether the arena contains no bytes.
            $vis const fn is_empty(&self) -> bool { self.len == 0 }
            /// Returns whether no additional byte can be written.
            $vis const fn is_full(&self) -> bool { self.len == CAP as $cprim }
            /// Returns the remaining byte capacity.
            $vis const fn remaining(&self) -> $cprim { CAP as $cprim - self.len }
            /// Returns whether `n` additional bytes fit in the arena.
            $vis const fn can_write(&self, n: $cprim) -> bool {
                let end = $crate::unwrap![some_or? self.len.checked_add(n), false];
                end <= CAP as $cprim
            }

            /* misc. */

            /// Removes all written bytes.
            $vis const fn clear(&mut self) { self.len = 0; }

            /// Compares two arenas for equality.
            $vis const fn eq(&self, other: &Self) -> bool {
                $crate::Slice::<u8>::eq(self.as_bytes(), other.as_bytes())
            }

            /* snapshot and rollback */

            /// Creates a rollback mark at the current byte length.
            $mvis const fn mark(&self) -> $Mark { <$Mark>::new(self.len) }
            /// Rolls back to `mark`, returning whether the mark was valid.
            $mvis const fn rollback(&mut self, mark: $Mark) -> bool {
                if mark.0 <= self.len { self.len = mark.0; true } else { false }
            }

            /* byte slices */

            /// Returns a byte slice over all the written data.
            $vis const fn as_bytes(&self) -> &[u8] {
                <$_>::slice_bytes(&self.data, 0, self._len_usize())
            }
            /// Returns an exclusive byte slice over all the written data.
            $vis const fn as_bytes_mut(&mut self) -> &mut [u8] {
                let len = self._len_usize();
                <$_>::slice_bytes_mut(&mut self.data, 0, len)
            }

            /// Writes a byte slice into the arena.
            $hvis const fn push_bytes(&mut self, bytes: &[u8]) -> Option<$Handle> {
                let start = self._len_usize();
                let end = $crate::unwrap![some? start.checked_add(bytes.len())];
                if end > CAP { return None; }
                let handle =
                    $crate::unwrap![ok_some? <$Handle>::try_from_usize(start, bytes.len())];
                $crate::whilst! { i in 0..bytes.len(); {
                    <$_>::write_byte(&mut self.data, start + i, bytes[i]);
                }}
                // `_GUARD_CAPACITY` proves every value through CAP fits `$cprim`.
                self.len = end as $cprim;
                Some(handle)
            }

            /// Returns the bytes described by `handle`.
            $hvis const fn read_bytes(&self, handle: $Handle) -> Option<&[u8]> {
                let (start, end) = $crate::unwrap![some? self._span_usize(handle)];
                Some(<$_>::slice_bytes(&self.data, start, end))
            }
            /// Returns the bytes described by `handle` exclusively.
            $hvis const fn read_bytes_mut(&mut self, handle: $Handle) -> Option<&mut [u8]> {
                let (start, end) = $crate::unwrap![some? self._span_usize(handle)];
                Some(<$_>::slice_bytes_mut(&mut self.data, start, end))
            }
            /// Replaces the bytes described by `handle`.
            $hvis const fn replace_bytes(&mut self, handle: $Handle, new: &[u8]) -> bool {
                let dst = $crate::unwrap![some_or? self.read_bytes_mut(handle), false];
                if dst.len() != new.len() { return false; }
                dst.copy_from_slice(new);
                true
            }

            /* single bytes */

            /// Writes a single byte into the arena.
            $hvis const fn push_byte(&mut self, byte: u8) -> Option<$Handle> {
                self.push_bytes(&[byte])
            }
            /// Reads the single byte described by `h`.
            $hvis const fn read_byte(&self, h: $Handle) -> Option<u8> {
                let (start, end) = $crate::unwrap![some? self._span_usize(h)];
                if end - start != 1 { return None; }
                Some(<$_>::read_byte(&self.data, start))
            }
            /// Returns the single byte described by `h` exclusively.
            $hvis const fn read_byte_mut(&mut self, h: $Handle) -> Option<&mut u8> {
                let (start, end) = $crate::unwrap![some? self._span_usize(h)];
                if end - start != 1 { return None; }
                Some(<$_>::read_byte_mut(&mut self.data, start))
            }
            /// Replaces the single byte described by `h`.
            $hvis const fn replace_byte(&mut self, h: $Handle, new: u8) -> bool {
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
            $hvis const fn view_bytes(&self, h: $Handle, count: $cprim) -> Option<&[u8]> {
                let (start, end) = $crate::unwrap![some? self._span_repeat_usize(h, count)];
                Some(<$_>::slice_bytes(&self.data, start, end))
            }

            /// Returns an exclusive slice starting at `handle`,
            /// and spanning `count` items of its length.
            ///
            /// Returns `None` if the repeated span overflows
            /// or extends beyond the written prefix.
            $hvis const fn view_bytes_mut(&mut self, h: $Handle, count: $cprim)
                -> Option<&mut [u8]> {
                let (start, end) = $crate::unwrap![some? self._span_repeat_usize(h, count)];
                Some(<$_>::slice_bytes_mut(&mut self.data, start, end))
            }

            /* shrinking the arena */

            /// Truncates the arena if `handle` describes its final region.
            $hvis const fn truncate_last(&mut self, h: $Handle) -> bool {
                let (_, end) = $crate::unwrap![some_or? self._span_usize(h), false];
                if end != self._len_usize() { return false; }
                self.len = h.offset_prim();
                true
            }
            /// Copies the final stored span into `dst` and removes it.
            $hvis const fn pop_into(&mut self, h: $Handle, dst: &mut [u8]) -> bool {
                let src = $crate::unwrap![some_or? self.read_bytes(h), false];
                if src.len() != dst.len() { return false; }
                dst.copy_from_slice(src);
                self.truncate_last(h)
            }
        }

        /* primitives */

        #[allow(dead_code, private_interfaces)]
        /// Implements push, read and replace for primitives.
        impl<const CAP: usize> $Arena<CAP> {
            _impl_arena_methods_for_prims!();

            /* bool */

            /// Pushes a `bool`. Returns its handle on success.
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
                match self.read_byte(handle) {
                    Some(0) => Some(false),
                    Some(1) => Some(true),
                    _ => None,
                }
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
                    let bytes = $crate::unwrap![some? self.read_bytes(handle)];
                    if bytes.len() != T_SIZE { return None; }
                    Some($oprim::from_le_bytes(
                        *$crate::unwrap![some? bytes.first_chunk::<T_SIZE>()]))
                }
                #[doc = "Replaces a `" $oprim
                "` from the given `handle`. Returns `true` on success."]
                $hvis const fn [<replace_ $oprim>](&mut self, handle: $Handle, val: $oprim)
                    -> bool {
                    const T_SIZE: usize = size_of::<$oprim>();
                    let bytes = $crate::unwrap![some_or? self.read_bytes_mut(handle), false];
                    if bytes.len() != T_SIZE { return false; }
                    let arr = $crate::unwrap![some_or? bytes.first_chunk_mut::<T_SIZE>(), false];
                    *arr = val.to_le_bytes();
                    true
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
                    if len > <$oprim>::MAX as usize { return None; }
                    let prefix = (len as $oprim).to_le_bytes();
                    let start = self._len_usize();
                    let total = $crate::unwrap![some? prefix.len().checked_add(len)];
                    let end = $crate::unwrap![some? start.checked_add(total)];
                    if end > CAP { return None; }
                    let handle = $crate::unwrap![ok_some? <$Handle>::try_from_usize(start, total)];
                    $crate::whilst! { i in 0..prefix.len(); {
                        <$_>::write_byte(&mut self.data, start + i, prefix[i]);
                    }}
                    let bytes = val.as_bytes();
                    $crate::whilst! { i in 0..bytes.len(); {
                        <$_>::write_byte(&mut self.data, start + prefix.len() + i, bytes[i]);
                    }}
                    self.len = end as $cprim;
                    Some(handle)
                }
                #[doc = "Reads a `&str` with a prefixed len of up to [`" $oprim "::MAX`] bytes"]
                /// from the given handle.
                ///
                /// # Errors
                /// Returns `None` if the handle is invalid or incomplete.
                $hvis const fn [<read_str_ $oprim>](&self, h: $Handle) -> Option<&str> {
                    const LEN_SIZE: usize = size_of::<$oprim>();
                    let (start, end) = $crate::unwrap![some? self._span_usize(h)];
                    let data_start = $crate::unwrap![some? start.checked_add(LEN_SIZE)];
                    if data_start > end { return None; }
                    let prefix = <$_>::slice_bytes(&self.data, start, data_start);
                    let prefix = $crate::unwrap![some? prefix.first_chunk::<LEN_SIZE>()];
                    let stored_len = <$oprim>::from_le_bytes(*prefix) as usize;
                    if stored_len != end - data_start { return None; }
                    let bytes = <$_>::slice_bytes(&self.data, data_start, end);
                    $crate::unwrap![ok_some $crate::Str::from_utf8(bytes)]
                }
                #[doc = "Replaces a `&str` with a prefixed len of up to [`" $oprim "::MAX`] bytes"]
                /// from the given handle. Returns `true` on success.
                ///
                /// Both strings have to have the same byte length.
                $hvis const fn [<replace_str_ $oprim>](&mut self, h: $Handle, val: &str) -> bool {
                    const LEN_SIZE: usize = size_of::<$oprim>();
                    let (start, end) = $crate::unwrap![some_or? self._span_usize(h), false];
                    let data_start = $crate::unwrap![some_or? start.checked_add(LEN_SIZE), false];
                    if data_start > end { return false; }
                    let prefix = <$_>::slice_bytes(&self.data, start, data_start);
                    let prefix = $crate::unwrap![some_or? prefix.first_chunk::<LEN_SIZE>(), false];
                    let stored_len = <$oprim>::from_le_bytes(*prefix) as usize;
                    if stored_len != end - data_start || stored_len != val.len() { return false; }
                    let dst = <$_>::slice_bytes_mut(&mut self.data, data_start, end);
                    dst.copy_from_slice(val.as_bytes());
                    true
                }
            }};
        }
        use _impl_arena_methods_for_prims;
    };
}
#[doc(inline)]
pub use arena_bytes;
