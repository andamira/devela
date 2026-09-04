// devela/src/num/grain/lim/_.rs
//
//! Boundary-aware integer representations and arithmetic.
//!
//! Includes bounded carriers, normalized scalars,
//! and operations defined relative to explicit ranges or unit boundaries.
//!
//! Storage niches and reserved representations remain orthogonal
//! and are provided by [`crate::num::grain::niche`].
//

crate::mods_in! {
    mod_ bound; // bound_int!
    // mod_ norm; //
}
crate::mods_out! { // _mods, _crate_internals, _hidden
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
    _hidden {
        pub use super::{
            bound::_hidden::*,
        };
    }
}
