// devela/src/yard/tags.rs
//
//! Private tags definitions for visual type categorization in documentation.
//
// MAYBE
// - _ORD
// - _STATE
//
// NOTES
// - icons used by _doc_location: 📍,
// - icons used by _doc_test_size_of: 📦,⚗️,

/// Aggregates multiple documentation tags into a single `#[doc = ...]` string.
#[doc(hidden)]
#[macro_export(local_inner_macros)] // local modifier needed to enable resolution
macro_rules! _tags {
    () => {""};
    ($($tag:ident)+) => { concat![$( $crate::_tags![@$tag], " "),+] };
    (@$tag:ident) =>  { $crate::paste! { [<_TAG_ $tag:upper>] !() }};
}
#[allow(unused)]
pub use _tags;

// helper for defining doc tags with an associated symbol and title attribute
macro_rules! _define_symbol_tags {
    ($( $tag:ident, $title:literal, $symbol:literal;)+) => {
        $crate::CONST! { hidden macro_export, // pub(crate), // NOTE: needed by define_error!
            $( $tag = concat![$crate::SPAN_OPEN!(), "'", $title, "'>", $symbol, "</span>"];)+
        }
    };
}
// SEMANTIC AXES AND CONCERNS
// - what it represents      (VALUE, NUM, LOGIC, DATA)
// - how it is realized      (MEM, LIFETIME, GUARD, NICHE)
// - where it applies        (PLATFORM, RUNTIME, TERM, WEB)
// - how it operates         (CODE, CONSTRUCTION, ITERATOR)
_define_symbol_tags! {
    /* thematic */

    // Representation shaped by a foreign, platform, or kernel ABI boundary.
    _TAG_ABI, "ABI-shaped representation", "🔻";
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
    // Cryptographic algorithms and protocol primitives.
    // Covers encryption, authentication, cryptographic hashing, and key-related machinery.
    _TAG_CRYPTO, "Cryptography", "🔐";
    // General data carriers and abstractions.
    // Focuses on representation and transport of information,
    // not on its semantic meaning or mathematical structure.
    _TAG_DATA, "Data", "🪪"; // (id-card)
    // Containers, collections, and their structural accessors.
    // Emphasizes organization and access over individual value semantics.
    _TAG_DATA_STRUCTURE, "Data structure (collection)", "🗃️";
    // Diagnostics, introspection, debugging intent.
    _TAG_DEBUG, "Debugging", "🐛";
    _TAG_DIR, "Direction and orientation", "🧭";
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
    _TAG_GAME, "Game and formal play", "🀄";
    _TAG_GEOM, "Geometry", "📐";
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
    // Language systems, grammars, and interpretation.
    // Covers human, programming, and representation languages.
    _TAG_LANG, "Language structure", "𝕃";
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
    // Individual elements belonging to a set, family, enum-set, or closed domain.
    // Use for selectable members, variants, capabilities, states, or symbols
    // when their main role is membership in a corresponding collection or algebra.
    // Not for arbitrary fields, values, or Rust enums merely because they are enums.
    _TAG_MEMBER, "Member of a set or family", "∈";
    // Items used primarily as method namespaces or operation groupings.
    // Not intended to carry semantic state of their own.
    _TAG_NAMESPACE, "Utility namespace", "🛠️";
    _TAG_NETWORK, "Networking", "📡";
    // Items whose main surface depends on excluded values or niche layout.
    _TAG_NICHE, "Niche memory optimizations", "⚗️";
    // Semantic absence, emptiness, or inert behavior.
    // Represents "nothing happens" or "nothing is present",
    // not an error, failure, or alternative outcome.
    _TAG_NO, "Absence, emptiness or a no-op effect", "∅";
    _TAG_NON_STANDARD, "Non-standard", "⚠️";
    // Numeric structures, operations, and mathematical computation.
    _TAG_NUM, "Numeric structures and computation", "⅀";
    // Order-based semantics, comparisons, sorted structures, or ranked traversal.
    // Covers total/partial ordering, min/max, range queries, sorted maps/sets,
    // priority queues, and binary-search-oriented APIs.
    // Not for types merely implementing `Ord` as a convenience bound.
    _TAG_ORD, "Ordering semantics", "≤";
    // Transforming input streams or symbolic sequences into structured meaning.
    // Covers scanners, parsers, parser states, parse errors, and grammar readers.
    // Excludes raw text storage and formatting.
    _TAG_PARSER, "Parsing", "::=";
    // Platform-dependent behavior or guarantees.
    // It is exclusive to specific platforms.
    _TAG_PLATFORM, "Platform-dependent", "🖥️";
    // Items whose main surface is about Rust primitive carriers.
    _TAG_PRIMITIVE, "Related to Rust primitives and their carriers", "⚙️";
    // Rules, messages, and state transitions of an interoperable communication contract.
    // Covers protocol vocabularies, framing, negotiation, and endpoint behavior.
    // Excludes generic transports, codecs, parsers, static formats, and ABIs.
    _TAG_PROTOCOL, "Communication protocol", "⇄";
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
    // Membership-based set semantics.
    // Covers finite sets, bit sets, enum sets, flag sets, and set-like collections.
    // Use with `bit` when the representation is a bit mask.
    _TAG_SET, "Set semantics", "⊆";
    // Propagated signal semantics: sampled streams, notifications, or control signals.
    _TAG_SIGNAL, "Signal", "📶";
    _TAG_STRING, "String storage and views", "🧶"; // (yarn)
    // Symbolic representations and manipulation of expressions.
    // Focuses on form and rewriting, not truth evaluation or inference.
    // Applicable across numeric, logical, and domain-specific languages.
    _TAG_SYMB, "Symbolic representation", "🔣";
    _TAG_TERM, "Terminal platform", "🮖";
    _TAG_TEXT, "Text", "𝐓";
    _TAG_TIME, "Time", "🕘";
    _TAG_UNIX, "Unix platform", "🐚";
    // UI-specific interaction, layout, semantic, and presentation abstractions.
    _TAG_UI, "User interface", "▦";
    _TAG_UID, "Identification", "🫆"; // (fingerprint)
    // Abstractions whose primary concern is the semantic meaning or
    // transformation of values, independent of storage, borrowing, or encoding.
    _TAG_VALUE, "Value semantics", "💱";
    // Wave, oscillatory, and signal-domain analysis
    // (frequency, phase, spectra, transforms).
    _TAG_WAVE, "Wave and signal analysis", "〰️";
    _TAG_WEB, "Web platform", "🌐";
    _TAG_WINDOWS, "Windows platform", "🪟";
    _TAG_WORD, "Fixed-width encoded word", "▣";

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
