//
#![doc = crate::_DOC_NUM_FIN_ORD!()] // public
#![doc = crate::_doc!(modules: crate::num::fin; ord)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: cmp)]
//

crate::mods_in! {
    mod_ cmp; // Cmp, cmp!
    mod_ order; // Order
}
crate::mods_out! { // _mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            cmp::_all::{Cmp, cmp},
            order::_all::*,
        };
    }
    _reexports {
        pub use crate::boot::rust::num::fin_ord::*;
    }
    _crate_internals {
    }
}
