// devela::code::paste
//

/// Allows to paste identifiers together.
///
#[doc = "*Reexported from the [`paste`](https://docs.rs/paste)* crate.\n\n---"]
///
/// Within the `paste!` macro, identifiers inside `[<`...`>]` are pasted
/// together to form a single identifier.
///
/// # Examples
/// ```
/// # use devela::paste;
/// paste! {
///     // Defines a const called `QRST`.
///     const [<Q R S T>]: &str = "success!";
/// }
///
/// fn main() {
///     assert_eq!(
///         paste! { [<Q R S T>].len() },
///         8,
///     );
/// }
/// ```
///
/// ## Case conversion
///
/// Use `$var:lower` or `$var:upper` in the segment list to convert an
/// interpolated segment to lower- or uppercase as part of the paste. For
/// example, `[<ld_ $reg:lower _expr>]` would paste to `ld_bc_expr` if invoked
/// with `$reg=Bc`.
///
/// Use `$var:snake` to convert CamelCase input to snake\_case.
/// Use `$var:camel` to convert snake\_case to CamelCase. These compose,
/// so for example `$var:snake:upper` would give you SCREAMING\_CASE.
///
/// The precise Unicode conversions are as defined by [`str::to_lowercase`][0]
/// and [`str::to_uppercase`][1].
///
/// [0]: https://doc.rust-lang.org/core/primitive.str.html#method.to_lowercase
/// [1]: https://doc.rust-lang.org/core/primitive.str.html#method.to_uppercase
///
/// ## Pasting documentation strings
///
/// Within the `paste!` macro, arguments to a `#[doc ...]` attribute are
/// implicitly concatenated together to form a coherent documentation string.
///
/// ```
/// # use devela::paste;
/// macro_rules! method_new {
///     ($ret:ident) => {
///         paste! {
///             #[doc = "Create a new `" $ret "` object."]
///             pub fn new() -> $ret { todo!() }
///         }
///     };
/// }
///
/// pub struct Paste {}
///
/// method_new!(Paste);  // expands to #[doc = "Create a new `Paste` object"]
/// ```
/// ---
/// ---
#[cfg_attr(feature = "nightly", doc(cfg(feature = "meta")))]
#[macro_export]
macro_rules! paste {
    ($($tt:tt)*) => {
        $crate::code::__paste!{ $($tt)* }
    }
}
#[doc(inline)]
pub use paste;
#[doc(hidden)] // dont export this
pub use paste_crate::paste as __paste;
