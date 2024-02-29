// devela::data::collections::destaque::method
//
//! Double-ended queue + double-ended stack.
//

#[cfg(feature = "alloc")]
use crate::{
    _deps::alloc::{vec, vec::Vec},
    mem::Boxed,
};
use crate::{
    data::{
        error::{DataError, DataResult as Result},
        {array_init, Array, Destaque, DestaqueIter},
    },
    mem::{Bare, Storage},
};
use DataError::{NotEnoughElements, NotEnoughSpace, OutOfBounds};
// IMPROVE use array_init
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
use core::mem::{transmute_copy, MaybeUninit};

// helper macro to impl methods for a Destque with custom index size.
macro_rules! impl_destaque {
    () => {
        impl_destaque![u8];
        impl_destaque![u16];
        impl_destaque![u32];
        impl_destaque![usize];
    };

    // $IDX : the index type. E.g. u8, usize
    ( $IDX:ty ) => { crate::code::paste! {
        /* constructors */

        #[doc = "# Methods for `Destaque" $IDX:camel "`\n\n"]
        /// --------------------------------------------------
        /// --------------------------------------------------
        impl<T: Clone, const CAP: usize> Destaque<T, CAP, $IDX, Bare> {}

        // S:Bare + T:Clone
        impl<T: Clone, const CAP: usize> Destaque<T, CAP, $IDX, Bare> {
            /// Returns an empty destaque, allocated in the stack,
            /// cloning `element` to fill the remaining free data.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let q = Destaque" $IDX:camel "::<_, 16>::new(0).unwrap();"]
            /// ```
            pub fn new(element: T) -> Result<Self> {
                if CAP <= $IDX::MAX as usize {
                    Ok(Self {
                        array: Array::<T, Bare, CAP>::with_cloned(element),
                        front: 0,
                        back: 0,
                        len: 0,
                    })
                } else {
                    Err(OutOfBounds(Some(CAP)))
                }
            }
        }

        // S:Bare + T:Copy
        impl<T: Copy, const CAP: usize> Destaque<T, CAP, $IDX, Bare> {
            /// Returns an empty destaque, allocated in the stack,
            /// copying `element` to fill the remaining free data, in compile-time.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::{Destaque" $IDX:camel ", unwrap};"]
            #[doc = "const S: Destaque" $IDX:camel
                "<i32, 16> = unwrap![ok Destaque" $IDX:camel "::new_copied(0)];"]
            /// ```
            pub const fn new_copied(element: T) -> Result<Self> {
                if CAP <= $IDX::MAX as usize {
                    let array = Array::with_copied(element);
                    Ok(Self { array, front: 0, back: 0, len: 0 })
                } else {
                    Err(OutOfBounds(Some(CAP)))
                }
            }
        }

        // S:Boxed + T:Clone
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        impl<T: Clone, const CAP: usize> Destaque<T, CAP, $IDX, Boxed> {
            /// Returns an empty destaque, allocated in the heap,
            /// cloning `element` to fill the remaining free data.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::{Boxed, Destaque" $IDX:camel "};"]
            #[doc = "let q = Destaque" $IDX:camel "::<_, 3, Boxed>::new(0);"]
            /// ```
            pub fn new(element: T) -> Self {
                Self {
                    array: Array::<T, Boxed, CAP>::with_cloned(element),
                    front: 0,
                    back: 0,
                    len: 0,
                }
            }
        }

        // S:Bare
        impl<T, const CAP: usize> Destaque<T, CAP, $IDX, Bare> {
            /// Converts an array into a [`full`][Self::is_full] stack.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let q = Destaque" $IDX:camel "::<_, 3>::from_array([1, 2, 3]);"]
            /// ```
            pub const fn from_array_const(arr: [T; CAP]) -> Destaque<T, CAP, $IDX, Bare> {
                Self {
                    array: Array::new_bare(arr),
                    front: 0,
                    back: 0,
                    len: CAP as $IDX,
                }
            }
        }

        impl<T, const CAP: usize, S: Storage> Destaque<T, CAP, $IDX, S> {
            /// Converts an array into a [`full`][Self::is_full] destaque.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let q = Destaque" $IDX:camel "::<_, 3>::from_array([1, 2, 3]);"]
            /// ```
            pub fn from_array(arr: [T; CAP]) -> Destaque<T, CAP, $IDX, S> {
                Self {
                    array: Array::new(arr),
                    front: 0,
                    back: 0,
                    len: CAP as $IDX,
                }
            }

            /* queries */

            /// Returns the number of destaqued elements.
            #[inline]
            pub const fn len(&self) -> $IDX {
                self.len as $IDX
            }

            /// Returns `true` if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let q = Destaque" $IDX:camel "::<i32, 8>::default();"]
            /// assert![q.is_empty()];
            /// ```
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.len() == 0
            }

            /// Returns `true` if the destaque is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let q = Destaque" $IDX:camel "::<_, 3>::from([1, 2, 3]);"]
            /// assert![q.is_full()];
            /// ```
            #[inline]
            pub const fn is_full(&self) -> bool {
                self.len() as usize == CAP
            }

            /// Returns the destaque's total capacity.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let q = Destaque" $IDX:camel "::<i32, 3>::default();"]
            /// assert_eq![3, q.capacity()];
            /// ```
            #[inline]
            pub const fn capacity(&self) -> $IDX {
                CAP as $IDX
            }

            /// Returns the destaque's remaining capacity.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<i32, 3>::default();"]
            /// assert_eq![3, q.remaining_capacity()];
            /// q.push_back(1)?;
            /// assert_eq![2, q.remaining_capacity()];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn remaining_capacity(&self) -> $IDX {
                CAP as $IDX - self.len()
            }

            //

            /// Returns the destaque as pair of shared slices, which contain, in order,
            /// the contents of the destaque.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let q = Destaque" $IDX:camel "::<_, 3>::from([1, 2, 3]);"]
            /// assert_eq![q.as_slices(), (&[1, 2, 3][..], &[][..])];
            /// ```
            #[inline]
            pub fn as_slices(&self) -> (&[T], &[T]) {
                if self.len == 0 {
                    (&[], &[])
                } else if self.front < self.back {
                    // Non-wrap-around case
                    let slice = &self.array[self.front as usize..self.back as usize];
                    (slice, &[])
                } else {
                    // Wrap-around case
                    let first_slice = &self.array[self.front as usize..CAP];
                    let second_slice = &self.array[..self.back as usize];
                    (first_slice, second_slice)
                }
            }

            /// Returns `true` if the destaque is contiguous.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 3>::from([1, 2, 3]);"]
            /// assert_eq![q.as_slices(), (&[1, 2, 3][..], &[][..])];
            /// assert![q.is_contiguous()];
            /// q.pop_back()?;
            /// q.push_front(4)?;
            /// assert![!q.is_contiguous()];
            /// assert_eq![q.as_slices(), (&[4][..], &[1, 2][..])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn is_contiguous(&self) -> bool {
                (self.front == 0 && self.back == 0) || (self.front < self.back)
            }

            /* push */

            /// Pushes a new `element` to the front of the destaque.
            ///
            /// `( 1 2 -- 3 1 2 )`
            /// # Errors
            /// Returns [`NotEnoughSpace`] if the destaque is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 3>::default();"]
            /// q.push_front(1)?;
            /// q.push_front(2)?;
            /// q.push_front(3)?;
            /// assert_eq![q.to_array(), Some([3, 2, 1])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn push_front(&mut self, element: T) -> Result<()> {
                if self.is_full() {
                    Err(NotEnoughSpace(Some(1)))
                } else {
                    self.front = (self.front + CAP as $IDX - 1) % CAP as $IDX;
                    self.array[self.front as usize] = element;
                    self.len += 1;
                    Ok(())
                }
            }

            /// Unchecked version of [`push_front`][Self::push_front].
            /// # Panics
            /// Panics if the destaque is full.
            #[inline]
            pub fn push_front_unchecked(&mut self, element: T) {
                self.front = (self.front + CAP as $IDX - 1) % CAP as $IDX;
                self.array[self.front as usize] = element;
                self.len += 1;
            }

            /// Pushes a new `element` to the front of the destaque,
            /// overriding an element from the bacl if the destaque is full.
            ///
            /// Returns `true` if an element was overridden, and `false` otherwise.
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 3>::from([1, 2]);"]
            /// assert_eq!(q.push_front_override(3), false);
            /// assert_eq![q.to_array(), Some([3, 1, 2])];
            /// assert_eq!(q.push_front_override(4), true);
            /// assert_eq![q.to_array(), Some([4, 3, 1])];
            /// ```
            pub fn push_front_override(&mut self, element: T) -> bool {
                let overridden = self.is_full();
                if overridden {
                    // drop_back
                    self.back = (self.back + CAP as $IDX - 1) % CAP as $IDX;
                    self.len -= 1;
                }
                // push_front
                self.front = (self.front + CAP as $IDX - 1) % CAP as $IDX;
                self.array[self.front as usize] = element;
                self.len += 1;
                overridden
            }

            /// Pushes a new `element` to the back of the destaque.
            ///
            /// This is the habitual *[`enqueue`]* operation for a single-ended **queue**.
            ///
            /// `( 1 2 -- 1 2 3 )`
            /// # Errors
            /// Returns [`NotEnoughSpace`] if the destaque is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 3>::default();"]
            /// q.push_back(1)?;
            /// q.push_back(2)?;
            /// q.push_back(3)?;
            /// assert_eq![q.to_array(), Some([1, 2, 3])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn push_back(&mut self, element: T) -> Result<()> {
                if self.is_full() {
                    Err(NotEnoughSpace(Some(1)))
                } else {
                    Ok(self.push_back_unchecked(element))
                }
            }
            /// Alias of [`push_back`][Self::push_back].
            ///
            /// This is the habitual *enqueue* operation for a single-ended **queue**.
            #[inline]
            pub fn enqueue(&mut self, element: T) -> Result<()> {
                self.push_back(element)
            }

            /// Unchecked version of [`push_back`][Self::push_back].
            /// # Panics
            /// Panics if the destaque is full.
            #[inline]
            pub fn push_back_unchecked(&mut self, element: T) {
                self.array[self.back as usize] = element;
                self.back = (self.back + 1) % CAP as $IDX;
                self.len += 1;
            }

            /// Pushes a new `element` to the back of the destaque,
            /// overriding the first element if the destaque is full.
            ///
            /// Returns `true` if an element was overridden, and `false` otherwise.
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 3>::from([1, 2]);"]
            /// assert_eq!(q.push_back_override(3), false);
            /// assert_eq![q.to_array(), Some([1, 2, 3])];
            /// assert_eq!(q.push_back_override(4), true);
            /// assert_eq![q.to_array(), Some([2, 3, 4])];
            /// ```
            pub fn push_back_override(&mut self, element: T) -> bool {
                let overridden = self.is_full();
                if overridden {
                    // drop_front
                    self.front = (self.front + 1) % CAP as $IDX;
                    self.len -= 1;
                }
                // push_back
                self.array[self.back as usize] = element;
                self.back = (self.back + 1) % CAP as $IDX;
                self.len += 1;
                overridden
            }

            /* pop */

            /// Pops the front element.
            ///
            /// This is the habitual *dequeue* operation for a signle-ended **queue**.
            ///
            /// `( 1 2 -- 2 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the queue is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            ///
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![1, q.pop_front()?];
            /// assert_eq![2, q.pop_front()?];
            /// assert_eq![3, q.pop_front()?];
            /// assert![q.is_empty()];
            /// # Ok(()) }
            /// ```
            /// # Features
            /// It's depends on `T: Clone`, unless the `unsafe_ptr` feature is enabled.
            #[inline]
            #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "unsafe_ptr", Clone))))]
            pub fn pop_front(&mut self) -> Result<T> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    // MOTIVATION: to not depend on T: Clone
                    // SAFETY: we're not gonna access the value, but move it out
                    let e = unsafe {
                        core::ptr::read((self.array.get_unchecked(self.front as usize)) as *const T)
                    };
                    self.front = (self.front + 1) % CAP as $IDX;
                    self.len -= 1;
                    Ok(e)
                }
            }

            /// Alias of [`pop_front`][Self::pop_front].
            ///
            /// This is the habitual *dequeue* operation for a single-ended **queue**.
            #[inline]
            #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "unsafe_ptr", Clone))))]
            pub fn dequeue(&mut self) -> Result<T> {
                self.pop_front()
            }

            /// Pops the back element.
            ///
            /// `( 1 2-- 1 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![3, q.pop_back()?];
            /// assert_eq![2, q.pop_back()?];
            /// assert_eq![1, q.pop_back()?];
            /// assert![q.is_empty()];
            /// # Ok(()) }
            /// ```
            /// # Features
            /// It's depends on `T: Clone`, unless the `unsafe_ptr` feature is enabled.
            #[inline]
            #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "unsafe_ptr", Clone))))]
            pub fn pop_back(&mut self) -> Result<T> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    self.back = (self.back + CAP as $IDX - 1) % CAP as $IDX;
                    // MOTIVATION: to not depend on T: Clone
                    // SAFETY: we're not gonna access the value, but move it out
                    let e = unsafe {
                        core::ptr::read((self.array.get_unchecked(self.back as usize)) as *const T)
                    };
                    self.len -= 1;
                    Ok(e)
                }
            }

            /* peek */

            /// Returns a shared reference to the back element.
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![&3, q.peek_back()?];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn peek_back(&self) -> Result<&T> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    let bi = self.idx_back(0);
                    Ok(&self.array[bi])
                }
            }

            /// Returns an exclusive reference to the back element.
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![&mut 3, q.peek_back_mut()?];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn peek_back_mut(&mut self) -> Result<&mut T> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    let bi = self.idx_back(0);
                    Ok(&mut self.array[bi])
                }
            }

            /// Returns a shared reference to the `nth` back element.
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least `nth` elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![&1, q.peek_nth_back(2)?];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn peek_nth_back(&self, nth: $IDX) -> Result<&T> {
                if self.len() <= nth {
                    Err(NotEnoughElements(Some(nth as usize)))
                } else {
                    let bi = self.idx_back(nth);
                    Ok(&self.array[bi])
                }
            }

            /// Returns an exclusive reference to the `nth` back element.
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least `nth` elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![&mut 1, q.peek_nth_back_mut(2)?];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn peek_nth_back_mut(&mut self, nth: $IDX) -> Result<&mut T> {
                if self.len() <= nth {
                    Err(NotEnoughElements(Some(nth as usize)))
                } else {
                    let bi = self.idx_back(nth);
                    Ok(&mut self.array[bi])
                }
            }

            /// Returns a shared reference to the front element.
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![&1, q.peek_front()?];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn peek_front(&self) -> Result<&T> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    let fi = self.idx_front(0);
                    Ok(&self.array[fi])
                }
            }

            /// Returns an exclusive reference to the front element.
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![&mut 1, q.peek_front_mut()?];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn peek_front_mut(&mut self) -> Result<&mut T> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    let fi = self.idx_front(0);
                    Ok(&mut self.array[fi])
                }
            }

            /// Returns a shared reference to the `nth` front element.
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least `nth` elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3, 4]);"]
            /// assert_eq![&3, q.peek_nth_front(2)?];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn peek_nth_front(&self, nth: $IDX) -> Result<&T> {
                if self.len() <= nth {
                    Err(NotEnoughElements(Some(nth as usize)))
                } else {
                    let bi = self.idx_front(nth);
                    Ok(&self.array[bi])
                }
            }

            /// Returns an exclusive reference to the `nth` front element.
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least `nth` elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3, 4]);"]
            /// assert_eq![&mut 3, q.peek_nth_front_mut(2)?];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn peek_nth_front_mut(&mut self, nth: $IDX) -> Result<&mut T> {
                if self.len() <= nth {
                    Err(NotEnoughElements(Some(nth as usize)))
                } else {
                    let bi = self.idx_front(nth);
                    Ok(&mut self.array[bi])
                }
            }

            /* clear */

            /// Clears the destaque.
            ///
            /// `( 1 2 -- )`
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3, 4]);"]
            /// q.clear();
            /// assert![q.is_empty()];
            /// ```
            pub fn clear(&mut self) {
                self.front = 0;
                self.back = 0;
                self.len = 0;
            }

            /* drop */

            /// Drops the back element.
            ///
            /// `( 1 2 -- 1 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2]);"]
            /// q.drop_back()?;
            /// assert_eq![q.to_array(), Some([1])];
            /// # Ok(()) }
            /// ```
            pub fn drop_back(&mut self) -> Result<()> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    self.back = (self.back + CAP as $IDX - 1) % CAP as $IDX;
                    self.len -= 1;
                    Ok(())
                }
            }

            /// Drops the front element.
            ///
            /// `( 1 2 -- 2 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2]);"]
            /// q.drop_front()?;
            /// assert_eq![q.to_array(), Some([2])];
            /// # Ok(()) }
            /// ```
            pub fn drop_front(&mut self) -> Result<()> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    self.front = (self.front as $IDX + 1) % CAP as $IDX;
                    self.len -= 1;
                    Ok(())
                }
            }

            /// Drops `n` elements from the back.
            ///
            /// `( 1 2 3 4 -- 1 )` for `n = 3`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't contain at least `nth` elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3, 4]);"]
            /// q.drop_n_back(3)?;
            /// assert_eq![q.to_array(), Some([1])];
            /// # Ok(()) }
            /// ```
            pub fn drop_n_back(&mut self, nth: $IDX) -> Result<()> {
                if self.len() <= nth {
                    Err(NotEnoughElements(Some(nth as usize)))
                } else {
                    self.back = (self.back + CAP as $IDX - nth) % CAP as $IDX;
                    self.len -= nth;
                    Ok(())
                }
            }

            /// Drops `n` elements from the front.
            ///
            /// `( 1 2 3 4 -- 4 )` for `n = 3`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't contain at least `nth` elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3, 4]);"]
            /// q.drop_n_front(3)?;
            /// assert_eq![q.to_array(), Some([4])];
            /// # Ok(()) }
            /// ```
            pub fn drop_n_front(&mut self, nth: $IDX) -> Result<()> {
                if self.len() <= nth {
                    Err(NotEnoughElements(Some(nth as usize)))
                } else {
                    self.front = (self.front + nth) % CAP as $IDX;
                    self.len -= nth;
                    Ok(())
                }
            }

            /* swap */

            /// Swaps the last two elements at the back of the destaque.
            ///
            /// `( 1 2 3 4 -- 1 2 4 3 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't contain at least 2 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 4>::from([1, 2, 3, 4]);"]
            /// q.swap_back();
            /// assert_eq![q.to_array(), Some([1, 2, 4, 3])];
            /// ```
            #[inline]
            pub fn swap_back(&mut self) -> Result<()> {
                if self.len() < 2 {
                    Err(NotEnoughElements(Some(2)))
                } else {
                    self.swap_back_unchecked();
                    Ok(())
                }
            }
            /// Unchecked version of [`swap_back`][Self::swap_back].
            /// # Panics
            /// Panics if the destaque doesn't contain at least 2 elements.
            #[inline]
            pub fn swap_back_unchecked(&mut self) {
                let bi0 = self.idx_back(0);
                let bi1 = self.idx_back(1);
                self.array.swap(bi0, bi1);
            }

            /// Swaps the first two elements at the front of the destaque.
            ///
            /// `( 1 2 3 4 -- 2 1 3 4 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't contain at least 2 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 4>::from([1, 2, 3, 4]);"]
            /// q.swap_front();
            /// assert_eq![q.to_array(), Some([2, 1, 3, 4])];
            /// ```
            #[inline]
            pub fn swap_front(&mut self) -> Result<()> {
                if self.len() < 2 {
                    Err(NotEnoughElements(Some(2)))
                } else {
                    self.swap_front_unchecked();
                    Ok(())
                }
            }
            /// Unchecked version of [`swap_front`][Self::swap_front].
            /// # Panics
            /// Panics if the destaque doesn't contain at least 2 elements.
            #[inline]
            pub fn swap_front_unchecked(&mut self) {
                let fi0 = self.idx_front(0);
                let fi1 = self.idx_front(1);
                self.array.swap(fi0, fi1);
            }

            /// Swaps the last two pairs of elements at the back of the destaque.
            ///
            /// `( 1 2 3 4 5 6 7 8 -- 1 2 3 4 7 8 5 6 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't contain at least 2 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 16>::from([1, 2, 3, 4, 5, 6, 7, 8]);"]
            /// q.swap2_back();
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4, 7, 8, 5, 6])];
            /// ```
            #[inline]
            pub fn swap2_back(&mut self) -> Result<()> {
                if self.len() < 4 {
                    Err(NotEnoughElements(Some(4)))
                } else {
                    self.swap2_back_unchecked();
                    Ok(())
                }
            }
            /// Unchecked version of [`swap2_back`][Self::swap2_back].
            /// # Panics
            /// Panics if the destaque doesn't contain at least 2 elements.
            pub fn swap2_back_unchecked(&mut self) {
                let bi0 = self.idx_back(0);
                let bi1 = self.idx_back(1);
                let bi2 = self.idx_back(2);
                let bi3 = self.idx_back(3);
                self.array.swap(bi1, bi3);
                self.array.swap(bi0, bi2);
            }

            /// Swaps the first two pairs of elements at the front of the destaque.
            /// `( 1 2 3 4 5 6 7 8 -- 3 4 1 2 5 6 7 8 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't contain at least 4 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 16>::from([1, 2, 3, 4, 5, 6, 7, 8]);"]
            /// q.swap2_front();
            /// assert_eq![q.to_array(), Some([3, 4, 1, 2, 5, 6, 7, 8])];
            /// ```
            #[inline]
            pub fn swap2_front(&mut self) -> Result<()> {
                if self.len() < 4 {
                    Err(NotEnoughElements(Some(4)))
                } else {
                    self.swap2_front_unchecked();
                    Ok(())
                }
            }
            /// Unchecked version of [`swap2_back`][Self::swap2_back].
            /// # Panics
            /// Panics if the destaque doesn't contain at least 2 elements.
            #[inline]
            pub fn swap2_front_unchecked(&mut self) {
                let fi0 = self.idx_front(0);
                let fi1 = self.idx_front(1);
                let fi2 = self.idx_front(2);
                let fi3 = self.idx_front(3);
                self.array.swap(fi1, fi3);
                self.array.swap(fi0, fi2);
            }

            /// Swaps the front and back elements.
            ///
            /// `( 1 2 3 4 -- 4 2 3 1 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't contain at least 2 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 6>::from([1, 2, 3, 4, 5]);"]
            /// q.swap_ends();
            /// assert_eq![q.to_array(), Some([5, 2, 3, 4, 1])];
            /// ```
            #[inline]
            pub fn swap_ends(&mut self) -> Result<()> {
                if self.len() < 2 {
                    Err(NotEnoughElements(Some(2)))
                } else {
                    let bi0 = self.idx_back(0);
                    let fi0 = self.idx_front(0);
                    self.array.swap(bi0, fi0);
                    Ok(())
                }
            }
            /// Swaps the front and back pairs of elements.
            ///
            /// `( 1 2 3 4 5 6 7 8 -- 7 8 3 4 5 6 1 2 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't contain at least 4 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 16>::from([1, 2, 3, 4, 5, 6, 7, 8]);"]
            /// q.swap2_ends();
            /// assert_eq![q.to_array(), Some([7, 8, 3, 4, 5, 6, 1, 2])];
            /// ```
            #[inline]
            pub fn swap2_ends(&mut self) -> Result<()> {
                if self.len() < 4 {
                    Err(NotEnoughElements(Some(4)))
                } else {
                    let bi0 = self.idx_back(0);
                    let bi1 = self.idx_back(1);
                    let fi0 = self.idx_front(0);
                    let fi1 = self.idx_front(1);
                    self.array.swap(bi0, fi1);
                    self.array.swap(bi1, fi0);
                    Ok(())
                }
            }

            /* rot */

            /// Rotates all the destaqued elements one place to the right.
            ///
            /// `( 1 2 3 4 --  4 1 2 3 )`
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<i32, 8>::from([2, 3]);"]
            /// q.push_front(1)?;
            /// q.push_back(4)?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4])];
            /// q.rot_right();
            /// assert_eq![q.to_array(), Some([4, 1, 2, 3])];
            /// # Ok(()) }
            /// ```
            pub fn rot_right(&mut self) {
                if !self.is_empty() {
                    // equivalent to: self.push_front(self.pop_back()?)?
                    self.back = (self.back + CAP as $IDX - 1) % CAP as $IDX;
                    self.front = (self.front + CAP as $IDX - 1) % CAP as $IDX;
                    self.array.swap(self.back as usize, self.front as usize);
                }
            }

            /// Rotates all the destaqued elements `n` places to the right.
            ///
            /// `( 1 2 3 4 --  2 3 4 1 )` for `n = 3`
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<i32, 8>::from([2, 3]);"]
            /// q.push_front(1)?;
            /// q.push_back(4)?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4])];
            /// q.rot_right_n(3);
            /// assert_eq![q.to_array(), Some([2, 3, 4, 1])];
            /// # Ok(()) }
            /// ```
            pub fn rot_right_n(&mut self, nth: $IDX) {
                // don't rotate more than necessary
                let nth = nth % self.len();
                for _ in 0..nth as usize {
                    self.back = (self.back + CAP as $IDX - 1) % CAP as $IDX;
                    self.front = (self.front + CAP as $IDX - 1) % CAP as $IDX;
                    self.array.swap(self.back as usize, self.front as usize);
                }
            }

            /// Rotates all the destaqued elements one place to the left.
            ///
            /// `( 1 2 3 4 --  2 3 4 1 )`
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<i32, 8>::from([2, 3]);"]
            /// q.push_front(1)?;
            /// q.push_back(4)?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4])];
            /// q.rot_left();
            /// assert_eq![q.to_array(), Some([2, 3, 4, 1])];
            /// # Ok(()) }
            /// ```
            pub fn rot_left(&mut self) {
                if !self.is_empty() {
                    // equivalent to: self.push_back(self.pop_front()?)?
                    self.array.swap(self.back as usize, self.front as usize);
                    self.back = (self.back + 1) % CAP as $IDX;
                    self.front = (self.front + 1) % CAP as $IDX;
                }
            }

            /// Rotates all the destaqued elements `n` places to the left.
            ///
            /// `( 1 2 3 4 --  4 1 2 3 )` for `nth = 3`
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<i32, 8>::from([2, 3]);"]
            /// q.push_front(1)?;
            /// q.push_back(4)?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4])];
            /// q.rot_left_n(3);
            /// assert_eq![q.to_array(), Some([4, 1, 2, 3])];
            /// # Ok(()) }
            /// ```
            pub fn rot_left_n(&mut self, nth: $IDX) {
                // don't rotate more than necessary
                let nth = nth as usize % self.len() as usize;
                for _ in 0..nth {
                    self.array.swap(self.back as usize, self.front as usize);
                    self.back = (self.back + 1) % CAP as $IDX;
                    self.front = (self.front + 1) % CAP as $IDX;
                }
            }
        }

        // T:Clone
        impl<T: Clone, const CAP: usize, S: Storage> Destaque<T, CAP, $IDX, S> {
            /// Pops the front element.
            ///
            /// `( 1 2 -- 2 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![1, q.pop_front()?];
            /// assert_eq![2, q.pop_front()?];
            /// assert_eq![3, q.pop_front()?];
            /// assert![q.is_empty()];
            /// # Ok(()) }
            /// ```
            #[inline]
            #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
            pub fn pop_front(&mut self) -> Result<T> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    let e = self.array[self.front as usize].clone();
                    self.front = (self.front + 1) % CAP as $IDX;
                    self.len -= 1;
                    Ok(e)
                }
            }
            /// Alias of [`pop_front`][Self::pop_front].
            ///
            /// This is the habitual *dequeue* operation for a single-ended **queue**.
            #[inline]
            #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
            pub fn dequeue(&mut self) -> Result<T> {
                self.pop_front()
            }

            /// Pops the back element.
            ///
            /// `( 1 2 -- 1 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 8>::from([1, 2, 3]);"]
            /// assert_eq![3, q.pop_back()?];
            /// assert_eq![2, q.pop_back()?];
            /// assert_eq![1, q.pop_back()?];
            /// assert![q.is_empty()];
            /// # Ok(()) }
            /// ```
            #[inline]
            #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
            // safe-only version that depends on T: Clone
            pub fn pop_back(&mut self) -> Result<T> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else {
                    self.back = (self.back + CAP as $IDX - 1) % CAP as $IDX;
                    let e = self.array[self.back as usize].clone();
                    self.len -= 1;
                    Ok(e)
                }
            }

            /* make_contiguous, to_vec, to_array */

            /// Makes the elements of the destaque contiguous, rearranging the elements
            /// so that they are in a single, continuous block starting from the front.
            ///
            /// This operation might rearrange the internal representation of the elements
            /// to ensure they are contiguous. It clones the default element provided during
            /// the destaque's construction to fill any gaps if necessary.
            ///
            /// Returns a mutable slice to the now contiguous elements.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            ///
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 5>::new(0).unwrap();"]
            /// q.push_back(1);
            /// q.push_back(2);
            /// q.push_front(5);
            /// assert_eq!(q.as_slices(), (&[5][..], &[1, 2][..]));
            ///
            /// assert_eq!(q.make_contiguous(0), &[5, 1, 2]);
            /// assert_eq!(q.as_slices(), (&[5, 1, 2][..], &[][..]));
            /// ```
            #[allow(clippy::needless_range_loop)]
            pub fn make_contiguous(&mut self, element: T) -> &mut [T] {
                // Early return if already contiguous or empty
                if self.is_contiguous() || self.len == 0 {
                    return &mut [];
                }

                // Create a temporary array filled with clones of the default_element
                let mut temp: [T; CAP] = core::array::from_fn(|_| element.clone());

                // IMPROVE: use the new array to construct the new self? BENCH

                // Rearrange elements into their correct positions in the temporary array
                for i in 0..self.len as usize {
                    let index = (self.front as usize + i) % CAP;
                    temp[i] = self.array[index as usize].clone(); // Clone from the current array to temp
                }

                // Move elements from temp back into self.array, now in a contiguous order
                // self.array[..self.len].copy_from_slice(&temp[..self.len]); // NOTE for Copy
                for i in 0..self.len as usize {
                    self.array[i] = temp[i].clone();
                }

                // Reset front and back to reflect the new contiguous layout
                self.front = 0;
                self.back = self.len;

                &mut self.array[..self.len as usize]
            }

            /// Returns the destaqued elements as a vector.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 5>::from([3, 4]);"]
            /// q.push_front(2)?;
            /// q.push_back(5)?;
            /// q.push_front(1)?;
            /// assert_eq![q.to_vec(), vec![1, 2, 3, 4, 5]];
            /// # Ok(()) }
            /// ```
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
            pub fn to_vec(&self) -> Vec<T> {
                if self.is_empty() {
                    vec![]
                } else {
                    let mut vec = Vec::with_capacity(self.len() as usize);
                    let mut index = self.front as usize;

                    // makes sure a full destaque is not rejected
                    vec.push(self.array[index].clone());
                    index = (index + 1) % CAP;

                    while index != self.back as usize {
                        vec.push(self.array[index].clone());
                        index = (index + 1) % CAP;
                    }
                    vec
                }
            }

            /// Returns some `LEN` destaqued elements as an array, or `None` if the destaque
            /// is empty, or there are not at least `LEN` elements.
            ///
            /// This is a non `alloc` alternative method to [`to_vec`][Self::to_vec].
            /// # Panics
            /// Panics if the new `LEN` sized array can't be allocated.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 5>::from([3, 4]);"]
            /// q.push_front(2)?;
            /// q.push_back(5)?;
            /// q.push_front(1)?;
            /// assert_eq![q.to_array::<5>(), Some([1, 2, 3, 4, 5])];
            /// # Ok(()) }
            /// ```
            /// # Features
            /// Makes use of the `unsafe_array` feature if enabled.
            pub fn to_array<const LEN: usize>(&self) -> Option<[T; LEN]> {
                // IMPROVE: use array_init
                // MAYBE return not option
                // TODO: improve from_iter
                // Some(Array::<T, LEN, S>::new())

                if self.is_empty() || LEN > self.len() as usize || LEN == 0 {
                    None
                } else {
                    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
                    let arr = {
                        let mut unarr: [MaybeUninit<T>; LEN] =
                            // SAFETY: we will initialize all the elements
                            unsafe { MaybeUninit::uninit().assume_init() };
                        for (n, i) in unarr.iter_mut().enumerate().take(LEN) {
                            let index = (self.front as usize + n) % CAP;
                            let _ = i.write(self.array[index].clone());
                        }
                        // SAFETY: we've initialized all the elements
                        unsafe { transmute_copy::<_, [T; LEN]>(&unarr) }
                    };

                    #[cfg(any(feature = "safe_data", not(feature = "unsafe_array")))]
                    let arr = core::array::from_fn(|n| {
                        let index = (self.front as usize + n) % CAP;
                        self.array[index].clone()
                    });

                    Some(arr)
                }
            }

            /* dup */

            /// Duplicates the back element at the back
            ///
            /// `( 1 2 -- 1 2 2 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty
            /// or [`NotEnoughSpace`] if it is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 4>::from([1, 2, 3]);"]
            /// q.dup_back()?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 3])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn dup_back(&mut self) -> Result<()> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else if self.is_full() {
                    Err(NotEnoughSpace(Some(1)))
                } else {
                    self.push_back_unchecked(self.peek_back()?.clone());
                    Ok(())
                }
            }

            /// Duplicates the front element at the front.
            ///
            /// `( 1 2 -- 1 1 2 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque is empty
            /// or [`NotEnoughSpace`] if it is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 4>::from([1, 2, 3]);"]
            /// q.dup_front()?;
            /// assert_eq![q.to_array(), Some([1, 1, 2, 3])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn dup_front(&mut self) -> Result<()> {
                if self.is_empty() {
                    Err(NotEnoughElements(Some(1)))
                } else if self.is_full() {
                    Err(NotEnoughSpace(Some(1)))
                } else {
                    self.push_front_unchecked(self.peek_front()?.clone());
                    Ok(())
                }
            }

            /// Duplicates the back pair of elements, at the back.
            ///
            /// `( 1 2 3 4 -- 1 2 3 4 3 4)`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 2 elements,
            /// or [`NotEnoughSpace`] if it doesn't have space for 2 additional elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 6>::from([1, 2, 3, 4]);"]
            /// q.dup2_back()?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4, 3, 4])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn dup2_back(&mut self) -> Result<()> {
                if self.len() < 2 {
                    Err(NotEnoughElements(Some(2)))
                } else if self.remaining_capacity() < 2 {
                    Err(NotEnoughSpace(Some(2)))
                } else {
                    let b0 = self.peek_back()?.clone();
                    let b1 = self.peek_nth_back(1)?.clone();
                    self.push_back_unchecked(b1);
                    self.push_back_unchecked(b0);
                    Ok(())
                }
            }

            /// Duplicates the front pair of elements, at the front.
            ///
            /// `( 1 2 3 4 -- 1 2 1 2 3 4)`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 2 elements,
            /// or [`NotEnoughSpace`] if it doesn't have space for 2 additional elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 6>::from([1, 2, 3, 4]);"]
            /// q.dup2_front()?;
            /// assert_eq![q.to_array(), Some([1, 2, 1, 2, 3, 4])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn dup2_front(&mut self) -> Result<()> {
                if self.len() < 2 {
                    Err(NotEnoughElements(Some(2)))
                } else if self.remaining_capacity() < 2 {
                    Err(NotEnoughSpace(Some(2)))
                } else {
                    let f0 = self.peek_front()?.clone();
                    let f1 = self.peek_nth_front(1)?.clone();
                    self.push_front_unchecked(f1);
                    self.push_front_unchecked(f0);
                    Ok(())
                }
            }

            /* over */

            /// Duplicates the second back element, at the back.
            ///
            /// `( 1 2 3 4 -- 1 2 3 4 3 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 2 elements,
            /// or [`NotEnoughSpace`] if it is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 7>::from([1, 2, 3, 4]);"]
            /// q.over_back()?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4, 3])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn over_back(&mut self) -> Result<()> {
                if self.len() < 2 {
                    Err(NotEnoughElements(Some(2)))
                } else if self.is_full() {
                    Err(NotEnoughSpace(Some(1)))
                } else {
                    self.push_back_unchecked(self.peek_nth_back(1)?.clone());
                    Ok(())
                }
            }

            /// Duplicates the second front element, at the front.
            ///
            /// `( 1 2 3 4 -- 2 1 2 3 4 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 2 elements,
            /// or [`NotEnoughSpace`] if it is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 7>::from([1, 2, 3, 4]);"]
            /// q.over_front()?;
            /// assert_eq![q.to_array(), Some([2, 1, 2, 3, 4])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn over_front(&mut self) -> Result<()> {
                if self.len() < 2 {
                    Err(NotEnoughElements(Some(2)))
                } else if self.is_full() {
                    Err(NotEnoughSpace(Some(1)))
                } else {
                    self.push_front_unchecked(self.peek_nth_front(1)?.clone());
                    Ok(())
                }
            }

            /// Duplicates the second back pair of elements, at the back.
            ///
            /// `( 1 2 3 4 5 6 7 8 -- 1 2 3 4 5 6 7 8 5 6 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 4 elements,
            /// or [`NotEnoughSpace`] if it doesn't have space for 2 additional elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 8>::from([1, 2, 3, 4, 5, 6]);"]
            /// q.over2_back()?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4, 5, 6, 3, 4])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn over2_back(&mut self) -> Result<()> {
                if self.len() < 4 {
                    Err(NotEnoughElements(Some(4)))
                } else if self.remaining_capacity() < 2 {
                    Err(NotEnoughSpace(Some(2)))
                } else {
                    let b2 = self.peek_nth_back(2)?.clone();
                    let b3 = self.peek_nth_back(3)?.clone();
                    self.push_back_unchecked(b3);
                    self.push_back_unchecked(b2);
                    Ok(())
                }
            }

            /// Duplicates the second front pair of elements, at the front.
            ///
            /// `( 1 2 3 4 5 6 7 8 -- 3 4 1 2 3 4 5 6 7 8 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 4 elements,
            /// or [`NotEnoughSpace`] if it doesn't have space for 2 additional elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 8>::from([1, 2, 3, 4, 5, 6]);"]
            /// q.over2_front()?;
            /// assert_eq![q.to_array(), Some([3, 4, 1, 2, 3, 4, 5, 6])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn over2_front(&mut self) -> Result<()> {
                if self.len() < 4 {
                    Err(NotEnoughElements(Some(4)))
                } else if self.remaining_capacity() < 2 {
                    Err(NotEnoughSpace(Some(2)))
                } else {
                    let f2 = self.peek_nth_front(2)?.clone();
                    let f3 = self.peek_nth_front(3)?.clone();
                    self.push_front_unchecked(f3);
                    self.push_front_unchecked(f2);
                    Ok(())
                }
            }

            /* tuck */

            /// Duplicates the back element, before the second back element.
            ///
            /// `( 1 2 3 4 -- 1 2 4 3 4 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 2 elements,
            /// or [`NotEnoughSpace`] if it is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            ///
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 7>::from([1, 2, 3, 4, 5]);"]
            /// q.tuck_back()?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 5, 4, 5])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn tuck_back(&mut self) -> Result<()> {
                if self.len() < 2 {
                    Err(NotEnoughElements(Some(2)))
                } else if self.is_full() {
                    Err(NotEnoughSpace(Some(1)))
                } else {
                    let b0 = self.peek_back()?.clone();
                    self.swap_back_unchecked();
                    self.push_back_unchecked(b0);
                    Ok(())
                }
            }

            /// Duplicates the front element, after the second front element.
            ///
            /// `( 1 2 3 4 -- 1 2 1 3 4 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 2 elements,
            /// or [`NotEnoughSpace`] if it is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 7>::from([1, 2, 3, 4, 5]);"]
            /// q.tuck_front()?;
            /// assert_eq![q.to_array(), Some([1, 2, 1, 3, 4, 5])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn tuck_front(&mut self) -> Result<()> {
                if self.len() < 2 {
                    Err(NotEnoughElements(Some(2)))
                } else if self.is_full() {
                    Err(NotEnoughSpace(Some(1)))
                } else {
                    let f0 = self.peek_front()?.clone();
                    self.swap_front_unchecked();
                    self.push_front_unchecked(f0);
                    Ok(())
                }
            }

            /// Duplicates the back pair of elements,
            /// before the second back pair of elements.
            ///
            /// `( 1 2 3 4 5 6 7 8 -- 1 2 3 4 7 8 5 6 7 8 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 4 elements,
            /// or [`NotEnoughSpace`] if it doesn't have space for 2 additional elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 7>::from([1, 2, 3, 4, 5]);"]
            /// q.tuck2_back()?;
            /// assert_eq![q.to_array(), Some([1, 4, 5, 2, 3, 4, 5])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn tuck2_back(&mut self) -> Result<()> {
                if self.len() < 4 {
                    Err(NotEnoughElements(Some(4)))
                } else if self.len() < 2 {
                    Err(NotEnoughSpace(Some(2)))
                } else {
                    let b0 = self.peek_nth_back(0)?.clone();
                    let b1 = self.peek_nth_back(1)?.clone();
                    self.swap2_back_unchecked();
                    self.push_back_unchecked(b1);
                    self.push_back_unchecked(b0);
                    Ok(())
                }
            }

            /// Duplicates the front pair of elements,
            /// after the second front pair of elements.
            ///
            /// `( 1 2 3 4 5 6 7 8 -- 1 2 3 4 1 2 5 6 7 8 )`
            /// # Errors
            /// Returns [`NotEnoughElements`] if the destaque doesn't have at least 4 elements,
            /// or [`NotEnoughSpace`] if it doesn't have space for 2 additional elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            /// # fn main() -> devela::data::DataResult<()> {
            #[doc = "let mut q = Destaque" $IDX:camel "::<u8, 7>::from([1, 2, 3, 4, 5]);"]
            /// q.tuck2_front()?;
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4, 1, 2, 5])];
            /// # Ok(()) }
            /// ```
            #[inline]
            pub fn tuck2_front(&mut self) -> Result<()> {
                if self.len() < 4 {
                    Err(NotEnoughElements(Some(4)))
                } else if self.len() < 2 {
                    Err(NotEnoughSpace(Some(2)))
                } else {
                    let f0 = self.peek_nth_front(0)?.clone();
                    let f1 = self.peek_nth_front(1)?.clone();
                    self.swap2_front_unchecked();
                    self.push_front_unchecked(f1);
                    self.push_front_unchecked(f0);
                    Ok(())
                }
            }
        }

        /* iterators */

        impl<T, const CAP: usize, S: Storage> Destaque<T, CAP, $IDX, S> {
            /// Returns an iterator.
            pub const fn iter(&self) -> DestaqueIter<'_, T, CAP, $IDX, S> {
                DestaqueIter {
                    destaque: self,
                    idx: 0,
                }
            }

            /* extend */

            /// Extends the back of the destaque from an iterator.
            ///
            /// `( 1 2 -- 1 2 3 4 5 6)` for `[3 4 5 6]`
            /// # Errors
            /// Returns [`NotEnoughSpace`] if the destaque becomes full before the iterator finishes.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 6>::from([1, 2, 3]);"]
            /// q.extend_back([4, 5, 6, 7, 8]);
            /// assert_eq![q.to_array(), Some([1, 2, 3, 4, 5, 6])];
            /// ```
            pub fn extend_back<I>(&mut self, iterator: I) -> Result<()>
            where
                I: IntoIterator<Item = T>,
            {
                let mut iter = iterator.into_iter();
                while !self.is_full() {
                    if let Some(e) = iter.next() {
                        self.push_back_unchecked(e);
                    } else {
                        return Ok(());
                    }
                }
                Err(NotEnoughSpace(None))
            }

            /// Extends the back of the destaque from an iterator,
            /// overriding elements from the front if the destaque is full.
            ///
            /// `( 1 2 3 -- 3 4 5 6)` for `[4 5 6]` and `CAP = 4`
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 4>::from([1, 2, 3]);"]
            /// assert_eq![q.extend_back_override([4, 5, 6]), true];
            /// assert_eq![q.to_array(), Some([3, 4, 5, 6])];
            /// ```
            pub fn extend_back_override<I>(&mut self, iterator: I) -> bool
            where
                I: IntoIterator<Item = T>,
            {
                let mut overriden = false;
                for element in iterator.into_iter() {
                    if self.is_full() {
                        overriden = true;
                        // drop_front
                        self.front = (self.front + 1) % CAP as $IDX;
                        self.len -= 1;
                    }
                    // push_back
                    self.array[self.back as usize] = element;
                    self.back = (self.back + 1) % CAP as $IDX;
                    self.len += 1;
                }
                overriden
            }

            /// Extends the front of the destaque from an iterator.
            ///
            /// `( 1 2 -- 6 5 4 3 1 2 )` for `[3 4 5 6]`
            /// # Errors
            /// Returns [`NotEnoughSpace`] if the destaque becomes full before the iterator finishes.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 6>::from([1, 2, 3]);"]
            /// q.extend_front([4, 5, 6, 7, 8]);
            /// assert_eq![q.to_array(), Some([6, 5, 4, 1, 2, 3])];
            /// ```
            pub fn extend_front<I>(&mut self, iterator: I) -> Result<()>
            where
                I: IntoIterator<Item = T>,
            {
                let mut iter = iterator.into_iter();
                while !self.is_full() {
                    if let Some(e) = iter.next() {
                        self.push_front_unchecked(e);
                    } else {
                        return Ok(());
                    }
                }
                Err(NotEnoughSpace(None))
            }

            /// Extends the front of the destaque from an iterator,
            /// overriding elements from the back if the destaque is full.
            ///
            /// `( 1 2 3 -- 6 5 4 1)` for `[4 5 6]` and `CAP = 4`
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<_, 4>::from([1, 2, 3]);"]
            /// assert_eq![q.extend_front_override([4, 5, 6]), true];
            /// assert_eq![q.to_array(), Some([6, 5, 4, 1])];
            /// ```
            pub fn extend_front_override<I>(&mut self, iterator: I) -> bool
            where
                I: IntoIterator<Item = T>,
            {
                let mut overriden = false;
                for element in iterator.into_iter() {
                    if self.is_full() {
                        overriden = true;
                        // drop_back
                        self.back = (self.back + CAP as $IDX - 1) % CAP as $IDX;
                        self.len -= 1;
                    }
                    // push_front
                    self.front = (self.front + CAP as $IDX - 1) % CAP as $IDX;
                    self.array[self.front as usize] = element;
                    self.len += 1;
                }
                overriden
            }

        }

        // T:PartialEq
        impl<T: PartialEq, const CAP: usize, S: Storage> Destaque<T, CAP, $IDX, S> {
            /// Returns true if the destaque contains `element`.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let q = Destaque" $IDX:camel "::<_, 6>::from([5, 78, 42, 33, 9]);"]
            ///
            /// assert![q.contains(&9)];
            /// assert![!q.contains(&8)];
            /// ```
            pub fn contains(&self, element: &T) -> bool {
                self.iter().any(|n| n == element)
            }
        }

        // TODO: drop_replace_default

        /* private helpers */

        impl<T, const CAP: usize, S: Storage> Destaque<T, CAP, $IDX, S> {
            // Returns the `nth` element's index counting from the back.
            #[inline]
            #[must_use]
            pub(super) const fn idx_back(&self, nth: $IDX) -> usize {
                (self.back as usize + CAP - nth as usize - 1) % CAP
            }
            // Returns the `nth` element's index counting from the front.
            #[inline]
            #[must_use]
            pub(super) const fn idx_front(&self, nth: $IDX) -> usize {
                (self.front as usize + nth as usize) % CAP
            }
        }
    }};
}
impl_destaque!();
