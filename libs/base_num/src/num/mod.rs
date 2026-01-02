// devela_base_num::num
//
#![doc = crate::_DOC_NUM!()]
#![doc = crate::_doc!(modules: crate; num: quant)] // error, geom, logic, niche, ord, rand
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: cmp, num)]
//

mod _internals; // upcasted_op!

mod int; // Int

pub mod quant; // ValueQuant

crate::structural_mods! { // _mods, _pub_mods, _workspace_internals
    _mods {
        pub use super::int::_all::*;
    }
    _pub_mods {
        pub use super::quant::_all::*;
    }
    _workspace_internals {
        pub use super::{
            _internals::*,
            int::_docs::*,
        };
    }
}
