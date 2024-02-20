// devela::data::collections::stack::methods::conversion
//
//!
//

use crate::{
    code::{sf, ConstDefault},
    data::{
        error::{DataError, DataResult as Result},
        {array_init, Array, Comparing, Stack},
    },
    mem::{cswap, Bare, Storage},
    result::Own,
};
#[cfg(feature = "alloc")]
use crate::{
    data::Vec,
    mem::{Box, Boxed},
};
use DataError::{NotEnoughElements, NotEnoughSpace, OutOfBounds};

// helper macro to impl methods for a Stack with custom index size.
//
// $IDX : the index type. E.g. u8, usize
macro_rules! impl_stack {
    ($IDX:ty : $( $NEW_IDX:ty ),+ ) => { crate::code::paste! {
        /* resize */

        /// # Stack resize.
        // S: ()
        impl<T: Default, const CAP: usize> Stack<T, Bare, CAP, $IDX> {
            /// Converts the current stack to a different capacity while preserving all existing elements.
            ///
            /// This method creates a new stack with the specified new capacity and moves the
            /// current elements into it. The operation ensures that the new stack can accommodate
            /// the number of elements currently held in the stack. It is designed to work with
            /// both increases and decreases in capacity, as long as the new capacity can fit the
            /// current number of elements.
            ///
            /// # Errors
            /// Returns [`OutOfBounds(Some(NEW_CAP))`][OutOfBounds] if `NEW_CAP < self.len()`.
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Stack" $IDX:camel ";"]
            #[doc = "let s = Stack" $IDX:camel "::<_, (), 8>::from([1, 2, 3, 4]);"]
            #[doc = "let less_cap: Stack" $IDX:camel "::<_, (), 4> = s.resize_default().unwrap();"]
            /// assert_eq![s.as_slice(), less_cap.as_slice()];
            #[doc = "let more_cap: Stack" $IDX:camel "::<_, (), 12> = s.resize_default().unwrap();"]
            /// assert_eq![s.as_slice(), more_cap.as_slice()];
            /// assert![s.resize_default::<2>().is_err()]; // too small
            /// ```
            #[inline]
            pub fn resize_default<const NEW_CAP: usize>(self) -> Result<Stack<T, Bare, NEW_CAP, $IDX>> {
                if NEW_CAP < (self.len() as usize) {
                    Err(OutOfBounds(Some(NEW_CAP)))
                } else {
                    let old_arr: [T; CAP] = self.array.into_array();
                    let mut new_arr = array_init![default [T; NEW_CAP], "safe_data", "unsafe_array"];
                    for (i, item) in old_arr.into_iter().enumerate().take(NEW_CAP) {
                        new_arr[i] = item;
                    }
                    Ok(Stack {
                        array: Array::new(new_arr),
                        len: self.len,
                    })
                }
            }
            /// Converts the current stack to a different capacity, dropping elements if needed.
            ///
            /// This method creates a new stack with the specified new capacity and moves the
            /// current elements into it. The operation will drop any elements that can't fit
            /// in the new capacity, starting with the first ones (from the front of the stack).
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Stack" $IDX:camel ";"]
            #[doc = "let s = Stack" $IDX:camel "::<_, (), 8>::from([1, 2, 3, 4]);"]
            #[doc = "let less_cap: Stack" $IDX:camel "::<_, (), 4> = s.resize_default_truncate();"]
            /// assert_eq![less_cap.as_slice(), s.as_slice()];
            #[doc = "let more_cap: Stack" $IDX:camel "::<_, (), 12> = s.resize_default_truncate();"]
            /// assert_eq![more_cap.as_slice(), s.as_slice()];
            #[doc = "let drop_cap: Stack" $IDX:camel "::<_, (), 2> = s.resize_default_truncate();"]
            /// assert_eq![drop_cap.as_slice(), &[3, 4]];
            /// ```
            #[inline]
            pub fn resize_default_truncate<const NEW_CAP: usize>(self) -> Stack<T, Bare, NEW_CAP, $IDX> {
                let start_idx = if self.len() as usize > NEW_CAP {
                    self.len() as usize - NEW_CAP
                } else {
                    0
                };
                let new_len = core::cmp::min(self.len(), NEW_CAP as $IDX);
                let old_arr: [T; CAP] = self.array.into_array();
                let mut new_arr = array_init![default [T; NEW_CAP], "safe_data", "unsafe_array"];
                for (i, item) in old_arr.into_iter().enumerate().skip(start_idx).take(NEW_CAP) {
                    new_arr[i - start_idx] = item;
                }
                Stack {
                    array: Array::new(new_arr),
                    len: new_len,
                }
            }
        }

        // S: Boxed
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        impl<T: Default, const CAP: usize> Stack<T, Boxed, CAP, $IDX> {
            /// Converts the current stack to a different capacity while preserving all existing elements.
            ///
            /// This method creates a new stack with the specified new capacity and moves the
            /// current elements into it. The operation ensures that the new stack can accommodate
            /// the number of elements currently held in the stack. It is designed to work with
            /// both increases and decreases in capacity, as long as the new capacity can fit the
            /// current number of elements.
            ///
            /// # Errors
            /// Returns [`OutOfBounds(Some(NEW_CAP))`][OutOfBounds] if `NEW_CAP < self.len()`.
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::all::{Boxed, Stack" $IDX:camel "};"]
            #[doc = "let s = Stack" $IDX:camel "::<_, Boxed, 8>::from([1, 2, 3, 4]);"]
            #[doc = "let less_cap: Stack" $IDX:camel "::<_, Boxed, 4> = s.clone().resize_default().unwrap();"]
            /// assert_eq![s.as_slice(), less_cap.as_slice()];
            #[doc = "let more_cap: Stack" $IDX:camel "::<_, Boxed, 12> = s.clone().resize_default().unwrap();"]
            /// assert_eq![s.as_slice(), more_cap.as_slice()];
            /// assert![s.resize_default::<2>().is_err()]; // too small
            /// ```
            #[inline]
            pub fn resize_default<const NEW_CAP: usize>(self) -> Result<Stack<T, Boxed, NEW_CAP, $IDX>> {
                if NEW_CAP < (self.len as usize) {
                    Err(OutOfBounds(Some(NEW_CAP)))
                } else {
                    let old_arr = self.array.into_vec();
                    let mut v = Vec::with_capacity(NEW_CAP);
                    for item in old_arr.into_iter().take(self.len as usize) {
                        v.push(item);
                    }
                    v.resize_with(NEW_CAP, Default::default);

                    let new_arr: Box<[T; NEW_CAP]> = v.into_boxed_slice().try_into().unwrap_or_else(|_| {
                        panic!("Failed to convert Box<[T]> to Box<[T; NEW_CAP={}]>", NEW_CAP)
                    });
                    Ok(Stack {
                        array: Array::new_boxed(new_arr),
                        len: self.len,
                    })
                }
            }
            /// Converts the current stack to a different capacity while preserving all existing elements.
            ///
            /// This method creates a new stack with the specified new capacity and moves the
            /// current elements into it. The operation will drop any elements that can't fit
            /// in the new capacity, starting with the first ones (from the front of the stack).
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::all::{Boxed, Stack" $IDX:camel "};"]
            #[doc = "let s = Stack" $IDX:camel "::<_, Boxed, 8>::from([1, 2, 3, 4]);"]
            #[doc = "let less_cap: Stack" $IDX:camel "::<_, Boxed, 4> = s.clone().resize_default_truncate();"]
            /// assert_eq![less_cap.as_slice(), s.as_slice()];
            #[doc = "let more_cap: Stack" $IDX:camel "::<_, Boxed, 12> = s.clone().resize_default_truncate();"]
            /// assert_eq![more_cap.as_slice(), s.as_slice()];
            #[doc = "let drop_cap: Stack" $IDX:camel "::<_, Boxed, 2> = s.resize_default_truncate();"]
            /// assert_eq![drop_cap.as_slice(), &[3, 4]];
            /// ```
            #[inline]
            pub fn resize_default_truncate<const NEW_CAP: usize>(self) -> Stack<T, Boxed, NEW_CAP, $IDX> {
                let mut old_vec = self.array.into_vec();

                // When reducing capacity, truncate elements from the front.
                if NEW_CAP < self.len as usize {
                    let excess = self.len as usize - NEW_CAP;
                    old_vec.drain(0..excess);
                }
                // Ensure the vector's length matches NEW_CAP before conversion.
                old_vec.resize_with(NEW_CAP, Default::default);

                let new_arr: Box<[T; NEW_CAP]> = old_vec.into_boxed_slice().try_into().unwrap_or_else(|_| {
                    panic!("Failed to convert Box<[T]> to Box<[T; NEW_CAP={}]>", NEW_CAP)
                });
                Stack {
                    array: Array::new_boxed(new_arr),
                    len: core::cmp::min(self.len as usize, NEW_CAP) as $IDX,
                }
            }
        }

        // S: ()
        impl<T: ConstDefault + Copy, const CAP: usize> Stack<T, Bare, CAP, $IDX> {

            /// Converts the current stack to a different capacity while preserving all existing elements.
            ///
            /// This method creates a new stack with the specified new capacity and moves the
            /// current elements into it. The operation ensures that the new stack can accommodate
            /// the number of elements currently held in the stack. It is designed to work with
            /// both increases and decreases in capacity, as long as the new capacity can fit the
            /// current number of elements.
            ///
            /// # Errors
            /// Returns [`OutOfBounds(Some(NEW_CAP))`][OutOfBounds] if `NEW_CAP < self.len()`.
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Stack" $IDX:camel ";"]
            #[doc = "const S: Stack" $IDX:camel "<i32, (), 8> = Stack" $IDX:camel "::own_new(0)"]
            ///     .s_const_unwrap().s.own_push(1).s.own_push(2).s.own_push(3).s;
            #[doc = "const T: Stack" $IDX:camel "<i32, (), 4> = S.own_resize_default().s_const_unwrap().s;"]
            /// assert_eq![S.as_slice(), T.as_slice()];
            /// let _ = S.own_resize_default::<2>().s_assert_err(); // too small
            /// ```
            #[inline]
            pub const fn own_resize_default<const NEW_CAP: usize>(self) -> Own<Result<Stack<T, Bare, NEW_CAP, $IDX>>, ()> {
                if NEW_CAP < (self.len as usize) {
                    Own::empty(Err(OutOfBounds(Some(NEW_CAP))))
                } else {
                    let old_arr: [T; CAP] = self.array.into_array_const();
                    let mut new_arr = array_init![const_default [T; NEW_CAP]];

                    let mut i = 0;
                    while i < self.len as usize {
                        new_arr[i] = old_arr[i];
                        i += 1;
                    }

                    Own::empty(Ok(Stack {
                        array: Array::new_const(new_arr),
                        len: self.len,
                    }))
                }
            }
            /// Converts the current stack to a different capacity, dropping elements if needed.
            ///
            /// This method creates a new stack with the specified new capacity and moves the
            /// current elements into it. The operation will drop any elements that can't fit
            /// in the new capacity, starting with the first ones (from the front of the stack).
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Stack" $IDX:camel ";"]
            #[doc = "const S: Stack" $IDX:camel "<i32, (), 8> = Stack" $IDX:camel "::own_new(0)"]
            ///     .s_const_unwrap().s.own_push(1).s.own_push(2).s.own_push(3).s;
            #[doc = "const T: Stack" $IDX:camel "<i32, (), 4> = S.own_resize_default_truncate().s;"]
            /// assert_eq![S.as_slice(), T.as_slice()];
            #[doc = "const U: Stack" $IDX:camel "<i32, (), 2> = S.own_resize_default_truncate().s;"]
            /// assert_eq![U.as_slice(), &[2, 3]];
            /// ```
            #[inline]
            pub const fn own_resize_default_truncate<const NEW_CAP: usize>(self)
                -> Own<Stack<T, Bare, NEW_CAP, $IDX>, ()> {
                let start_idx = if self.len as usize > NEW_CAP {
                    self.len as usize - NEW_CAP
                } else {
                    0
                };
                let new_len = Comparing(NEW_CAP).min(self.len as usize);
                let old_arr: [T; CAP] = self.array.into_array_const();
                let mut new_arr = array_init![const_default [T; NEW_CAP]];

                let mut i = 0;
                while i < new_len {
                    new_arr[i] = old_arr[i + start_idx];
                    i += 1;
                }

                Own::empty(Stack {
                    array: Array::new_const(new_arr),
                    len: new_len as $IDX,
                })
            }
        }

        /* convert */

        // S: Bare
        /// # Stack index-size conversion.
        impl<T, const CAP: usize> Stack<T, Bare, CAP, $IDX> {
            $(
            /// Converts the current stack index size `IDX` to a `NEW_IDX`.
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `CAP > `[`" $IDX:camel "::MAX`]."]
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::data::*;"]
            #[doc = "let s = Stack" $IDX:camel "::<_, (), 6>::from([1, 2, 3, 4]);"]
            #[doc = "let t: Stack" $NEW_IDX:camel "::<_, (), 6> = s.to_idx_" $NEW_IDX "().unwrap();"]
            /// assert_eq![s.as_slice(), t.as_slice()];
            /// ```
            #[inline]
            pub fn [<to_idx_ $NEW_IDX>](self) -> Result<Stack<T, Bare, CAP, $NEW_IDX>> {
                if CAP <= $NEW_IDX::MAX as usize {
                    Ok(Stack { array: self.array, len: self.len as $NEW_IDX })
                } else {
                    Err(NotEnoughSpace(Some($NEW_IDX::MAX as usize - CAP)))
                }
            }
            )+
        }
        // S: Boxed
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        impl<T, const CAP: usize> Stack<T, Boxed, CAP, $IDX> {
            $(
            /// Converts the current stack index size `IDX` to a `NEW_IDX`.
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `CAP > `[`" $IDX:camel "::MAX`]."]
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::all::*;"]
            #[doc = "let s = Stack" $IDX:camel "::<_, Boxed, 6>::from([1, 2, 3]);"]
            #[doc = "let t: Stack" $NEW_IDX:camel "::<_, Boxed, 6> = s.to_idx_" $NEW_IDX "().unwrap();"]
            /// assert_eq![t.as_slice(), &[1, 2, 3]];
            /// ```
            #[inline]
            pub fn [<to_idx_ $NEW_IDX>](self) -> Result<Stack<T, Boxed, CAP, $NEW_IDX>> {
                if CAP <= $NEW_IDX::MAX as usize {
                    Ok(Stack { array: self.array, len: self.len as $NEW_IDX })
                } else {
                    Err(NotEnoughSpace(Some($NEW_IDX::MAX as usize - CAP)))
                }
            }
            )+
        }
        // S: Bare, T: Copy
        impl<T: Copy, const CAP: usize> Stack<T, Bare, CAP, $IDX> {
            $(
            /// Converts the current stack index size `IDX` to a `NEW_IDX`.
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `CAP > `[`" $IDX:camel "::MAX`]."]
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::data::*;"]
            #[doc = "const S: Stack" $IDX:camel "<i32, (), 6> = Stack" $IDX:camel "::own_new(0)"]
            ///     .s_const_unwrap().s.own_push(1).s.own_push(2).s.own_push(3).s;
            #[doc = "const T: Stack" $NEW_IDX:camel "<i32, (), 6> = S.own_to_idx_"
                $NEW_IDX "().s_const_unwrap().s;"]
            /// assert_eq![S.as_slice(), T.as_slice()];
            /// ```
            #[inline]
            pub const fn [<own_to_idx_ $NEW_IDX>](self) -> Own<Result<Stack<T, Bare, CAP, $NEW_IDX>>, ()> {
                if CAP <= $NEW_IDX::MAX as usize {
                    Own::empty(Ok(Stack { array: self.array, len: self.len as $NEW_IDX }))
                } else {
                    Own::empty(Err(NotEnoughSpace(Some($NEW_IDX::MAX as usize - CAP))))
                }
            }
            )+
        }
    }};
}
impl_stack![u8: u8, u16, u32, usize];
impl_stack![u16: u8, u16, u32, usize];
impl_stack![u32: u8, u16, u32, usize];
impl_stack![usize: u8, u16, u32, usize];
