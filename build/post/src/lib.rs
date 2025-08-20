// devela_postbuild::lib
//
//!
//

/* global configuration */
//
// lints
//
// (Most lints are defined in ::devela::Cargo.toml::lints)
#![deny(rustdoc::missing_debug_implementations)]
// nightly (flags)
#![cfg_attr(nightly_doc, feature(doc_cfg))]

/* imports */

extern crate self as devela_postbuild;
