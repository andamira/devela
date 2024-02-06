// devela::data::methods_own
//
//! owning stack const operations, in compile-time
//! depends on S:(), T:Copy
//

use crate::{
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
            let mut arr = self.array.into_array_const();
            cswap![arr[self.len - 2], arr[self.len - 1]];
            Own::new(Self::from_array_const(arr), Ok(()))
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
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_swap_uc().state;
    /// assert_eq![S.as_slice(), &[2, 1]];
    /// ```
    #[inline]
    pub const fn own_swap_uc(self) -> Own<Self, ()> {
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
            let mut arr = self.array.into_array_const();
            cswap![arr[self.len - 4], arr[self.len - 2]];
            cswap![arr[self.len - 3], arr[self.len - 1]];
            Own::new(Self::from_array_const(arr), Ok(()))
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
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([1, 2, 3, 4]).own_swap2_uc().state;
    /// assert_eq![S.as_slice(), &[3, 4, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_swap2_uc(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        cswap![arr[self.len - 4], arr[self.len - 2]];
        cswap![arr[self.len - 3], arr[self.len - 1]];
        Own::new(Self::from_array_const(arr), ())
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
            let mut sta = self;
            sta.len -= 1;
            Own::empty_ok(sta)
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
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_drop_uc().state;
    /// assert_eq![S.as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_drop_uc(self) -> Own<Self, ()> {
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
            let mut sta = self;
            sta.len -= n;
            Own::empty_ok(sta)
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
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([1, 2, 3, 4]).own_drop_n_uc(3).state;
    /// assert_eq![S.as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_drop_n_uc(self, n: usize) -> Own<Self, ()> {
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
            let mut arr = self.array.into_array_const();
            cswap![arr[self.len - 2], arr[self.len - 1]];
            let mut sta = Self::from_array_const(arr);
            sta.len -= 1;
            Own::empty_ok(sta)
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
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_nip_uc().state;
    /// assert_eq![S.as_slice(), &[2]];
    /// ```
    #[inline]
    pub const fn own_nip_uc(self) -> Own<Self, ()> {
        let mut arr = self.array.into_array_const();
        cswap![arr[self.len - 2], arr[self.len - 1]];
        let mut sta = Self::from_array_const(arr);
        sta.len -= 1;
        Own::empty(sta)
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
        let len = self.len;
        if len == CAP {
            Own::new(self, Err(NotEnoughSpace(Some(1))))
        } else {
            let mut arr = self.array.into_array_const();
            arr[len] = e;
            let mut sta = Self::from_array_const(arr);
            sta.len = len + 1;
            Own::new(sta, Ok(()))
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
    /// const S: Stack<i32, (), 2> = Stack::new_copied(0).own_push_uc(1).own_push_uc(2).state;
    /// assert_eq![S.as_slice(), &[1, 2]];
    /// ```
    #[inline]
    pub const fn own_push_uc(self, e: T) -> Own<Self, ()> {
        let len = self.len;
        let mut arr = self.array.into_array_const();
        arr[len] = e;
        let mut sta = Self::from_array_const(arr);
        sta.len = len + 1;
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
            let arr = self.array.into_array_const();
            let e = arr[self.len - 1];
            let mut sta = Self::from_array_const(arr);
            sta.len = self.len - 1;
            Own::new(sta, Ok(e))
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
    /// const S: Own<Stack<i32, (), 3>, i32> = Stack::from_array_const([1, 2, 3]).own_pop_uc();
    /// assert_eq![S.state.as_slice(), &[1, 2]];
    /// assert_eq![S.value, 3];
    /// ```
    #[inline]
    pub const fn own_pop_uc(self) -> Own<Self, T> {
        let arr = self.array.into_array_const();
        let e = arr[self.len - 1];
        let mut sta = Self::from_array_const(arr);
        sta.len = self.len - 1;
        Own::new(sta, e)
    }
}
