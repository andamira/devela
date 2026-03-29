// devela_base_core::text::layout
//
#![doc = crate::_DOC_TEXT_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::text; layout)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod tests;

// mod config; // WIP: Text[Break|Width|Elide]Mode, TextSymbolConfig
mod engine; // TextLayout
// mod line; // WIP: TextLineIter
mod result; // TextFit, TextLayoutStep
mod symbol; // TextCohesion, TextLayoutSpan, TextSymbol,
// mod wrap; // WIP: TextBreakKind, TextLine, TextSegment[Kind]

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // config::*,
            engine::*,
            // line::*,
            result::*,
            symbol::*,
            // wrap::*,
        };
    }
}
