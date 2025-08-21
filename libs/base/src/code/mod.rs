// devela_base::code
//
//! Code reflective synthesis.
//

pub mod error; // ExtError, errors definitions
pub mod util; // utility macros and functions

crate::items! { // structural access: _pub_mods, _workspace_private, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods {
        pub use super::{
            error::_all::*, util::_all::*,
        };
    }
    pub(super) mod _workspace_private { #[allow(unused_imports)]
        pub/*workspace*/ use super::util::_workspace_private::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
