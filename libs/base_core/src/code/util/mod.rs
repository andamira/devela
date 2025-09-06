// devela_base_core::code::util
//
#![doc = crate::_DOC_CODE_UTIL!()]
//

/* private to the workspace */
mod _doc; // doc_!, doc_availability! `doc_link!`, doc_miri_warn!, std_core!
mod _links; // DOCLINK_*!
mod _mod_docs; // _DOC_*!
mod _tags; // EMOJI_*! TAG_*!
mod _reexport; // reexport!, reexport_from!
mod _use; // _use!

mod reexports; // re-exported macros from devela_base_macros

mod asserts; // assertion macros
mod capture; // capture_[first|last|tail_tuple]!
mod cfg_if; // cfg_if!
mod cfor; // cfor!
mod deprecate; // deprecate_feature!
mod doclink; // doclink!
mod enumset; // enumset!
mod ident; // ident_const_index!
mod impl_trait; // impl_trait!
mod include; // include_from!, mod_from!
mod items; // items!, sf!
mod is; // is!
mod maybe; // maybe!
mod methods; // methods_as_fns
mod paste; // paste! (wrapped for docs)
mod r#const; // CONST!
mod structural; // structural_mods!

#[doc(hidden)]
pub use paste::__paste; // (called from paste!)

structural::structural_mods! { // _mods, _workspace_internals
    _mods {
        pub use super::{
            asserts::_all::*, capture::*, cfg_if::*, cfor::*, deprecate::*, doclink::*, enumset::*,
            ident::*, impl_trait::*, include::*, items::*, is::*, maybe::*, methods::*, paste::*,
            r#const::*, reexports::*, structural::*,
        };
    }
    _workspace_internals {
        pub use super::{_doc::*, _links::*, _mod_docs::*, _tags::*, _reexport::*, _use::*};
    }
}
