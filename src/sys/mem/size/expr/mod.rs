// devela::sys::mem::size::expr
//
// Original source code by Joshua Nelson, licensed as BSD-3,
// https://crates.io/crates/size-of-trait/1.1.3

/// A helper macro used to bridge the gap between compile-time and runtime.
#[doc(hidden)]
pub const fn __size_of_expr<T>(_zero_len_fn_ptr_array: [impl FnOnce() -> [T; 0]; 0]) -> usize {
    crate::Mem::size_of::<T>()
}

/// Returns the size of an expression in bytes.
///
/// - The expression will not be evaluated.
/// - This can be used in `const` contexts.
///
/// # Example
/// ```
/// # use devela::size_of_expr;
/// async fn f() {
///     let x = 1;
///     core::future::ready(()).await;
/// }
/// const SIZE: usize = size_of_expr!(f());
/// assert_eq!(SIZE, 2);
/// ```
/// # Features
/// Makes use of [`unreachable_unchecked`][core::hint::unreachable_unchecked]
/// if the `unsafe_hint` feature is enabled.
#[doc = crate::doc_!(vendor: "size_of_trait")]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! size_of_expr {
    ($expr: expr) => {
        $crate::sys::mem::__size_of_expr(
            // The array of function pointers is created in an unreachable branch
            // of an if-else block to avoid evaluating it.
            if true {
                []
            } else {
                #[cfg(any(feature = "safe_mem", not(feature = "unsafe_hint")))]
                loop {}
                #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_hint"))]
                unsafe {
                    $crate::unreachable_unchecked()
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
pub use size_of_expr;

/* tests */

// has to be a separate file because of experimental syntax
#[cfg(all(test, nightly_coro, feature = "alloc"))]
mod test_coro;

#[cfg(test)]
mod tests {
    use super::size_of_expr;

    async fn f() {
        let _x = 1;
        core::future::ready(()).await;
        let _y = 2;
    }

    #[test]
    fn api() {
        const C: usize = size_of_expr!(f());
        const D: usize = size_of_expr!(0_u8);
        const E: usize = size_of_expr!(());
        const F: usize = size_of_expr!(0_usize);

        assert_eq!(C, 2);
        assert_eq!(D, 1);
        assert_eq!(E, 0);
        assert_eq!(F, size_of::<usize>());
    }

    #[test]
    fn edge_cases() {
        // 1. works with references:
        const _: [(); size_of_expr!(&Some(42))] = [(); size_of::<*const ()>()];
        let _ = size_of_expr!(&Some(42));

        // 2. works with temporaries:
        #[allow(dropping_copy_types)]
        const _: [(); size_of_expr!(Some(drop(())).as_ref())] = [(); size_of::<*const ()>()];
        #[allow(dropping_copy_types)]
        let _ = size_of_expr!(Some(drop(())).as_ref());

        // 3. Does not move the named stuff
        struct NotCopy {}
        let it = NotCopy {};
        assert_eq!(size_of_expr!(it), 0);
        drop(it);

        // 4. Does not even borrow the named stuff
        let mut it = ();
        let r = &mut it;
        assert_eq!(size_of_expr!(it), 0);
        #[allow(dropping_references)]
        drop(r);
    }
}
