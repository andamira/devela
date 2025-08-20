// devela_base::code::util::reexports
//
//! Reexported macros and hints.
//

#![allow(unused_imports)]

// #[doc = crate::TAG_DEVELA_BASE_MACROS!()] // FIXME: update CONST! macro
#[doc = concat!(super::SPAN_OPEN!(),
"'defined in `devela_base_macros` workspace library'>", "<small>base_macros</small></span>")]
pub use devela_base_macros::{
    cif, coalesce, compile, compile_attr, compile_doc, field_of, ident_total,
    ident_total_unique, ident_unique,
};
