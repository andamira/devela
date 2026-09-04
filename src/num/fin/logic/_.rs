// devela/src/num/fin/logic/_.rs
//
#![doc = crate::_DOC_NUM_FIN_LOGIC!()] // public
#![doc = crate::_doc!(modules: crate::num::fin; logic)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod bool; // ConstBool, False, True
    // mod bops;
    // mod choice;
    // mod items;
    // mod linear; // WIP
    // mod trool;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            bool::{ConstBool, False, True, const_bool},
            // bops::*;
            // choice::*;
            // items::*;
            // linear::*;
            // trool::*;
        };
    }
}
