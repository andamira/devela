// devela_base_num::num
//
#![doc = crate::_DOC_NUM!()]
// #![doc = crate::_doc!(modules: crate; num: )] // error, geom, logic, niche, ord, quant, rand
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: cmp, num)]
//

mod _internals; // upcasted_op!

mod int; // Int
mod quant; // ValueQuant

crate::structural_mods! { // _mods
    _mods {

        pub use super::{
            int::_all::*,
            quant::_all::*,
        };
    }
    _workspace_internals {
        pub use super::{
            _internals::*,
            int::_docs::*,
        };
    }
}
