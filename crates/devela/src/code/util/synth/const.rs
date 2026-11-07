//
//! Public facade for the bootstrapped `CONST!` macro.
//!
//! Normal builds re-export the implementation directly. Rustdoc uses a
//! forwarding macro to avoid duplicating documentation across re-exports.
//

#[cfg(not(doc))]
pub use crate::boot::CONST;

#[cfg(doc)]
#[doc(hidden)]
#[doc = crate::_tags!(code construction)]
/// A helper for constructing macro constants.
#[doc = crate::_doc_meta!{
    location("code/util/synth", macro CONST),
}]
#[doc = include_str!("../../../boot/const.md")]
#[macro_export]
macro_rules! __devela_CONST_docs {
    ($($tokens:tt)*) => { $crate::CONST·! { $($tokens)* } };
}
#[cfg(doc)]
#[doc(inline)]
pub use __devela_CONST_docs as CONST;
