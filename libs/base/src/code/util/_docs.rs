// devela_base::code::util::_docs
//
//! Docs headings for modules across crates.
//

#![allow(missing_docs, reason = "hidden internals for the workspace")]

crate::CONST! { hidden macro_export,
/* templates:
#![doc = crate::_DOC_X!()]
    _DOC_ = "";
*/
    _DOC_CODE =  "Code reflective synthesis.";

    _DOC_DATA =  "Data handling and manipulation.";

    _DOC_GAME =  "Game-development and interactive applications.";

    _DOC_LANG = "Language functionality, <abbr
title = 'Domain Specific Language'>DSL</abbr>s
and <abbr title = 'Foreign Function Interface'>FFI</abbr>s.";
    _DOC_LANG_FFI = "<abbr title = 'Foreign Function Interface'>FFI</abbr>
bindings and interoperability.\n\n
Bridges to external languages and platforms.";
    _DOC_LANG_FFI_C = "<a
href='https://en.wikipedia.org/wiki/C_(programming_language)'>C</a> interfacing.";

    _DOC_MEDIA =  "Multimedia functionality.";

    _DOC_NUM =  "Numerical types and math operations.";

    _DOC_PHYS =  "Physical units and measurements.";
    _DOC_PHYS_TIME =  "Time and calendar types and operations.";

    _DOC_SYS =  "System interfaces and hardware abstractions.";

    _DOC_TEXT = "Text types and operations, text processing.";
    _DOC_TEXT_ASCII = "ASCII strings and characters.";
    _DOC_TEXT_CHAR = "Unicode scalars.";
    _DOC_TEXT_FMT = "Strings formatting.";
    _DOC_TEXT_PARSE = "String parsing without structured semantics.";
    _DOC_TEXT_STR = "String types and related functionality.";

    _DOC_UI =  "User interface functionality.";

    _DOC_WORK =  "Work management, concurrency handling.";
}
