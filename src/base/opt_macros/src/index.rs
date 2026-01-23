// devela_macros::index
//
//!
//

#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate devela_base_std as base;
extern crate self as devela_macros;
macro_rules! __crate_name {
    () => {
        "devela_macros"
    };
}
pub(crate) use __crate_name;

use proc_macro::TokenStream as TS;

mod bodies;
use bodies::*;

// mod yard;
// mod _doc;

#[doc = base::_tags!(construction niche procedural_macro)]
#[doc = include_str!("docs/enumint.md")]
#[doc = base::_doc_location!("code/util")] // IMPROVE: not a valid route in this crate
#[doc = concat!("# Example\n```\n", include_str!("../examples/enumint.rs"), "\n```")]
#[proc_macro] #[rustfmt::skip]
pub fn enumint(input: TS) -> TS { body_enumint(input) }
