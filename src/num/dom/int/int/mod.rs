// devela::num::dom::int::int
//
//! Defines [`Int`], [`int!`]←WIP.
//

pub(crate) mod _docs; // _INT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()

// mod define; // int! WIP
mod wrapper; // Int

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            // define::*,
            wrapper::_all::*,
        };
    }
    _crate_internals {
        pub use super::_docs::*;
    }
}
