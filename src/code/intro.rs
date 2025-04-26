// devela::code::intro
//
//! defines the [`Introspect`] struct.
//

/// Minimal introspection interface.
///
/// ```
/// # devela::Introspect;
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
