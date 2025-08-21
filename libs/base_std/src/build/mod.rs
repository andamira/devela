// devela_base::build
//
//! Build utilities.
//

devela_base::items! { // structural access: _pub_mods, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods { #![allow(unused_imports)]
        // pub use super::{
        //     util::_all::*,
        // };
    }
    pub(super) mod _internals { #![allow(unused_imports)]
    }
    pub(super) mod _all { #![allow(unused_imports)]
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
