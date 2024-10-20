// devela::mem::size::expr
//
// Original source code by Joshua Nelson, licensed as BSD-3,
// https://crates.io/crates/size-of-trait/1.1.3

/// Returns the size of an expression in bytes.
///
/// - The expression will not be evaluated.
/// - This can be used in `const` contexts.
///
/// # Example
/// ```
/// # use devela::mem_size_of_expr;
/// async fn f() {
///     let x = 1;
///     core::future::ready(()).await;
/// }
/// const SIZE: usize = mem_size_of_expr!(f());
/// assert_eq!(SIZE, 2);
/// ```
///
/// # Features
/// Makes use of [`unreachable_unchecked`][core::hint::unreachable_unchecked]
/// if the `unsafe_hint` feature is enabled.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! mem_size_of_expr {
    ($expr: expr) => {
        $crate::mem::__mem_size_of_expr(
            // The array of function pointers is created in an unreachable branch
            // of an if-else block to avoid evaluating it.
            if true {
                []
            } else {
                #[cfg(any(feature = "safe_mem", not(feature = "unsafe_hint")))]
                loop {}
                #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_hint"))]
                unsafe {
                    core::hint::unreachable_unchecked()
                }

                #[expect(unreachable_code, reason = "avoid evaluating this branch")]
                {
                    [|| [$expr; 0]; 0]
                }
            },
        )
    };
}
#[doc(inline)]
pub use mem_size_of_expr;

// A helper macro used to bridge the gap between compile-time and runtime.
#[doc(hidden)]
pub const fn __mem_size_of_expr<T>(_zero_len_fn_ptr_array: [impl FnOnce() -> [T; 0]; 0]) -> usize {
    crate::mem::mem_size_of::<T>()
}

/* tests */

// has to be a separate file because of experimental syntax
#[cfg(all(test, feature = "nightly_coro", feature = "alloc"))]
mod test_coro;

#[cfg(test)]
mod tests {
    use super::mem_size_of_expr;
    use crate::mem::mem_size_of;

    async fn f() {
        let _x = 1;
        core::future::ready(()).await;
        let _y = 2;
    }

    #[test]
    fn api() {
        const C: usize = mem_size_of_expr!(f());
        const D: usize = mem_size_of_expr!(0_u8);
        const E: usize = mem_size_of_expr!(());
        const F: usize = mem_size_of_expr!(0_usize);

        assert_eq!(C, 2);
        assert_eq!(D, 1);
        assert_eq!(E, 0);
        assert_eq!(F, mem_size_of::<usize>());
    }

    #[test]
    fn edge_cases() {
        // 1. works with references:
        const _: [(); mem_size_of_expr!(&Some(42))] = [(); mem_size_of::<*const ()>()];
        let _ = mem_size_of_expr!(&Some(42));

        // 2. works with temporaries:
        #[allow(dropping_copy_types)]
        const _: [(); mem_size_of_expr!(Some(drop(())).as_ref())] =
            [(); mem_size_of::<*const ()>()];
        #[allow(dropping_copy_types)]
        let _ = mem_size_of_expr!(Some(drop(())).as_ref());

        // 3. Does not move the named stuff
        struct NotCopy {}
        let it = NotCopy {};
        assert_eq!(mem_size_of_expr!(it), 0);
        drop(it);

        // 4. Does not even borrow the named stuff
        let mut it = ();
        let r = &mut it;
        assert_eq!(mem_size_of_expr!(it), 0);
        #[allow(dropping_references)]
        drop(r);
    }
}
