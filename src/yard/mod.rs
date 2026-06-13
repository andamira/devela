// devela/src/yard/mod.rs
//
#![cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#![cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
//
#![doc = crate::_tags!(internal)]
#![doc = crate::_DOC_YARD!()] // internal
#![doc = crate::_doc!(modules: crate; yard: dep)]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_YARD!()]
//

mod _doc; // _doc[_<availability|location|meta|size_of|vendor|warn_miri>], _ABBR*, _TAG*, tags!
mod _env; // __dbg!, __std!, _std_core!
mod _policy; // _devela_policy!
mod _reexport_macro; // _reexport!
mod _use; // _use!

mod alias; // aliases for attributes and derives
pub mod dep; // dependencies; reexported hidden from /index.rs
mod fragments; // doclines for root modules: _DOC_*!
mod tags; // docs for tagging items: _ABBR_!*, _TAG_*!, _tags!

// documented internal re-exports
#[doc(inline)]
pub use {_doc::_all::*, _policy::_devela_policy, _reexport_macro::_reexport, _use::_use};

crate::structural_mods! { // _crate_internals, _hidden
    _crate_internals {
        pub(crate) use super::{
            _doc::_crate_internals::*,
            _env::*,
            _reexport_macro::_reexport,
            _use::_use,
            //
            fragments::*,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            _doc::_hidden::*,
            _policy::{_devela_policy, __devela_unreachable_unchecked},
            _use::_use_or_shim,
            //
            alias::*,
            fragments::{_ABBR_PCG, _ABBR_PRNG},
            tags::*, // needed by _tags!
        };
    }
}
