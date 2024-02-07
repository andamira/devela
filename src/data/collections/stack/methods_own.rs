// devela::data::methods_own
//
//! owning stack const operations, in compile-time
//! depends on S:(), T:Copy
//
// Algorithms are implemented in the unchecked versions by:
// - creating a copy of the inner array.
// - performing specific operations in it.
// - creating the new stack.
// - adjusting its length if necessary.
// - returning it.

use crate::{
    code::sf,
    data::{
        error::{DataErrors, DataResult as Result},
        {Array, Stack},
    },
    mem::{cswap, Storage},
    result::Own,
};
use DataErrors::{NotEnoughElements, NotEnoughSpace};

/// # Chainable *const* operations
///
/// Depends on `T` being `Copy`.
/// Every method is *const* and returns [`Own`][crate::Own]`<Self, V>`.
impl<T: Copy, const CAP: usize> Stack<T, (), CAP> {
    /* clear */

    /// Clears the stack in compile-time.
    ///
    /// `( 1 2 3 -- )`
    /// # Examples
    /// ```
    /// # use devela::data::Stack;
    /// const S: Stack<i32, (), 3> = Stack::from_array_const([1, 2, 3]).own_clear();
    /// assert![S.is_empty()];
    /// ```
    pub const fn own_clear(self) -> Self {
        let mut sta = self;
        sta.len = 0;
        sta
    }

    /* push */

    /// Pushes a new element to the top of the stack in compile-time.
    ///
    /// `( 1 -- 1 2 )`
    /// # Errors
    /// Returns `Own<S,`[`NotEnoughSpace`]`>` if the stack is full.
    /// # Examples
    /// ```
    /// # use devela::all::{DataResult, DataErrors, Own, Stack};
    /// const S: Stack<i32, (), 2> = Stack::new_copied(0)
    ///     .own_push(1).state_ok()
    ///     .own_push(2).state_ok()
    ///     .own_push(3).state_err();
    /// assert_eq![S.as_slice(), &[1, 2]];
    /// assert![S.own_push(3).value.is_err_and(|e| matches![e, DataErrors::NotEnoughSpace(_)])];
    /// ```
    #[inline]
    pub const fn own_push(self, e: T) -> Own<Self, Result<()>> {
        if self.len == CAP {
            Own::new(self, Err(NotEnoughSpace(Some(1))))
        } else {
            self.own_push_unchecked(e).into_result_const()
        }
    }

    /// Pushes a new element to the top of the stack in compile-time, unchecked version.
    ///
    /// `( 1 -- 1 2 )`
    /// # Panics
    /// Panics if the stack is full.
    /// # Examples
    /// ```
    /// # use devela::all::{DataResult, Stack};
    /// const S: Stack<i32, (), 2> =
    ///     Stack::new_copied(0).own_push_unchecked(1).state.own_push_unchecked(2).state;
    /// assert_eq![S.as_slice(), &[1, 2]];
    /// ```
    #[inline]
    pub const fn own_push_unchecked(self, e: T) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        arr[self.len] = e;
        let mut sta = Self::from_array_const(arr);
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
    /// # use devela::all::{DataResult, Own, Stack};
    /// const S: Own<Stack<i32, (), 3>, DataResult<i32>> = Stack::from_array_const([1, 2, 3]).own_pop();
    /// S.assert_state(|s| s.as_slice() == &[1, 2]).assert_eq_value(&Ok(3));
    /// ```
    #[inline]
    pub const fn own_pop(self) -> Own<Self, Result<T>> {
        if self.len == 0 {
            Own::new(self, Err(NotEnoughElements(Some(1))))
        } else {
            self.own_pop_unchecked().into_result_const()
        }
    }

    /// Pops the top stack element in compile-time, unchecked version.
    ///
    /// `( 1 2 3 -- 1 2 )`
    /// # Panics
    /// Panics if the stack is empty.
    /// # Examples
    /// ```
    /// # use devela::all::{DataResult, Own, Stack};
    /// const S: Own<Stack<i32, (), 3>, i32> = Stack::from_array_const([1, 2, 3]).own_pop_unchecked();
    /// assert_eq![S.state.as_slice(), &[1, 2]];
    /// assert_eq![S.value, 3];
    /// ```
    #[inline]
    pub const fn own_pop_unchecked(self) -> Own<Self, T> {
        let arr = self.array.into_array_const();
        let e = arr[self.len - 1];
        let mut sta = Self::from_array_const(arr);
        sta.len = self.len - 1;
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_drop().state_ok();
    /// assert_eq![S.as_slice(), &[1]];
    ///
    /// const T: Stack<i32, (), 2> = Stack::new_copied(0).own_drop().state_err();
    /// assert![T.is_empty()];
    /// ```
    #[inline]
    pub const fn own_drop(self) -> Own<Self, Result<()>> {
        if self.len == 0 {
            Own::new(self, Err(NotEnoughElements(Some(1))))
        } else {
            self.own_drop_unchecked().into_result_const()
        }
    }
    /// Swaps the top two pair stack elements, unchecked version.
    ///
    /// `( 1 2 -- 1 )`
    /// # Panics
    /// Panics if the stack is empty.
    /// # Examples
    /// ```
    /// # use devela::all::Stack;
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_drop_unchecked().state;
    /// assert_eq![S.as_slice(), &[1]];
    /// ```
    #[inline]
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([1, 2, 3, 4]).own_drop_n(3).state_ok();
    /// assert_eq![S.as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_drop_n(self, n: usize) -> Own<Self, Result<()>> {
        if self.len < n {
            Own::new(self, Err(NotEnoughElements(Some(n))))
        } else {
            self.own_drop_n_unchecked(n).into_result_const()
        }
    }
    /// Drops the top `n` stack elements, unchecked version.
    ///
    /// `( 1 2 3 4 -- 1 )` for `n == 3`
    /// # Panics
    /// Panics if the stack doesn't contain at least `n` elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> =
    ///     Stack::from_array_const([1, 2, 3, 4]).own_drop_n_unchecked(3).state;
    /// assert_eq![S.as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_drop_n_unchecked(self, n: usize) -> Own<Self, ()> {
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_nip().state_ok();
    /// assert_eq![S.as_slice(), &[2]];
    ///
    /// const T: Stack<i32, (), 2> = Stack::new_copied(0).own_push(1).state_ok().own_nip().state_err();
    /// assert_eq![T.as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_nip(self) -> Own<Self, Result<()>> {
        if self.len < 2 {
            Own::new(self, Err(NotEnoughElements(Some(2))))
        } else {
            self.own_nip_unchecked().into_result_const()
        }
    }

    /// Drops the next of stack element in compile-time, unchecked version.
    ///
    /// `( 1 2 -- 2 )`
    /// # Panics
    /// Panics if the stack doesn't contain at least 2 elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_nip_unchecked().state;
    /// assert_eq![S.as_slice(), &[2]];
    /// ```
    #[inline]
    pub const fn own_nip_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        cswap![arr[self.len - 2], arr[self.len - 1]];
        let mut sta = Self::from_array_const(arr);
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([1, 2, 3, 4]).own_nip2().state_ok();
    /// assert_eq![S.as_slice(), &[3, 4]];
    /// ```
    #[inline]
    pub const fn own_nip2(self) -> Own<Self, Result<()>> {
        if self.len < 4 {
            Own::new(self, Err(NotEnoughElements(Some(4))))
        } else {
            self.own_nip2_unchecked().into_result_const()
        }
    }

    /// Drops the pair of next stack elements, unchecked version.
    ///
    /// `( 1 2 3 4 -- 3 4 )`
    /// # Panics
    /// Panics if the stack doesn't contain at least 4 elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([1, 2, 3, 4]).own_nip2_unchecked().state;
    /// assert_eq![S.as_slice(), &[3, 4]];
    /// ```
    #[inline]
    pub const fn own_nip2_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        cswap![arr[self.len - 4], arr[self.len - 2]];
        cswap![arr[self.len - 3], arr[self.len - 1]];
        let mut sta = Self::from_array_const(arr);
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
    /// # use devela::all::{DataResult, Own, Stack};
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_swap().state_ok();
    /// assert_eq![S.as_slice(), &[2, 1]];
    ///
    /// const T: Stack<i32, (), 1> = Stack::from_array_const([1]).own_swap().state_err();
    /// assert_eq![T.as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_swap(self) -> Own<Self, Result<()>> {
        if self.len < 2 {
            Own::new(self, Err(NotEnoughElements(Some(2))))
        } else {
            self.own_swap_unchecked().into_result_const()
        }
    }
    /// Swaps the top two stack elements in compile-time, unchecked version.
    ///
    /// `( 1 2 -- 2 1 )`
    /// # Panics
    /// Panics if the stack doesn't contain at least 2 elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_swap_unchecked().state;
    /// assert_eq![S.as_slice(), &[2, 1]];
    /// ```
    #[inline]
    pub const fn own_swap_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        cswap![arr[self.len - 2], arr[self.len - 1]];
        Own::empty(Self::from_array_const(arr))
    }

    /// Swaps the top two pair stack elements in compile-time.
    ///
    /// `( 1 2 3 4 -- 3 4 1 2 )`
    /// # Errors
    /// Returns `Own<self,`[`NotEnoughElements`]`>`
    /// if the stack doesn't contain at least 4 elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([1, 2, 3, 4]).own_swap2().state_ok();
    /// assert_eq![S.as_slice(), &[3, 4, 1, 2]];
    ///
    /// const T: Stack<i32, (), 3> = Stack::from_array_const([1, 2, 3]).own_swap2().state_err();
    /// assert_eq![T.as_slice(), &[1, 2, 3]];
    /// ```
    #[inline]
    pub const fn own_swap2(self) -> Own<Self, Result<()>> {
        if self.len < 4 {
            Own::new(self, Err(NotEnoughElements(Some(4))))
        } else {
            self.own_swap2_unchecked().into_result_const()
        }
    }
    /// Swaps the top two pair stack elements in compile-time, unchecked version.
    ///
    /// `( 1 2 3 4 -- 3 4 1 2 )`
    /// # Panics
    /// Panics if the stack doesn't contain at least 4 elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> =
    ///     Stack::from_array_const([1, 2, 3, 4]).own_swap2_unchecked().state;
    /// assert_eq![S.as_slice(), &[3, 4, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_swap2_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        cswap![arr[self.len - 4], arr[self.len - 2]];
        cswap![arr[self.len - 3], arr[self.len - 1]];
        Own::new(Self::from_array_const(arr), ())
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([0, 1, 2, 3]).own_rot().state_ok();
    /// assert_eq![S.as_slice(), &[0, 2, 3, 1]];
    /// ```
    #[inline]
    pub const fn own_rot(self) -> Own<Self, Result<()>> {
        if self.len < 3 {
            Own::new(self, Err(NotEnoughElements(Some(3))))
        } else {
            self.own_rot_unchecked().into_result_const()
        }
    }

    /// Rotates the top three stack elements, clockwise, unchecked version.
    ///
    /// `( 1 2 3 -- 2 3 1 ) `
    /// # Errors
    /// Returns `Own<self,`[`NotEnoughElements`]`>`
    /// if the stack doesn't contain at least 3 elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([0, 1, 2, 3]).own_rot_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 2, 3, 1]];
    /// ```
    #[inline]
    // WAIT: [const_swap](https://github.com/rust-lang/rust/issues/83163)
    pub const fn own_rot_unchecked(self) -> Own<Self, ()> {
        let len = self.len;
        let mut arr = self.array.into_array_const();
        let [a, b, c] = [arr[len - 3], arr[len - 2], arr[len - 1]];
        arr[len - 3] = b;
        arr[len - 2] = c;
        arr[len - 1] = a;
        let sta = Self::from_array_const(arr);
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([0, 1, 2, 3]).own_rot_cc().state_ok();
    /// assert_eq![S.as_slice(), &[0, 3, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_rot_cc(self) -> Own<Self, Result<()>> {
        if self.len < 3 {
            Own::new(self, Err(NotEnoughElements(Some(3))))
        } else {
            self.own_rot_cc_unchecked().into_result_const()
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([0, 1, 2, 3]).own_rot_cc_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 3, 1, 2]];
    /// ```
    #[inline]
    // WAIT: [const_swap](https://github.com/rust-lang/rust/issues/83163)
    pub const fn own_rot_cc_unchecked(self) -> Own<Self, ()> {
        let len = self.len;
        let mut arr = self.array.into_array_const();
        let [a, b, c] = [arr[len - 3], arr[len - 2], arr[len - 1]];
        arr[len - 3] = c;
        arr[len - 2] = a;
        arr[len - 1] = b;
        let sta = Self::from_array_const(arr);
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 7> =
    ///     Stack::from_array_const([0, 1, 2, 3, 4, 5, 6]).own_rot2().state_ok();
    /// assert_eq![S.as_slice(), &[0, 3, 4, 5, 6, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_rot2(self) -> Own<Self, Result<()>> {
        if self.len < 6 {
            Own::new(self, Err(NotEnoughElements(Some(6))))
        } else {
            self.own_rot2_unchecked().into_result_const()
        }
    }
    /// Rotates the top six stack elements, clockwise, two times, unchecked version.
    ///
    /// `( 1 2 3 4 5 6 -- 3 4 5 6 1 2 ) `
    /// # Panics
    /// Panics if the stack doesn't contain at least 6 elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 7> =
    ///     Stack::from_array_const([0, 1, 2, 3, 4, 5, 6]).own_rot2_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 3, 4, 5, 6, 1, 2]];
    /// ```
    #[inline]
    // WAIT: [const_swap](https://github.com/rust-lang/rust/issues/83163)
    pub const fn own_rot2_unchecked(self) -> Own<Self, ()> {
        let len = self.len;
        let mut arr = self.array.into_array_const();
        let [a, b, c, d, e, f] = sf! {[
            arr[len - 6], arr[len - 5], arr[len - 4], arr[len - 3], arr[len - 2], arr[len - 1],
        ]};
        arr[len - 6] = c;
        arr[len - 5] = d;
        arr[len - 4] = e;
        arr[len - 3] = f;
        arr[len - 2] = a;
        arr[len - 1] = b;
        let sta = Self::from_array_const(arr);
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 7> =
    ///     Stack::from_array_const([0, 1, 2, 3, 4, 5, 6]).own_rot2_cc().state_ok();
    /// assert_eq![S.as_slice(), &[0, 5, 6, 1, 2, 3, 4]];
    /// ```
    #[inline]
    pub const fn own_rot2_cc(self) -> Own<Self, Result<()>> {
        if self.len < 6 {
            Own::new(self, Err(NotEnoughElements(Some(6))))
        } else {
            self.own_rot2_cc_unchecked().into_result_const()
        }
    }
    /// Rotates the top six stack elements, counter-clockwise, two times, unchecked version.
    ///
    /// `( 1 2 3 4 5 6 -- 5 6 1 2 3 4 ) `
    /// # Panics
    /// Panics if the stack doesn't contain at least 6 elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 7> =
    ///     Stack::from_array_const([0, 1, 2, 3, 4, 5, 6]).own_rot2_cc_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 5, 6, 1, 2, 3, 4]];
    /// ```
    #[inline]
    // WAIT: [const_swap](https://github.com/rust-lang/rust/issues/83163)
    pub const fn own_rot2_cc_unchecked(self) -> Own<Self, ()> {
        let len = self.len;
        let mut arr = self.array.into_array_const();
        let [a, b, c, d, e, f] = sf! {[
            arr[len - 6], arr[len - 5], arr[len - 4], arr[len - 3], arr[len - 2], arr[len - 1]
        ]};
        arr[len - 6] = e;
        arr[len - 5] = f;
        arr[len - 4] = a;
        arr[len - 3] = b;
        arr[len - 2] = c;
        arr[len - 1] = d;
        let sta = Self::from_array_const(arr);
        Own::empty(sta)
    }

    /* dup */

    /// Duplicates the top stack element.
    ///
    /// `( 1 -- 1 1 )`
    /// # Errors
    /// Returns `Own<self,`[`NotEnoughElements`]`>` if the stack is empty or
    /// `Own<self,`[`NotEnoughSpace`]`>` if the stack is full.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 3> = Stack::new_copied(0).own_push(1).state.own_dup().state_ok();
    /// assert_eq![S.as_slice(), &[1, 1]];
    /// ```
    #[inline]
    pub const fn own_dup(self) -> Own<Self, Result<()>> {
        if self.len == 0 {
            Own::new(self, Err(NotEnoughElements(Some(1))))
        } else if self.len == CAP {
            Own::new(self, Err(NotEnoughSpace(Some(1))))
        } else {
            self.own_dup_unchecked().into_result_const()
        }
    }
    /// Duplicates the top stack element, unchecked version.
    ///
    /// `( 1 -- 1 1 )`
    /// # Panics
    /// Panics if the stack is either empty or full.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 3> = Stack::new_copied(0).own_push(1).state.own_dup_unchecked().state;
    /// assert_eq![S.as_slice(), &[1, 1]];
    /// ```
    #[inline]
    pub const fn own_dup_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        arr[self.len] = arr[self.len - 1];
        let mut sta = Self::from_array_const(arr);
        sta.len = self.len + 1;
        Own::empty(sta)
    }

    /// Duplicates the top stack pair of elements.
    ///
    /// `( 1 2 -- 1 2 1 2 )`
    /// # Errors
    /// Returns `Own<self,`[`NotEnoughElements`]`>`
    /// if the stack doesn't have at least 2 elements,
    /// or `Own<self,`[`NotEnoughSpace`]`>`
    /// if it doesn't have enough space for 2 extra elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 6> =
    ///     Stack::from_array_const([0, 1, 2, 0, 0, 0]).own_drop_n(3).state.own_dup2().state_ok();
    /// assert_eq![S.as_slice(), &[0, 1, 2, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_dup2(self) -> Own<Self, Result<()>> {
        if self.len < 2 {
            Own::new(self, Err(NotEnoughElements(Some(2))))
        } else if self.len > CAP - 2 {
            Own::new(self, Err(NotEnoughSpace(Some(2))))
        } else {
            self.own_dup2_unchecked().into_result_const()
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 6> =
    ///     Stack::from_array_const([0, 1, 2, 0, 0, 0]).own_drop_n(3).state.own_dup2_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 1, 2, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_dup2_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        let a = arr[self.len - 2];
        let b = arr[self.len - 1];
        arr[self.len] = a;
        arr[self.len + 1] = b;
        let mut sta = Self::from_array_const(arr);
        sta.len = self.len + 2;
        Own::empty(sta)
    }

    /* over */

    /// Duplicates the next of stack element to the top.
    ///
    /// `( 1 2 -- 1 2 1 )`
    /// # Errors
    /// Returns `Own<self,`[`NotEnoughElements`]`>`
    /// if the stack doesn't have at least 2 elements,
    /// or `Own<self,`[`NotEnoughSpace`]`>`
    /// if it doesn't have enough space for 2 extra elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> =
    ///     Stack::from_array_const([0, 1, 2, 0]).own_drop().state.own_over().state_ok();
    /// assert_eq![S.as_slice(), &[0, 1, 2, 1]];
    /// ```
    #[inline]
    pub const fn own_over(self) -> Own<Self, Result<()>> {
        if self.len < 2 {
            Own::new(self, Err(NotEnoughElements(Some(2))))
        } else if self.len == CAP {
            Own::new(self, Err(NotEnoughSpace(Some(1))))
        } else {
            self.own_over_unchecked().into_result_const()
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> =
    ///     Stack::from_array_const([0, 1, 2, 0]).own_drop().state.own_over_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 1, 2, 1]];
    /// ```
    #[inline]
    pub const fn own_over_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const(); // operate over a copy of the inner array
        arr[self.len] = arr[self.len - 2];
        let mut sta = Self::from_array_const(arr); // recreate the stack and adjust the length
        sta.len = self.len + 1;
        Own::empty(sta)
    }

    /// Duplicates the next of stack pair of elements to the top.
    ///
    /// `( 1 2 3 4 -- 1 2 3 4 1 2 )`
    /// # Errors
    /// Returns `Own<self,`[`NotEnoughElements`]`>`
    /// if the stack doesn't have at least 4 elements,
    /// or `Own<self,`[`NotEnoughSpace`]`>`
    /// if it doesn't have enough space for 2 extra elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 7> =
    ///     Stack::from_array_const([0, 1, 2, 3, 4, 0, 0]).own_drop_n(2).state.own_over2().state_ok();
    /// assert_eq![S.as_slice(), &[0, 1, 2, 3, 4, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_over2(self) -> Own<Self, Result<()>> {
        if self.len < 4 {
            Own::new(self, Err(NotEnoughElements(Some(4))))
        } else if CAP - self.len < 2 {
            Own::new(self, Err(NotEnoughSpace(Some(2))))
        } else {
            self.own_over2_unchecked().into_result_const()
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 7> =
    ///     Stack::from_array_const([0, 1, 2, 3, 4, 0, 0]).own_drop_n(2).state.own_over2_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 1, 2, 3, 4, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_over2_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        let a = arr[self.len - 4];
        let b = arr[self.len - 3];
        arr[self.len] = a;
        arr[self.len + 1] = b;
        let mut sta = Self::from_array_const(arr);
        sta.len = self.len + 2;
        Own::empty(sta)
    }

    /* tuck */

    /// Duplicates the top element before the next of stack element.
    ///
    /// `( 1 2 -- 2 1 2 )`
    /// # Errors
    /// Returns `Own<self,`[`NotEnoughElements`]`>`
    /// if the stack doesn't have at least 2 elements,
    /// or `Own<self,`[`NotEnoughSpace`]`>`
    /// if it doesn't have enough space for 1 extra elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> =
    ///     Stack::from_array_const([0, 1, 2, 0]).own_drop().state.own_tuck().state_ok();
    /// assert_eq![S.as_slice(), &[0, 2, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_tuck(self) -> Own<Self, Result<()>> {
        if self.len < 2 {
            Own::new(self, Err(NotEnoughElements(Some(2))))
        } else if self.len == CAP {
            Own::new(self, Err(NotEnoughSpace(Some(1))))
        } else {
            self.own_tuck_unchecked().into_result_const()
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 4> =
    ///     Stack::from_array_const([0, 1, 2, 0]).own_drop().state.own_tuck_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 2, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_tuck_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        let a = arr[self.len - 1];
        cswap![arr[self.len - 2], arr[self.len - 1]];
        arr[self.len] = a;
        let mut sta = Self::from_array_const(arr);
        sta.len = self.len + 1;
        Own::empty(sta)
    }

    /// Duplicates the top pair of elements before the next of stack pair of elements.
    ///
    /// `( 1 2 3 4 -- 3 4 1 2 3 4 )`
    /// # Errors
    /// Returns `Own<self,`[`NotEnoughElements`]`>`
    /// if the stack doesn't have at least 4 elements,
    /// or `Own<self,`[`NotEnoughSpace`]`>`
    /// if it doesn't have enough space for 2 extra elements.
    /// # Examples
    /// ```
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 7> =
    ///     Stack::from_array_const([0, 1, 2, 3, 4, 0, 0]).own_drop_n(2).state.own_tuck2_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 3, 4, 1, 2, 3, 4]];
    /// ```
    #[inline]
    pub const fn own_tuck2(self) -> Own<Self, Result<()>> {
        if self.len < 4 {
            Own::new(self, Err(NotEnoughElements(Some(4))))
        } else if CAP - self.len < 2 {
            Own::new(self, Err(NotEnoughSpace(Some(2))))
        } else {
            self.own_tuck2_unchecked().into_result_const()
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
    /// # use devela::all::{Own, Stack};
    /// const S: Stack<i32, (), 7> =
    ///     Stack::from_array_const([0, 1, 2, 3, 4, 0, 0]).own_drop_n(2).state.own_tuck2_unchecked().state;
    /// assert_eq![S.as_slice(), &[0, 3, 4, 1, 2, 3, 4]];
    /// ```
    #[inline]
    pub const fn own_tuck2_unchecked(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        // swap2
        cswap![arr[self.len - 4], arr[self.len - 2]];
        cswap![arr[self.len - 3], arr[self.len - 1]];
        // over2
        let a = arr[self.len - 4];
        let b = arr[self.len - 3];
        arr[self.len] = a;
        arr[self.len + 1] = b;
        let mut sta = Self::from_array_const(arr);
        sta.len = self.len + 2;
        Own::empty(sta)
    }
}
