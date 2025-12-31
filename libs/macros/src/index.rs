// devela_macros::index
//
//!
//

#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate devela_base_std as base;

extern crate self as devela_macros;
use proc_macro::TokenStream as TS;

mod bodies;
use bodies::*;

#[doc = devela_base_core::_TAG_CONSTRUCTION!()]
#[doc = devela_base_core::_TAG_NICHE!()]
#[doc = devela_base_core::_TAG_PROCEDURAL_MACRO!()]
#[doc = include_str!("docs/enumint.md")]
#[doc = concat!("# Example\n```\n", include_str!("../examples/enumint.rs"), "\n```")]
#[proc_macro] #[rustfmt::skip]
pub fn enumint(input: TS) -> TS { body_enumint(input) }
