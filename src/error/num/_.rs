// devela/src/error/num/_.rs
//
#![doc = crate::_DOC_ERROR_NUM!()] // public
#![doc = crate::_doc!(modules: crate::error; num)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod highest; // RETHINK
    mod define;
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            highest::*,
            define::*,
        };
    }
}
