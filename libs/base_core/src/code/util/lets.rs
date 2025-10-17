// devela_base_core::code::util::lets
//
//! Defines the [`lets!`] macro.
//
// IMPROVE: support enum variants with data (see enumset)

/// A concise macro for declaring multiple variables at once.
///
/// Supports two syntax styles:
/// - Individual assignments: `name = value`
/// - Enum scoping: `@Enum::{alias = Variant}`
///
/// # Examples
/// ```
/// # use devela_base_core::lets;
/// // Individual assignments
/// lets![name = "John", age = 30, active = true];
///
/// // Enum scoping - creates shortcuts to enum variants
/// enum Color { Red, Green, Blue }
/// lets![@Color::{R = Red, G = Green, B = Blue}];
/// // Equivalent to:
/// // let R = Color::Red;
/// // let G = Color::Green;
/// // let B = Color::Blue;
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _lets {
    (
    // individual assignments
    $($id:ident=$val:expr),+) => { $( let $id = $val; )+ };
    (
    // enum scoping
    @ $enum:ident::{ $($id:ident=$var:ident),+ } ) => {
        $( let $id = $enum::$var; )+
    };
}
#[doc(inline)]
pub use _lets as lets;
