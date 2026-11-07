//
//! Public facade for the bootstrapped `mods_out!` macro.
//!
//! Normal builds re-export the implementation directly. Rustdoc uses a
//! forwarding macro to avoid duplicating documentation across re-exports.
//

#[cfg(not(doc))]
pub use crate::boot::mods_out;

#[cfg(doc)]
#[doc(hidden)]
#[doc = crate::_tags!(code)]
/// Defines a standardized module structure for organizing visibility and re-exports.
#[doc = crate::_doc_meta!{
    location("code/util/synth", macro mods_out),
}]
#[doc = include_str!("../../../boot/mods_out.md")]
#[macro_export]
macro_rules! __devela_mods_out_docs {
    ($($tokens:tt)*) => {
        $crate::mods_out·! { $($tokens)* }
    };
}
#[cfg(doc)]
#[doc(inline)]
pub use __devela_mods_out_docs as mods_out;

// NOTE: this duplicates the added docs, multiple times.
//
// #[doc = crate::_tags!(code)]
// /// Defines a standardized module structure for organizing visibility and re-exports.
// #[doc = crate::_doc_meta!{
//     location("code/util/synth", macro mods_out),
// }]
// pub use crate::boot::mods_out;
