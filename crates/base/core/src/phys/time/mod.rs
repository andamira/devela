// devela_base_core::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()] // public
#![doc = crate::_doc!(modules: crate::phys; time)]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(hr)]
//

mod _reexport; // SYMLINK from /src/phys/time/_reexport_core.rs

mod errors {
    use crate::{_TAG_TIME, define_error};

    define_error! { individual: pub struct Timeout;
        #[derive(Default)], +location: "phys/time", +tag: _TAG_TIME!(),
        DOC_KEY_ALREADY_EXISTS = "The operation has exceeded the allowed execution time.",
        self+f => write!(f, "The operation has exceeded the allowed execution time.")
    }
}

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::errors::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
