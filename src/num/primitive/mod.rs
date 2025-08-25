// devela::num::primitive
//
//! Helpers for converting between primitives.
//

mod cast; // PrimitiveCast
mod join; // PrimitiveJoin
mod split; // PrimitiveSplit

crate::items! { // structural access: _mods, _reexports, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{cast::*, join::*, split::*};
    }
    mod _reexports {
        pub use devela_base::Cast;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _reexports::*};
    }
}
