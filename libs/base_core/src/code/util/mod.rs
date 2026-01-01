// devela_base_core::code::util
//
#![doc = crate::_DOC_CODE_UTIL!()]
//

#[cfg(test)]
mod tests;

mod _doc; // doc_!, doc_availability! `doc_link!`, doc_miri_warn!
mod _env; // __dbg!, __std!, _std_core!
mod _links; // _DOCLINK_*!
mod _mod_docs; // _DOC_*!
mod _tags; // EMOJI_*! _TAG_*!
mod _reexport_macro; // reexport!
mod _use; // _use!

mod _reexport;

mod asserts; // (assertion macros)
mod capture; // capture_[first|last|tail_tuple]!
mod cfg_if; // cfg_if!
mod r#const; // CONST!
mod debug; // , compile_warning!, fn_name!
mod deprecate; // deprecate_feature!
mod doclink; // doclink!
mod enumset; // enumset!
mod ident; // ident_const_index!
mod impl_trait; // impl_trait!
mod include; // include_from!, mod_from!, mod_path!
mod is; // is!
mod items; // items!, sf!
mod lets; // lets!
mod maybe; // maybe!
mod methods; // methods_as_fns!
mod paste; // paste! (wrapped for docs)
mod structural; // structural_mods!
mod type_count; // type_count!
mod whilst; // whilst!
mod write; // write_at!

structural::structural_mods! { // _mods, _crate_internals, _workspace_internals
    _mods {
        // NOTE: in sync with /devela/code/util/_reexport.rs:
        #[doc(inline)]
        pub use super::{
            _reexport::*,
            //
            asserts::_all::*,
            capture::{capture_first, capture_last, capture_tail_tuple},
            cfg_if::cfg_if,
            r#const::CONST,
            debug::{compile_warn, fn_name},
            deprecate::deprecate_feature,
            doclink::doclink,
            enumset::enumset,
            ident::ident_const_index,
            impl_trait::impl_trait,
            include::{include_from, mod_from, mod_path},
            is::is,
            items::{items, sf},
            lets::lets,
            maybe::maybe,
            methods::methods_as_fns,
            paste::paste,
            structural::structural_mods,
            type_count::type_count,
            whilst::whilst,
            write::write_at,
        };
    }
    _crate_internals {
        pub(crate) use super::_env::{__dbg, __std, _std_core};
    }
    _workspace_internals {
        pub use super::{
            _doc::{_doc, _doc_availability, _doc_miri_warn},
            _links::*, _mod_docs::*, _tags::*,
            _reexport_macro::_reexport,
            _use::_use,
            doclink::DOCLINK_CUSTOM_DOMAIN,
            paste::__paste,
        };
    }
}
