// devela::num::ord
//
#![doc = crate::_DOC_NUM_ORD!()]
//!
#![doc = crate::_doc!(extends: cmp)]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/num/ord/_reexport.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport_core::*;
        #[doc(inline)]
        pub use devela_base_core::num::ord::{Cmp, cmp};
    }
}
