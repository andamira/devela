// devela_macros::lib
//
//!
//

#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate self as devela_macros;
use proc_macro::TokenStream as TS;

mod bodies;
use bodies::*;

#[doc = include_str!("docs/enumint.md")]
#[doc = concat!("# Example\n```\n", include_str!("../examples/enumint.rs"), "\n```")]
#[proc_macro] #[rustfmt::skip]
pub fn enumint(input: TS) -> TS { body_enumint(input) }
