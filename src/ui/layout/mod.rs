// devela/src/ui/layout/mod.rs
//
#![doc = crate::_DOC_UI_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::ui; layout)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod _test;

mod receipt; // Layout1d, LayoutReceipt
mod unit; // Lunit, aliases: Ui<Ext|Pos|Rect|Stride|>

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            receipt::*,
            unit::*,
        };
    }
}
