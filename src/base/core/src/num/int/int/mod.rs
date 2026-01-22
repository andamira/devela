// devela_base_core::num::int::int
//
//! Defines [`Int`], [`define_int!`]←WIP.
//

pub(crate) mod _docs; // _INT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()

// mod define; // define_int! WIP
mod wrapper; // Int

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // define::*,
            wrapper::_all::*,
        };
    }
    _workspace_internals {
        pub use super::_docs::*;
    }
}
