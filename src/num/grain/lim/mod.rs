// devela/src/num/grain/lim/mod.rs
//
//! Boundary-aware integer representations and arithmetic.
//!
//! Includes bounded carriers, normalized scalars,
//! and operations defined relative to explicit ranges or unit boundaries.
//!
//! Storage niches and reserved representations remain orthogonal
//! and are provided by [`crate::num::grain::niche`].
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
