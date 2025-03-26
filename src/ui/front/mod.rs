// revela::ui::front
//
//! UI frontends.
//

#[cfg(feature = "term")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
pub mod term;

crate::items! { // structural access: _mods, _pub_mods, _all
    // #[allow(unused)]
    // pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    // mod _mods { #![allow(unused)]
    // }
    mod _pub_mods { #![allow(unused)]
        #[cfg(feature = "term")]
        pub use super::term::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        // pub use super::_mods::*;
        pub use super::_pub_mods::*;
    }
}
