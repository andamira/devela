// devela_base_core::code::util::_tags
//
//! Private tags definitions for visual type categorization in documentation.
//

#![allow(missing_docs, reason = "hidden internals for the workspace")]

crate::CONST! { hidden macro_export,
    _EMOJI_ALLOCATOR = "🧺"; // 🧮, (basket, abacus)
    _EMOJI_ATOMIC = "⚛️"; // ⚛️, 🔬, 🌐
    _EMOJI_CODEC = "🥡"; // 🥡, 🔏, ⇄, (takeout-box)
    _EMOJI_CONCURRENCY = "🧵"; // 🧵, 🪡, (thread, needle)
    _EMOJI_DEBUG = "🐛"; // 🐛,
    _EMOJI_DATA = "🪪"; // 🪪, 🗂️, 🧩, (id-card)
    _EMOJI_DATA_STRUCTURE = "🗃️"; // 📇,🗃️,📦,🧩,🗂️,
    _EMOJI_ERROR = "🚩"; // ❌,🚫,📛,🚧,📉,🚩,
    // _EMOJI_ERROR_COMPOSITE = "+"; // 📎,🧩,📦,🖇️,🗂️,
    _EMOJI_EVENT = "🎫"; // 🎫, 🎟️,
    _EMOJI_EXPERIMENTAL = "🧪";
    _EMOJI_EXAMPLE = "✨"; // ✨, 📘, 🪄
    _EMOJI_FAKE = "🎭"; // 🎭, 👻, 🦄, 🐛
    _EMOJI_FFI = "🛡️"; // 🛡️, ✅
    _EMOJI_FONT = "🅵"; // 🅵,, 🅰, ℱ, 𝔉, 𝕱, 𝐅
    _EMOJI_FMT = "🖹"; // 🖹, 📄, 📝, 🄵, ✎, ℱ, 𝔽
    _EMOJI_GEOM = "📐";
    _EMOJI_GEOM_DIR = "🧭";
    _EMOJI_HASH = "🔀"; // 🔀,
    _EMOJI_ITERATOR = "🔄"; // 🔄,
    _EMOJI_NAMESPACE = "🌐"; // 🌐,📛,
    _EMOJI_NICHE = "⚗️"; // ⚗️,♟️,🧩,🧮,
    _EMOJI_NON_STANDARD = "⚠️";
    _EMOJI_NO = "∅"; // ∅, ⊘, ⬛
    _EMOJI_NUM = "𝟙"; // 🔢, 🔟, ❶, ➀, 𝟙
    _EMOJI_PRIMITIVE = "⚙️"; // ⚙️,
    _EMOJI_QUANT = "📏";
    _EMOJI_RAND = "🎲"; // 🎲, 🎰, 🔀
    _EMOJI_RESULT = "⚖️"; // ⚖️,↔️,✅,🗳,
    _EMOJI_TEXT = "𝐓"; // 𝐓, 𝓣, 𝔸, 🄰
    _EMOJI_TIME = "🕘"; // 🕘, ⏳, 📅
    _EMOJI_UID = "🫆"; // 🫆, 🆔, (fingerprint, id-button)
    //
    SPAN_OPEN = "<span class='stab portability' title=";
    // SPAN_OPEN = "<span class='tag-emoji' title=";

    _TAG_ALLOCATOR = concat!($crate::SPAN_OPEN!(), "'Memory allocator-related item'>",
        $crate::_EMOJI_ALLOCATOR!(), "</span>");
    _TAG_ATOMIC = concat!($crate::SPAN_OPEN!(), "'Atomic-related item'>",
        $crate::_EMOJI_ATOMIC!(), "</span>");
    _TAG_CODEC = concat!($crate::SPAN_OPEN!(), "'Codec-related item'>",
        $crate::_EMOJI_CODEC!(), "</span>");
    _TAG_CONCURRENCY = concat!($crate::SPAN_OPEN!(), "'Concurrency-related item'>",
        $crate::_EMOJI_CONCURRENCY!(), "</span>");
    _TAG_DATA = concat!($crate::SPAN_OPEN!(), "'Data-related item'>",
        $crate::_EMOJI_DATA!(), "</span>");
    _TAG_DATA_STRUCTURE = concat!($crate::SPAN_OPEN!(), "'Data-structure (collection)'>",
        $crate::_EMOJI_DATA_STRUCTURE!(), "</span>");
    _TAG_DEBUG = concat!($crate::SPAN_OPEN!(), "'Debug-related item'>",
        $crate::_EMOJI_DEBUG!(), "</span>");
    _TAG_ERROR = concat!($crate::SPAN_OPEN!(), "'Error type'>",
        $crate::_EMOJI_ERROR!(), "</span>");
    _TAG_ERROR_COMPOSITE = concat!($crate::SPAN_OPEN!(), "'Composite or aggregate error type'>",
        $crate::_EMOJI_ERROR!(), "+</span>");
    _TAG_EVENT = concat!($crate::SPAN_OPEN!(), "'Event-related item'>",
        $crate::_EMOJI_EVENT!(), "</span>");
    _TAG_EXPERIMENTAL = concat!($crate::SPAN_OPEN!(), "'Experimental functionality'>",
        $crate::_EMOJI_EXPERIMENTAL!(), "</span>");
    _TAG_EXAMPLE = concat!($crate::SPAN_OPEN!(), "'Illustrative example type'>",
        $crate::_EMOJI_EXAMPLE!(), "</span>");
    _TAG_FAKE = concat!($crate::SPAN_OPEN!(), "'Fake or mock implementation for testing'>",
        $crate::_EMOJI_FAKE!() ,"</span>");
    _TAG_FFI = concat!("<span class='stab portability' title='FFI-safe item or interface'>",
        $crate::_EMOJI_FFI!(), "</span>");
    _TAG_FONT = concat!("<span class='stab portability' title='Font or glyph-related item'>",
        $crate::_EMOJI_FONT!(), "</span>");
    _TAG_FMT = concat!($crate::SPAN_OPEN!(), "'Formatting and text representation item'>",
        $crate::_EMOJI_FMT!(), "</span>");
    _TAG_GEOM = concat!($crate::SPAN_OPEN!(), "'Geometry or spatial-related item'>",
        $crate::_EMOJI_GEOM!(), "</span>");
    _TAG_GEOM_DIR = concat!($crate::SPAN_OPEN!(), "'Direction, orientation or spatial symmetry item'>",
        $crate::_EMOJI_GEOM_DIR!(), "</span>");
    _TAG_HASH = concat!($crate::SPAN_OPEN!(), "'Hashing-related item'>",
        $crate::_EMOJI_HASH!(), "</span>");
    _TAG_ITERATOR = concat!($crate::SPAN_OPEN!(), "'Iterator or iterator adapter'>",
        $crate::_EMOJI_ITERATOR!(), "</span>");
    _TAG_NAMESPACE = concat!($crate::SPAN_OPEN!(), "'Namespaced functionality'>",
        $crate::_EMOJI_NAMESPACE!(), "</span>");
    _TAG_NICHE = concat!($crate::SPAN_OPEN!(), "'Niche-based memory optimizations'>",
        $crate::_EMOJI_NICHE!(), "</span>");
    _TAG_NON_STANDARD = concat!($crate::SPAN_OPEN!(),
    "'Non-standard. Expect poor cross-compatibility'>", $crate::_EMOJI_NON_STANDARD!(), "</span>");
    _TAG_NO = concat!($crate::SPAN_OPEN!(), "'Absence, emptiness, or a no-op effect'>",
        $crate::_EMOJI_NO!(), "</span>");
    _TAG_NUM = concat!($crate::SPAN_OPEN!(), "'Numeric value-related item'>",
        $crate::_EMOJI_NUM!(), "</span>");
    _TAG_PRIMITIVE = concat!($crate::SPAN_OPEN!(), "'Rust primitive type'>",
        $crate::_EMOJI_PRIMITIVE!(), "</span>");
    _TAG_QUANT = concat!($crate::SPAN_OPEN!(), "'1-dimensional measurement or quantity'>",
        $crate::_EMOJI_QUANT!(), "</span>");
    _TAG_RAND = concat!($crate::SPAN_OPEN!(), "'Randomness-related item'>",
        $crate::_EMOJI_RAND!(), "</span>");
    _TAG_RESULT = concat!($crate::SPAN_OPEN!(), "'Resolution or outcome-related item'>",
        $crate::_EMOJI_RESULT!() ,"</span>");
    _TAG_TEXT = concat!($crate::SPAN_OPEN!(), "'Text-related type'>",
        $crate::_EMOJI_TEXT!() ,"</span>");
    _TAG_TIME = concat!($crate::SPAN_OPEN!(), "'Time-related type'>",
        $crate::_EMOJI_TIME!() ,"</span>");
    _TAG_UID = concat!($crate::SPAN_OPEN!(), "'Unique-identification-related item'>",
        $crate::_EMOJI_UID!(), "</span>");

    /* misc. */

    _TAG_MAYBE_STD = concat!($crate::SPAN_OPEN!(),
        "'re-exported from `std` when available, otherwise replaced with an internal equivalent'>`?std`</span>");

    _TAG_OPTIONAL_STD = concat!($crate::SPAN_OPEN!(),
        "'uses `std` features when enabled; or employs fallbacks otherwise'>`±std`</span>");

    _TAG_CODEGEN_BUILD = concat!($crate::SPAN_OPEN!(),
    "'code generated in the build script'>", "<small>cgen</small></span>");
    _TAG_PROCEDURAL_MACRO = concat!($crate::SPAN_OPEN!(),
    "'procedural macro'>", "<small>proc</small></span>");

    /* optional dependencies */

    // used in: work::sync::atomic

    _TAG_ATOMIC_CORE_PORTABLE = concat!($crate::SPAN_OPEN!(),
        "'re-exported either from `core` or from the `portable-atomic` crate'>",
        "`?core`</span>");
    _DOC_ATOMIC_CORE_PORTABLE = concat!("*Re-exported either from `core` or from the ",
        "[`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---");

    // used in: work::sync::re-exports and work::future::re-exports

    _TAG_ATOMIC_ALLOC_PORTABLE_UTIL = concat!($crate::SPAN_OPEN!(),
        "'re-exported either from `alloc` or from the `portable-atomic-util` crate'>",
        "`?alloc`</span>");
    // _DOC_ATOMIC_ALLOC_PORTABLE_UTIL = concat!("*Re-exported either from `alloc` or from the ",
    //     "[`portable-atomic-util`](https://docs.rs/portable-atomic-util)* crate.\n\n---");
}
