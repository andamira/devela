// devela/src/num/grain/lim/mod.rs
//
//! Bounded numeric carriers and range-preserving arithmetic.
//

mod bound; // bound_int!
// mod norm; //

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            bound::_all::*,
            // norm::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            bound::_crate_internals::*,
        };
    }
}
