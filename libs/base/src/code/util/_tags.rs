// devela_base::code::util::_tags
//
//! Private tags definitions for visual type categorization in documentation.
//

#![allow(missing_docs, reason = "hidden internals for the workspace")]

crate::CONST! { hidden macro_export,
    EMOJI_ALLOCATOR = "🧺"; // 🧮, (basket, abacus)
    EMOJI_ATOMIC = "⚛️"; // ⚛️, 🔬, 🌐
    EMOJI_CODEC = "🥡"; // 🥡, 🔏, ⇄, (takeout-box)
    EMOJI_CONCURRENCY = "🧵"; // 🧵, 🪡, (thread, needle)
    EMOJI_DATA = "🪪"; // 🪪, 🗂️, 🧩, (id-card)
    EMOJI_DATA_STRUCTURE = "🗃️"; // 📇,🗃️,📦,🧩,🗂️,
    EMOJI_ERROR = "🚩"; // ❌,🚫,📛,🚧,📉,🚩,
    // EMOJI_ERROR_COMPOSITE = "+"; // 📎,🧩,📦,🖇️,🗂️,
    EMOJI_EVENT = "🎫"; // 🎫, 🎟️,
    EMOJI_EXPERIMENTAL = "🧪";
    EMOJI_FAKE = "🎭"; // 🎭, 👻, 🦄, 🐛
    EMOJI_FFI = "🛡️"; // 🛡️, ✅
    EMOJI_FONT = "🅵"; // 🅵,, 🅰, ℱ, 𝔉, 𝕱, 𝐅
    EMOJI_FMT = "🖹"; // 🖹, 📄, 📝, 🄵, ✎, ℱ, 𝔽
    EMOJI_GEOM = "📐";
    EMOJI_HASH = "🔀"; // 🔀,
    EMOJI_ITERATOR = "🔄"; // 🔄,
    EMOJI_NAMESPACE = "🌐"; // 🌐,📛,
    EMOJI_NICHE = "⚗️"; // ⚗️,♟️,🧩,🧮,
    EMOJI_NON_STANDARD = "⚠️";
    EMOJI_NO = "∅"; // ∅, ⊘, ⬛
    EMOJI_NUM = "𝟙"; // 🔢, 🔟, ❶, ➀, 𝟙
    EMOJI_PRIMITIVE = "⚙️"; // ⚙️,
    EMOJI_QUANT = "📏";
    EMOJI_RAND = "🎲"; // 🎲, 🎰, 🔀
    EMOJI_RESULT = "⚖️"; // ⚖️,↔️,✅,🗳,
    EMOJI_TEXT = "𝐓"; // 𝐓, 𝓣, 𝔸, 🄰
    EMOJI_TIME = "🕘"; // 🕘, ⏳, 📅
    EMOJI_UID = "🫆"; // 🫆, 🆔, (fingerprint, id-button)
    //
    SPAN_OPEN = "<span class='stab portability' title=";
    // SPAN_OPEN = "<span class='tag-emoji' title=";

    TAG_ALLOCATOR = concat!($crate::SPAN_OPEN!(), "'Allocator-related item'>",
        $crate::EMOJI_ALLOCATOR!(), "</span>");
    TAG_ATOMIC = concat!($crate::SPAN_OPEN!(), "'Atomic-related item'>",
        $crate::EMOJI_ATOMIC!(), "</span>");
    TAG_CODEC = concat!($crate::SPAN_OPEN!(), "'Codec-related-related item'>",
        $crate::EMOJI_CODEC!(), "</span>");
    TAG_CONCURRENCY = concat!($crate::SPAN_OPEN!(), "'Concurrency-related-related item'>",
        $crate::EMOJI_CONCURRENCY!(), "</span>");
    TAG_DATA = concat!($crate::SPAN_OPEN!(), "'Data-related item'>",
        $crate::EMOJI_DATA!(), "</span>");
    TAG_DATA_STRUCTURE = concat!($crate::SPAN_OPEN!(), "'Data-structure (collection)'>",
        $crate::EMOJI_DATA_STRUCTURE!(), "</span>");
    TAG_ERROR = concat!($crate::SPAN_OPEN!(), "'Individual error type'>",
        $crate::EMOJI_ERROR!(), "</span>");
    TAG_ERROR_COMPOSITE = concat!($crate::SPAN_OPEN!(), "'Composite error type'>",
        $crate::EMOJI_ERROR!(), "+</span>");
    TAG_EVENT = concat!($crate::SPAN_OPEN!(), "'Event-related item'>",
        $crate::EMOJI_EVENT!(), "</span>");
    TAG_EXPERIMENTAL = concat!($crate::SPAN_OPEN!(), "'Experimental functionality'>",
        $crate::EMOJI_EXPERIMENTAL!(), "</span>");
    TAG_FAKE = concat!($crate::SPAN_OPEN!(), "'A fake implementation for testing.'>",
        $crate::EMOJI_FAKE!() ,"</span>");
    TAG_FFI = concat!("<span class='stab portability' title='Ffi-safe version'>",
        $crate::EMOJI_FFI!(), "</span>");
    TAG_FONT = concat!("<span class='stab portability' title='Font-related item'>",
        $crate::EMOJI_FONT!(), "</span>");
    TAG_FMT = concat!($crate::SPAN_OPEN!(), "'Text Formatting & Representation item'>",
        $crate::EMOJI_FMT!(), "</span>");
    TAG_GEOM = concat!($crate::SPAN_OPEN!(), "'Geometric multi-dimensional item'>",
        $crate::EMOJI_GEOM!(), "</span>");
    TAG_HASH = concat!($crate::SPAN_OPEN!(), "'Hashing-related item'>",
        $crate::EMOJI_HASH!(), "</span>");
    TAG_ITERATOR = concat!($crate::SPAN_OPEN!(), "'Iterator or iterator adapter'>",
        $crate::EMOJI_ITERATOR!(), "</span>");
    TAG_NAMESPACE = concat!($crate::SPAN_OPEN!(), "'Namespaced functionality'>",
        $crate::EMOJI_NAMESPACE!(), "</span>");
    TAG_NICHE = concat!($crate::SPAN_OPEN!(), "'Niche-based memory optimizations'>",
        $crate::EMOJI_NICHE!(), "</span>");
    TAG_NON_STANDARD = concat!($crate::SPAN_OPEN!(),
    "'Non-standard. Expect poor cross-compatibility'>", $crate::EMOJI_NON_STANDARD!(), "</span>");
    TAG_NO = concat!($crate::SPAN_OPEN!(), "'Absence, emptiness, or a no-op effect'>",
        $crate::EMOJI_NO!(), "</span>");
    TAG_NUM = concat!($crate::SPAN_OPEN!(), "'Numeric value-related item'>",
        $crate::EMOJI_NUM!(), "</span>");
    TAG_PRIMITIVE = concat!($crate::SPAN_OPEN!(), "'Rust primitive type'>",
        $crate::EMOJI_PRIMITIVE!(), "</span>");
    TAG_QUANT = concat!($crate::SPAN_OPEN!(), "'1-dimensional measurement or quantity'>",
        $crate::EMOJI_QUANT!(), "</span>");
    TAG_RAND = concat!($crate::SPAN_OPEN!(), "'Randomness-related item'>",
        $crate::EMOJI_RAND!(), "</span>");
    TAG_RESULT = concat!($crate::SPAN_OPEN!(), "'Resolution or outcome-related item'>",
        $crate::EMOJI_RESULT!() ,"</span>");
    TAG_TEXT = concat!($crate::SPAN_OPEN!(), "'Text-related type'>",
        $crate::EMOJI_TEXT!() ,"</span>");
    TAG_TIME = concat!($crate::SPAN_OPEN!(), "'Time-related type'>",
        $crate::EMOJI_TIME!() ,"</span>");
    TAG_UID = concat!($crate::SPAN_OPEN!(), "'Unique-identification-related item'>",
        $crate::EMOJI_UID!(), "</span>");


    TAG_MAYBE_STD = concat!($crate::SPAN_OPEN!(),
        "'re-exported from rust&#39;s `std` or recreated if `not(std)`'>`?std`</span>");

    /* workspace libraries (small) */


    TAG_DEVELA_BASE = concat!($crate::SPAN_OPEN!(),
        "'provided by `devela_base` workspace library'>", "<small>base</small></span>");
    TAG_DEVELA_BASE_MACROS = concat!($crate::SPAN_OPEN!(),
        "'provided by `devela_base_macros` workspace library'>", "<small>base_macros</small></span>");
    TAG_DEVELA_BASE_ALLOC = concat!($crate::SPAN_OPEN!(),
        "'provided by `devela_base_alloc` workspace library'>", "<small>base_alloc</small></span>");
    TAG_DEVELA_BASE_STD = concat!($crate::SPAN_OPEN!(),
        "'provided by `devela_base_std` workspace library'>", "<small>base_std</small></span>");

    TAG_DEVELA_MACROS = concat!($crate::SPAN_OPEN!(),
        "'provided by `devela_macros` workspace library'>", "<small>macros</small></span>");

    TAG_DEVELA_DATA = concat!($crate::SPAN_OPEN!(),
        "'provided by `devela_data` workspace library'>", "<small>data</small></span>");
    TAG_DEVELA_NUM = concat!($crate::SPAN_OPEN!(),
        "'provided by `devela_num` workspace library'>", "<small>num</small></span>");

    /* optional dependencies */

    // used in: work::sync::atomic

    TAG_ATOMIC_CORE_PORTABLE = concat!($crate::SPAN_OPEN!(),
        "'re-exported either from `core` or from the `portable-atomic` crate'>",
        "`?core`</span>");
    DOC_ATOMIC_CORE_PORTABLE = concat!("*Re-exported either from `core` or from the ",
        "[`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---");

    // used in: work::sync::re-exports and work::future::re-exports

    TAG_ATOMIC_ALLOC_PORTABLE_UTIL = concat!($crate::SPAN_OPEN!(),
        "'re-exported either from `alloc` or from the `portable-atomic-util` crate'>",
        "`?alloc`</span>");
    DOC_ATOMIC_ALLOC_PORTABLE_UTIL = concat!("*Re-exported either from `alloc` or from the ",
        "[`portable-atomic-util`](https://docs.rs/portable-atomic-util)* crate.\n\n---");
}
