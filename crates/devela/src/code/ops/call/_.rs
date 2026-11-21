//
#![doc = crate::_DOC_CODE_OPS_CALL!()] // public
#![doc = crate::_doc!(modules: crate::code::ops; call)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: ops)]
//

crate::mods_in! {
    #[cfg(feature = "code")]
    mod semantics; // Call[Semantics|BindTime|Context|Dispatch|Openness|Storage]
}
crate::mods_out! { // _mods, _reexports
    _mods {
        #[cfg(feature = "code")]
        pub use super::{
            semantics::*,
        };
    }
    _reexports {
        pub use crate::boot::rust::code::ops_call::*;
    }
}
