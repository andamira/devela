// devela::lang::ffi::js::helpers
//
//! Defines internal JS helpers: [`js_doc!`], [`js_reexport!`].
//
// TOC
// - js_doc!
// - js_reexport!

/// Helper for Web API doc links.
#[rustfmt::skip]
macro_rules! _js_doc {
    ( // Link to an API Object
        $API:literal) => { concat!["([", $API,
        "](https://developer.mozilla.org/en-US/docs/Web/API/", $API, "))"] };
    ( // Link to a method of an API Object
        $API:literal, $method:literal) => { concat![ "([", $method,
        "](https://developer.mozilla.org/en-US/docs/Web/API/", $API, "/", $method, "))"] };

    /* special cases */

    (canvas $method:literal) => { concat!["([", $method,
        "](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/",
        $method, "))"] };
    (console $method:literal) => { concat!["([", $method,
        "](https://developer.mozilla.org/en-US/docs/Web/API/console/",
        $method, "_static))"] };
}
pub(crate) use _js_doc as js_doc;

/// Generates dual JS method variants: one allocating (`-> String`) and one buffer-based (`-> &str`).
///
/// Creates two methods from one declaration:
/// - `$method()` - Allocates and returns a `String` (requires `alloc`)
/// - `$method_buf(&mut [u8])` - Uses provided buffer, returns `&str`
///
/// # Example:
/// The following code
/// ```ignore
/// js_method_buf_string! {
///     #[doc = js_doc!("Window", "name")]
///     /// Returns the window name.
///     name, window_name
/// }
/// ```
/// would produce:
/// ```ignore
/// #[doc = js_doc!("Window", "name")]
/// /// Returns the window name.
/// #[cfg(feature = "alloc")]
/// #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
/// pub fn name() -> String { Js::read_string(|ptr, len| unsafe { window_name(ptr, len) }) }
///
/// #[doc = js_doc!("Window", "name")]
/// /// Returns the window name.
/// pub fn name_buf(buffer: &mut [u8]) -> &str {
///     Js::read_str(buffer, |ptr, len| unsafe { window_name(ptr, len) })
/// }
/// ```
#[rustfmt::skip]
macro_rules! _js_method_buf_string {
    (
    $(#[$fn_attrs:meta])*
    $method:ident,
    $call_fn:ident $(,)?
    ) => { $crate::paste! {
        /* allocating version */

        $(#[$fn_attrs])*
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        pub fn $method() -> String {
            $crate::Js::read_string(|ptr, len| unsafe { $call_fn(ptr, len) })
        }

        /* non-allocating version */

        $(#[$fn_attrs])*
        pub fn [<$method _buf>](buffer: &mut [u8]) -> &str {
            $crate::Js::read_str(buffer, |ptr, len| unsafe { $call_fn(ptr, len) })
        }
    }};
}
pub(crate) use _js_method_buf_string;

/// Helps re-exporting javascript functions.
///
/// # Example
/// ```ignore
/// # use devela::js_reexport;
/// js_reexport! {
///     [ module: "env" ]
///     pub safe fn same_fn_name(x: f64, y: f64, w: f64, h: f64);
///     pub(crate) safe fn "js_fn_name" rust_fn_name(x: f64, y: f64, w: f64, h: f64);
///     unsafe fn "js_fn" rs_fn(ptr: *const u8, len: usize, x: f64, y: f64);
/// }
/// // The previous code generates:
///
/// // #[link(wasm_import_module = "env")]
/// // unsafe extern "C" {
/// //     pub safe fn same_fn_name(x: f64, y: f64, w: f64, h: f64);
/// //
/// //     #[link_name = "js_fn_name"]
/// //     pub(cate) safe fn rust_fn_name(x: f64, y: f64, w: f64, h: f64);
/// //
/// //     #[link_name = "js_fn"]
/// //     unsafe fn rs_fn(ptr: *const u8, len: usize, x: f64, y: f64);
/// // }
/// ```
///
/// Use *safe* fn when:
/// - The function does not perform pointer dereferencing or other memory-unsafe operations.
/// - It always behaves safely (e.g., a function that just draws to the Canvas API).
///
/// Use *unsafe* fn if:
/// - The function can mutate raw memory (e.g., passing buffers, pointers).
/// - It performs DOM manipulations that might trigger undefined behavior.
/// - It can throw exceptions that Rust cannot catch.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
macro_rules! _js_reexport {
    (
        // # Args
        // [            header section
        // $module:     optional js module name (defaults to "env")
        // ]
        // $fn_attrs:   optional attributes and doc comments
        // $vis:        visibility of the extern function (defaults to private)
        // safe|unsafe  optional safety specifier (defaults to unsafe)
        // $js_fn:      optional link_name (different javascript function name)
        // $fn:         imported rust function name (default same js name)

        [ $(module: $module:literal)? $(,)? ]
        $(
            $(#[$fn_attrs:meta])*
            $vis:vis $(safe$($_s:block)?)? $(unsafe$($_u:block)?)?
            fn
            $($js_fn:literal)?
            $fn:ident
            ($($param:ident: $param_ty:ty),* $(,)?) $(-> $fn_return:ty)?;
        )*
    ) => {
        $( #[link(wasm_import_module = $module)] )?
        unsafe extern "C" { $(
            $(#[$fn_attrs])*
            $( #[link_name = $js_fn] )?
            $vis $(safe$($_s)?)? $(unsafe$($_u)?)?
            fn $fn($($param: $param_ty),*) $(-> $fn_return)?;
        )* }
    };
}
/// Dummy safe fallback of the real `_js_reexport!` macro.
#[rustfmt::skip]
#[cfg(any(not(feature = "unsafe_ffi"), windows))]
macro_rules! _js_reexport { ($($tt:tt)*) => {}; }

#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
pub(crate) use _js_reexport as js_reexport;
