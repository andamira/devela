// devela::run::app
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_RUN_APP!()] // private
#![doc = crate::_doc!(modules: crate::run; app)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]
//

mod control;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            control::*,
        };
    }
}
