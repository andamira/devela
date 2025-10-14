// devela_base_core::code::util
//
#![doc = crate::_DOC_CODE_UTIL!()]
//

/* private to the workspace */
mod _doc; // doc_!, doc_availability! `doc_link!`, doc_miri_warn!, std_core!
mod _links; // _DOCLINK_*!
mod _mod_docs; // _DOC_*!
mod _tags; // EMOJI_*! _TAG_*!
mod _reexport; // reexport!, reexport_from!
mod _use; // _use!

mod reexports;

mod asserts; // assertion macros
mod capture; // capture_[first|last|tail_tuple]!
mod cfg_if; // cfg_if!
mod cfor; // cfor!
mod r#const; // CONST!
mod deprecate; // deprecate_feature!
mod doclink; // doclink!
mod enumset; // enumset!
mod ident; // ident_const_index!
mod impl_trait; // impl_trait!
mod include; // include_from!, mod_from!, mod_path!
mod is; // is!
mod items; // items!, sf!
mod maybe; // maybe!
mod methods; // methods_as_fns!
mod paste; // paste! (wrapped for docs)
mod structural; // structural_mods!
mod write; // write_bytes!

structural::structural_mods! { // _mods, _workspace_internals
    _mods {
        // NOTE: in sync with /devela/code/util/reexports.rs:
        #[doc(inline)]
        pub use super::{
            reexports::*,
            //
            asserts::_all::*,
            capture::{capture_first, capture_last, capture_tail_tuple},
            cfg_if::cfg_if,
            cfor::cfor,
            r#const::CONST,
            deprecate::deprecate_feature,
            doclink::doclink,
            enumset::enumset,
            ident::ident_const_index,
            impl_trait::impl_trait,
            include::{include_from, mod_from, mod_path},
            is::is,
            items::{items, sf},
            maybe::maybe,
            methods::methods_as_fns,
            paste::paste,
            structural::structural_mods,
            write::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_doc::_std_core;
    }
    _workspace_internals {
        pub use super::{
            _doc::{_doc, _doc_availability, _doc_miri_warn},
            _links::*, _mod_docs::*, _tags::*,
            _reexport::_reexport,
            _use::_use,
            doclink::DOCLINK_CUSTOM_DOMAIN,
            paste::__paste,
        };
    }
}
