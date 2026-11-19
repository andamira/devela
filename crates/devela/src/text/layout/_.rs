//
#![doc = crate::_DOC_TEXT_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::text; layout)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(all(test, feature = "text_layout"))]
    mod _test;

    #[cfg(feature = "text_layout")]
    mod engine; // TextLayout
    #[cfg(feature = "text_layout")]
    mod line; // TextLineIter
    #[cfg(feature = "text_layout")]
    mod result; // TextFit, TextLayoutStep
    #[cfg(feature = "text_layout")]
    mod symbol; // Text<Break|Elide>Mode, TextCohesion, TextLayoutSpan, TextSymbol[Config]

    mod textel; // Textel, TextelWidth, TextelWidthMode

    #[cfg(feature = "text_layout")]
    mod_ wrap; // TextBreakKind, TextLine, TextSegment[Kind], TextWrapIter
}
crate::mods_out! { // _mods
    _mods {
        pub use super::textel::*;
        #[cfg(feature = "text_layout")]
        pub use super::{
            engine::*,
            line::*,
            result::*,
            symbol::*,
            wrap::_all::*,
        };
    }
}
