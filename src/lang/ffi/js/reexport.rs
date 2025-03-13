// devela::lang::ffi::js::reexport
//
//! Defines the [`js_reexport`] macro.
//

/// Helps re-exporting javascript functions.
///
/// # Example
/// ```
/// # use devela::js_reexport;
/// js_reexport! {
///     [ module: "env" ]
///     pub safe fn same_fn_name(x: f64, y: f64, w: f64, h: f64);
///     pub(crate) safe fn "js_fn_name" rust_fn_name(x: f64, y: f64, w: f64, h: f64);
///     unsafe fn "js_fn" rs_fn(ptr: *const u8, len: usize, x: f64, y: f64);
/// }
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
#[doc(hidden)]
#[macro_export]
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

        [
        $(module: $module:literal)? $(,)?
        ]
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

        /* // how it's manually done:
        #[link(wasm_import_module = "api_canvas")]
        unsafe extern "C" {
            #[link_name = "fill_rect"]
            pub safe fn fill_rect(x: f64, y: f64, w: f64, h: f64);
            pub safe fn set_color(r: u8, g: u8, b: u8);
        }
        */
    };
}
#[doc(inline)]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
pub use _js_reexport as js_reexport;
