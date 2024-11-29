// devela::code::macros::const

/// A helper for constructing macro constants.
///
/// # Examples
/// ```
/// # use devela::CONST;
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
/// // Supports multiple definitions, ended by ;
/// CONST! {
///     DOC_1 = "Doc 1." ;
///     pub(crate) DOC_2 = "Doc 2.";
/// }
/// assert_eq![DOC_1![], "Doc 1."];
/// assert_eq![DOC_2![], "Doc 2."];
///
/// // A good use-case is for repeated documentation
/// /// Function 1, version a.
/// #[doc = DOC_1!()]
/// pub fn version_1a() {}
/// /// Function 1, version b.
/// #[doc = DOC_1!()]
/// pub fn version_1b() {}
///
/// // It also supports…
/// CONST!{ /* …empty declarations */ }
/// ```
// NOTE: Fixed to use it from the root: https://github.com/rust-lang/rust/pull/52234
#[doc(hidden)]
#[macro_export]
macro_rules! _CONST {
    ($(
        $(#[$attrs:meta])*
        $vis:vis $NAME:ident = $expr:expr
     );* $(;)?) => {
        $(
            $(#[$attrs])*
            #[allow(unused_macro)]
            macro_rules! $NAME { () => { $expr } }
            $vis use $NAME;
        )*
    };
}
#[doc(inline)]
pub use _CONST as CONST;
