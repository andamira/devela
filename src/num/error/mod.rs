// devela::num::error
//
#![doc = crate::_DOC_NUM_ERROR!()] // public
#![doc = crate::_doc!(modules: crate::num; error)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod definitions;

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[doc(inline)]
        pub use super::{
            definitions::*,
        };
    }
    _reexports {
        pub use devela_base_core::num::error::{
            // individual errors:
            IncompatibleBounds,
            NoInverse,
            MismatchedSizes,
            NonNegativeRequired,
            PositiveRequired,
            NonZeroRequired,
            Overflow,
            // composite errors:
            IntError, IntResult,
            NicheValueError,
        };
    }
}
