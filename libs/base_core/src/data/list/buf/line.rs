// devela_base_core::data::list::buf::line
//
//! Defines [`define_bufline!`].
//

/// Linear index interpreter over contiguous storage.
///
/// Interprets a contiguous storage backend as a linear buffer,
/// where elements occupy a prefix of the underlying storage.
///
/// The storage strategy determines ownership and drop behavior,
/// while `len` defines the logical extent of the buffer.
///
/// # Invariants
/// - `0 <= len <= capacity(S)`
/// - Logical element `i` is stored at physical index `i`
/// - Only elements in `storage[0 .. len)` are considered part of the buffer
///
/// # Storage backends
/// - Owned array (`[T; CAP]`)
/// - Owned uninitialized array (`[MaybeUninit<T>; CAP]`)
/// - Owned option array (`[Option<T>; CAP]`)
/// - Exclusive slice (`&'a mut [T]`)
/// - Shared slice (`&'a [T]`)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufLine<'a, T, S> {
    storage: S,
    len: usize,
    _marker: crate::PhantomData<&'a T>,
}

/* common */

#[rustfmt::skip]
impl<'a, T, S> BufLine<'a, T, S> {
    /// Returns the number of elements currently stored in the buffer.
    pub const fn len(&self) -> usize { self.len }
    /// Returns `true` if the buffer contains no elements.
    pub const fn is_empty(&self) -> bool { self.len == 0 }
}

/* owned array */

/// Fully initialized storage.
///
/// # Invariants
/// - All CAP slots always contain a valid T
/// - len controls logical membership, not initialization
/// - Dropping the array drops all CAP elements
///
/// Consequences
/// - Cannot move out `T` safely
/// - Pop must be Copy or Clone
/// - Shrinking len does not affect drop behavior
#[rustfmt::skip]
impl<T, const CAP: usize> BufLine<'_, T, [T; CAP]> {
    /// Returns the fixed capacity of the buffer.
    pub const fn capacity(&self) -> usize { CAP }

    /// Returns `true` if the buffer has reached its capacity.
    pub const fn is_full(&self) -> bool { self.len == self.capacity() }

    /* construct */

    /// Creates a buffer from a fully initialized array with logical length 0.
    pub const fn new(storage: [T; CAP]) -> Self {
        Self { storage, len: 0, _marker: crate::PhantomData }
    }

    /// Creates a new fully initialized buffer with logical length 0.
    pub const fn new_init() -> Self where T: crate::ConstInitCore {
        Self { storage: [T::INIT; CAP], len: 0, _marker: crate::PhantomData }
    }

    /// Creates a buffer from a fully initialized array and an explicit length.
    ///
    /// # Panics
    /// Panics if `len > CAP`.
    pub const fn from_array(storage: [T; CAP], len: usize) -> Self {
        assert!(len <= CAP);
        Self { storage, len, _marker: crate::PhantomData }
    }

    /// Creates a new buffer by cloning all the possible elements from `src`,
    /// after initializing the capacity with the `init` value.
    pub fn from_slice_clone(src: &[T], init: T) -> Option<Self> where T: Clone {
        if src.len() > CAP { return None; }
        let mut storage = crate::array_from_fn(|_| init.clone());
        crate::whilst! { i in 0..src.len(); { storage[i] = src[i].clone(); }}
        Some(Self { storage, len: src.len(), _marker: crate::PhantomData })
    }

    /// Creates a new buffer by copying all the possible elements from `src`,
    /// after initializing the capacity with the `init` value.
    pub const fn from_slice_copy(src: &[T], init: T) -> Option<Self> where T: Copy {
        if src.len() > CAP { return None; }
        let mut storage = [init; CAP];
        crate::whilst! { i  in 0..src.len(); { storage[i] = src[i]; }}
        Some(Self { storage, len: src.len(), _marker: crate::PhantomData })
    }

    /// Creates a new buffer by moving all the possible elements from `src`,
    /// and replacing them with the default value,
    /// after initializing the capacity with the default value.
    pub fn from_slice_move_default(src: &mut [T]) -> Option<Self> where T: Default {
        if src.len() > CAP { return None; }
        let mut storage = crate::array_from_fn(|_| T::default());
        crate::whilst! { i in 0..src.len(); { storage[i] = crate::Mem::take(&mut src[i]); }}
        Some(Self { storage, len: src.len(), _marker: crate::PhantomData })
    }

    /* deconstruct */

    /// Sets the logical length to zero.
    ///
    /// Does not drop elements.
    pub const fn clear(&mut self) { self.len = 0; }

    /// Sets the logical length to `min(new_len, len)`.
    ///
    /// If `new_len >= len`, this is a no-op.
    pub const fn truncate(&mut self, new_len: usize) {
        if new_len <= self.len { self.len = new_len; }
    }

    /* push */

    /// Appends a value to the back of the buffer.
    ///
    /// Returns `Err(value)` if the buffer is full.
    pub fn push_back(&mut self, value: T) -> Result<(), T> {
        if self.is_full() { return Err(value); }
        self.storage[self.len] = value;
        self.len += 1;
        Ok(())
    }

    /// Appends a copy of `value` to the back of the buffer.
    ///
    /// Returns `Err(value)` if the buffer is full.
    pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
        if self.is_full() { return Err(value); }
        self.storage[self.len] = value;
        self.len += 1;
        Ok(())
    }

    /* pop */

    /// Removes and returns a cloned value from the back of the buffer.
    pub fn pop_back_clone(&mut self) -> Option<T> where T: Clone {
        if self.is_empty() { return None; }
        self.len -= 1;
        Some(self.storage[self.len].clone())
    }
    /// Removes and returns a copied value from the back of the buffer.
    pub const fn pop_back_copy(&mut self) -> Option<T> where T: Copy {
        if self.is_empty() { return None; }
        self.len -= 1;
        Some(self.storage[self.len])
    }

    /* peek */

    /// Returns a reference to the last element without removing it.
    pub const fn peek_back(&self) -> Option<&T> {
        if self.is_empty() { return None; }
        Some(&self.storage[self.len - 1])
    }
    /// Returns a reference to the last element without removing it.
    pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
        if self.is_empty() { return None; }
        Some(&mut self.storage[self.len - 1])
    }

    /* get */

    /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() { return None; }
        Some(&self.storage[index])
    }

    /// Returns an exclusive reference to the element at `index`, or `None` if out of bounds.
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() { return None; }
        Some(&mut self.storage[index])
    }

    /* take */

    /// Takes the value at `index`, replacing it with `T::default()`.
    pub fn take_default(&mut self, index: usize) -> Option<T> where T: Default {
        if index >= self.len() { return None; }
        Some(crate::Mem::replace(&mut self.storage[index], T::default()))
    }

    /// Takes the value at `index`, replacing it with `T::INIT`.
    pub const fn take_init(&mut self, index: usize) -> Option<T> where T: crate::ConstInitCore {
        if index >= self.len() { return None; }
        Some(crate::Mem::replace(&mut self.storage[index], T::INIT))
    }

    /// Takes the value at `index`, replacing it with `other`.
    pub fn take_with(&mut self, index: usize, other: T) -> Option<T> {
        if index >= self.len() { return None; }
        Some(crate::Mem::replace(&mut self.storage[index], other))
    }

    /// Takes the value at `index`, replacing it with a copy of `other`.
    pub const fn take_with_copy(&mut self, index: usize, other: T) -> Option<T> where T: Copy {
        if index >= self.len() { return None; }
        Some(crate::Mem::replace(&mut self.storage[index], other))
    }

    /* slice */

    /// Returns the initialized prefix as a slice.
    pub const fn as_slice(&self) -> &[T] {
        crate::Slice::range_to(&self.storage, self.len)
    }

    /// Returns the initialized prefix as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        crate::Slice::range_to_mut(&mut self.storage, self.len)
    }
}

/* owned uninit array */

/// Partially initialized storage.
///
/// # Invariants
/// - Only slots 0 .. len are initialized
/// - Slots len .. CAP are uninitialized and must never be dropped as T
/// - Drop behavior depends on len
///
/// Consequences
/// - pop_back can safely move out T
/// - Real drop operations are meaningful
/// - len controls both logical membership and initialization
///
/// Note:
/// In this implementation, `len` tracks initialization.
/// Shrinking the buffer drops elements beyond the new length.
#[rustfmt::skip]
#[cfg(all(not(base_safe_mem), feature = "unsafe_array"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
impl<T, const CAP: usize> BufLine<'_, T, [crate::MaybeUninit<T>; CAP]> {
    /// Returns the fixed capacity of the buffer.
    pub const fn capacity(&self) -> usize { CAP }

    /// Returns `true` if the buffer has reached its capacity.
    pub const fn is_full(&self) -> bool { self.len == self.capacity() }

    /* construct */

    /// Creates an empty buffer with uninitialized storage.
    pub const fn new() -> Self {
        Self {
            storage: [const { crate::MaybeUninit::uninit() }; CAP],
            len: 0,
            _marker: crate::PhantomData,
        }
    }
    /// Creates a buffer by moving all elements from an array.
    ///
    /// Initializes exactly `N` elements.
    ///
    /// # Panics
    /// Panics if `N > CAP`.
    pub fn from_array_exact<const N: usize>(src: [T; N]) -> Self {
        assert!(N <= CAP);
        let mut storage = [const { crate::MaybeUninit::uninit() }; CAP];
        let src = crate::ManuallyDrop::new(src);
        let ptr = src.as_ptr();
        // SAFETY: each element is read exactly once
        crate::whilst! { i in 0..N; { storage[i].write(unsafe { ptr.add(i).read() }); }}
        Self { storage, len: N, _marker: crate::PhantomData }
    }

    /// Creates a buffer from raw uninitialized storage.
    ///
    /// # Safety
    /// Caller must guarantee:
    /// - `storage[0..len]` are initialized
    /// - `storage[len..]` are uninitialized
    pub const unsafe fn from_array_unchecked(
        storage: [crate::MaybeUninit<T>; CAP],
        len: usize,
    ) -> Self {
        debug_assert!(len <= CAP);
        Self { storage, len, _marker: crate::PhantomData }
    }

    /// Creates a new buffer by cloning all the possible elements from `src`.
    pub fn from_slice_clone(src: &[T]) -> Option<Self> where T: Clone {
        if src.len() > CAP { return None; }
        let mut storage = [const { crate::MaybeUninit::uninit() }; CAP];
        crate::whilst! { i in 0..src.len(); { storage[i].write(src[i].clone()); }}
        Some(Self { storage, len: src.len(), _marker: crate::PhantomData })
    }

    /// Creates a new buffer by copying all the possible elements from `src`.
    pub const fn from_slice_copy(src: &[T]) -> Option<Self> where T: Copy {
        if src.len() > CAP { return None; }
        let mut storage = [const { crate::MaybeUninit::uninit() }; CAP];
        crate::whilst! { i in 0..src.len(); { storage[i].write(src[i]); }}
        Some(Self { storage, len: src.len(), _marker: crate::PhantomData })
    }

    /// Creates a new buffer by moving all the possible elements from `src`,
    /// and replacing them with the default value.
    pub fn from_slice_move_default(src: &mut [T]) -> Option<Self> where T: Default {
        if src.len() > CAP { return None; }
        let mut storage = [const { crate::MaybeUninit::uninit() }; CAP];
        crate::whilst! { i in 0..src.len(); { storage[i].write(crate::Mem::take(&mut src[i])); }}
        Some(Self { storage, len: src.len(), _marker: crate::PhantomData })
    }

    /// Creates a new buffer by moving all the possible elements from `src`,
    /// and replacing them with the initializing value.
    pub const fn from_slice_move_init(src: &mut [T]) -> Option<Self> where T: crate::ConstInitCore {
        if src.len() > CAP { return None; }
        let mut storage = [const { crate::MaybeUninit::uninit() }; CAP];
        crate::whilst! { i in 0..src.len(); {
            storage[i].write(crate::Mem::replace(&mut src[i], T::INIT));
        }}
        Some(Self { storage, len: src.len(), _marker: crate::PhantomData })
    }

    /* deconstruct */

    /// Drops all initialized elements and resets the buffer.
    pub fn clear(&mut self) {
        while self.len != 0 {
            self.len -= 1;
            // SAFETY: by invariant, slots 0..old_len are initialized. We decrement len first,
            // so `self.len` now indexes the last previously-initialized slot.
            unsafe { self.storage[self.len].assume_init_drop(); }
        }
    }

    /// Drops the last element without returning it.
    pub fn drop_back(&mut self) -> bool {
        if self.len == 0 { return false; }
        self.len -= 1;
        // SAFETY: The index we drop is < old_len so it is initialized.
        unsafe { self.storage[self.len].assume_init_drop(); }
        true
    }

    /// Sets the length to `min(new_len, len)`, dropping the truncated elements.
    ///
    /// If `new_len >= len`, this is a no-op.
    pub fn truncate(&mut self, new_len: usize) {
        while self.len > new_len {
            self.len -= 1;
            // SAFETY: see above
            unsafe { self.storage[self.len].assume_init_drop(); }
        }
    }

    /* push */

    /// Appends a value to the back of the buffer.
    ///
    /// Returns `Err(value)` if the buffer is full.
    pub const fn push_back(&mut self, value: T) -> Result<(), T> {
        if self.len == CAP { return Err(value); }
        self.storage[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    /// Removes and returns the last element.
    ///
    /// This moves the value out without requiring `T: Copy` or `T: Clone`.
    pub const fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        // SAFETY: `index < self.len`, so the slot is initialized per invariant.
        Some(unsafe { self.storage[self.len].assume_init_read() })
    }

    /* peek */

    /// Returns a reference to the last element without removing it.
    pub const fn peek_back(&self) -> Option<&T> {
        if self.is_empty() { return None; }
        // SAFETY: `index < self.len`, so the slot is initialized per invariant.
        Some(unsafe { &*self.storage[self.len - 1].as_ptr() })
    }
    /// Returns a reference to the last element without removing it.
    pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
        if self.is_empty() { return None; }
        // SAFETY: `index < self.len`, so the slot is initialized per invariant.
        Some(unsafe { &mut *self.storage[self.len - 1].as_mut_ptr() })
    }

    /* get */

    /// Returns a reference to the element at `index`, if within bounds.
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len { return None; }
        // SAFETY: `index < self.len`, so the slot is initialized per invariant.
        Some(unsafe { &*self.storage[index].as_ptr() })
    }

    /// Returns an exclusive reference to the element at `index`, if within bounds.
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len { return None; }
        // SAFETY: `index < self.len`, so the slot is initialized per invariant.
        Some(unsafe { &mut *self.storage[index].as_mut_ptr() })
    }

    /* take */

    /// Takes the value at `index`, replacing it with `value`.
    pub fn take_with(&mut self, index: usize, value: T) -> Option<T> {
        if index >= self.len { return None; }
        // SAFETY: `index < self.len`, so the slot is initialized per invariant.
        let old = unsafe { self.storage[index].assume_init_read() };
        self.storage[index].write(value);
        Some(old)
    }

    /// Takes the value at `index`, replacing it with `T::default()`.
    pub fn take_default(&mut self, index: usize) -> Option<T> where T: Default {
        self.take_with(index, T::default())
    }

    /// Takes the value at `index`, replacing it with `T::INIT`.
    pub const fn take_init(&mut self, index: usize) -> Option<T> where T: crate::ConstInitCore {
        if index >= self.len { return None; }
        // SAFETY: `index < self.len`, so the slot is initialized per invariant.
        let old = unsafe { self.storage[index].assume_init_read() };
        self.storage[index].write(T::INIT);
        Some(old)
    }

    /* slice */

    /// Returns the initialized prefix as a slice.
    pub const fn as_slice(&self) -> &[T] {
        // SAFETY: `0..self.len` is initialized per invariant.
        unsafe { core::slice::from_raw_parts(self.storage.as_ptr() as *const T, self.len) }
    }

    /// Returns the initialized prefix as a mutable slice.
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: `0..self.len` is initialized per invariant.
        unsafe { core::slice::from_raw_parts_mut(self.storage.as_mut_ptr() as *mut T, self.len) }
    }
}

/* owned array of options */

/// Fully initialized storage using `Option<T>` as a drop boundary.
///
/// # Invariants
/// - Slots `0 .. len` are `Some`
/// - Slots `len .. CAP` are logically outside the buffer
/// - No holes are permitted in the initialized prefix
///
/// # Notes
/// - `Option<T>` is used to control initialization and dropping, not sparsity
/// - `len` is the number of elements
/// - Methods never access storage past `len`
#[rustfmt::skip]
impl<T, const CAP: usize> BufLine<'_, T, [Option<T>; CAP]> {
    /// Returns the fixed capacity of the buffer.
    pub const fn capacity(&self) -> usize { CAP }

    /// Returns `true` if the buffer has reached its capacity.
    pub const fn is_full(&self) -> bool { self.len == self.capacity() }

    /* construct */

    /// Creates a buffer from a fully initialized array with logical length 0.
    pub const fn new() -> Self {
        Self { storage: [const { None }; CAP], len: 0, _marker: crate::PhantomData }
    }

    /// Creates a buffer from a fully initialized array of clonable elements.
    pub fn from_array_clone<const N: usize>(src: [T; N]) -> Self where T: Clone {
        assert!(N <= CAP);
        let mut storage = [const { None }; CAP];
        crate::whilst! { i in 0..N; { storage[i] = Some(src[i].clone()); }}
        Self { storage, len: N, _marker: crate::PhantomData }
    }

    /// Creates a buffer from a fully initialized array of copiable elements.
    pub const fn from_array_copy<const N: usize>(src: [T; N]) -> Self where T: Copy {
        assert!(N <= CAP);
        let mut storage = [const { None }; CAP];
        crate::whilst! { i in 0..N; { storage[i] = Some(src[i]); }}
        Self { storage, len: N, _marker: crate::PhantomData }
    }

    /// Creates a buffer from an array of options and an explicit logical length,
    /// without validating the linear invariant.
    ///
    /// # Panics
    /// Panics if `len > CAP`.
    ///
    /// # Safety
    /// Caller must guarantee:
    /// - `len <= CAP`
    /// - `storage[0..len]` are `Some`
    /// - `storage[len..CAP]` are `None`
    pub const unsafe fn from_array_unchecked(storage: [Option<T>; CAP], len: usize) -> Self {
        debug_assert!(len <= CAP);
        Self { storage, len, _marker: crate::PhantomData }
    }

    /// Creates a buffer from an array of options, validating the linear invariant.
    ///
    /// Returns `None` if:
    /// - a `None` appears before a `Some`
    /// - any element after the prefix is `Some`
    pub fn from_array_linear(storage: [Option<T>; CAP]) -> Option<Self> {
        let mut len = 0;
        crate::whilst! { i in 0..CAP; { if storage[i].is_some() { len += 1; } else { break; } }}
        crate::whilst! { i in len,..CAP; { if storage[i].is_some() { return None; } }}
        Some(Self { storage, len, _marker: crate::PhantomData })
    }

    /// Creates a buffer from an array of options by taking the prefix of `Some` values.
    ///
    /// The logical length is inferred as the index of the first `None`.
    /// Elements after the first `None` are ignored and need not be `None`.
    ///
    /// Returns `None` if a `None` appears before a `Some` in the prefix.
    pub fn from_array_prefix(storage: [Option<T>; CAP]) -> Option<Self> {
        let mut len = 0;
        crate::whilst! { i in 0..CAP; { if storage[i].is_some() { len += 1; } else { break; } }}
        Some(Self { storage, len, _marker: crate::PhantomData })
    }

    /// Creates a new buffer by cloning all the possible elements from `src`.
    pub fn from_slice_clone(src: &[T]) -> Option<Self> where T: Clone {
        if src.len() > CAP { return None; }
        let mut storage = [const { None }; CAP];
        crate::whilst! { i in 0..src.len(); { storage[i] = Some(src[i].clone()); }}
        Some(Self { storage, len: src.len(), _marker: crate::PhantomData })
    }

    /// Creates a new buffer by copying all the possible elements from `src`.
    pub const fn from_slice_copy(src: &[T]) -> Option<Self> where T: Copy {
        if src.len() > CAP { return None; }
        let mut storage = [const { None }; CAP];
        crate::whilst! { i  in 0..src.len(); { storage[i] = Some(src[i]); }}
        Some(Self { storage, len: src.len(), _marker: crate::PhantomData })
    }

    /* deconstruct */

    /// Clears the buffer, dropping all elements.
    pub fn clear(&mut self) {
        crate::whilst! { i in 0..self.len; { self.storage[i] = None; }}
        self.len = 0;
    }

    /// Truncates the buffer to `new_len`, dropping excess elements.
    pub fn truncate(&mut self, new_len: usize) {
        if new_len >= self.len { return; }
        crate::whilst! { i in new_len,..self.len; { self.storage[i] = None; }}
        self.len = new_len;
    }

    /* push */

    /// Appends a value to the back of the buffer.
    ///
    /// Returns `Err(value)` if the buffer is full.
    pub fn push_back(&mut self, value: T) -> Result<(), T> {
        if self.is_full() { return Err(value); }
        self.storage[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    /// Appends a copy of `value` to the back of the buffer.
    ///
    /// Returns `Err(value)` if the buffer is full.
    pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
        if self.is_full() { return Err(value); }
        self.storage[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    /* pop */

    /// Removes and returns a value from the back of the buffer.
    pub const fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        self.len -= 1;
        self.storage[self.len].take()
    }

    /* peek */

    /// Returns a reference to the last element without removing it.
    pub const fn peek_back(&self) -> Option<&T> {
        if self.is_empty() { return None; }
        self.storage[self.len - 1].as_ref()
    }
    /// Returns a reference to the last element without removing it.
    pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
        if self.is_empty() { return None; }
        self.storage[self.len - 1].as_mut()
    }

    /* get */

    /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() { return None; }
        self.storage[index].as_ref()
    }

    /// Returns an exclusive reference to the element at `index`, or `None` if out of bounds.
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() { return None; }
        self.storage[index].as_mut()
    }

    /* swap */

    /// Removes and returns the value at `index`, filling the gap with the last element.
    ///
    /// Decrements `len`. Does not preserve order.
    pub fn swap_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len { return None; }
        let last = self.len - 1;
        self.len = last;
        if index == last {
            self.storage[last].take()
        } else {
            let value = self.storage[index].take();
            self.storage[index] = self.storage[last].take();
            value
        }
    }

    /// Removes and returns the value at `index`, filling the gap with the last element.
    ///
    /// Decrements `len`. Does not preserve order.
    pub const fn swap_remove_copy(&mut self, index: usize) -> Option<T> where T: Copy {
        if index >= self.len { return None; }
        let last = self.len - 1;
        self.len = last;
        if index == last {
            self.storage[last]
        } else {
            let value = self.storage[index];
            self.storage[index] = self.storage[last];
            value
        }
    }

    /* slice */

    /// Returns the initialized prefix as a slice.
    pub const fn as_slice(&self) -> &[Option<T>] {
        crate::Slice::range_to(&self.storage, self.len)
    }

    /// Returns the initialized prefix as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [Option<T>] {
        crate::Slice::range_to_mut(&mut self.storage, self.len)
    }
}

/* exclusive slice */

/// Buffer view over an exclusive slice.
///
/// # Invariants
/// - Elements are stored in `storage[0 .. len)`
/// - `len <= storage.len()`
///
/// # Notes
/// - This type does not own its elements
/// - Dropping or shrinking the buffer does not drop values
/// - Mutations affect the underlying slice
#[rustfmt::skip]
impl<'a, T> BufLine<'a, T, &'a mut [T]> {
    /// Returns the capacity of the underlying slice.
    pub const fn capacity(&self) -> usize { self.storage.len() }

    /// Returns `true` if the buffer has reached its capacity.
    pub const fn is_full(&self) -> bool { self.len == self.capacity() }

    /* construct */

    /// Creates a buffer over an exclusive slice with logical length 0.
    pub const fn new(storage: &'a mut [T]) -> Self {
        Self { storage, len: 0, _marker: crate::PhantomData }
    }

    /// Creates a buffer over an exclusive slice with an explicit length.
    ///
    /// # Panics
    /// Panics if `len > storage.len()`.
    pub const fn from_slice(storage: &'a mut [T], len: usize) -> Self {
        assert!(len <= storage.len());
        Self { storage, len, _marker: crate::PhantomData }
    }

    /* deconstruct */

    /// Sets the logical length to zero.
    ///
    /// Does not drop elements.
    pub const fn clear(&mut self) { self.len = 0; }

    /// Sets the logical length to `min(new_len, len)`.
    ///
    /// If `new_len >= len`, this is a no-op.
    pub const fn truncate(&mut self, new_len: usize) {
        if new_len <= self.len { self.len = new_len; }
    }

    /* push */

    /// Appends a value to the back of the buffer.
    ///
    /// Returns `Err(value)` if the buffer is full.
    pub fn push_back(&mut self, value: T) -> Result<(), T> {
        if self.is_full() { return Err(value); }
        self.storage[self.len] = value;
        self.len += 1;
        Ok(())
    }
    /// Appends a copy of `value` to the back of the buffer.
    ///
    /// Returns `Err(value)` if the buffer is full.
    pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
        if self.is_full() { return Err(value); }
        self.storage[self.len] = value;
        self.len += 1;
        Ok(())
    }

    /* pop */

    /// Removes and returns a cloned value from the back of the buffer.
    pub fn pop_back_clone(&mut self) -> Option<T> where T: Clone {
        if self.is_empty() { return None; }
        self.len -= 1;
        Some(self.storage[self.len].clone())
    }
    /// Removes and returns a copied value from the back of the buffer.
    pub const fn pop_back_copy(&mut self) -> Option<T> where T: Copy {
        if self.is_empty() { return None; }
        self.len -= 1;
        Some(self.storage[self.len])
    }

    /* peek */

    /// Returns a reference to the last element without removing it.
    pub const fn peek_back(&self) -> Option<&T> {
        if self.is_empty() { return None; }
        Some(&self.storage[self.len - 1])
    }
    /// Returns a reference to the last element without removing it.
    pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
        if self.is_empty() { return None; }
        Some(&mut self.storage[self.len - 1])
    }

    /* get */

    /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() { return None; }
        Some(&self.storage[index])
    }

    /// Returns an exclusive reference to the element at `index`, or `None` if out of bounds.
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() { return None; }
        Some(&mut self.storage[index])
    }

    /* slice */

    /// Returns the initialized prefix as a slice.
    pub const fn as_slice(&self) -> &[T] {
        crate::Slice::range_to(&self.storage, self.len)
    }

    /// Returns the initialized prefix as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        crate::Slice::range_to_mut(&mut self.storage, self.len)
    }
}

/* shared slice  */

/// Read-only buffer view over a shared slice.
///
/// # Invariants
/// - Elements are read from `storage[0 .. len)`
/// - `len <= storage.len()`
///
/// # Notes
/// - This type does not own its elements
/// - No mutation or removal operations are supported
/// - `len` limits the visible prefix
#[rustfmt::skip]
impl<'a, T> BufLine<'a, T, &'a [T]> {
    /// Returns the capacity of the underlying slice.
    pub const fn capacity(&self) -> usize { self.storage.len() }

    /// Returns `true` if the buffer has reached its capacity.
    pub const fn is_full(&self) -> bool { self.len == self.capacity() }

    /* construct */

    /// Creates a buffer over a shared slice with logical length 0.
    pub const fn new(storage: &'a [T]) -> Self {
        Self { storage, len: 0, _marker: crate::PhantomData }
    }

    /// Creates a buffer over a shared slice with an explicit length.
    ///
    /// # Panics
    /// Panics if `len > storage.len()`.
    pub const fn from_slice(storage: &'a [T], len: usize) -> Self {
        assert!(len <= storage.len());
        Self { storage, len, _marker: crate::PhantomData }
    }

    /* peek */

    /// Returns a reference to the last element without removing it.
    pub const fn peek_back(&self) -> Option<&T> {
        if self.is_empty() { return None; }
        Some(&self.storage[self.len - 1])
    }

    /* get */

    /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        Some(&self.storage[index])
    }

    /* slice */

    /// Returns the initialized prefix as a slice.
    pub const fn as_slice(&self) -> &[T] {
        crate::Slice::range_to(&self.storage, self.len)
    }
}
