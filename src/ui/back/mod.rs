// revela::ui::back
//
//! UI backends.
#![doc = crate::_doc!(modules: crate::ui; back: miniquad, crossterm)]
//

mod cap;
mod definition;

#[cfg(feature = "dep_crossterm")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_crossterm")))]
pub mod crossterm;
#[cfg(feature = "dep_miniquad")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "dep_miniquad")))]
pub mod miniquad;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        #[cfg(ui··)]
        pub use super::{cap::*, definition::*};
    }
    _pub_mods {
        #[cfg(feature = "dep_crossterm")]
        pub use super::crossterm::_all::*;
        #[cfg(feature = "dep_miniquad")]
        pub use super::miniquad::_all::*;
    }
}
