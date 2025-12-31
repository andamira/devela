// devela_base_core::code::util::_tags
//
//! Private tags definitions for visual type categorization in documentation.
//

#![allow(missing_docs, reason = "hidden internals for the workspace")]

crate::CONST! { hidden macro_export,
    _EMOJI_ALLOCATION = "🧺"; // 🧮, (basket, abacus)
    _EMOJI_APPLE = "🍏"; // 🍏,🍎,  (green apple,  apple)
    _EMOJI_ASSERT = "💯"; // 💯,
    _EMOJI_ATOMIC = "⚛️"; // ⚛️, 🔬, 🌐,
    _EMOJI_AUDIO = "🔊"; // 🎧,🎼,🎶,🎜 ,🎝 ,🎵,🔈,🔉,🔊,🕪 ,🕩 ,🕨 ,🕫 ,🕬 ,📢,
    _EMOJI_CODE = "⌗"; // ⌗,≡,§,⧉,
    _EMOJI_CODEC = "🥡"; // 🥡, 🔏, ⇄, (takeout-box)
    _EMOJI_COLOR = "🎨"; // 🎨,
    _EMOJI_CONCURRENCY = "🧵"; // 🧵, 🪡, (thread, needle)
    _EMOJI_CONSTRUCTION = "🏗️"; // 🏗️,🏭,
    _EMOJI_DEBUG = "🐛"; // 🐛,
    _EMOJI_DATA = "🪪"; // 🪪, 🗂️, 🧩, (id-card)
    _EMOJI_DATA_STRUCTURE = "🗃️"; // 📇,🗃️,📦,🧩,🗂️,
    _EMOJI_ERROR = "🚩"; // ❌,🚫,📛,🚧,📉,🚩,
    // _EMOJI_ERROR_COMPOSITE = "+"; // 📎,📦,🖇️,🗂️,
    _EMOJI_EVENT = "🎫"; // 🎫, 🎟️, 🎊, 🎉,
    _EMOJI_EXPERIMENTAL = "🧪";
    _EMOJI_EXAMPLE = "✨"; // ✨, 📘, 🪄,
    _EMOJI_FAKE = "🎭"; // 🎭, 👻, 🦄, 🐛,
    _EMOJI_FFI = "🛡️"; // 🛡️, ✅
    _EMOJI_FS = "📁"; // 📁,💾,🗄️,📄
    _EMOJI_FONT = "🅵"; // 🅵,, 🅰, ℱ, 𝔉, 𝕱, 𝐅
    _EMOJI_FMT = "🖹"; // 🖹, 📄, 📝, 🄵, ✎, ℱ, 𝔽
    _EMOJI_GEOM = "📐";
    _EMOJI_GEOM_DIR = "🧭";
    _EMOJI_GUARD = "🔒"; // 🔒,🪢,⏹️ ,
    _EMOJI_HASH = "🔀"; // 🔀,
    _EMOJI_IMAGE = "🖼️"; // 🖼️,📷,
    _EMOJI_INIT = "🌱"; // 🌱,🎬,〽️,🆕,🌑,🌚
    _EMOJI_INTERACTION = "🎮"; // 🎮,👤,✋,🖱️,⌨️,
    _EMOJI_IO = "🔌"; // 🔌,
    _EMOJI_ITERATOR = "🔄"; // 🔄,
    _EMOJI_LAYOUT = "🧱"; // 🧱,
    _EMOJI_LIFETIME = "🍃"; // 🍃,⏳,🍂,
    _EMOJI_LINUX = "🐧";
    // _EMOJI_LOCATION = "🖈"; // 🖈,📌,📍,
    _EMOJI_LOG = "🪵"; // 🪵,👣,📜,📊,🧾
    _EMOJI_LOGIC = "∧"; // ∧,⊨,⊢,⊙
    _EMOJI_MAYBE = "🤷"; // 🤷,💁, (shrugging, tipping hand)
    // Items whose subject matter is how values relate to bytes, bits,
    // alignment, size, layout, validity, or shape in memory
    _EMOJI_MEM = "🫗"; // 🫗,◧, ◨, ▣ (glass pouring)
    _EMOJI_NAMESPACE = "🛠️"; // 🛠️,🔧,🧰,🚙,🌐,📛,
    _EMOJI_NETWORK = "📡"; // 🖧 ,📡,
    _EMOJI_NICHE = "⚗️"; // ⚗️,♟️,🧩,🧮,
    _EMOJI_NON_STANDARD = "⚠️";
    _EMOJI_NO = "∅"; // ∅, ⊘, ⬛
    _EMOJI_NUM = "⅀"; // ⅀,∑,×,±,π,🔢,½,¾,🖩,🔟,𝟙,⒈,𝟷,𝟏
    _EMOJI_PLATFORM = "🖥️"; // 🖥️,💻,📱,📲,
    _EMOJI_PRIMITIVE = "⚙️"; // ⚙️,
    _EMOJI_QUANT = "📏";
    _EMOJI_RAND = "🎲"; // 🎲, 🎰, 🔀
    _EMOJI_RESULT = "⚖️"; // ⚖️,↔️,✅,🗳,
    _EMOJI_RUNTIME = "⬡"; // ⬡,
    _EMOJI_TERM = "🮖"; // 🮴 ,🮖,🖳 ,⌨️ ,⎚,❯,🗔 ,
    _EMOJI_TEXT = "𝐓"; // 𝐓, 𝓣, 𝔸, 🄰
    _EMOJI_TIME = "🕘"; // 🕘, ⏳, 📅
    _EMOJI_UI = "▦"; // ▦,🗔  ,▣,⌗,◧,◨,⊞
    _EMOJI_UID = "🫆"; // 🫆, 🆔, (fingerprint, id-button)
    _EMOJI_WAVE = "〰️"; // 〰️, 🌊,
    _EMOJI_WEB = "🌐"; // 🌐,🕸️,🌍,
    _EMOJI_WINDOWS = "🪟"; // 🪟,
    //
    SPAN_OPEN = "<span class='stab portability' title=";
    // SPAN_OPEN = "<span class='tag-emoji' title=";

    // _ALLOCATION: allocation mechanisms
    _TAG_ALLOCATION = concat!($crate::SPAN_OPEN!(), "'Memory allocation'>",
        $crate::_EMOJI_ALLOCATION!(), "</span>");
    _TAG_APPLE = concat!($crate::SPAN_OPEN!(), "'Apple platform'>",
        $crate::_EMOJI_APPLE!(), "</span>");
    _TAG_ASSERT = concat!($crate::SPAN_OPEN!(), "'Assertion'>",
        $crate::_EMOJI_ASSERT!(), "</span>");
    _TAG_ATOMIC = concat!($crate::SPAN_OPEN!(), "'Atomic'>",
        $crate::_EMOJI_ATOMIC!(), "</span>");
    _TAG_AUDIO = concat!($crate::SPAN_OPEN!(), "'Audio'>",
        $crate::_EMOJI_AUDIO!(), "</span>");
    // _CODE: structure, compilation, syntax. Items that operate on, reason about,
    // or structurally transform Rust code itself, rather than runtime values
    _TAG_CODE = concat!($crate::SPAN_OPEN!(), "'Code structure and compilation'>",
        $crate::_EMOJI_CODE!(), "</span>");
    _TAG_CODEC = concat!($crate::SPAN_OPEN!(), "'Encoding and decoding'>",
        $crate::_EMOJI_CODEC!(), "</span>");
    _TAG_COLOR = concat!($crate::SPAN_OPEN!(), "'Color'>",
        $crate::_EMOJI_COLOR!(), "</span>");
    _TAG_CONCURRENCY = concat!($crate::SPAN_OPEN!(), "'Concurrency'>",
        $crate::_EMOJI_CONCURRENCY!(), "</span>");
    _TAG_CONSTRUCTION = concat!($crate::SPAN_OPEN!(), "'Construction'>",
        $crate::_EMOJI_CONSTRUCTION!(), "</span>");
    _TAG_DATA = concat!($crate::SPAN_OPEN!(), "'Data'>",
        $crate::_EMOJI_DATA!(), "</span>");
    _TAG_DATA_STRUCTURE = concat!($crate::SPAN_OPEN!(), "'Data structure (collection)'>",
        $crate::_EMOJI_DATA_STRUCTURE!(), "</span>");
    // _DEBUG: diagnostics, introspection, debugging intent
    _TAG_DEBUG = concat!($crate::SPAN_OPEN!(), "'Debugging'>",
        $crate::_EMOJI_DEBUG!(), "</span>");
    _TAG_ERROR = concat!($crate::SPAN_OPEN!(), "'Error'>",
        $crate::_EMOJI_ERROR!(), "</span>");
    _TAG_ERROR_COMPOSITE = concat!($crate::SPAN_OPEN!(), "'Composite error'>",
        $crate::_EMOJI_ERROR!(), "+</span>");
    // _EVENT: occurrences and event vocabularies
    _TAG_EVENT = concat!($crate::SPAN_OPEN!(), "'Event'>",
        $crate::_EMOJI_EVENT!(), "</span>");
    _TAG_EXPERIMENTAL = concat!($crate::SPAN_OPEN!(), "'Experimental'>",
        $crate::_EMOJI_EXPERIMENTAL!(), "</span>");
    _TAG_EXAMPLE = concat!($crate::SPAN_OPEN!(), "'Example'>",
        $crate::_EMOJI_EXAMPLE!(), "</span>");
    _TAG_FAKE = concat!($crate::SPAN_OPEN!(), "'Mock or fake implementation'>",
        $crate::_EMOJI_FAKE!() ,"</span>");
    _TAG_FFI = concat!("<span class='stab portability' title='FFI safe'>",
        $crate::_EMOJI_FFI!(), "</span>");
    _TAG_FS = concat!("<span class='stab portability' title='File system'>",
        $crate::_EMOJI_FS!(), "</span>");
    _TAG_FONT = concat!("<span class='stab portability' title='Font or glyph'>",
        $crate::_EMOJI_FONT!(), "</span>");
    _TAG_FMT = concat!($crate::SPAN_OPEN!(), "'Formatting'>",
        $crate::_EMOJI_FMT!(), "</span>");
    _TAG_GEOM = concat!($crate::SPAN_OPEN!(), "'Geometry'>",
        $crate::_EMOJI_GEOM!(), "</span>");
    _TAG_GEOM_DIR = concat!($crate::SPAN_OPEN!(), "'Direction and orientation'>",
        $crate::_EMOJI_GEOM_DIR!(), "</span>");
    // RAII / drop-driven behavior
    _TAG_GUARD = concat!($crate::SPAN_OPEN!(), "'Scoped guard'>",
        $crate::_EMOJI_GUARD!(), "</span>");
    _TAG_HASH = concat!($crate::SPAN_OPEN!(), "'Hashing'>",
        $crate::_EMOJI_HASH!(), "</span>");
    _TAG_IMAGE = concat!($crate::SPAN_OPEN!(), "'Image'>",
        $crate::_EMOJI_IMAGE!(), "</span>");
    _TAG_INIT = concat!($crate::SPAN_OPEN!(), "'Initialization'>",
        $crate::_EMOJI_INIT!(), "</span>");
    // _INTERACTION: human intent vocabulary
    _TAG_INTERACTION = concat!($crate::SPAN_OPEN!(), "'Human interaction'>",
        $crate::_EMOJI_INTERACTION!(), "</span>");
    _TAG_IO = concat!("<span class='stab portability' title='Input and output'>",
        $crate::_EMOJI_IO!(), "</span>");
    _TAG_ITERATOR = concat!($crate::SPAN_OPEN!(), "'Iterator or iterator adapter'>",
        $crate::_EMOJI_ITERATOR!(), "</span>");
    // _LAYOUT: Arrangement in conceptual or visual space, not in RAM.
    _TAG_LAYOUT = concat!($crate::SPAN_OPEN!(), "'Spatial layout'>",
        $crate::_EMOJI_LAYOUT!(), "</span>");
    // _LIFETIME: Lifetime / scoped validity (views, borrows, guards, ownership relations)
    _TAG_LIFETIME = concat!($crate::SPAN_OPEN!(), "'Lifetime'>",
        $crate::_EMOJI_LIFETIME!(), "</span>");
    _TAG_LINUX = concat!($crate::SPAN_OPEN!(), "'Linux platform'>",
        $crate::_EMOJI_LINUX!(), "</span>");
    _TAG_LOG = concat!($crate::SPAN_OPEN!(), "'Logging'>",
        $crate::_EMOJI_LOG!(), "</span>");
    _TAG_LOGIC = concat!($crate::SPAN_OPEN!(), "'Logic'>",
        $crate::_EMOJI_LOGIC!(), "</span>");
    // _MAYBE: applies to different underlying representations with different
    // guarantees that are deliberately collapsed behind a single abstraction
    _TAG_MAYBE = concat!($crate::SPAN_OPEN!(), "'Conditional representation'>",
        $crate::_EMOJI_MAYBE!(), "</span>");
    // Memory form / representation (POD / bit validity / alignment / erased forms)
    _TAG_MEM = concat!($crate::SPAN_OPEN!(), "'Memory representation'>",
        $crate::_EMOJI_MEM!(), "</span>");
    // _NAMESPACE: deliberate operation containers (or by association)
    _TAG_NAMESPACE = concat!($crate::SPAN_OPEN!(), "'Utility namespace'>",
        $crate::_EMOJI_NAMESPACE!(), "</span>");
    _TAG_NETWORK = concat!($crate::SPAN_OPEN!(), "'Networking'>",
        $crate::_EMOJI_NETWORK!(), "</span>");
    _TAG_NICHE = concat!($crate::SPAN_OPEN!(), "'Niche memory optimizations'>",
        $crate::_EMOJI_NICHE!(), "</span>");
    _TAG_NON_STANDARD = concat!($crate::SPAN_OPEN!(), "'Non-standard'>",
        $crate::_EMOJI_NON_STANDARD!(), "</span>");
    _TAG_NO = concat!($crate::SPAN_OPEN!(), "'Absence, emptiness, or a no-op effect'>",
        $crate::_EMOJI_NO!(), "</span>");
    _TAG_NUM = concat!($crate::SPAN_OPEN!(), "'Numeric structures and computation'>",
        $crate::_EMOJI_NUM!(), "</span>");
    _TAG_PLATFORM = concat!($crate::SPAN_OPEN!(), "'Platform-dependent'>",
        $crate::_EMOJI_PLATFORM!(), "</span>");
    _TAG_PRIMITIVE = concat!($crate::SPAN_OPEN!(), "'Rust primitive'>",
        $crate::_EMOJI_PRIMITIVE!(), "</span>");
    _TAG_QUANT = concat!($crate::SPAN_OPEN!(), "'Quantitative relations'>",
        $crate::_EMOJI_QUANT!(), "</span>");
    _TAG_RAND = concat!($crate::SPAN_OPEN!(), "'Randomness'>",
        $crate::_EMOJI_RAND!(), "</span>");
    _TAG_RESULT = concat!($crate::SPAN_OPEN!(), "'Outcome'>",
        $crate::_EMOJI_RESULT!() ,"</span>");
    _TAG_RUNTIME = concat!($crate::SPAN_OPEN!(), "'Runtime'>",
        $crate::_EMOJI_RUNTIME!() ,"</span>");
    _TAG_TERM = concat!($crate::SPAN_OPEN!(), "'Terminal platform'>",
        $crate::_EMOJI_TERM!() ,"</span>");
    _TAG_TEXT = concat!($crate::SPAN_OPEN!(), "'Text'>",
        $crate::_EMOJI_TEXT!() ,"</span>");
    _TAG_TIME = concat!($crate::SPAN_OPEN!(), "'Time'>",
        $crate::_EMOJI_TIME!() ,"</span>");
    _TAG_UI = concat!($crate::SPAN_OPEN!(), "'User interface'>",
        $crate::_EMOJI_UI!(), "</span>");
    _TAG_UID = concat!($crate::SPAN_OPEN!(), "'Identification'>",
        $crate::_EMOJI_UID!(), "</span>");
    _TAG_WAVE = concat!($crate::SPAN_OPEN!(), "'Wave and signal analysis'>",
        $crate::_EMOJI_WAVE!(), "</span>");
    _TAG_WEB = concat!($crate::SPAN_OPEN!(), "'Web platform'>",
        $crate::_EMOJI_WEB!(), "</span>");
    _TAG_WINDOWS = concat!($crate::SPAN_OPEN!(), "'Windows platform'>",
        $crate::_EMOJI_WINDOWS!(), "</span>");

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
