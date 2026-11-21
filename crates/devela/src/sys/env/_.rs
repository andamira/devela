//
#![doc = crate::_DOC_SYS_ENV!()] // public
#![doc = crate::_doc!(modules: crate::sys; env)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: env)]
//

crate::mods_in! {
    mod_ arg;
    mod namespace;

}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            arg::_all::*,
            namespace::*,
        };

    }
    _reexports {
        pub use crate::boot::rust::sys::env::*;
    }
}
