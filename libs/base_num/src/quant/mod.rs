// devela_base_num::quant
//
#![doc = crate::_DOC_NUM_QUANT!()]
//

mod value; // ValueQuant

crate::structural_mods! { // _mods
    _mods {
        pub use super::value::*;
    }
}
