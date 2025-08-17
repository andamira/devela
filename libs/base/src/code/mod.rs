// devela_base::code
//
//! Code reflective synthesis.
//

pub mod util; // utility macros and functions

crate::items! { // structural access: _pub_mods, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods {
        pub use super::{
            util::_all::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
