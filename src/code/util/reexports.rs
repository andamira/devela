// devela::code::util::reexports
//
//! Reexported macros and hints.
//

crate::mod_path!(+pub _c "../../../libs/base_core/src/code/util/reexports.rs");

// NOTE: in sync with /libs/base_core/src/code/util/mod.rs:
#[doc(inline)] #[rustfmt::skip]
pub use devela_base_core::{
    assert_eq_all, assert_approx_eq_all, const_assert,
    capture_first, capture_last, capture_tail_tuple,
    cfg_if,
    cfor,
    CONST,
    deprecate_feature,
    doclink,
    enumset,
    ident_const_index,
    impl_trait,
    include_from, mod_from, mod_path,
    is,
    items, sf,
    lets,
    maybe,
    methods_as_fns,
    paste,
    structural_mods,
    write_at,
    // devela_code_macros:
    // cif, compile, compile_attr, compile_doc,
    // ident_total, ident_total_unique, ident_unique,
    // coalesce, field_of,
    // repeat,
};

#[cfg(feature = "devela_macros")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "devela_macros")))]
#[doc(inline)] #[rustfmt::skip]
pub use devela_macros::{
    enumint,
};
