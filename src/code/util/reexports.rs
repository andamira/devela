// devela::code::util::reexports
//
//! Reexported macros and hints.
//

crate::mod_path!(+pub _c "../../../libs/base/src/code/util/reexports.rs");

#[doc(inline)] #[rustfmt::skip]
pub use devela_base::{
    CONST,
    assert_eq_all, assert_approx_eq_all,
    capture_first, capture_last, capture_tail_tuple,
    cfg_if,
    cfor,
    const_assert,
    define_error,
    deprecate_feature,
    enumset,
    ident_const_index,
    impl_trait,
    include_from, mod_from, mod_path,
    is,
    items, sf,
    maybe,
    methods_as_fns,
    paste,
    structural_mods,
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
