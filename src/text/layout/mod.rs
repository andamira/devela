// devela/src/text/layout/mod.rs
//
#![doc = crate::_DOC_TEXT_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::text; layout)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod _tests;

mod config; // Text<Break|Width|Elide>Mode, TextSymbolConfig
mod engine; // TextLayout
mod line; // TextLineIter
mod result; // TextFit, TextLayoutStep
mod symbol; // TextCohesion, TextLayoutSpan, TextSymbol
mod textel; // Textel
mod wrap; // TextBreakKind, TextLine, TextSegment[Kind]

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            config::*,
            engine::*,
            line::*,
            result::*,
            symbol::*,
            textel::*,
            wrap::_all::*,
        };
    }
}
