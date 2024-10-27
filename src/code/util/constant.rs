// devela::code::util::constant
//
//!
//

/// A helper for constructing macro constants.
///
/// # Example
/// ```
/// # use devela::CONST;
/// // equal to a literal
/// CONST!{A = 125}
///
/// const A1: bool = A![] == 125_u8;
/// const A2: bool = A![] == 125_i32;
/// assert![A1];
/// assert![A2];
///
/// // equal to a block
/// CONST!{B = {2 * 15 / 3}}
/// assert_eq![B![], 10]
/// ```
#[rustfmt::skip]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! CONST {
    (
    // accepts a literal
    $NAME:ident = $literal:literal
    ) => {
        #[allow(unused_macro)]
        macro_rules! $NAME { () => {$literal} }
    };
    (
    // accepts a block
    $NAME:ident = $block:block
    ) => {
        #[allow(unused_macro)]
        macro_rules! $NAME { () => {$block} }
    };
}
#[doc(inline)]
pub use CONST;
