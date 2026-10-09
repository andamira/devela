//
//! Ecosystem extensions and adapters for devela.
//

// #![deny(rustdoc::missing_debug_implementations)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

/* imports */

extern crate self as devela_extend;

pub use devela;
pub use devela::all::*;
