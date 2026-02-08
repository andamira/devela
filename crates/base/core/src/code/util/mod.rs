// devela_base_core::code::util
//
#![doc = crate::_DOC_CODE_UTIL!()] // public
#![doc = crate::_doc!(modules: crate::code; util)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: hint)]
//

#[cfg(test)]
mod tests;

mod _reexport; // SYMLINK from /src/code/util/_reexport_core.rs

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
mod paste; // paste!, (wrapped for docs)
mod structural; // structural_mods!
mod type_count; // type_count!
mod unlikely; // cold_path, likely, unlikely
mod whilst; // whilst!
mod write; // write_at!

structural::structural_mods! { // _mods, _reexports, _workspace_internals
    _mods {
        // NOTE: in sync with /devela/code/util/mod.rs:
        #[doc(inline)]
        pub use super::{
            asserts::{assert_eq_all, assert_approx_eq_all, const_assert},
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
            unlikely::{cold_path, likely, unlikely},
            whilst::whilst,
            write::write_at,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
    _workspace_internals {
        pub use super::{
            doclink::DOCLINK_CUSTOM_DOMAIN,
            paste::__paste,
        };
    }
}
