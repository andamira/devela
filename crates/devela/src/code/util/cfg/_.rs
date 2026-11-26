//
#![doc = crate::_DOC_CODE_UTIL_CFG!()] // public
#![doc = crate::_doc!(modules: crate::code::util; cfg)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod deprecate; // deprecate_feature!
}
crate::mods_out! { // _mods, _reexports,
    _mods {
        #[doc(inline)]
        pub use super::{
            deprecate::deprecate_feature,
        };
    }
    _reexports {
        pub use crate::boot::rust::code::util_cfg::*;

        #[doc = crate::_tags!(code procedural_macro)]
        pub use crate::boot::synth::{cif, compile, compile_attr};
    }
}
