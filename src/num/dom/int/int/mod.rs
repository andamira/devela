// devela::num::dom::int::int
//
//! Defines [`Int`], [`define_int!`]←WIP.
//

pub(crate) mod _docs; // _INT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()

// mod define; // define_int! WIP
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
