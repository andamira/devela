// devela/src/ui/frame/mod.rs
//
#![doc = crate::_DOC_UI_FRAME!()] // public
#![doc = crate::_doc!(modules: crate::ui; frame)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

mod id; // UiId, UiKey, UiScope
mod frame; // UiFrame, UiPhase
// mod mem; // UiMemory
// mod cache; // UiCache
mod output; // UiOutput

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            id::*,
            frame::*,
            output::*,
        };
    }
}
