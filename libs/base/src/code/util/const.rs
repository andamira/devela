// devela_base::code::util::const
//
//! Defines the [CONST!] macro.
//

/// A helper for constructing macro constants.
///
/// It accepts either a series of expressions or a series of functions.
///
/// # Examples
/// ```
/// # use devela_base::CONST;
///
/// CONST!{ /* Supports empty declarations */ }
///
/// // Supports any expresion
/// CONST!{EXPR = 2 * 15 / 3}
/// assert_eq![EXPR![], 10u8];
/// assert_eq![EXPR![], 10i8];
///
/// CONST!{
///     /// Supports docs, attributes, and visibility.
///     #[macro_export] // attribute needed if public
///     pub ARRAY = [1, 2, 3]
/// }
/// assert_eq![ARRAY![], [1u16, 2, 3]];
/// assert_eq![ARRAY![], [1i32, 2, 3]];
///
/// // Supports multiple definitions of constant expressions, ended with ;
/// CONST! {
///     DOC_1 = "Document expression." ;
///     DOC_2 = "Document expression." ;
/// }
/// assert_eq![DOC_1![], "Document expression."];
///
/// // A good use-case is for repeated documentation
/// /// Function 1, version a.
/// #[doc = DOC_1!()]
/// pub fn version_1a() {}
/// /// Function 1, version b.
/// #[doc = DOC_1!()]
/// pub fn version_1b() {}
///
/// // Supports multiple definitions of constants functions, ended with ;
/// CONST! {
///     /// Supports *const* functions.
///     FN_1 =
///     /// Returns `n × 5`.
///     #[inline] #[must_use]
///     pub const fn fn_1(n: i32) -> i64 { (n * 5) as i64 };
///
///     /// You can repeat functions.
///     pub(crate) FN_2 = pub const fn fn_2(c: char) { };
///
///     /// Supports optional *unsafe*.
///     FN_3 = pub const unsafe fn fn_3() {};
///
///     // NOTE: It's not possible to mix expressions and functions in the same invocation.
///     // EXPR_ERR = "Compile fails if this line is uncommented";
/// }
/// pub struct Fns;
/// impl Fns {
///     FN_1!{}
///     FN_2!{}
///     FN_3!{}
/// }
///
/// assert_eq![Fns::fn_1(0i32), 0i64];
/// assert_eq![Fns::fn_1(5), 25];
/// let _: () = Fns::fn_2('a');
/// unsafe { Fns::fn_3(); }
///
/// // Supports giving a shared visibility for all defined constants
/// CONST! { pub(crate),
///     E1 = 1 + 1;
///     E2 = 2 + 2;
///     // pub E3 = 3 + 3; // shared visibility can't be overriden
/// }
/// CONST! { pub(crate),
///     F1 = pub const fn f1(a: i32) -> i32 { a + 1 };
///     F2 = pub const fn f2(a: i32) -> i32 { a + 2 };
///     // pub F3 = pub const fn f3(a: i32) -> i32 { a + 3 };
/// }
/// ```
// Related links
// - https://doc.rust-lang.org/reference/items/external-blocks.html#functions
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _CONST {
    (
    // Either multiple `const fn`
    $(
        $(#[$CONST_ATTRS:meta])*
        $item_vis:vis $CONST_NAME:ident =
            $(#[$fn_attrs:meta])*
            $fn_vis:vis const
            $(async$($_a:block)?)? $(safe$($_s:block)?)? $(unsafe$($_u:block)?)?
            fn $fn:ident($($param:ident: $param_ty:ty),* $(,)?)
        $(-> $fn_return:ty)?
        $fn_body:block
    );* $(;)?) => {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macros)]
            macro_rules! $CONST_NAME {
                () => {
                    $(#[$fn_attrs])*
                    $fn_vis const $(async$($_a)?)? $(safe$($_s)?)? $(unsafe$($_u)?)?
                    fn $fn($($param: $param_ty),*) $(-> $fn_return)? $fn_body
                }
            }
            $item_vis use $CONST_NAME;
        )*
    };
    (
    $shared_vis:vis, // (shared visibility alternative)
    $(
        $(#[$CONST_ATTRS:meta])*
        $CONST_NAME:ident =
            $(#[$fn_attrs:meta])*
            $fn_vis:vis const
            $(async$($_a:block)?)? $(safe$($_s:block)?)? $(unsafe$($_u:block)?)?
            fn $fn:ident($($param:ident: $param_ty:ty),* $(,)?)
        $(-> $fn_return:ty)?
        $fn_body:block
    );* $(;)?) => {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macros)]
            macro_rules! $CONST_NAME {
                () => {
                    $(#[$fn_attrs])*
                    $shared_vis const $(async$($_a)?)? $(safe$($_s)?)? $(unsafe$($_u)?)?
                    fn $fn($($param: $param_ty),*) $(-> $fn_return)? $fn_body
                }
            }
            $shared_vis use $CONST_NAME;
        )*
    };
    (

    // Either multiple expressions
    $(
        $(#[$CONST_ATTRS:meta])*
        $item_vis:vis $CONST_NAME:ident = $expr:expr
     );* $(;)?) => {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macro)]
            macro_rules! $CONST_NAME { () => { $expr } }
            $item_vis use $CONST_NAME;
        )*
    };
    (
    $shared_vis:vis, // (shared visibility alternative)
    $(
        $(#[$CONST_ATTRS:meta])*
        $CONST_NAME:ident = $expr:expr
     );* $(;)?) => {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macro)]
            macro_rules! $CONST_NAME { () => { $expr } }
            $shared_vis use $CONST_NAME;
        )*
    };
    (

    // Either multiple hidden macros exported
    hidden macro_export,
    $(
        $(#[$CONST_ATTRS:meta])*
        $CONST_NAME:ident = $expr:expr
     );* $(;)?) => { $crate::paste! {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macro)]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! [< _ $CONST_NAME >] { () => { $expr } }

            #[doc(hidden)]
            pub use [< _ $CONST_NAME >] as $CONST_NAME;
        )*
    }};
    (

    // Either multiple visible macros exported
    inline macro_export,
    $(
        $(#[$CONST_ATTRS:meta])*
        $CONST_NAME:ident = $expr:expr
     );* $(;)?) => { $crate::paste! {
        $(
            $(#[$CONST_ATTRS])*
            #[allow(unused_macro)]
            #[macro_export]
            #[cfg_attr(cargo_primary_package, doc(hidden))]
            macro_rules! [< _ $CONST_NAME >] { () => { $expr } }

            #[doc(inline)]
            pub use [< _ $CONST_NAME >] as $CONST_NAME;
        )*
    }};
}
#[doc(inline)]
pub use _CONST as CONST;
