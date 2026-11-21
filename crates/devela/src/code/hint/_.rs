//
#![doc = crate::_DOC_CODE_HINT!()] // public
#![doc = crate::_doc!(modules: crate::code; hint)]
#![doc = crate::_doc!(flat: "code")]
#![doc = crate::_doc!(extends: hint)]
//

crate::mods_in! {
    mod likely;
}
crate::mods_out! { // _reexports
    _mods {
        pub use super::{
            likely::{likely, unlikely}
        };
    }
    _reexports {
        pub use crate::boot::rust::code::hint::*;
    }
}
