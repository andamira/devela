//
//! Numeric types for memory-efficient representations.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod_ bitten; // BittenU*
    mod non_value; // NonValue*, NonMax*, NonMin*
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            bitten::_all::*,
            non_value::*,
        };
    }
    _crate_internals {
        pub(crate) use super::bitten::_crate_internals::*;
    }
}
