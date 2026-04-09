// devela::yard
//
#![cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#![cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
//
#![doc = crate::_tags!(internal)]
#![doc = crate::_DOC_YARD!()] // internal
#![doc = crate::_doc!(modules: crate; yard: _dep)]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_YARD!()]
//

mod _env; // __dbg!, __std!, _std_core!
mod _policy; // _devela_policy!
mod _reexport_macro; // _reexport!
mod _use; // _use!

pub mod _dep;

// documented internal re-exports RETHINK
// #[doc(inline)]
// pub use crate::{
//     _doc::_doc::{_doc, _doc_availability, _doc_location, _doc_miri_warn},
//     yard::{_policy::_devela_policy, _reexport_macro::_reexport, _use::_use},
// };

crate::structural_mods! { // _crate_internals, _hidden
    _crate_internals {
        pub(crate) use super::{
            _env::*,
            _reexport_macro::_reexport,
            _use::_use,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            _policy::{_devela_policy, __devela_unreachable_unchecked},
        };
    }
}
