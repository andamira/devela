// devela/src/code/util/synth/items.rs
//
//! Defines [`items`].
//

#[doc = crate::_tags!(code fmt)]
/// Groups *`items`* together and expands them as if they were written directly.
#[doc = crate::_doc_meta!{
    location("code/util/synth", macro items),
}]
/// It can be useful to apply an attribute to a group of items.
///
/// It can also preserve the formatting of the code provided as arguments,
/// but the [`sf!`][crate::sf] macro is better for that, since it works with any arbitrary
/// code sequences like statements, expressions… instead of with just Rust items.
///
/// # Examples
/// ```
/// use devela::items;
///
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
macro_rules! items· { ( $($item:item)* ) => { $($item)* }; }
#[doc(inline)]
pub use items· as items;
