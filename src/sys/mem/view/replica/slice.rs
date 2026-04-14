// devela::sys::mem::view::replica::slice
//
//! Defines [`MemReplicaError`], [`MemReplicaSlice`].
//

use crate::{MemReplicaError, Slice, is, unwrap, whilst};

#[doc = crate::_tags!(mem layout lifetime)]
/// Mutable replicated view over a backing slice.
#[doc = crate::_doc_location!("sys/mem/layout")]
///
/// `MemReplicaSlice` stores logical elements in an interleaved physical layout
/// such that each logical index maps to `N` channel-separated replica slots.
///
/// The backing slice is physical storage, while [`len`](Self::len) and
/// [`capacity`](Self::capacity) are expressed in logical elements.
#[derive(Debug, PartialEq)]
pub struct MemReplicaSlice<'a, T, const N: usize> {
    storage: &'a mut [T],
    channel_offset_bytes: usize,
    num_channels: usize,
    len: usize,
    capacity: usize,
    elems_per_chunk: usize,
    stride_elems: usize,
}

impl<'a, T, const N: usize> MemReplicaSlice<'a, T, N> {
    /// Creates a replicated logical view over `storage`.
    ///
    /// The layout is validated against the concrete element type `T` and replica
    /// count `N`. On success, the returned view starts logically empty, even though
    /// the backing slice may already contain arbitrary data.
    ///
    /// # Errors
    /// Returns:
    /// - [`MemReplicaError::ZeroReplicas`] if `N == 0`.
    /// - [`MemReplicaError::TooManyReplicas`] if `N > num_channels()`.
    /// - [`MemReplicaError::ZeroSizedType`] if `T` is zero-sized.
    /// - [`MemReplicaError::OffsetTooSmall`] if the channel offset is smaller than
    ///   one element.
    /// - [`MemReplicaError::MisalignedOffset`] if the channel offset is not a
    ///   multiple of `size_of::<T>()`.
    /// - [`MemReplicaError::StorageTooSmall`] if the backing slice cannot hold at
    ///   least one logical chunk.
    pub const fn new(
        storage: &'a mut [T],
        channel_offset_bytes: usize,
        num_channels: usize,
    ) -> Result<Self, MemReplicaError> {
        use MemReplicaError as E;
        is! { num_channels == 0, return Err(E::ZeroChannels) }
        is! { N == 0, return Err(E::ZeroReplicas) }
        is! { N > num_channels, return Err(E::TooManyReplicas) }

        let elem_size = size_of::<T>();
        is! { elem_size == 0, return Err(E::ZeroSizedType) }
        is! { channel_offset_bytes < elem_size, return Err(E::OffsetTooSmall) }
        is! { channel_offset_bytes % elem_size != 0, return Err(E::MisalignedOffset) }

        let elems_per_chunk = channel_offset_bytes / elem_size;
        let stride_elems = num_channels * elems_per_chunk;
        is! { stride_elems == 0, return Err(E::StorageTooSmall) }

        let full_strides = storage.len() / stride_elems;
        let capacity = full_strides * elems_per_chunk;
        is! { capacity == 0, return Err(E::StorageTooSmall) }
        Ok(Self {
            storage,
            channel_offset_bytes,
            num_channels,
            len: 0,
            capacity,
            elems_per_chunk,
            stride_elems,
        })
    }

    /// Returns the byte distance between adjacent channel bases.
    pub const fn channel_offset_bytes(self) -> usize {
        self.channel_offset_bytes
    }
    /// Returns the total number of channels described by this layout.
    pub const fn num_channels(self) -> usize {
        self.num_channels
    }

    /// Returns the number of initialized logical elements.
    pub const fn len(&self) -> usize {
        self.len
    }
    /// Returns `true` if no logical elements have been inserted yet.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Returns the logical capacity.
    ///
    /// This is the maximum number of logical elements that can be addressed by this
    /// replicated view, not the length of the backing slice.
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Sets the initialized logical length.
    ///
    /// This controls which logical indices are considered readable through
    /// [`get`](Self::get), [`get_replica`](Self::get_replica), and
    /// [`replicas`](Self::replicas).
    ///
    /// It does not write or clear storage. It only changes the logical
    /// publication boundary within the existing capacity.
    ///
    /// This is useful after staging values with [`set`](Self::set) before
    /// exposing them as initialized logical elements.
    ///
    /// # Errors
    /// Returns [`MemReplicaError::OutOfBounds`] if `new_len > self.capacity()`.
    pub const fn set_len(&mut self, new_len: usize) -> Result<(), MemReplicaError> {
        is! { new_len > self.capacity, return Err(MemReplicaError::OutOfBounds) }
        self.len = new_len;
        Ok(())
    }

    /// Returns the first replica for `logical_index`.
    ///
    /// This is a convenience alias for [`get_replica`](Self::get_replica) with `replica = 0`.
    pub const fn get(&self, logical_index: usize) -> Option<&T> {
        self.get_replica(0, logical_index)
    }
    /// Returns a shared reference to one replica of a logical element.
    ///
    /// Returns `None` if `replica >= N` or if `logical_index` lies outside the
    /// initialized logical length.
    pub const fn get_replica(&self, replica: usize, logical_index: usize) -> Option<&T> {
        is! { replica >= N || logical_index >= self.len, return None }
        let idx = unwrap![some? self.physical_index(replica, logical_index)];
        Slice::get(self.storage, idx)
    }
    /// Returns an exclusive reference to one replica of a logical element.
    ///
    /// Returns `None` under the same conditions as
    /// [`get_replica`](Self::get_replica).
    pub const fn get_replica_mut(
        &mut self,
        replica: usize,
        logical_index: usize,
    ) -> Option<&mut T> {
        is! { replica >= N || logical_index >= self.len, return None }
        let idx = unwrap![some? self.physical_index(replica, logical_index)];
        Slice::get_mut(self.storage, idx)
    }

    /// Returns shared references to all replicas of one logical element.
    ///
    /// The returned array is ordered by replica index, from `0` to `N - 1`.
    ///
    /// Returns `None` if `logical_index` lies outside the initialized logical length.
    pub const fn replicas(&self, logical_index: usize) -> Option<[&T; N]> {
        is! { logical_index >= self.len, return None }
        let first = unwrap![some? self.get_replica(0, logical_index)];
        let mut out = [first; N];
        whilst! { replica in 1..N; {
            out[replica] = unwrap![some? self.get_replica(replica, logical_index)];
        }}
        Some(out)
    }

    /// Maps `(replica, logical_index)` to a physical index in the backing slice.
    ///
    /// The mapping uses chunked interleaving:
    /// each chunk spans one channel-offset worth of elements, and each logical
    /// chunk is repeated across all channels before the next chunk begins.
    ///
    /// Returns `None` if `replica >= N` or if `logical_index` lies outside the
    /// logical capacity.
    pub const fn physical_index(&self, replica: usize, logical_index: usize) -> Option<usize> {
        is! { replica >= N || logical_index >= self.capacity, return None }
        let chunk_idx = logical_index / self.elems_per_chunk;
        let offset_in_chunk = logical_index % self.elems_per_chunk;
        Some(chunk_idx * self.stride_elems + replica * self.elems_per_chunk + offset_in_chunk)
    }
}

impl<'a, T: Copy, const N: usize> MemReplicaSlice<'a, T, N> {
    /// Inserts `value` at the current logical end and advances the logical length.
    ///
    /// The value is written to all `N` replicas.
    ///
    /// # Errors
    /// Returns [`MemReplicaError::Full`] if no logical capacity remains.
    pub const fn insert(&mut self, value: T) -> Result<(), MemReplicaError> {
        is! { self.len >= self.capacity, return Err(MemReplicaError::Full) }
        unwrap![ok? self.set(self.len, value)];
        self.len += 1;
        Ok(())
    }

    /// Writes `value` into all replicas of `logical_index`.
    ///
    /// Unlike [`insert`](Self::insert), this does not modify the logical length.
    /// This allows writing within capacity before that logical slot is considered
    /// initialized.
    ///
    /// # Errors
    /// Returns [`MemReplicaError::OutOfBounds`] if `logical_index` lies outside the
    /// logical capacity.
    pub const fn set(&mut self, logical_index: usize, value: T) -> Result<(), MemReplicaError> {
        is! { logical_index >= self.capacity, return Err(MemReplicaError::OutOfBounds) }
        whilst! { replica in 0..N; {
            let idx = unwrap![some_ok_or?
                self.physical_index(replica, logical_index),
                MemReplicaError::OutOfBounds];
            self.storage[idx] = value;
        }}
        Ok(())
    }
}
