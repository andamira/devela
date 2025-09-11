// devela_base_num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

pub(crate) mod _docs; // _FLOAT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()
// mod consts; // FloatConst
// mod wrapper; // Float

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     // consts::_all::*,
        //     // wrapper::_all::*,
        // };
    }
    _workspace_internals {
        pub use super::_docs::*;
    }
}
