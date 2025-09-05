// devela::text::str::macros
//
//! Defines the [`str!`] macro.
//

#[cfg(feature = "dep_const_str")]
mod dep_const_str; // str!
#[cfg(feature = "dep_const_str")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_const_str")))]
pub use dep_const_str::*;
