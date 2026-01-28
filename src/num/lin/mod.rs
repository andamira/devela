// devela::num::lin
//
#![doc = crate::_DOC_NUM_LIN!()]
//!
#![doc = crate::_doc!(flat:"num")]
//

mod matrix; // Matrix*
// mod optim;
mod vector; // Vector*

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            matrix::_all::*,
            // optim::_all::*,
            vector::_all::*,
        };
    }
}
