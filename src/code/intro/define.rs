// devela::code::intro::define
//
//! Defines the [`Introspect`] trait.
//

/// Introspection core trait
///
/// Provides structural metadata about types and values.
/// Implement for any type to describe its fundamental properties.
///
// Minimal introspection interface.
///
/// ```ignore
/// # use devela::Introspect;
/// enum MyKind { A, B, C }
///
/// struct MyTypeA;
/// struct MyTypeB;
///
/// impl Introspect for MyTypeA {
///     type Kind = MyKind;
///     fn intro_kind(&self) -> Self::Kind { MyKind::A }
/// }
///
/// impl Introspect for MyTypeB {
///     type Kind = MyKind;
///     fn intro_kind(&self) -> Self::Kind { MyKind::B }
/// }
/// ```
pub trait Introspect {
    /// A type representing the kind of `Self`.
    type Kind;

    /// Returns the kind/type of the implementing value.
    fn intro_kind(&self) -> Self::Kind;
}
