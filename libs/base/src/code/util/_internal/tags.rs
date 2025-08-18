// devela_base::code::util::_internal::tags
//
//! Private tags definitions for visual type categorization in documentation.
//

#![allow(missing_docs, reason = "hidden internals for the workspace")]

crate::CONST! { pub/*workspace*/,
    #[macro_export] #[doc(hidden)] EMOJI_ALLOCATOR = "🧺"; // 🧮, (basket, abacus)
    #[macro_export] #[doc(hidden)] EMOJI_ATOMIC = "⚛️"; // ⚛️,🔬,🌐
    #[macro_export] #[doc(hidden)] EMOJI_DATA_STRUCTURE = "📦"; // 📦,🧩,🗂️,
    #[macro_export] #[doc(hidden)] EMOJI_ERROR = "🚩"; // ❌,🚫,📛,🚧,📉,🚩,
    // EMOJI_ERROR_COMPOSITE = "+"; // 📎,🧩,📦,🖇️,🗂️,
    #[macro_export] #[doc(hidden)] EMOJI_EXPERIMENTAL = "🧪";
    #[macro_export] #[doc(hidden)] EMOJI_FAKE = "🎭"; // 🎭, 👻, 🦄, 🐛
    #[macro_export] #[doc(hidden)] EMOJI_FFI = "🛡️"; // 🛡️, ✅
    #[macro_export] #[doc(hidden)] EMOJI_FONT = "🅵"; // 🅵,, 🅰, ℱ, 𝔉, 𝕱, 𝐅
    #[macro_export] #[doc(hidden)] EMOJI_FMT = "🖹"; // 🖹, 📄, 📝, 🄵, ✎, ℱ, 𝔽
    #[macro_export] #[doc(hidden)] EMOJI_GEOM = "📐";
    #[macro_export] #[doc(hidden)] EMOJI_ITERATOR = "🔄"; // 🔄,
    #[macro_export] #[doc(hidden)] EMOJI_NAMESPACE = "🌐"; // 🌐,📛,
    #[macro_export] #[doc(hidden)] EMOJI_NICHE = "⚗️"; // ⚗️,♟️,🧩,🧮,
    #[macro_export] #[doc(hidden)] EMOJI_NON_STANDARD = "⚠️";
    #[macro_export] #[doc(hidden)] EMOJI_NO = "∅"; // ∅, ⊘, ⬛
    #[macro_export] #[doc(hidden)] EMOJI_NUM = "𝟙"; // 🔢, 🔟, ❶, ➀, 𝟙
    #[macro_export] #[doc(hidden)] EMOJI_PRIMITIVE = "⚙️"; // ⚙️,
    #[macro_export] #[doc(hidden)] EMOJI_QUANT = "📏";
    #[macro_export] #[doc(hidden)] EMOJI_RAND = "🎲"; // 🎲, 🎰, 🔀
    #[macro_export] #[doc(hidden)] EMOJI_RESULT = "⚖️"; // ⚖️,↔️,✅,🗳,
    #[macro_export] #[doc(hidden)] EMOJI_TEXT = "𝐓"; // 𝐓, 𝓣, 𝔸, 🄰
    #[macro_export] #[doc(hidden)] EMOJI_TIME = "🕘"; // 🕘, ⏳, 📅
    //
    #[macro_export] #[doc(hidden)] SPAN_OPEN = "<span class='stab portability' title=";
    // SPAN_OPEN = "<span class='tag-emoji' title=";
    //
    #[macro_export] #[doc(hidden)]
    TAG_ALLOCATOR = concat!($crate::SPAN_OPEN!(), "'Allocator-related item'>",
        $crate::EMOJI_ALLOCATOR!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_ATOMIC = concat!($crate::SPAN_OPEN!(), "'Atomic-related item'>",
        $crate::EMOJI_ATOMIC!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_DATA_STRUCTURE =
        concat!($crate::SPAN_OPEN!(), "'General-purpose data structure'>",
        $crate::EMOJI_DATA_STRUCTURE!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_ERROR = concat!($crate::SPAN_OPEN!(), "'Individual error type'>",
        $crate::EMOJI_ERROR!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_ERROR_COMPOSITE =
        concat!($crate::SPAN_OPEN!(), "'Composite error type'>",
        $crate::EMOJI_ERROR!(), "+</span>");
    #[macro_export] #[doc(hidden)]
    TAG_EXPERIMENTAL = concat!(
        "<span class='stab portability' title='Experimental functionality'>",
        $crate::EMOJI_EXPERIMENTAL!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_FAKE = concat!($crate::SPAN_OPEN!(), "'A fake implementation for testing.'>",
        $crate::EMOJI_FAKE!() ,"</span>");
    #[macro_export] #[doc(hidden)]
    TAG_FFI = concat!("<span class='stab portability' title='Ffi-safe version'>",
        $crate::EMOJI_FFI!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_FONT = concat!("<span class='stab portability' title='Font-related item'>",
        $crate::EMOJI_FONT!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_FMT =
        concat!($crate::SPAN_OPEN!(), "'Text Formatting & Representation item'>",
        $crate::EMOJI_FMT!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_GEOM =
        concat!($crate::SPAN_OPEN!(), "'Geometric multi-dimensional item'>",
        $crate::EMOJI_GEOM!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_ITERATOR = concat!($crate::SPAN_OPEN!(), "'Iterator or iterator adapter'>",
        $crate::EMOJI_ITERATOR!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_NAMESPACE = concat!($crate::SPAN_OPEN!(), "'Namespaced functionality'>",
        $crate::EMOJI_NAMESPACE!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_NICHE = concat!($crate::SPAN_OPEN!(), "'Type with niche-based memory optimizations'>",
        $crate::EMOJI_NICHE!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_NON_STANDARD = concat!(
        "<span class='stab portability' title='Non-standard. Expect poor cross-compatibility'>",
        $crate::EMOJI_NON_STANDARD!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_NO = concat!($crate::SPAN_OPEN!(), "'Absence, emptiness, or a no-op effect'>",
        $crate::EMOJI_NO!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_NUM = concat!($crate::SPAN_OPEN!(), "'Numeric value-related item'>",
        $crate::EMOJI_NUM!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_PRIMITIVE = concat!($crate::SPAN_OPEN!(), "'Rust primitive type'>",
        $crate::EMOJI_PRIMITIVE!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_QUANT = concat!($crate::SPAN_OPEN!(), "'1-dimensional measurement or quantity'>",
        $crate::EMOJI_QUANT!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_RAND = concat!($crate::SPAN_OPEN!(), "'Randomness-related item'>",
        $crate::EMOJI_RAND!(), "</span>");
    #[macro_export] #[doc(hidden)]
    TAG_RESULT = concat!($crate::SPAN_OPEN!(), "'Resolution or outcome-related item'>",
        $crate::EMOJI_RESULT!() ,"</span>");
    #[macro_export] #[doc(hidden)]
    TAG_TEXT = concat!($crate::SPAN_OPEN!(), "'Text-related type'>",
        $crate::EMOJI_TEXT!() ,"</span>");
    #[macro_export] #[doc(hidden)]
    TAG_TIME = concat!($crate::SPAN_OPEN!(), "'Time-related type'>",
        $crate::EMOJI_TIME!() ,"</span>");

    #[macro_export] #[doc(hidden)]
    TAG_MAYBE_STD = concat!($crate::SPAN_OPEN!(),
        "'re-exported from rust&#39;s `std` or recreated if `not(std)`'>`?std`</span>");

    /* optional dependencies */

    // used in: work::sync::atomic
    #[macro_export] #[doc(hidden)]
    TAG_ATOMIC_CORE_PORTABLE = concat!($crate::SPAN_OPEN!(),
        "'re-exported either from `core` or from the `portable-atomic` crate'>",
        "`?core`</span>");
    #[macro_export] #[doc(hidden)]
    DOC_ATOMIC_CORE_PORTABLE = concat!("*Re-exported either from `core` or from the ",
        "[`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---");

    // used in: work::sync::re-exports and work::future::re-exports
    #[macro_export] #[doc(hidden)]
    TAG_ATOMIC_ALLOC_PORTABLE_UTIL = concat!($crate::SPAN_OPEN!(),
        "'re-exported either from `alloc` or from the `portable-atomic-util` crate'>",
        "`?alloc`</span>");
    #[macro_export] #[doc(hidden)]
    DOC_ATOMIC_ALLOC_PORTABLE_UTIL = concat!("*Re-exported either from `alloc` or from the ",
        "[`portable-atomic-util`](https://docs.rs/portable-atomic-util)* crate.\n\n---");
}
