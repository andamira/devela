//
#![doc = crate::_DOC_CODE_ANY!()] // public
#![doc = crate::_doc!(modules: crate::code; any)]
#![doc = crate::_doc!(flat: "code")]
#![doc = crate::_doc!(extends: any)]
//

crate::mods_in! {
    mod ext;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::ext::*;
    }
    _reexports {
        pub use crate::boot::rust::code::any::*;
    }
}
