// devela_base_num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

pub(crate) mod _docs; // FORMULA_*!()
mod wrapper; // Int

crate::structural_mods! { // _mods
    _mods {
        pub use super::wrapper::_all::*;
    }
    _workspace_internals {
        pub use super::_docs::*;
    }
}
