// devela::code::util::reexports
//
//! Reexported macros and hints.
//

use crate::_reexport_from;

// from workspace base
_reexport_from!("../../../libs/base/src/code/util/reexports.rs", _c);

#[doc(inline)] #[rustfmt::skip]
pub use devela_base::{
    CONST,
    cdbg,
    cfg_if,
    define_error,
    deprecate_feature,
    include_from, mod_from,
    is,
    items, sf,
    paste,
    // devela_code_macros:
    // cif, compile, compile_attr, compile_doc,
    // ident_total, ident_total_unique, ident_unique,
    // coalesce, field_of,
};

#[cfg(feature = "devela_macros")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "devela_macros")))]
#[doc = crate::TAG_DEVELA_MACROS!()]
#[doc(inline)] #[rustfmt::skip]
pub use devela_macros::{
    enumint,
};
