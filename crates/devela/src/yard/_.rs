//
#![cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#![cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
//
#![doc = crate::_tags!(internal)]
#![doc = "Scaffolding, taxonomy, and documentation support."] // internal
#![doc = crate::_doc!(modules: crate; yard)]
#![doc = crate::_doc!(hr)]
//!
//! > This space exist so the rest can be clean.
//

crate::mods_in! {
        mod_ _doc; // _doc[_<availability|location|meta|size_of|vendor|warn_miri>], _TAG*, tags!, …
        mod _env; // __dbg!, __std!, _std_core!
        mod _use; // _use!

        #[cfg(root_code)]
        mod alias; // aliases for attributes and derives
        mod fragments; // doclines for root modules: _DOC_*!
        mod tags; // docs for tagging items: _ABBR_!*, _TAG_*!, _tags!
}
crate::mods_out! { // _reexports, _crate_internals, _hidden
    // RETHINK
    // _mods {
    //     pub use super::{
    //         _doc::_all::*,
    //     }
    // }
    _reexports {
        #[doc(inline)]
        pub use crate::boot::_policy::_devela_policy;
        pub use super::_use::_use;
    }
    _crate_internals {
        pub(crate) use super::{
            _doc::_crate_internals::*,
            _env::*,
            _use::_use,
            //
            fragments::*,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use crate::boot::_policy::{_devela_policy, __devela_unreachable_unchecked};
        #[doc(hidden)]
        pub use super::{
            _doc::_hidden::*,
            _use::_use_or_shim,
            //
            fragments::{_ABBR_PCG, _ABBR_PRNG},
            tags::*, // needed by _tags!
        };
        #[doc(hidden)]
        #[cfg(root_code)]
        pub use super::alias::*;
    }
}
