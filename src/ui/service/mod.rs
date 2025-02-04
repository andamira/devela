// revela::ui::service
//
//! Ui services.
//

mod cap;
mod definition;

#[cfg(feature = "dep_crossterm")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_crossterm")))]
pub mod crossterm;
#[cfg(feature = "dep_miniquad")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_miniquad")))]
pub mod miniquad;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods {
        #[cfg(ui··)]
        pub use super::{cap::*, definition::*};
    }
    mod _pub_mods { #![allow(unused)]
        #[cfg(feature = "dep_crossterm")]
        pub use super::crossterm::_all::*;
        #[cfg(feature = "dep_miniquad")]
        pub use super::miniquad::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
