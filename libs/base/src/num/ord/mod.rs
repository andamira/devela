// devela_base::num::ord
//
#![doc = crate::_DOC_NUM_ORD!()]
//

mod compare; // `Compare`
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            compare::*,
            reexports::*,
        };
    }
}
