// devela::code::macros::const
//
//!
//

/// A helper for constructing macro constants.
///
/// # Examples
/// ```
/// # use devela::CONST;
/// CONST!{EXPR = 2 * 15 / 3}
/// assert_eq![EXPR![], 10u8];
/// assert_eq![EXPR![], 10i8];
///
/// CONST!{
///     /// Supports attributes and visibility.
///     #[macro_export]
///     pub(crate) ARRAY = [1, 2, 3]
/// }
/// assert_eq![ARRAY![], [1u16, 2, 3]];
/// assert_eq![ARRAY![], [1i32, 2, 3]];
/// ```
#[rustfmt::skip]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! CONST {
    (
        $(#[$attrs:meta])*
        $vis:vis $NAME:ident = $expr:expr
    ) => {
        $(#[$attrs])*
        #[allow(unused_macro)]
        macro_rules! $NAME { () => { $expr } }
        $vis use $NAME;
    };
}
#[doc(inline)]
pub use CONST;
