// devela_base::code::util
//
//! Utility macros and hint functions.
//

crate::items! {
    #[doc(hidden)] pub mod _tags; // EMOJI_*! TAG_*!
    #[doc(hidden)] pub mod _reexport; // reexport!, reexport_from!
}

mod reexports; // re-exported macros from devela_base_macros

mod cdbg; // cdbg!
mod define_error; // define_error!
mod deprecate; // deprecate_feature!
mod include; // include_from!, mod_from!
mod items; // items!, sf!
mod is; // is!
mod paste; // paste! (wrapped for docs)
mod r#const; // CONST!

#[doc(hidden)]
pub use paste::__paste; // (called from paste!)

crate::items! { // structural access: _mods, _workspace_private, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _workspace_private::*;

    mod _mods {
        pub use super::{
            cdbg::*, define_error::*, deprecate::*, include::*, items::*, is::*, paste::*,
            r#const::*, reexports::*,
        };
    }
    pub(super) mod _workspace_private { #[allow(unused_imports)]
        pub use super::{_tags::*, _reexport::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
