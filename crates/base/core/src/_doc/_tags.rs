// devela_base_core::_doc::tags
//
//! Private tags definitions for visual type categorization in documentation.
//
// MAYBE
// - _ORD
// - _STATE

/// Aggregates multiple documentation tags into a single `#[doc = ...]` string.
#[doc(hidden)]
#[macro_export(local_inner_macros)] // local modifier needed to enable resolution
macro_rules! _tags {
    ($($tag:ident)+) => { concat![$( _tags![@$tag], " "),+] };
    (@$tag:ident) =>  { $crate::paste! { [<_TAG_ $tag:upper>] !() }};
}
pub use _tags;

// helper for defining doc tags with an associated symbol and title attribute
macro_rules! define_symbol_tags {
    ($( $tag:ident, $title:literal, $symbol:literal;)+) => {
        $crate::CONST! { hidden macro_export,
            $( $tag = concat![$crate::SPAN_OPEN!(), "'", $title, "'>", $symbol, "</span>"];)+
        }
    };
}
// SEMANTIC AXES AND CONCERNS
// - what it represents      (VALUE, NUM, LOGIC, DATA)
// - how it is realized      (MEM, LIFETIME, GUARD, NICHE)
// - where it applies        (PLATFORM, RUNTIME, TERM, WEB)
// - how it operates         (CODE, CONSTRUCTION, ITERATOR)
define_symbol_tags! {
    /* thematic */

    // Abstract algebraic structures and laws.
    // Covers groups, rings, modules, algebras, and related operations,
    // independent of representation or numeric evaluation.
    _TAG_ALG, "Algebraic structure", "𝔄";
    // Allocation mechanisms
    _TAG_ALLOCATION, "Memory allocation", "🧺"; // (basket)
    _TAG_APPLE, "Apple platform", "🍏";
    // Invariants, contracts, and conditions that must hold.
    // Not for predicates, filters, or conditional behavior.
    _TAG_ASSERT, "Assertion", "💯";
    _TAG_ATOMIC, "Atomic", "⚛️";
    _TAG_AUDIO, "Audio", "🔊";
    _TAG_BIT, "Bit-focused", "▫️";
    // Items that operate on or reason about Rust code itself,
    // including syntax, compilation, and structural transformation.
    _TAG_CODE, "Code structure and compilation", "⌗";
    _TAG_CODEC, "Encoding and decoding", "🥡"; // (takeout-box)
    _TAG_COLOR, "Color", "🎨";
    _TAG_CONCURRENCY, "Concurrency", "🧵"; // (thread)
    // Construction patterns and builders.
    // Emphasizes how values or structures come into existence,
    // not what they represent once built.
    _TAG_CONSTRUCTION, "Construction", "🏗️";
    // General data carriers and abstractions.
    // Focuses on representation and transport of information,
    // not on its semantic meaning or mathematical structure.
    _TAG_DATA, "Data", "🪪"; // (id-card)
    // Containers and collections that organize multiple values.
    // Emphasizes structure over individual value semantics.
    _TAG_DATA_STRUCTURE, "Data structure (collection)", "🗃️";
    // Diagnostics, introspection, debugging intent.
    _TAG_DEBUG, "Debugging", "🐛";
    // Actual error types representing failure states.
    // Not for fallible abstractions or result carriers.
    _TAG_ERROR, "Error", "🚩";
    _TAG_ERROR_COMPOSITE, "Composite error", "🚩+";
    // Discrete occurrences or event vocabularies.
    _TAG_EVENT, "Event", "🎫";
    _TAG_EXAMPLE, "Example", "✨";
    // Structural execution semantics and invocation mechanics.
    // Independent of specific runtime systems or schedulers.
    _TAG_EXEC, "Execution semantics", "↦";
    // Subject to change; APIs or semantics are not yet stabilized.
    _TAG_EXPERIMENTAL, "Experimental", "🧪";
    // A fake implementation intentionally diverges from real-world semantics,
    // while preserving the role or shape of the real abstraction.
    _TAG_FAKE, "Intentional non-real or placeholder implementation", "🎭";
    // Types and APIs that are safe to use across foreign language boundaries.
    _TAG_FFI, "FFI safe", "🛡️";
    _TAG_FS, "File system", "📁";
    _TAG_FONT, "Font or glyph", "🅵";
    _TAG_FMT, "Formatting", "🖹";
    _TAG_GEOM, "Geometry", "📐";
    _TAG_GEOM_DIR, "Direction and orientation", "🧭";
    // RAII abstractions whose semantics are driven by scope exit (`Drop`).
    _TAG_GUARD, "Scoped guard", "🔒";
    _TAG_HASH, "Hashing", "🔀";
    _TAG_IMAGE, "Image", "🖼️";
    _TAG_INIT, "Initialization", "🌱";
    // Human intent vocabulary
    _TAG_INTERACTION, "Human interaction", "🎮";
    _TAG_INTROSPECT, "Structural introspection", "🔎";
    _TAG_IO, "Input and output", "🔌";
    // Iterator traits and adapters operating on sequential iteration.
    _TAG_ITERATOR, "Iterator or iterator adapter", "🔄";
    // Arrangement in conceptual or visual space, not in RAM.
    _TAG_LAYOUT, "Spatial layout", "🧱";
    // Borrowed views, scoped validity, and ownership relations.
    // Applies when Rust lifetime semantics are the primary design constraint,
    // not merely an implementation detail.
    _TAG_LIFETIME, "Lifetime", "🍃";
    // Linear algebraic structures and operations.
    // Covers vector spaces, matrices, linear maps, and linear optimization.
    // Implies algebraic linearity, not data layout, storage, or sequencing.
    _TAG_LIN, "Linear algebraic structure", "⊕";
    _TAG_LINUX, "Linux platform", "🐧";
    // Sequential data structures with ordered elements.
    // Emphasizes layout and traversal (arrays, lists, queues, stacks).
    // Excludes associative, keyed, or graph-based collections.
    _TAG_LIST, "Sequential data structures", "≡";
    // _TAG_LOCATION, "", "🖈";
    _TAG_LOG, "Logging", "🪵";
    // Abstractions whose primary subject is formal or mathematical logic:
    // truth, relations, inference, or logical composition.
    _TAG_LOGIC, "Logic", "∧";
    // Abstractions that unify multiple representations or execution paths
    // behind a single interface, typically via runtime selection.
    _TAG_MAYBE, "Conditional representation", "🤷"; // shrugging
    // Memory form / representation (layout, bits, alignment, validity),
    // independent of allocation strategy. Excludes allocation strategy and lifetime semantics.
    _TAG_MEM, "Memory representation", "🫗"; // (glass pouring)
    // Items used primarily as method namespaces or operation groupings.
    // Not intended to carry semantic state of their own.
    _TAG_NAMESPACE, "Utility namespace", "🛠️";
    _TAG_NETWORK, "Networking", "📡";
    _TAG_NICHE, "Niche memory optimizations", "⚗️";
    // Semantic absence, emptiness, or inert behavior.
    // Represents "nothing happens" or "nothing is present",
    // not an error, failure, or alternative outcome.
    _TAG_NO, "Absence, emptiness or a no-op effect", "∅";
    _TAG_NON_STANDARD, "Non-standard", "⚠️";
    // Numeric structures, operations, and mathematical computation.
    _TAG_NUM, "Numeric structures and computation", "⅀";
    // Platform-dependent behavior or guarantees.
    _TAG_PLATFORM, "Platform-dependent", "🖥️";
    _TAG_PRIMITIVE, "Rust primitive", "⚙️";
    // Quantitative relations and measured magnitudes.
    // Implies numeric structure, but focuses on measurement and comparison.
    _TAG_QUANT, "Quantitative relations", "📏";
    _TAG_RAND, "Randomness", "🎲";
    // Terminal outcome values in a control or computation flow,
    // not producers or fallible abstractions.
    _TAG_RESULT, "Outcome", "⚖️";
    // Live orchestration of execution flows.
    // Covers schedulers, tasks, async runtimes, and progression of active systems.
    // Excludes structural invocation semantics.
    _TAG_RUNTIME, "Runtime", "⬡";
    // Symbolic representations and manipulation of expressions.
    // Focuses on form and rewriting, not truth evaluation or inference.
    // Applicable across numeric, logical, and domain-specific languages.
    _TAG_SYMB, "Symbolic representation", "🔣";
    _TAG_TERM, "Terminal platform", "🮖";
    _TAG_TEXT, "Text", "𝐓";
    _TAG_TIME, "Time", "🕘";
    _TAG_UNIX, "Unix platform", "🐚";
    // Marks host-level contexts that anchor user interaction
    // (e.g., windows, terminal screens, UI surface identities).
    // Not event or intent semantics.
    _TAG_UI, "User interface boundary", "▦";
    _TAG_UID, "Identification", "🫆"; // (fingerprint)
    // Abstractions whose primary concern is the semantic meaning or
    // transformation of values, independent of storage, borrowing, or encoding.
    _TAG_VALUE, "Value semantics", "💱";
    // Wave, oscillatory, and signal-domain analysis
    // (frequency, phase, spectra, transforms).
    _TAG_WAVE, "Wave and signal analysis", "〰️";
    _TAG_WEB, "Web platform", "🌐";
    _TAG_WINDOWS, "Windows platform", "🪟";

    /* misc. */

    _TAG_MAYBE_STD,
        "re-exported from `std` when available, otherwise replaced with an internal equivalent",
        "?std";
    _TAG_OPTIONAL_STD, "uses `std` features when enabled; or employs fallbacks otherwise", "±std";
    _TAG_CODEGEN_BUILD, "code generated in the build script", "<small>cgen</small>";
    // Should only be documented with the __docs_internal feature enabled
    _TAG_INTERNAL, "Internal item", "🥷";
    _TAG_PROCEDURAL_MACRO, "procedural macro", "<small>proc</small>";
    _TAG_WIP, "Work In Progress", "🚧";

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
