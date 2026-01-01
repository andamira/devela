// devela_base_core::code::util::_tags
//
//! Private tags definitions for visual type categorization in documentation.
//

// helper for defining doc tags with symbol and a title attribute attribute
macro_rules! define_symbol_tags {
    ($( $tag:ident, $title:literal, $symbol:literal;)+) => {
        $crate::CONST! { hidden macro_export,
            $( $tag = concat![$crate::SPAN_OPEN!(), "'", $title, "'>", $symbol, "</span>"];)+
        }
    };
}
define_symbol_tags! {
    /* thematic */

    // _ALLOCATION: allocation mechanisms
    _TAG_ALLOCATION, "Memory allocation", "🧺"; // 🧮, (basket, abacus)
    _TAG_APPLE, "Apple platform", "🍏"; // 🍏,🍎, (green-apple, red-apple)
    _TAG_ASSERT, "Assertion", "💯"; // 💯
    _TAG_ATOMIC, "Atomic", "⚛️"; // ⚛️, 🔬, 🌐,
    _TAG_AUDIO, "Audio", "🔊"; // 🎧,🎼,🎶,🎜 ,🎝 ,🎵,🔈,🔉,🔊,🕪 ,🕩 ,🕨 ,🕫 ,🕬 ,📢,
    // _CODE: structure, compilation, syntax. Items that operate on, reason about,
    // or structurally transform Rust code itself, rather than runtime values
    _TAG_CODE, "Code structure and compilation", "⌗"; // ⌗,≡,§,⧉,
    _TAG_CODEC, "Encoding and decoding", "🥡"; // 🥡, 🔏, ⇄, (takeout-box)
    _TAG_COLOR, "Color", "🎨"; // 🎨,
    _TAG_CONCURRENCY, "Concurrency", "🧵"; // 🧵, 🪡, (thread, needle)
    _TAG_CONSTRUCTION, "Construction", "🏗️"; // 🏗️,🏭,
    _TAG_DATA, "Data", "🪪"; // 🪪, 🗂️, 🧩, (id-card)
    _TAG_DATA_STRUCTURE, "Data structure (collection)", "🗃️"; // 📇,🗃️,📦,🧩,🗂️,
    // _DEBUG: diagnostics, introspection, debugging intent
    _TAG_DEBUG, "Debugging", "🐛"; // 🐛,
    _TAG_ERROR, "Error", "🚩"; // ❌,🚫,📛,🚧,📉,🚩,
    _TAG_ERROR_COMPOSITE, "", "🚩+"; // 📎,📦,🖇️,🗂️,
    // _EVENT: occurrences and event vocabularies
    _TAG_EVENT, "Event", "🎫"; // 🎫, 🎟️, 🎊, 🎉,
    _TAG_EXPERIMENTAL, "Experimental", "🧪";
    _TAG_EXAMPLE, "Example", "✨"; // ✨, 📘, 🪄,
    _TAG_FAKE, "Mock or fake implementation", "🎭"; // 🎭, 👻, 🦄, 🐛,
    _TAG_FFI, "FFI safe", "🛡️"; // 🛡️, ✅
    _TAG_FS, "File system", "📁"; // 📁,💾,🗄️,📄
    _TAG_FONT, "Font or glyph", "🅵"; // 🅵,, 🅰, ℱ, 𝔉, 𝕱, 𝐅
    _TAG_FMT, "Formatting", "🖹"; // 🖹, 📄, 📝, 🄵, ✎, ℱ, 𝔽
    _TAG_GEOM, "Geometry", "📐";
    _TAG_GEOM_DIR, "Direction and orientation", "🧭";
    // RAII / drop-driven behavior
    _TAG_GUARD, "Scoped guard", "🔒"; // 🔒,🪢,⏹️ ,
    _TAG_HASH, "Hashing", "🔀"; // 🔀,
    _TAG_IMAGE, "Image", "🖼️"; // 🖼️,📷,
    _TAG_INIT, "Initialization", "🌱"; // 🌱,🎬,〽️,🆕,🌑,🌚
    // _INTERACTION: human intent vocabulary
    _TAG_INTERACTION, "Human interaction", "🎮"; // 🎮,👤,✋,🖱️,⌨️,
    _TAG_IO, "Input and output", "🔌"; // 🔌,
    _TAG_ITERATOR, "Iterator or iterator adapter", "🔄"; // 🔄,
    // _LAYOUT: Arrangement in conceptual or visual space, not in RAM.
    _TAG_LAYOUT, "Spatial layout", "🧱"; // 🧱,
    // _LIFETIME: Lifetime / scoped validity (views, borrows, guards, ownership relations)
    _TAG_LIFETIME, "Lifetime", "🍃"; // 🍃,⏳,🍂,
    _TAG_LINUX, "Linux platform", "🐧";
    // _TAG_LOCATION, "", "🖈"; // 🖈,📌,📍,
    _TAG_LOG, "Logging", "🪵"; // 🪵,👣,📜,📊,🧾
    _TAG_LOGIC, "Logic", "∧"; // ∧,⊨,⊢,⊙
    // _MAYBE: applies to different underlying representations with different
    // guarantees that are deliberately collapsed behind a single abstraction
    _TAG_MAYBE, "Conditional representation", "🤷"; // 🤷,💁, (shrugging, tipping hand)
    // Memory form / representation (POD / bit validity / alignment / erased forms)
    _TAG_MEM, "Memory representation", "🫗"; // 🫗,◧, ◨, ▣ (glass pouring)
    // _NAMESPACE: deliberate operation containers (or by association)
    _TAG_NAMESPACE, "Utility namespace", "🛠️"; // 🛠️,🔧,🧰,🚙,🌐,📛,
    _TAG_NETWORK, "Networking", "📡"; // 🖧 ,📡,
    _TAG_NICHE, "Niche memory optimizations", "⚗️"; // ⚗️,♟️,🧩,🧮,
    _TAG_NON_STANDARD, "Non-standard", "⚠️";
    _TAG_NO, "Absence, emptiness or a no-op effect", "∅"; // ∅, ⊘, ⬛
    _TAG_NUM, "Numeric structures and computation", "⅀"; // ⅀,∑,×,±,π,🔢,½,¾,🖩,🔟,𝟙,⒈,𝟷,𝟏
    _TAG_PLATFORM, "Platform-dependent", "🖥️"; // 🖥️,💻,📱,📲,
    _TAG_PRIMITIVE, "Rust primitive", "⚙️"; // ⚙️,
    _TAG_QUANT, "Quantitative relations", "📏";
    _TAG_RAND, "Randomness", "🎲"; // 🎲, 🎰, 🔀
    _TAG_RESULT, "Outcome", "⚖️"; // ⚖️,↔️,✅,🗳,
    _TAG_RUNTIME, "Runtime", "⬡"; // ⬡,
    _TAG_TERM, "Terminal platform", "🮖"; // 🮴 ,🮖,🖳 ,⌨️ ,⎚,❯,🗔 ,
    _TAG_TEXT, "Text", "𝐓"; // 𝐓, 𝓣, 𝔸, 🄰
    _TAG_TIME, "Time", "🕘"; // 🕘, ⏳, 📅
    _TAG_UI, "User interface", "▦"; // ▦,🗔  ,▣,⌗,◧,◨,⊞
    _TAG_UID, "Identification", "🫆"; // 🫆, 🆔, (fingerprint, id-button)
    _TAG_WAVE, "Wave and signal analysis", "〰️"; // 〰️, 🌊,
    _TAG_WEB, "Web platform", "🌐"; // 🌐,🕸️,🌍,
    _TAG_WINDOWS, "Windows platform", "🪟"; // 🪟,

    /* misc. */

    _TAG_MAYBE_STD,
        "re-exported from `std` when available, otherwise replaced with an internal equivalent",
        "?std";
    _TAG_OPTIONAL_STD, "uses `std` features when enabled; or employs fallbacks otherwise", "±std";
    _TAG_CODEGEN_BUILD, "code generated in the build script", "<small>cgen</small>";
    _TAG_PROCEDURAL_MACRO, "procedural macro", "<small>proc</small>";

    /* optional dependencies */

    // used in: work::sync::atomic
    _TAG_ATOMIC_CORE_PORTABLE,
        "re-exported either from `core` or from the `portable-atomic` crate", "`?core`";
    // used in: work::sync::re-exports and work::future::re-exports
    _TAG_ATOMIC_ALLOC_PORTABLE_UTIL,
        "re-exported either from `alloc` or from the `portable-atomic-util` crate", "`?alloc`";
}

crate::CONST! { hidden macro_export,
    SPAN_OPEN = "<span class='stab portability' title=";

    /* optional dependencies */

    // used in: work::sync::atomic
    _DOC_ATOMIC_CORE_PORTABLE = concat!("*Re-exported either from `core` or from the ",
        "[`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---");
    // used in: work::sync::re-exports and work::future::re-exports
    // _DOC_ATOMIC_ALLOC_PORTABLE_UTIL = concat!("*Re-exported either from `alloc` or from the ",
    //     "[`portable-atomic-util`](https://docs.rs/portable-atomic-util)* crate.\n\n---");
}
