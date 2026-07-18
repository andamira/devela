// devela/src/ui/view/scale/mod.rs
//
#![doc = crate::_DOC_UI_VIEW_SCALE!()] // private
#![doc = crate::_doc!(modules: crate::ui::view; scale)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

mod cell; // UiCellMetric
mod density; // UiDensity
mod round; // UiRound
// mod scale; // UiScaleProfile, UiTextScale

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            cell::*,
            density::*,
            round::*,
            // scale::*,
        };
    }
}
