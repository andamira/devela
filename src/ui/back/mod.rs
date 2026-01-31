// revela::ui::back
//
#![doc = crate::_DOC_UI_BACK!()] // public
#![doc = crate::_doc!(modules: crate::ui; back: miniquad, crossterm)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

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
        pub use super::definition::*;
    }
    _pub_mods {
        #[cfg(feature = "dep_crossterm")]
        pub use super::crossterm::_all::*;
        #[cfg(feature = "dep_miniquad")]
        pub use super::miniquad::_all::*;
    }
}
