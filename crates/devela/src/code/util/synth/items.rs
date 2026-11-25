//
//! Public facade for the bootstrapped `items!` macro.
//!
//! Normal builds re-export the implementation directly. Rustdoc uses a
//! forwarding macro to avoid duplicating documentation across re-exports.
//

#[cfg(not(doc))]
pub use crate::boot::items;

#[cfg(doc)]
#[doc(hidden)]
#[doc = crate::_tags!(code fmt)]
/// Groups *`items`* together and expands them as if they were written directly.
#[doc = crate::_doc_meta!{
    location("code/util/synth", macro items),
}]
#[doc = include_str!("../../../boot/synth/items.md")]
#[macro_export]
macro_rules! __devela_items_docs {
    ($($tokens:tt)*) => { $crate::items·! { $($tokens)* } };
}
#[cfg(doc)]
#[doc(inline)]
pub use __devela_items_docs as items;
