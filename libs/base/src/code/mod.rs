// devela_base::code
//
#![doc = crate::_DOC_CODE!()]
//

mod reexports;

pub mod error; // ExtError, errors definitions
pub mod ops; // ::core::ops::*
pub mod util; // utility macros and functions

crate::items! { // structural access: _mods, _pub_mods, _workspace_private, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods {
        pub use super::reexports::*;
    }
    mod _pub_mods {
        pub use super::{
            error::_all::*, ops::*, util::_all::*,
        };
    }
    pub(super) mod _workspace_private { #[allow(unused_imports)]
        pub/*workspace*/ use super::util::_workspace_private::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
