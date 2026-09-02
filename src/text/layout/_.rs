// devela/src/text/layout/_.rs
//
#![doc = crate::_DOC_TEXT_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::text; layout)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod engine; // TextLayout
    mod line; // TextLineIter
    mod result; // TextFit, TextLayoutStep
    mod symbol; // Text<Break|Elide>Mode, TextCohesion, TextLayoutSpan, TextSymbol[Config]
    mod textel; // Textel, TextelWidth, TextelWidthMode
    mod wrap; // TextBreakKind, TextLine, TextSegment[Kind], TextWrapIter
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            engine::*,
            line::*,
            result::*,
            symbol::*,
            textel::*,
            wrap::_all::*,
        };
    }
}
