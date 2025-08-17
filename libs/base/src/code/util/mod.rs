// devela_base::code::util
//
//! Utility macros and hint functions.
//

mod cdbg; // cdbg!
mod deprecate; // deprecate_feature!
mod include; // include_from!, mod_from!
mod items; // items!, sf!
mod is; // is!
mod paste; // paste! (wrapped for docs)
mod r#const; // CONST!

#[doc(hidden)]
pub use paste::__paste; // (called from paste!)

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{
            cdbg::*, deprecate::*, include::*, items::*, is::*, paste::*, r#const::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
