// devela::data::methods_own
//
//! owning stack const operations, in compile time
//! depends on S:(), T:Copy
//

use crate::{
    data::{
        error::{DataErrors, DataResult as Result},
        {Array, Stack},
    },
    mem::{cswap, Storage},
};
use DataErrors::{NotEnoughElements, NotEnoughSpace};

/// # Chainable, owned const operations.
// `S:(), T:Copy`
impl<T: Copy, const CAP: usize> Stack<T, (), CAP> {
    /* clear */

    /// Clears the stack in conmpilation time.
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

    /// Swaps the top two stack elements in compile time.
    ///
    /// `( 1 2 -- 2 1 )`
    /// # Errors
    /// Returns [`NotEnoughElements`] if the stack doesn't contain at least 2 elements.
    /// # Examples
    /// ```
    /// # use devela::data::{DataResult, DirectStack, Stack};
    /// const S: DataResult<DirectStack<i32, 2>> = Stack::from_array_const([1, 2]).own_swap();
    /// assert_eq![S.unwrap().as_slice(), &[2, 1]];
    /// ```
    #[inline]
    pub const fn own_swap(self) -> Result<Self> {
        if self.len < 2 {
            Err(NotEnoughElements(Some(2)))
        } else {
            let mut arr = self.array.into_array_const();
            cswap![arr[self.len - 2], arr[self.len - 1]];
            Ok(Self::from_array_const(arr))
        }
    }
    /// Swaps the top two stack elements in compile time, unchecked version.
    ///
    /// `( 1 2 -- 2 1 )`
    /// # Panics
    /// Panics if the stack doesn't contain at least 2 elements.
    /// # Examples
    /// ```
    /// # use devela::data::{DataResult, DirectStack, Stack};
    /// const S: DirectStack<i32, 2> = Stack::from_array_const([1, 2]).own_swap_uc();
    /// assert_eq![S.as_slice(), &[2, 1]];
    /// ```
    #[inline]
    pub const fn own_swap_uc(self) -> Self {
        let mut arr = self.array.into_array_const();
        cswap![arr[self.len - 2], arr[self.len - 1]];
        Self::from_array_const(arr)
    }

    /// Swaps the top two pair stack elements in compile time.
    ///
    /// `( 1 2 3 4 -- 3 4 1 2 )`
    /// # Errors
    /// Returns [`NotEnoughElements`] if the stack doesn't contain at least 4 elements.
    /// # Examples
    /// ```
    /// # use devela::data::{DataResult, Stack};
    /// const S: DataResult<Stack<i32, (), 4>> = Stack::from_array_const([1, 2, 3, 4]).own_swap2();
    /// assert_eq![S.unwrap().as_slice(), &[3, 4, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_swap2(self) -> Result<Self> {
        if self.len < 4 {
            Err(NotEnoughElements(Some(4)))
        } else {
            let mut arr = self.array.into_array_const();
            cswap![arr[self.len - 4], arr[self.len - 2]];
            cswap![arr[self.len - 3], arr[self.len - 1]];
            Ok(Self::from_array_const(arr))
        }
    }
    /// Swaps the top two pair stack elements in compile time, unchecked version.
    ///
    /// `( 1 2 3 4 -- 3 4 1 2 )`
    /// # Panics
    /// Panics if the stack doesn't contain at least 4 elements.
    /// # Examples
    /// ```
    /// # use devela::data::Stack;
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([1, 2, 3, 4]).own_swap2_uc();
    /// assert_eq![S.as_slice(), &[3, 4, 1, 2]];
    /// ```
    #[inline]
    pub const fn own_swap2_uc(self) -> Self {
        let mut arr = self.array.into_array_const();
        cswap![arr[self.len - 4], arr[self.len - 2]];
        cswap![arr[self.len - 3], arr[self.len - 1]];
        Self::from_array_const(arr)
    }

    /* drop */

    /// Drops the top stack element in compile time.
    ///
    /// `( 1 2 -- 1 )`
    /// # Errors
    /// Returns [`NotEnoughElements`] if the stack is empty.
    /// # Examples
    /// ```
    /// # use devela::data::{DataResult, Stack};
    /// const S: DataResult<Stack<i32, (), 2>> = Stack::from_array_const([1, 2]).own_drop();
    /// assert_eq![S.unwrap().as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_drop(self) -> Result<Self> {
        if self.len == 0 {
            Err(NotEnoughElements(Some(1)))
        } else {
            let mut sta = self;
            sta.len -= 1;
            Ok(sta)
        }
    }
    /// Swaps the top two pair stack elements, unchecked version.
    ///
    /// `( 1 2 -- 1 )`
    /// # Panics
    /// Panics if the stack is empty.
    /// # Examples
    /// ```
    /// # use devela::data::Stack;
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_drop_uc();
    /// assert_eq![S.as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_drop_uc(self) -> Self {
        let mut sta = self;
        sta.len -= 1;
        sta
    }

    /// Drops the top `n` stack elements.
    ///
    /// `( 1 2 3 4 -- 1 )` for `n == 3`
    /// # Errors
    /// Returns [`NotEnoughElements`] if the stack doesn't contain at least `nth` elements.
    /// # Examples
    /// ```
    /// # use devela::data::{DataResult, Stack};
    /// const S: DataResult<Stack<i32, (), 4>> = Stack::from_array_const([1, 2, 3, 4]).own_drop_n(3);
    /// assert_eq![S.unwrap().as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_drop_n(self, n: usize) -> Result<Self> {
        if self.len < n {
            Err(NotEnoughElements(Some(n)))
        } else {
            let mut sta = self;
            sta.len -= n;
            Ok(sta)
        }
    }
    /// Drops the top `n` stack elements, unchecked version.
    ///
    /// `( 1 2 3 4 -- 1 )` for `n == 3`
    /// # Panics
    /// Panics if the stack doesn't contain at least `n` elements.
    /// # Examples
    /// ```
    /// # use devela::data::Stack;
    /// const S: Stack<i32, (), 4> = Stack::from_array_const([1, 2, 3, 4]).own_drop_n_uc(3);
    /// assert_eq![S.as_slice(), &[1]];
    /// ```
    #[inline]
    pub const fn own_drop_n_uc(self, n: usize) -> Self {
        let mut sta = self;
        sta.len -= n;
        sta
    }

    /* nip */

    /// Drops the next of stack element in compilation time.
    ///
    /// `( 1 2 -- 2 )`
    /// # Errors
    /// Returns [`NotEnoughElements`] if the stack doesn't contain at least 2 elements.
    /// # Examples
    /// ```
    /// # use devela::all::{chain, DataResult, Stack};
    /// const S: DataResult<Stack<i32, (), 2>> = Stack::from_array_const([1, 2]).own_nip();
    /// assert_eq![S.unwrap().as_slice(), &[2]];
    ///
    /// const S2: DataResult<Stack<i32, (), 2>> =
    ///     chain![Stack::new_copied(0),own_push(1) => unwrap()].own_nip();
    /// assert![S2.is_err()];
    /// ```
    #[inline]
    pub const fn own_nip(self) -> Result<Self> {
        if self.len < 2 {
            Err(NotEnoughElements(Some(2)))
        } else {
            let mut arr = self.array.into_array_const();
            cswap![arr[self.len - 2], arr[self.len - 1]];
            let mut sta = Self::from_array_const(arr);
            sta.len -= 1;
            Ok(sta)
        }
    }

    /// Drops the next of stack element in compilation time, unchecked version.
    ///
    /// `( 1 2 -- 2 )`
    /// # Panics
    /// Panics if the stack doesn't contain at least 2 elements.
    /// # Examples
    /// ```
    /// # use devela::data::Stack;
    /// const S: Stack<i32, (), 2> = Stack::from_array_const([1, 2]).own_nip_uc();
    /// assert_eq![S.as_slice(), &[2]];
    /// ```
    #[inline]
    pub const fn own_nip_uc(self) -> Self {
        let mut arr = self.array.into_array_const();
        cswap![arr[self.len - 2], arr[self.len - 1]];
        let mut sta = Self::from_array_const(arr);
        sta.len -= 1;
        sta
    }

    /* push */

    /// Pushes a new element to the top of the stack in compilation time.
    ///
    /// `( 1 -- 1 2 )`
    /// # Errors
    /// Returns [`NotEnoughSpace`] if the stack is full.
    /// # Examples
    /// ```
    /// # use devela::all::{chain, DataResult, DataErrors, Stack};
    /// const S: Stack<i32, (), 2> = chain![Stack::new_copied(0),own_push(1),own_push(2)=>unwrap()];
    /// assert_eq![S.as_slice(), &[1, 2]];
    ///
    /// const S2: DataResult<Stack<i32, (), 2>> = S.own_push(3);
    /// assert![S2.is_err()];
    /// ```
    #[inline]
    pub const fn own_push(self, e: T) -> Result<Self> {
        let len = self.len;
        if len == CAP {
            Err(NotEnoughSpace(Some(1)))
        } else {
            let mut arr = self.array.into_array_const();
            arr[len] = e;
            let mut sta = Self::from_array_const(arr);
            sta.len = len + 1;
            Ok(sta)
        }
    }

    /// Pushes a new element to the top of the stack in compilation time, unchecked version.
    ///
    /// `( 1 -- 1 2 )`
    /// # Panics
    /// Panics if the stack is full.
    /// # Examples
    /// ```
    /// # use devela::all::{DataResult, Stack};
    /// const S: Stack<i32, (), 2> = Stack::new_copied(0).own_push_uc(1).own_push_uc(2);
    /// assert_eq![S.as_slice(), &[1, 2]];
    /// ```
    #[inline]
    pub const fn own_push_uc(self, e: T) -> Self {
        let len = self.len;
        let mut arr = self.array.into_array_const();
        arr[len] = e;
        let mut sta = Self::from_array_const(arr);
        sta.len = len + 1;
        sta
    }

    /* pop */

    /// Pops the top stack element in compilation time.
    ///
    /// `( 1 2 3 -- 1 2 )`
    /// # Errors
    /// Returns [`NotEnoughElements`] if the stack is empty.
    /// # Examples
    /// ```
    /// # use devela::all::{DataResult, Stack};
    /// const S: DataResult<(Stack<i32, (), 3>, i32)> = Stack::from_array_const([1, 2, 3]).own_pop();
    /// let (s, p) = S.unwrap();
    /// assert_eq![s.as_slice(), &[1, 2]];
    /// assert_eq![p, 3];
    /// ```
    #[inline]
    pub const fn own_pop(self) -> Result<(Self, T)> {
        if self.len == 0 {
            Err(NotEnoughElements(Some(1)))
        } else {
            let arr = self.array.into_array_const();
            let e = arr[self.len - 1];
            let mut sta = Self::from_array_const(arr);
            sta.len = self.len - 1;
            Ok((sta, e))
        }
    }

    /// Pops the top stack element in compilation time, unchecked version.
    ///
    /// `( 1 2 3 -- 1 2 )`
    /// # Panics
    /// Panics if the stack is empty.
    /// # Examples
    /// ```
    /// # use devela::all::{DataResult, Stack};
    /// const S: (Stack<i32, (), 3>, i32) = Stack::from_array_const([1, 2, 3]).own_pop_uc();
    /// assert_eq![S.0.as_slice(), &[1, 2]];
    /// assert_eq![S.1, 3];
    /// ```
    #[inline]
    pub const fn own_pop_uc(self) -> (Self, T) {
        let arr = self.array.into_array_const();
        let e = arr[self.len - 1];
        let mut sta = Self::from_array_const(arr);
        sta.len = self.len - 1;
        (sta, e)
    }
}
