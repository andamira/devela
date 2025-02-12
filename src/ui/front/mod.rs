// revela::ui::front
//
//! UI frontends.
//

crate::items! { // structural access: _pub_mods, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods { #![allow(unused)]
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
