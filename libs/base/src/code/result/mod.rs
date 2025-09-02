// devela_base::code::result
//
#![doc = crate::_DOC_CODE_RESULT!()]
//

mod chain_hook; // Chain, Hook
mod mismatch; // Mismatch
mod own; // Own
mod reexports;
mod value_quant; // ValueQuant

crate::structural_mods! { // _mods
    _mods {
        pub use super::{chain_hook::*, mismatch::*, own::*, reexports::*, value_quant::*};
    }
}
