// devela_base_core::code::util::methods
//
//! Defines [`methods_as_fns`].
//
// NOTE:
// keyword order for functions declaration is `pub`, `default`, `const`, `async`, `unsafe`, `extern`
//
// - IMPROVE: support generics, other use cases.
// - TODO: make corresponding fns_as_methods.

/// Defines standalone functions that call associated methods.
///
/// Supports various function qualifiers (const, async, unsafe) and attributes.
///
/** # Examples
```
# use devela_base_core::methods_as_fns;
struct MyType;
impl MyType {
    pub const fn add(a: i32, b: i32) -> i32 { a + b }
    pub async fn fetch(url: &str) -> &str { "fetched" }
    pub const unsafe fn dangerous() {}
}

methods_as_fns! {
    pub MyType =>
    #[inline]
    +const add(a: i32, b: i32) -> i32,
    #[must_use]
    +async fetch(url: &str) -> &str,
    +const +unsafe dangerous()
}
assert_eq!(add(5, 2), 7);

// syntax to name the functions differently than the method (can't be mixed):
methods_as_fns! {
    pub(crate) MyType =>
    add|addition(a: i32, b: i32) -> i32,
}
assert_eq!(addition(5, 2), 7);
```
**/
// we are employing a trick, e.g. `$(+const$($_c:lifetime)?)?` for the optional fn modifiers,
// where the never expected lifetime allows to refer to the non-identifier `+const` later on;
// and the same for the async and unsafe fn modifiers.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _methods_as_fns {
    (
    // same name for method and function
        $fn_vis:vis
        $item:path => $(
        $(#[$fn_attr:meta])*
        $(+const$($_c:lifetime)?)? $(+async$($_a:lifetime)?)? $(+unsafe$($_u:lifetime)?)?
        $method:ident ($($arg_name:ident : $arg_ty:ty),*) $(-> $ret_ty:ty)?
    ),* $(,)?) => {
        $(
            // calls an unsafe method, and marks the function as unsafe as well
            #[$crate::compile(some( $(unsafe$($_u)?)? ))]
            $(#[$fn_attr])*
            $fn_vis $(const$($_c)?)? $(async$($_a)?)? $(unsafe$($_u)?)?
            fn $method($($arg_name: $arg_ty),*) $(-> $ret_ty)? {
                unsafe { <$item>::$method( $($arg_name),*) $(.await$($_a)?)? }
            }
            // calls a safe method
            #[$crate::compile(none( $(unsafe$($_u)?)? ))]
            $(#[$fn_attr])*
            $fn_vis $(const$($_c)?)? $(async$($_a)?)?
            fn $method($($arg_name: $arg_ty),*) $(-> $ret_ty)? {
                <$item>::$method( $($arg_name),*) $(.await$($_a)?)?
            }
        )*
    };
    (
    // different name for method and function
        $fn_vis:vis $item:path => $(
        $(#[$fn_attr:meta])*
        $(+const$($_c:lifetime)?)? $(+async$($_a:lifetime)?)? $(+unsafe$($_u:lifetime)?)?
        $method:ident|$fn:ident ($($arg_name:ident : $arg_ty:ty),*) $(-> $ret_ty:ty)?
    ),* $(,)?) => {
        $(
            // calls an unsafe method, and marks the function as unsafe as well
            #[$crate::compile(some( $(unsafe$($_u)?)? ))]
            $(#[$fn_attr])*
            $fn_vis $(const$($_c)?)? $(async$($_a)?)? $(unsafe$($_u)?)?
            fn $fn($($arg_name: $arg_ty),*) $(-> $ret_ty)? {
                unsafe { <$item>::$method( $($arg_name),*) $(.await$($_a)?)? }
            }
            // calls a safe method
            #[$crate::compile(none( $(unsafe$($_u)?)? ))]
            $(#[$fn_attr])*
            $fn_vis $(const$($_c)?)? $(async$($_a)?)?
            fn $fn($($arg_name: $arg_ty),*) $(-> $ret_ty)? {
                <$item>::$method( $($arg_name),*) $(.await$($_a)?)?
            }
        )*
    };
}
pub use _methods_as_fns as methods_as_fns;

// TODO
/*
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #![allow(dead_code)]

    use crate::methods_as_fns;

    struct TestMethods;

    #[allow(dead_code)]
    impl TestMethods {
        pub const fn add(a: i32, b: i32) -> i32 { a + b }
        pub fn sub(a: i32, b: i32) -> i32 { a - b }
        pub fn trim(name: &str) -> &str { name.trim() }
        pub async fn fetch(_url: &str) -> &str { "fetched" }
        #[cfg(all(unsafe··, not(base_safe_code)))]
        pub const unsafe fn noop() {}
    }

    // Test same-name pattern with attributes
    methods_as_fns! {
        pub TestMethods =>
        #[inline] +const add(a: i32, b: i32) -> i32,
        #[cold] #[allow(unused)] sub(a: i32, b: i32) -> i32,
        #[must_use] +async fetch(url: &str) -> &str,
        #[must_use] trim(name: &str) -> &str,
    }
    #[cfg(all(unsafe··, not(base_safe_code)))]
    methods_as_fns! { pub TestMethods => +const +unsafe noop() }

    // Test different-name pattern with attributes
    methods_as_fns! {
        pub(crate) TestMethods =>
        #[inline(always)] +const add|add_numbers(a: i32, b: i32) -> i32,
        #[cold] sub|subtract(a: i32, b: i32) -> i32,
        +async fetch|fetch_data(url: &str) -> &str,
        #[must_use] trim|do_trim(name: &str) -> &str,
    }
    #[cfg(all(unsafe··, not(base_safe_code)))]
    methods_as_fns! { pub TestMethods => +const +unsafe noop|do_nothing() }

    #[test]
    fn test_all_variants() {
        // Test same-name functions
        crate::const_assert!(eq add(2, 3), 5);
        assert_eq!(sub(5, 2), 3);
        assert_eq!(trim(" world "), "world");
        #[cfg(all(unsafe··, not(base_safe_code)))]
        unsafe { noop(); }

        // Test different-name functions
        crate::const_assert!(eq add_numbers(2, 3), 5);
        assert_eq!(subtract(5, 2), 3);
        assert_eq!(do_trim("   test   "), "test");
        #[cfg(all(unsafe··, not(base_safe_code)))]
        unsafe { do_nothing(); }
    }

    #[test]
    #[cfg(feature = "dep_tokio")]
    fn test_async_functions_with_tokio() {
        use ::tokio::runtime::Builder;
        assert_eq![
            Builder::new_multi_thread().enable_all().build().unwrap().block_on(fetch("url")),
            "fetched",
        ];
    }
    #[test]
    #[cfg(feature = "std")]
    #[cfg(not(feature = "dep_portable_atomic_util"))]
    fn test_async_functions_with_ext_future() {
        use crate::FutureExt;
        assert_eq!(fetch("test.com").block_on(), "fetched");
        assert_eq!(fetch_data("api.com").block_on(), "fetched");
    }
}
*/
