// devela_base_core::text::layout
//
#![doc = crate::_DOC_TEXT_LAYOUT!()]
//!
//

#[cfg(test)]
mod tests;

mod engine; // TextLayout
mod result; // TextFit, TextLayoutStep
mod symbol; // TextCohesion, TextCursor, TextSpan, TextSymbol,
mod unit; // TextUnit, TextIndex

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            engine::*,
            result::*,
            symbol::*,
            unit::*,
        };
    }
}
