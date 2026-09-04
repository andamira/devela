// devela/src/num/alg/_.rs
//
#![doc = crate::_DOC_NUM_ALG!()] // public
#![doc = crate::_doc!(modules: crate::num; alg)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ decomp;
    mod_ matrix; // Matrix*
    // mod_ solve;
    mod_ vector; // Vector*
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            // decomp::_all::*,
            matrix::_all::*,
            // solve::_all::*,
            vector::_all::*,
        };
    }
}
