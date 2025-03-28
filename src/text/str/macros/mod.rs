// devela::text::str::macros
//
//! Defines the [`str!`] and [`strjoin!`] macros.
//

mod join; // strjoin!
pub use join::*;

#[cfg(feature = "dep_const_str")]
mod dep_const_str; // str!
#[cfg(feature = "dep_const_str")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_const_str")))]
pub use dep_const_str::*;
