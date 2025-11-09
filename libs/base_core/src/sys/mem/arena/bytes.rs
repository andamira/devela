// devela_base_core::sys::mem::arena::bytes
//
//! Defines [`ArenaBytes`], [`ArenaMark`].
//
// TOC
// - struct ArenaMark
// - struct ArenaBytes
// - internal helpers
// - fundamental methods
// - primitives (de)coding

use crate::{ArenaHandle, Slice, unwrap};

#[cfg(feature = "unsafe_array")]
type ArenaByteUnit = crate::MaybeUninit<u8>;
#[cfg(not(feature = "unsafe_array"))]
type ArenaByteUnit = u8;

/// Append-only mark for snapshots and rollback in an arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArenaMark(usize);
impl ArenaMark {
    const fn new(mark: usize) -> Self {
        ArenaMark(mark)
    }
}

/// An heterogeneous, safe byte arena.
///
/// # Features
/// Uses `unsafe_array` to leverage `MaybeUninit` and avoid initializing the full capacity.
/// And uses `unsafe_slice` for further performance gains.
#[derive(Clone, Debug)]
pub struct ArenaBytes<const CAP: usize> {
    data: [ArenaByteUnit; CAP],
    len: usize,
}

/* misc. trait impls */
impl<const CAP: usize> Eq for ArenaBytes<CAP> {}
#[rustfmt::skip] // Note: this is not recursive, but calls the manual method.
impl<const CAP: usize> PartialEq for ArenaBytes<CAP> {
    fn eq(&self, other: &Self) -> bool { ArenaBytes::eq(self, other) }
}
#[rustfmt::skip]
impl<const CAP: usize> Default for ArenaBytes<CAP> { fn default() -> Self { Self::new() } }

// Internal helpers to abstract away MaybeUninit representations
#[rustfmt::skip]
impl<const CAP: usize> ArenaBytes<CAP> {

    #[inline(always)]
    const fn _read_byte(&self, i: usize) -> u8 {
        #[cfg(not(feature = "unsafe_array"))]
        return self.data[i];
        #[cfg(feature = "unsafe_array")]
        unsafe { self.data[i].assume_init_read() }
    }
    #[inline(always)]
    const fn _read_byte_mut(&mut self, i: usize) -> &mut u8 {
        #[cfg(not(feature = "unsafe_array"))]
        return &mut self.data[i];
        #[cfg(feature = "unsafe_array")]
        unsafe { self.data[i].assume_init_mut() }
    }
    #[inline(always)]
    const fn _write_byte(&mut self, i: usize, b: u8) {
        #[cfg(not(feature = "unsafe_array"))]
        { self.data[i] = b; }
        #[cfg(feature = "unsafe_array")]
        self.data[i].write(b);
    }

    #[inline(always)]
    const fn _slice_bytes(&self, start: usize, end: usize) -> &[u8] {
        #[cfg(not(feature = "unsafe_array"))] // safe
        return Slice::range(&self.data, start, end);
        #[cfg(all(feature = "unsafe_array", not(feature = "unsafe_slice")))] // unsafe
        unsafe { Slice::range(
            Slice::from_raw_parts(self.data.as_ptr().cast::<u8>(), CAP), start, end) }
        #[cfg(all(feature = "unsafe_array", feature = "unsafe_slice"))] // unsafest
        unsafe { Slice::range_unchecked(
            Slice::from_raw_parts(self.data.as_ptr().cast::<u8>(), CAP), start, end) }
    }
    #[inline(always)]
    const fn _slice_bytes_mut(&mut self, start: usize, end: usize) -> &mut [u8] {
        #[cfg(not(feature = "unsafe_array"))] // safe
        return Slice::range_mut(&mut self.data, start, end);
        #[cfg(all(feature = "unsafe_array", not(feature = "unsafe_slice")))] // unsafe
        unsafe { Slice::range_mut(
            Slice::from_raw_parts_mut(self.data.as_mut_ptr().cast::<u8>(), CAP), start, end) }
        #[cfg(all(feature = "unsafe_array", feature = "unsafe_slice"))] // unsafest
        unsafe { Slice::range_mut_unchecked(
            Slice::from_raw_parts_mut(self.data.as_mut_ptr().cast::<u8>(), CAP), start, end) }
    }
}

mod main_implementations {
    use super::{ArenaBytes, ArenaHandle, ArenaMark, Slice, unwrap};
    #[cfg(feature = "unsafe_array")]
    use crate::MaybeUninit;
    use crate::{is, lets, whilst};

    // Fundamental methods.
    #[rustfmt::skip]
    impl<const CAP: usize> ArenaBytes<CAP> {

        /// Returns a new empty arena.
        ///
        /// # Features
        /// Uses `unsafe_array` to avoid initializing the full capacity.
        #[inline(always)]
        pub const fn new() -> Self {
            #[cfg(not(feature = "unsafe_array"))]
            { Self { data: [0_u8; CAP], len: 0 } }
            #[cfg(feature = "unsafe_array")]
            { Self { data: [MaybeUninit::<u8>::uninit(); CAP], len: 0 } }
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

        /// Compares two arenas for equality.
        pub const fn eq(&self, other: &Self) -> bool {
            Slice::<u8>::eq(self.as_bytes(), other.as_bytes())
        }

        /* snapshot and rollback */

        #[inline(always)]
        /// Snapshot a position in the arena.
        pub const fn mark(&self) -> ArenaMark { ArenaMark::new(self.len) }
        #[inline(always)]
        /// Rollback to a previous marked position.
        pub const fn rollback(&mut self, m: ArenaMark) { self.len = m.0; }

        /* byte slices */

        /// Returns a byte slice over all the written data.
        #[inline(always)]
        pub const fn as_bytes(&self) -> &[u8] { self._slice_bytes(0, self.len) }
        /// Returns an exclusive byte slice over all the written data.
        #[inline(always)]
        pub const fn as_bytes_mut(&mut self) -> &mut [u8] { self._slice_bytes_mut(0, self.len) }

        /// Write a byte slice into the arena.
        pub const fn push_bytes(&mut self, bytes: &[u8]) -> Option<ArenaHandle> {
            unwrap!(some_if? self.len.checked_add(bytes.len()), |v| v <= CAP);
            lets![start = self.len, handle = ArenaHandle::new(start, bytes.len())];
            whilst! { i in 0..bytes.len(); {
                self._write_byte(self.len, bytes[i]);
                self.len += 1;
            }}
            Some(handle)
        }

        /// Read a shared slice over the written bytes.
        pub const fn read_bytes(&self, handle: ArenaHandle) -> Option<&[u8]> {
            is![handle.offset + handle.len > self.len; return None];
            Some(self._slice_bytes(handle.offset, handle.offset + handle.len))
        }
        /// Read an exclusive slice over the written bytes.
        pub const fn read_bytes_mut(&mut self, h: ArenaHandle) -> Option<&mut [u8]> {
            if h.offset + h.len > self.len { return None; }
            Some(self._slice_bytes_mut(h.offset, h.offset + h.len))
        }

        /// Replace the bytes for `handle`. Lengths must match. Returns `false` otherwise.
        pub const fn replace_bytes(&mut self, h: ArenaHandle, new: &[u8]) -> bool {
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
        pub const fn push_byte(&mut self, byte: u8) -> Option<ArenaHandle> {
            if self.len + 1 > CAP { return None; }
            self._write_byte(self.len, byte);
            let handle = ArenaHandle::new(self.len, 1);
            self.len += 1;
            Some(handle)
        }
        /// Read a byte previously written.
        pub const fn read_byte(&self, handle: ArenaHandle) -> Option<u8> { // should I return &u8?
            is![handle.offset + 1 > self.len; return None];
            Some(self._read_byte(handle.offset))
        }
        /// Read a byte previously written.
        pub const fn read_byte_mut(&mut self, handle: ArenaHandle) -> Option<&mut u8> {
            is![handle.offset + 1 > self.len; return None];
            Some(self._read_byte_mut(handle.offset))
        }
        /// Replace the bytes for `handle`. Length must match.
        pub const fn replace_byte(&mut self, handle: ArenaHandle, new: u8) -> bool {
            if handle.len != 1 { return false; }
            self._write_byte(handle.offset, new);
            true
        }

        /* views over multiple values */

        /// Returns a shared slice starting at `handle` and spanning `count` items of its length.
        ///
        /// Returns `None` if...
        pub const fn view_bytes(&self, h: ArenaHandle, count: usize) -> Option<&[u8]> {
            let total = h.len * count;
            if h.offset + total > self.len { return None; }
            Some(self._slice_bytes(h.offset, h.offset + total))
        }

        /// Returns an exclusive slice starting at `handle` and spanning `count` items of its length.
        ///
        /// Returns `None` if...
        pub const fn view_bytes_mut(&mut self, h: ArenaHandle, count: usize) -> Option<&mut [u8]> {
            let total = h.len * count;
            if h.offset + total > self.len { return None; }
            Some(self._slice_bytes_mut(h.offset, h.offset + total))
        }

        /* shrinking the arena */

        /// Truncates the arena if the handle corresponds to the last region.
        pub const fn truncate_last(&mut self, h: ArenaHandle) -> bool {
            let end = h.offset + h.len;
            if end != self.len { return false; }
            self.len = h.offset;
            true
        }

        /// Copies the last stored value into `dst` and removes it.
        pub const fn pop_into(&mut self, h: ArenaHandle, dst: &mut [u8]) -> bool {
            if h.len != dst.len() { return false; }
            if let Some(src) = self.read_bytes(h) {
                whilst! { i in 0..h.len; { dst[i] = src[i]; }}
                self.truncate_last(h)
            } else {
                false
            }
        }
    }
}

mod impl_primitives {
    use super::{ArenaBytes, ArenaHandle, unwrap};
    use crate::{Str, is};

    /// Implements push, read and replace for primitives.
    impl<const CAP: usize> ArenaBytes<CAP> {
        impl_for_primitives!();

        /* bool */

        /// Pushes a `char`. Returns its handle on success.
        ///
        /// # Errors
        /// Returns `None` if there’s insufficient capacity.
        pub const fn push_bool(&mut self, val: bool) -> Option<ArenaHandle> {
            self.push_byte(val as u8)
        }
        /// Reads a `bool` from the given `handle`.
        ///
        /// # Errors
        /// Returns `None` if the handle is invalid or incomplete.
        pub const fn read_bool(&self, handle: ArenaHandle) -> Option<bool> {
            is![let Some(b) = self.read_byte(handle); Some(b != 0); None]
        }
        /// Replaces a `bool` from the given `handle`. Returns `true` on success.
        pub const fn replace_bool(&mut self, handle: ArenaHandle, val: bool) -> bool {
            self.replace_byte(handle, val as u8)
        }

        /* char */

        /// Pushes a `char`. Returns its handle on success.
        ///
        /// # Errors
        /// Returns `None` if there’s insufficient capacity.
        #[inline(always)]
        pub const fn push_char(&mut self, val: char) -> Option<ArenaHandle> {
            self.push_u32(val as u32)
        }
        /// Reads a `char` from the given `handle`.
        ///
        /// # Errors
        /// Returns `None` if the handle is invalid or incomplete.
        #[inline(always)]
        pub const fn read_char(&self, handle: ArenaHandle) -> Option<char> {
            is![let Some(c) = self.read_u32(handle); char::from_u32(c); None]
        }
        /// Replaces a `char` from the given `handle`. Returns `true` on success.
        #[inline(always)]
        pub const fn replace_char(&mut self, handle: ArenaHandle, val: char) -> bool {
            self.replace_u32(handle, val as u32)
        }
    }

    /// Helper to implement push, read & replace methods over primitives.
    macro_rules! impl_for_primitives {
        () => {
            impl_for_primitives!(single-byte: u8, i8);
            impl_for_primitives!(multi-byte:
                u16, u32, u64, u128, usize, i16, i32, i64, i128, isize, f32, f64,
            );
            impl_for_primitives!(str_len: u8, u16, u32, usize);
        };
        (single-byte: $($T:ty),+ $(,)?) => { $( impl_for_primitives!(%single-byte: $T); )+ };
        (%single-byte: $T:ty) => { $crate::paste! {
            #[doc = "Pushes a `" $T "`. Returns its handle on success."]
            ///
            /// # Errors
            /// Returns `None` if there’s insufficient capacity.
            #[inline(always)]
            pub const fn [<push_ $T>](&mut self, val: $T) -> Option<ArenaHandle> {
                self.push_byte(val as u8)
            }
            #[doc = "Reads a `" $T "` from the given `handle`."]
            ///
            /// # Errors
            /// Returns `None` if the handle is invalid or incomplete.
            pub const fn [<read_ $T>](&self, handle: ArenaHandle) -> Option<$T> {
                is![let Some(b) = self.read_byte(handle); Some(b as $T); None]
            }
            #[doc = "Replaces a `" $T "` from the given `handle`. Returns `true` on success."]
            pub const fn [<replace_ $T>](&mut self, handle: ArenaHandle, val: $T) -> bool {
                self.replace_byte(handle, val as u8)
            }
        }};
        (multi-byte: $($T:ty),+ $(,)?) => { $( impl_for_primitives!(%multi-byte: $T); )+ };
        (%multi-byte: $T:ty) => { $crate::paste! {
            #[doc = "Pushes a `" $T "` in little-endian order. Returns its handle on success."]
            ///
            /// # Errors
            /// Returns `None` if there’s insufficient capacity.
            #[inline(always)]
            pub const fn [<push_ $T>](&mut self, val: $T) -> Option<ArenaHandle> {
                self.push_bytes(&val.to_le_bytes())
            }
            #[doc = "Reads a `" $T "` in little-endian order from the given `handle`."]
            ///
            /// # Errors
            /// Returns `None` if the handle is invalid or incomplete.
            pub const fn [<read_ $T>](&self, handle: ArenaHandle) -> Option<$T> {
                const T_SIZE: usize = size_of::<$T>();
                if let Some(bytes) = self.read_bytes(handle) {
                    Some($T::from_le_bytes(*unwrap![some? bytes.first_chunk::<{T_SIZE}>()]))
                } else { None }
            }
            #[doc = "Replaces a `" $T "` from the given `handle`. Returns `true` on success."]
            pub const fn [<replace_ $T>](&mut self, handle: ArenaHandle, val: $T) -> bool {
                if let Some(b) = self.read_bytes_mut(handle) {
                    if let Some(arr) = b.first_chunk_mut::<{size_of::<$T>()}>() {
                        *arr = val.to_le_bytes();
                        return true;
                    }
                }
                false
            }
        }};
        (str_len: $($T:ty),+ $(,)?) => { $( impl_for_primitives!(%str_len: $T); )+ };
        (%str_len: $T:ty) => { $crate::paste! {
            #[doc = "Pushes a `&str` with a prefixed len of up to [`" $T "::MAX`] bytes."]
            /// Returns its handle on success.
            ///
            /// # Errors
            /// Returns `None` if there’s insufficient capacity or the string is too long.
            pub const fn [<push_str_ $T>](&mut self, val: &str) -> Option<ArenaHandle> {
                let len = val.len();
                if len <= <$T>::MAX as usize {
                    let prefix = unwrap![some? self.[<push_ $T>](len as $T)];
                    let data = unwrap![some? self.push_bytes(&val.as_bytes())];
                    Some(ArenaHandle::new(prefix.offset, prefix.len + data.len))
                } else { None }
            }

            #[doc = "Reads a `&str` with a prefixed len of up to [`" $T "::MAX`] bytes"]
            /// from the given `handle`.
            ///
            /// # Errors
            /// Returns `None` if the handle is invalid or incomplete.
            pub const fn [<read_str_ $T>](&self, handle: ArenaHandle) -> Option<&str> {
                let len_size = size_of::<$T>();
                let handle = ArenaHandle::new(handle.offset + len_size, handle.len - len_size);
                if let Ok(s) = Str::from_utf8(unwrap![some? self.read_bytes(handle)]) {
                    Some(s)
                } else { None }
            }

            #[doc = "Replaces a `&str` with a prefixed len of up to [`" $T "::MAX`] bytes"]
            /// from the given `handle`. Returns `true` on success.
            ///
            /// Both strings have to have the same byte length.
            pub const fn [<replace_str_ $T>](&mut self, handle: ArenaHandle, val: &str) -> bool {
                let len_size = size_of::<$T>();
                let handle = ArenaHandle::new(handle.offset + len_size, handle.len - len_size);
                is![handle.len != val.len(); return false];
                if let Some(dst) = self.read_bytes_mut(handle) {
                    dst.copy_from_slice(val.as_bytes());
                    return true;
                }
                false
            }
        }};
    }
    use impl_for_primitives;
}
