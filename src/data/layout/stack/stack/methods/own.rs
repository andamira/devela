// devela::data::layout::stack::stack::methods_own
//
//! owning stack const operations, in compile-time
//! depends on T: Copy, S: Bare
//
// Algorithms are implemented in the unchecked versions by:
// - creating a copy of the inner array.
// - performing specific operations in it.
// - creating the new stack.
// - adjusting its length if necessary.
// - returning it.

use crate::{
    Bare, DataNotEnough, MismatchedCapacity, NotEnoughElements, NotEnoughSpace, Own, Stack, cswap,
    sf,
};

// helper macro to impl methods for a Stack with custom index size.
//
// $IDX : the index type. E.g. u8, usize
macro_rules! impl_stack {
    () => {
        impl_stack![own
            u8:"_stack_u8", u16:"_stack_u16", u32:"_stack_u32", usize:"_stack_usize"];
    };

    // general own_ ops
    //
    // $IDX : the index type. E.g. u8, usize
    // $cap:  the capability feature that enables the given implementation. E.g "_stack_u8".
    (own $( $IDX:ty: $cap:literal ),+) => {
        $(
            #[cfg(feature = $cap )]
            impl_stack![@own $IDX:$cap];
        )+
    };
    (@own $IDX:ty : $cap:literal) => { $crate::paste! {
        // T: Copy, S: Bare
        /// # Chainable *const* operations depending on `T: Copy`
        ///
        /// Every method is *const* and returns [`Own`][crate::Own]`<Self, V>`.
        impl<T: Copy, const CAP: usize> Stack<T, CAP, $IDX, Bare> {
            /* constructors */

            /// Returns an empty stack, allocated in the stack,
            /// copying `element` to fill the remaining free data.
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $IDX "::MAX`]"]
            /// or if `CAP > isize::MAX / size_of::<T>()`.
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::{Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 16> = Stack" $IDX:camel
                "::own_new(0).s_const_unwrap().s;"]
            /// ```
            pub const fn own_new(element: T) -> Own<Result<Self, MismatchedCapacity>, ()> {
                Own::empty(Self::new_copied(element))
            }

            /* clear */

            /// Clears the stack in compile-time.
            ///
            /// `( 1 2 3 -- )`
            /// # Examples
            /// ```
            #[doc = "# use devela::{Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 3> = Stack" $IDX:camel
                "::from_array_copy([1, 2, 3]).own_clear().s;"]
            /// assert![S.is_empty()];
            /// ```
            pub const fn own_clear(self) -> Own<Self, ()> {
                let mut sta = self;
                sta.len = 0;
                Own::empty(sta)
            }

            /* push */

            /// Pushes a new `element` to the top of the stack in compile-time.
            ///
            /// `( 1 -- 1 2 )`
            /// # Errors
            /// Returns `Own<S,`[`NotEnoughSpace`]`>` if the stack is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::own_new(0).s_const_unwrap().s"]
            ///     .own_push(1).v_assert_ok().s
            ///     .own_push(2).v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[1, 2]];
            /// assert![S.own_push(3).v.is_err_and(|e| matches![e, NotEnoughSpace])];
            /// ```
            pub const fn own_push(self, element: T) -> Own<Self, Result<(), NotEnoughSpace>> {
                if self.len as usize == CAP {
                    Own::new(self, Err(NotEnoughSpace(Some(1))))
                } else {
                    self.own_push_unchecked(element).v_const_wrap_ok()
                }
            }

            /// Pushes a new `element` to the top of the stack in compile-time, unchecked version.
            ///
            /// `( 1 -- 1 2 )`
            /// # Panics
            /// Panics if the stack is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::Stack" $IDX:camel ";"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::own_new(0).s_const_unwrap().s"]
            ///     .own_push_unchecked(1).s.own_push_unchecked(2).s;
            /// assert_eq![S.as_slice(), &[1, 2]];
            /// ```
            pub const fn own_push_unchecked(self, element: T) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                arr[self.len as usize] = element;
                let mut sta = Self::from_array_copy(arr);
                sta.len = self.len + 1;
                Own::empty(sta)
            }

            /* pop */

            /// Pops the top stack element in compile-time.
            ///
            /// `( 1 2 3 -- 1 2 )`
            /// # Errors
            /// Returns `Own<S,`[`NotEnoughElements`]`>` if the stack is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::{NotEnoughElements, Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Own<Stack" $IDX:camel
                "<i32, 3>, Result<i32, NotEnoughElements>> =\n"]
            #[doc = "    Stack" $IDX:camel "::from_array_copy([1, 2, 3]).own_pop();"]
            /// S.s_assert(|s| s.as_slice() == &[1, 2]).v_assert_eq(&Ok(3));
            /// ```
            pub const fn own_pop(self) -> Own<Self, Result<T, NotEnoughElements>> {
                if self.len == 0 {
                    Own::new(self, Err(NotEnoughElements(Some(1))))
                } else {
                    self.own_pop_unchecked().v_const_wrap_ok()
                }
            }

            /// Pops the top stack element in compile-time, unchecked version.
            ///
            /// `( 1 2 3 -- 1 2 )`
            /// # Panics
            /// Panics if the stack is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Own<Stack" $IDX:camel "<i32, 3>, i32> =\n"]
            #[doc = "    Stack" $IDX:camel "::from_array_copy([1, 2, 3]).own_pop_unchecked();"]
            /// S.s_assert(|s| s.as_slice() == &[1, 2]).v_assert_eq(&3);
            /// ```
            pub const fn own_pop_unchecked(self) -> Own<Self, T> {
                let arr = self.data.into_array_copy();
                let e = arr[self.len as usize - 1];
                let mut sta = Self::from_array_copy(arr);
                sta.len = self.len - 1;
                Own::new(sta, e)
            }

            /* peek */

            /// Peeks the top stack element in compile-time.
            ///
            /// `( 1 2 3 -- 1 2 )`
            /// # Errors
            /// Returns `Own<S,`[`NotEnoughElements`]`>` if the stack is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::{NotEnoughElements, Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Own<Stack" $IDX:camel
                "<i32, 3>, Result<i32, NotEnoughElements>> =\n"]
            #[doc = "   Stack" $IDX:camel "::from_array_copy([1, 2, 3]).own_peek();"]
            /// S.s_assert(|s| s.as_slice() == &[1, 2, 3]).v_assert_eq(&Ok(3));
            /// ```
            pub const fn own_peek(self) -> Own<Self, Result<T, NotEnoughElements>> {
                if self.len == 0 {
                    Own::new(self, Err(NotEnoughElements(Some(1))))
                } else {
                    self.own_peek_unchecked().v_const_wrap_ok()
                }
            }

            /// Peeks the top stack element in compile-time, unchecked version.
            ///
            /// `( 1 2 3 -- 1 2 )`
            /// # Panics
            /// Panics if the stack is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Own<Stack" $IDX:camel "<i32, 3>, i32> = Stack" $IDX:camel
                "::from_array_copy([1, 2, 3])"]
            ///     .own_peek_unchecked();
            /// S.s_assert(|s| s.as_slice() == &[1, 2, 3]).v_assert_eq(&3);
            /// ```
            pub const fn own_peek_unchecked(self) -> Own<Self, T> {
                let arr = self.data.into_array_copy();
                let e = arr[self.len as usize - 1];
                let sta = Self::from_array_copy(arr);
                Own::new(sta, e)
            }

            /* drop */

            /// Drops the top stack element in compile-time.
            ///
            /// `( 1 2 -- 1 )`
            /// # Errors
            /// Returns `Own<self,`[`NotEnoughElements`]`>` if the stack is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::from_array_copy([1, 2])"]
            ///     .own_drop().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[1]];
            ///
            #[doc = "const T: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::own_new(0).s_const_unwrap().s"]
            ///     .own_drop().v_assert_err().s;
            /// assert![T.is_empty()];
            /// ```
            pub const fn own_drop(self) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len == 0 {
                    Own::new(self, Err(NotEnoughElements(Some(1))))
                } else {
                    self.own_drop_unchecked().v_const_wrap_ok()
                }
            }
            /// Swaps the top two pair stack elements, unchecked version.
            ///
            /// `( 1 2 -- 1 )`
            /// # Panics
            /// Panics if the stack is empty.
            /// # Examples
            /// ```
            #[doc = "# use devela::Stack" $IDX:camel ";"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::from_array_copy([1, 2])"]
            ///     .own_drop_unchecked().s;
            /// assert_eq![S.as_slice(), &[1]];
            /// ```
            pub const fn own_drop_unchecked(self) -> Own<Self, ()> {
                let mut sta = self;
                sta.len -= 1;
                Own::empty(sta)
            }

            /// Drops the top `n` stack elements.
            ///
            /// `( 1 2 3 4 -- 1 )` for `n == 3`
            /// # Errors
            /// Returns `Own<self,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least `n` elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([1, 2, 3, 4])"]
            ///     .own_drop_n(3).v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[1]];
            /// ```
            pub const fn own_drop_n(self, n: $IDX) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len < n {
                    Own::new(self, Err(NotEnoughElements(Some(n as usize))))
                } else {
                    self.own_drop_n_unchecked(n).v_const_wrap_ok()
                }
            }
            /// Drops the top `n` stack elements, unchecked version.
            ///
            /// `( 1 2 3 4 -- 1 )` for `n == 3`
            /// # Panics
            /// Panics if the stack doesn't contain at least `n` elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([1, 2, 3, 4])"]
            ///     .own_drop_n_unchecked(3).s;
            /// assert_eq![S.as_slice(), &[1]];
            /// ```
            pub const fn own_drop_n_unchecked(self, n: $IDX) -> Own<Self, ()> {
                let mut sta = self;
                sta.len -= n;
                Own::empty(sta)
            }

            /* nip */

            /// Drops the next of stack element in compile-time.
            ///
            /// `( 1 2 -- 2 )`
            /// # Errors
            /// Returns `Own<self,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least 2 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::from_array_copy([1, 2])"]
            ///     .own_nip().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[2]];
            ///
            #[doc = "const T: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::own_new(0).s_const_unwrap().s"]
            ///     .own_push(1).v_assert_ok().s.own_nip().v_assert_err().s;
            /// assert_eq![T.as_slice(), &[1]];
            /// ```
            pub const fn own_nip(self) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len < 2 {
                    Own::new(self, Err(NotEnoughElements(Some(2))))
                } else {
                    self.own_nip_unchecked().v_const_wrap_ok()
                }
            }

            /// Drops the next of stack element in compile-time, unchecked version.
            ///
            /// `( 1 2 -- 2 )`
            /// # Panics
            /// Panics if the stack doesn't contain at least 2 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::from_array_copy([1, 2])"]
            ///     .own_nip_unchecked().s;
            /// assert_eq![S.as_slice(), &[2]];
            /// ```
            pub const fn own_nip_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                cswap![tmp: arr[self.len as usize - 2], arr[self.len as usize - 1]];
                let mut sta = Self::from_array_copy(arr);
                sta.len -= 1;
                Own::empty(sta)
            }

            /// Drops the pair of next stack elements.
            ///
            /// `( 1 2 3 4 -- 3 4 )`
            /// # Errors
            /// Returns `Own<self,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least 4 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([1, 2, 3, 4])"]
            ///     .own_nip2().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[3, 4]];
            /// ```
            pub const fn own_nip2(self) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len < 4 {
                    Own::new(self, Err(NotEnoughElements(Some(4))))
                } else {
                    self.own_nip2_unchecked().v_const_wrap_ok()
                }
            }

            /// Drops the pair of next stack elements, unchecked version.
            ///
            /// `( 1 2 3 4 -- 3 4 )`
            /// # Panics
            /// Panics if the stack doesn't contain at least 4 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([1, 2, 3, 4])"]
            ///     .own_nip2_unchecked().s;
            /// assert_eq![S.as_slice(), &[3, 4]];
            /// ```
            pub const fn own_nip2_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                cswap![tmp: arr[self.len as usize - 4], arr[self.len as usize - 2]];
                cswap![tmp: arr[self.len as usize - 3], arr[self.len as usize - 1]];
                let mut sta = Self::from_array_copy(arr);
                sta.len -= 2;
                Own::empty(sta)
            }

            /* swap */

            /// Swaps the top two stack elements in compile-time.
            ///
            /// `( 1 2 -- 2 1 )`
            /// # Errors
            /// Returns `Own<self,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least 2 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::from_array_copy([1, 2])"]
            ///     .own_swap().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[2, 1]];
            ///
            #[doc = "const T: Stack" $IDX:camel "<i32, 1> = Stack" $IDX:camel
                "::from_array_copy([1])"]
            ///     .own_swap().v_assert_err().s;
            /// assert_eq![T.as_slice(), &[1]];
            /// ```
            pub const fn own_swap(self) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len < 2 {
                    Own::new(self, Err(NotEnoughElements(Some(2))))
                } else {
                    self.own_swap_unchecked().v_const_wrap_ok()
                }
            }
            /// Swaps the top two stack elements in compile-time, unchecked version.
            ///
            /// `( 1 2 -- 2 1 )`
            /// # Panics
            /// Panics if the stack doesn't contain at least 2 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 2> = Stack" $IDX:camel
                "::from_array_copy([1, 2])"]
            ///     .own_swap_unchecked().s;
            /// assert_eq![S.as_slice(), &[2, 1]];
            /// ```
            pub const fn own_swap_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                cswap![tmp: arr[self.len as usize - 2], arr[self.len as usize - 1]];
                Own::empty(Self::from_array_copy(arr))
            }

            /// Swaps the top two pair stack elements in compile-time.
            ///
            /// `( 1 2 3 4 -- 3 4 1 2 )`
            /// # Errors
            /// Returns `Own<self,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least 4 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([1, 2, 3, 4])"]
            ///     .own_swap2().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[3, 4, 1, 2]];
            ///
            #[doc = "const T: Stack" $IDX:camel "<i32, 3> = Stack" $IDX:camel
                "::from_array_copy([1, 2, 3])"]
            ///     .own_swap2().v_assert_err().s;
            /// assert_eq![T.as_slice(), &[1, 2, 3]];
            /// ```
            pub const fn own_swap2(self) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len < 4 {
                    Own::new(self, Err(NotEnoughElements(Some(4))))
                } else {
                    self.own_swap2_unchecked().v_const_wrap_ok()
                }
            }
            /// Swaps the top two pair stack elements in compile-time, unchecked version.
            ///
            /// `( 1 2 3 4 -- 3 4 1 2 )`
            /// # Panics
            /// Panics if the stack doesn't contain at least 4 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([1, 2, 3, 4])"]
            ///     .own_swap2_unchecked().s;
            /// assert_eq![S.as_slice(), &[3, 4, 1, 2]];
            /// ```
            pub const fn own_swap2_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                cswap![tmp: arr[self.len as usize - 4], arr[self.len as usize - 2]];
                cswap![tmp: arr[self.len as usize - 3], arr[self.len as usize - 1]];
                Own::new(Self::from_array_copy(arr), ())
            }

            /* rot */

            /// Rotates the top three stack elements, clockwise.
            ///
            /// `( 1 2 3 -- 2 3 1 ) `
            /// # Errors
            /// Returns `Own<self,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least 3 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3])"]
            ///     .own_rot().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[0, 2, 3, 1]];
            /// ```
            pub const fn own_rot(self) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len < 3 {
                    Own::new(self, Err(NotEnoughElements(Some(3))))
                } else {
                    self.own_rot_unchecked().v_const_wrap_ok()
                }
            }

            /// Rotates the top three stack elements, clockwise, unchecked version.
            ///
            /// `( 1 2 3 -- 2 3 1 ) `
            /// # Panics
            /// Panics if the stack doesn’t contain at least 6 elements.
            /// if the stack doesn't contain at least 3 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3])"]
            ///     .own_rot_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 2, 3, 1]];
            /// ```
            // WAIT: [const_swap](https://github.com/rust-lang/rust/pull/134757)
            pub const fn own_rot_unchecked(self) -> Own<Self, ()> {
                let len = self.len as usize;
                let mut arr = self.data.into_array_copy();
                let [a, b, c] = [arr[len - 3], arr[len - 2], arr[len - 1]];
                arr[len - 3] = b;
                arr[len - 2] = c;
                arr[len - 1] = a;
                let sta = Self::from_array_copy(arr);
                Own::empty(sta)
            }

            /// Rotates the top three stack elements, counter-clockwise.
            ///
            /// `( 1 2 3 -- 3 1 2 ) `
            /// # Errors
            /// Returns `Own<S,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least 3 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3])"]
            ///     .own_rot_cc().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[0, 3, 1, 2]];
            /// ```
            pub const fn own_rot_cc(self) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len < 3 {
                    Own::new(self, Err(NotEnoughElements(Some(3))))
                } else {
                    self.own_rot_cc_unchecked().v_const_wrap_ok()
                }
            }
            /// Rotates the top three stack elements, counter-clockwise, unchecked version.
            ///
            /// `( 1 2 3 -- 3 1 2 ) `
            /// # Errors
            /// Returns `Own<S,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least 3 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3])"]
            ///     .own_rot_cc_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 3, 1, 2]];
            /// ```
            // WAIT: [const_swap](https://github.com/rust-lang/rust/pull/134757)
            pub const fn own_rot_cc_unchecked(self) -> Own<Self, ()> {
                let len = self.len as usize;
                let mut arr = self.data.into_array_copy();
                let [a, b, c] = [arr[len - 3], arr[len - 2], arr[len - 1]];
                arr[len - 3] = c;
                arr[len - 2] = a;
                arr[len - 1] = b;
                let sta = Self::from_array_copy(arr);
                Own::empty(sta)
            }

            /// Rotates the top six stack elements, clockwise, two times.
            ///
            /// `( 1 2 3 4 5 6 -- 3 4 5 6 1 2 ) `
            /// # Errors
            /// Returns `Own<self,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least 6 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 7> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3, 4, 5, 6])"]
            ///     .own_rot2().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[0, 3, 4, 5, 6, 1, 2]];
            /// ```
            pub const fn own_rot2(self) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len < 6 {
                    Own::new(self, Err(NotEnoughElements(Some(6))))
                } else {
                    self.own_rot2_unchecked().v_const_wrap_ok()
                }
            }
            /// Rotates the top six stack elements, clockwise, two times, unchecked version.
            ///
            /// `( 1 2 3 4 5 6 -- 3 4 5 6 1 2 ) `
            /// # Panics
            /// Panics if the stack doesn't contain at least 6 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 7> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3, 4, 5, 6])"]
            ///     .own_rot2_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 3, 4, 5, 6, 1, 2]];
            /// ```
            // WAIT: [const_swap](https://github.com/rust-lang/rust/pull/134757)
            pub const fn own_rot2_unchecked(self) -> Own<Self, ()> {
                let len = self.len as usize;
                let mut arr = self.data.into_array_copy();
                let [a, b, c, d, e, f] = sf! {[
                    arr[len - 6], arr[len - 5], arr[len - 4], arr[len - 3], arr[len - 2], arr[len - 1],
                ]};
                arr[len - 6] = c;
                arr[len - 5] = d;
                arr[len - 4] = e;
                arr[len - 3] = f;
                arr[len - 2] = a;
                arr[len - 1] = b;
                let sta = Self::from_array_copy(arr);
                Own::empty(sta)
            }

            /// Rotates the top six stack elements, counter-clockwise, two times.
            ///
            /// `( 1 2 3 4 5 6 -- 5 6 1 2 3 4 ) `
            /// # Errors
            /// Returns `Own<self,`[`NotEnoughElements`]`>`
            /// if the stack doesn't contain at least 6 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 7> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3, 4, 5, 6])"]
            ///     .own_rot2_cc().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[0, 5, 6, 1, 2, 3, 4]];
            /// ```
            pub const fn own_rot2_cc(self) -> Own<Self, Result<(), NotEnoughElements>> {
                if self.len < 6 {
                    Own::new(self, Err(NotEnoughElements(Some(6))))
                } else {
                    self.own_rot2_cc_unchecked().v_const_wrap_ok()
                }
            }
            /// Rotates the top six stack elements, counter-clockwise, two times, unchecked version.
            ///
            /// `( 1 2 3 4 5 6 -- 5 6 1 2 3 4 ) `
            /// # Panics
            /// Panics if the stack doesn't contain at least 6 elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 7> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3, 4, 5, 6])"]
            ///     .own_rot2_cc_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 5, 6, 1, 2, 3, 4]];
            /// ```
            // WAIT: [const_swap](https://github.com/rust-lang/rust/pull/134757)
            pub const fn own_rot2_cc_unchecked(self) -> Own<Self, ()> {
                let len = self.len as usize;
                let mut arr = self.data.into_array_copy();
                let [a, b, c, d, e, f] = sf! {[
                    arr[len - 6], arr[len - 5], arr[len - 4],
                    arr[len - 3], arr[len - 2], arr[len - 1]
                ]};
                arr[len - 6] = e;
                arr[len - 5] = f;
                arr[len - 4] = a;
                arr[len - 3] = b;
                arr[len - 2] = c;
                arr[len - 1] = d;
                let sta = Self::from_array_copy(arr);
                Own::empty(sta)
            }

            /* dup */

            /// Duplicates the top stack element.
            ///
            /// `( 1 -- 1 1 )`
            /// # Errors
            /// Returns `Own<self,`[`DataNotEnough::Elements`]`>` if the stack is empty or
            /// `Own<self,`[`DataNotEnough::Space`]`>` if the stack is full.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 3> = Stack" $IDX:camel
                "::own_new(0).s_const_unwrap().s"]
            ///     .own_push(1).s.own_dup().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[1, 1]];
            /// ```
            pub const fn own_dup(self) -> Own<Self, Result<(), DataNotEnough>> {
                if self.len == 0 {
                    Own::new(self, Err(DataNotEnough::Elements(Some(1))))
                } else if self.len as usize == CAP {
                    Own::new(self, Err(DataNotEnough::Space(Some(1))))
                } else {
                    self.own_dup_unchecked().v_const_wrap_ok()
                }
            }
            /// Duplicates the top stack element, unchecked version.
            ///
            /// `( 1 -- 1 1 )`
            /// # Panics
            /// Panics if the stack is either empty or full.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 3> = Stack" $IDX:camel
                "::own_new(0).s_const_unwrap().s"]
            ///     .own_push(1).s.own_dup_unchecked().s;
            /// assert_eq![S.as_slice(), &[1, 1]];
            /// ```
            pub const fn own_dup_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                arr[self.len as usize] = arr[self.len as usize - 1];
                let mut sta = Self::from_array_copy(arr);
                sta.len = self.len + 1;
                Own::empty(sta)
            }

            /// Duplicates the top stack pair of elements.
            ///
            /// `( 1 2 -- 1 2 1 2 )`
            /// # Errors
            /// Returns `Own<self,`[`DataNotEnough::Elements`]`>`
            /// if the stack doesn't have at least 2 elements,
            /// or `Own<self,`[`DataNotEnough::Space`]`>`
            /// if it doesn't have enough space for 2 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 6> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 0, 0, 0])"]
            ///     .own_drop_n(3).s.own_dup2().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[0, 1, 2, 1, 2]];
            /// ```
            pub const fn own_dup2(self) -> Own<Self, Result<(), DataNotEnough>> {
                if self.len < 2 {
                    Own::new(self, Err(DataNotEnough::Elements(Some(2))))
                } else if self.len as usize > CAP - 2 {
                    Own::new(self, Err(DataNotEnough::Space(Some(2))))
                } else {
                    self.own_dup2_unchecked().v_const_wrap_ok()
                }
            }
            /// Duplicates the top stack pair of elements, unchecked version.
            ///
            /// `( 1 2 -- 1 2 1 2 )`
            /// # Panics
            /// Panics if the stack doesn't have at least 2 elements,
            /// or if it doesn't have enough space for 2 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 6> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 0, 0, 0])"]
            ///     .own_drop_n(3).s.own_dup2_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 1, 2, 1, 2]];
            /// ```
            pub const fn own_dup2_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                let a = arr[self.len as usize - 2];
                let b = arr[self.len as usize - 1];
                arr[self.len as usize] = a;
                arr[self.len as usize + 1] = b;
                let mut sta = Self::from_array_copy(arr);
                sta.len = self.len + 2;
                Own::empty(sta)
            }

            /* over */

            /// Duplicates the next of stack element to the top.
            ///
            /// `( 1 2 -- 1 2 1 )`
            /// # Errors
            /// Returns `Own<self,`[`DataNotEnough::Elements`]`>`
            /// if the stack doesn't have at least 2 elements,
            /// or `Own<self,`[`DataNotEnough::Space`]`>`
            /// if it doesn't have enough space for 2 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 0])"]
            ///     .own_drop().s.own_over().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[0, 1, 2, 1]];
            /// ```
            pub const fn own_over(self) -> Own<Self, Result<(), DataNotEnough>> {
                if self.len < 2 {
                    Own::new(self, Err(DataNotEnough::Elements(Some(2))))
                } else if self.len as usize == CAP {
                    Own::new(self, Err(DataNotEnough::Space(Some(1))))
                } else {
                    self.own_over_unchecked().v_const_wrap_ok()
                }
            }
            /// Duplicates the next of stack element to the top.
            ///
            /// `( 1 2 -- 1 2 1 )`
            /// # Panics
            /// Panics if the stack doesn't have at least 2 elements,
            /// or if it doesn't have enough space for 2 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 0])"]
            ///     .own_drop().s.own_over_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 1, 2, 1]];
            /// ```
            pub const fn own_over_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                arr[self.len as usize] = arr[self.len as usize - 2];
                let mut sta = Self::from_array_copy(arr);
                sta.len = self.len + 1;
                Own::empty(sta)
            }

            /// Duplicates the next of stack pair of elements to the top.
            ///
            /// `( 1 2 3 4 -- 1 2 3 4 1 2 )`
            /// # Errors
            /// Returns `Own<self,`[`DataNotEnough::Elements`]`>`
            /// if the stack doesn't have at least 4 elements,
            /// or `Own<self,`[`DataNotEnough::Space`]`>`
            /// if it doesn't have enough space for 2 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 7> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3, 4, 0, 0])"]
            ///     .own_drop_n(2).s.own_over2().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[0, 1, 2, 3, 4, 1, 2]];
            /// ```
            pub const fn own_over2(self) -> Own<Self, Result<(), DataNotEnough>> {
                if self.len < 4 {
                    Own::new(self, Err(DataNotEnough::Elements(Some(4))))
                } else if CAP - (self.len as usize) < 2 {
                    Own::new(self, Err(DataNotEnough::Space(Some(2))))
                } else {
                    self.own_over2_unchecked().v_const_wrap_ok()
                }
            }
            /// Duplicates the next of stack pair of elements to the top.
            ///
            /// `( 1 2 3 4 -- 1 2 3 4 1 2 )`
            /// # Panics
            /// Panics if the stack doesn't have at least 4 elements,
            /// or if it doesn't have enough space for 2 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 7> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3, 4, 0, 0])"]
            ///     .own_drop_n(2).s.own_over2_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 1, 2, 3, 4, 1, 2]];
            /// ```
            pub const fn own_over2_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                let a = arr[self.len as usize - 4];
                let b = arr[self.len as usize - 3];
                arr[self.len as usize] = a;
                arr[self.len as usize + 1] = b;
                let mut sta = Self::from_array_copy(arr);
                sta.len = self.len + 2;
                Own::empty(sta)
            }

            /* tuck */

            /// Duplicates the top element before the next of stack element.
            ///
            /// `( 1 2 -- 2 1 2 )`
            /// # Errors
            /// Returns `Own<self,`[`DataNotEnough::Elements`]`>`
            /// if the stack doesn't have at least 2 elements,
            /// or `Own<self,`[`DataNotEnough::Space`]`>`
            /// if it doesn't have enough space for 1 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 0])"]
            ///     .own_drop().s.own_tuck().v_assert_ok().s;
            /// assert_eq![S.as_slice(), &[0, 2, 1, 2]];
            /// ```
            pub const fn own_tuck(self) -> Own<Self, Result<(), DataNotEnough>> {
                if self.len < 2 {
                    Own::new(self, Err(DataNotEnough::Elements(Some(2))))
                } else if self.len as usize == CAP {
                    Own::new(self, Err(DataNotEnough::Space(Some(1))))
                } else {
                    self.own_tuck_unchecked().v_const_wrap_ok()
                }
            }
            /// Duplicates the top element before the next of stack element, unchecked version.
            ///
            /// `( 1 2 -- 2 1 2 )`
            /// # Panics
            /// Panics if the stack doesn't have at least 2 elements,
            /// or if it doesn't have enough space for 1 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 4> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 0])"]
            ///     .own_drop().s.own_tuck_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 2, 1, 2]];
            /// ```
            pub const fn own_tuck_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                let a = arr[self.len as usize - 1];
                cswap![tmp: arr[self.len as usize - 2], arr[self.len as usize - 1]];
                arr[self.len as usize] = a;
                let mut sta = Self::from_array_copy(arr);
                sta.len = self.len + 1;
                Own::empty(sta)
            }

            /// Duplicates the top pair of elements before the next of stack pair of elements.
            ///
            /// `( 1 2 3 4 -- 3 4 1 2 3 4 )`
            /// # Errors
            /// Returns `Own<self,`[`DataNotEnough::Elements`]`>`
            /// if the stack doesn't have at least 4 elements,
            /// or `Own<self,`[`DataNotEnough::Space`]`>`
            /// if it doesn't have enough space for 2 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 7> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3, 4, 0, 0])"]
            ///     .own_drop_n(2).s.own_tuck2_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 3, 4, 1, 2, 3, 4]];
            /// ```
            pub const fn own_tuck2(self) -> Own<Self, Result<(), DataNotEnough>> {
                if self.len < 4 {
                    Own::new(self, Err(DataNotEnough::Elements(Some(4))))
                } else if CAP - (self.len as usize) < 2 {
                    Own::new(self, Err(DataNotEnough::Space(Some(2))))
                } else {
                    self.own_tuck2_unchecked().v_const_wrap_ok()
                }
            }
            /// Duplicates the top pair of elements before the next of stack pair of elements,
            /// unchecked version.
            ///
            /// `( 1 2 3 4 -- 3 4 1 2 3 4 )`
            /// # Panics
            /// Panics if the stack doesn't have at least 4 elements,
            /// or if it doesn't have enough space for 2 extra elements.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Own, Stack" $IDX:camel "};"]
            #[doc = "const S: Stack" $IDX:camel "<i32, 7> = Stack" $IDX:camel
                "::from_array_copy([0, 1, 2, 3, 4, 0, 0])"]
            ///     .own_drop_n(2).s.own_tuck2_unchecked().s;
            /// assert_eq![S.as_slice(), &[0, 3, 4, 1, 2, 3, 4]];
            /// ```
            pub const fn own_tuck2_unchecked(self) -> Own<Self, ()> {
                let mut arr = self.data.into_array_copy();
                // swap2
                cswap![tmp: arr[self.len as usize - 4], arr[self.len as usize - 2]];
                cswap![tmp: arr[self.len as usize - 3], arr[self.len as usize - 1]];
                // over2
                let a = arr[self.len as usize - 4];
                let b = arr[self.len as usize - 3];
                arr[self.len as usize] = a;
                arr[self.len as usize + 1] = b;
                let mut sta = Self::from_array_copy(arr);
                sta.len = self.len + 2;
                Own::empty(sta)
            }
        }
    }};
}
impl_stack!();
