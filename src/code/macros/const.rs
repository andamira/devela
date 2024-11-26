// devela::code::macros::const
//
//!
//

/// A helper for constructing macro constants.
///
/// # Examples
/// ```
/// # use devela::CONST;
/// CONST!{LITERAL = 125}
/// const L1: bool = LITERAL![] == 125_u8;
/// const L2: bool = LITERAL![] == 125_i32;
/// assert![L1];
/// assert![L2];
///
/// CONST!{BLOCK = {2 * 15 / 3}}
/// assert_eq![BLOCK![], 10];
///
/// CONST!{
///     /// Supports attributes and visibility.
///     #[macro_export]
///     pub(crate) ARRAY = {[1,2,3]}
/// }
/// assert_eq![ARRAY![], [1u8, 2, 3]];
/// assert_eq![ARRAY![], [1i32, 2, 3]];
/// ```
#[rustfmt::skip]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! CONST {
    (
        $(#[$attrs:meta])*
        $vis:vis $NAME:ident = $literal:literal
    ) => {
        $(#[$attrs])*
        #[allow(unused_macro)]
        macro_rules! $NAME { () => { $literal } }
        $vis use $NAME;
    };
    (
        $(#[$attrs:meta])*
        $vis:vis $NAME:ident = $block:block
    ) => {
        $(#[$attrs])*
        #[allow(unused_macro)]
        macro_rules! $NAME { () => { $block } }
        $vis use $NAME;
    };
}
#[doc(inline)]
pub use CONST;
