// devela/src/ui/view/draw/mod.rs
//
#![doc = crate::_DOC_UI_VIEW_DRAW!()] // private
#![doc = crate::_doc!(modules: crate::ui::view; draw)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

mod draw; // UiDraw, UiDrawKind
mod list; // UiDrawList, UiDrawListView

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            draw::*,
            list::*,
        };
    }
}
