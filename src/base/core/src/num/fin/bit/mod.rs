// devela_base_core::num::fin::bit
//
#![doc = crate::_DOC_NUM_FIN_BIT!()] // private
#![doc = crate::_doc!(modules: crate::num::fin; bit)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

// internals
// mod _benches;
mod _docs;

mod ops; // BitOps
mod wise; // Bitwise

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ops::*,
            wise::*,
        };
    }
}
