// devela::num::lin
//
#![doc = crate::_DOC_NUM_LIN!()] // public
#![doc = crate::_doc!(modules: crate::num; lin)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

// mod decomp;
mod matrix; // Matrix*
// mod solve;
mod vector; // Vector*

crate::structural_mods! { // _mods
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
