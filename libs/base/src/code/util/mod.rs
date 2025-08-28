// devela_base::code::util
//
#![doc = crate::_DOC_CODE_UTIL!()]
//

/* private to the workspace */
mod _doc; // doc_!, doc_availability! doc_miri_warn!, std_core!
mod _mod_docs; // _DOC_*!
mod _tags; // EMOJI_*! TAG_*!
mod _reexport; // reexport!, reexport_from!

mod reexports; // re-exported macros from devela_base_macros

mod cfg_if; // cfg_if!
mod cfor; // cfor!
mod deprecate; // deprecate_feature!
mod impl_trait; // impl_trait!
mod include; // include_from!, mod_from!
mod items; // items!, sf!
mod is; // is!
mod paste; // paste! (wrapped for docs)
mod r#const; // CONST!

#[doc(hidden)]
pub use paste::__paste; // (called from paste!)

crate::items! { // structural access: _mods, _workspace_private, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _workspace_private::*;

    mod _mods {
        pub use super::{
            cfg_if::*, cfor::*, deprecate::*, impl_trait::*, include::*, items::*, is::*,
            paste::*, r#const::*, reexports::*,
        };
    }
    pub(super) mod _workspace_private { #[allow(unused_imports)]
        pub use super::{_doc::*, _mod_docs::*, _tags::*, _reexport::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
