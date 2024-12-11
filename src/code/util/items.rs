// devela::code::util::items
//
//! skip formatting macros
//

/// *`s`kip `f`ormatting* wrapper macro.
///
/// Preserves the formatting of the code provided as arguments, by relying on
/// the fact that `rustfmt` does not usually apply formatting inside macros.
///
/// *Rust will format macros only if they use parenthesis `()` and the input is
/// separated by commas, then it formats is the same way as function call.*
///
/// This macro can be used as an alternative to the `#[rustfmt::skip]` attribute,
/// specially in places where it can't be applied yet on stable rust.
///
/// # Examples
/// ```
/// # use devela::code::sf;
/// // rustfmt has no powers here
/// sf! { println!(); for i in 0..3 { print!{"{i} "} } println!(); }
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! sf { ( $($line:tt)+ ) => { $($line)+ }; }
#[doc(inline)]
pub use sf;

/// Groups items together and expands them as if they were written directly.
///
/// It can be useful to apply an attribute to a group of items.
///
/// It can also preserve the formatting of the code provided as arguments,
/// but the [`sf`] macro is better for that, since it works with any arbitrary
/// code sequences like statements, expressions… instead of with just Rust items.
///
/// # Examples
/// ```
/// # use devela::code::items;
/// #[cfg(feature = "std")]
/// items! {
///     mod something {
///         pub struct SomeThing;
///     }
///     pub use something::SomeThing;
/// }
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! items { ( $($item:item)* ) => { $($item)* }; }
#[doc(inline)]
pub use items;
