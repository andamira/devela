// devela_base_core::code::util::items
//
//! Defines skip formatting macros ([`items`], [`sf`]).
//

#[doc = crate::_TAG_CODE!()]
#[doc = crate::_TAG_FMT!()]
/// Groups *`items`* together and expands them as if they were written directly.
#[doc = crate::_doc!(location: "code/util")]
///
/// It can be useful to apply an attribute to a group of items.
///
/// It can also preserve the formatting of the code provided as arguments,
/// but the [`sf`] macro is better for that, since it works with any arbitrary
/// code sequences like statements, expressions… instead of with just Rust items.
///
/// # Examples
/// ```
/// # use devela_base_core::items;
/// #[cfg(feature = "something")]
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

#[doc = crate::_TAG_CODE!()]
#[doc = crate::_TAG_FMT!()]
/// *`s`kip `f`ormatting* macro.
#[doc = crate::_doc!(location: "code/util")]
///
/// Preserves the formatting of the code provided as arguments, by relying on
/// the fact that `rustfmt` does not usually apply formatting inside macros.
///
/// *Rust will format macros only if they use parenthesis `()`
/// and the input is separated by commas, as if it were a function call.*
///
/// This macro can be used as an alternative to the `#[rustfmt::skip]` attribute,
/// specially in places where it can't be applied yet on stable rust.
///
/// # Examples
/// ```
/// # use devela_base_core::sf;
/// // rustfmt has no powers here
/// sf! { println!(); for i in 0..3 { print!{"{i} "} } println!(); }
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! sf { ( $($line:tt)+ ) => { $($line)+ }; }
#[doc(inline)]
pub use sf;
