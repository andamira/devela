//
#![cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#![cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
//
#![doc = crate::_tags!(internal)]
#![doc = crate::_DOC_YARD!()] // internal
#![doc = crate::_doc!(modules: crate; yard)]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod fragments; // doclines for root modules: _DOC_*!
}
crate::mods_out! { // _crate_internals
    _crate_internals {
        pub(crate) use super::{
            fragments::*,
        };
    }
}
