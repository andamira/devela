// devela_base_core::_doc::_mod_docs
//
//! Docs headings for modules across crates and misc. abbreviations.
//

#![allow(missing_docs, reason = "hidden internals for the workspace")]

crate::CONST! { hidden macro_export,
    /* abbreviations */

    /// - <https://en.wikipedia.org/wiki/Domain-specific_language>
    _ABBR_DSL  = "<abbr title='Domain-specific Language'>DSL</abbr>";
    /// - <https://www.unicode.org/reports/tr29/#:~:text=An-,extended%20grapheme%20cluster,-is>
    _ABBR_EGC  = "<abbr title='Extended Grapheme Cluster'>EGC</abbr>";
    /// - <https://en.wikipedia.org/wiki/Foreign_function_interface>
    _ABBR_FFI  = "<abbr title='Foreign Function Interface'>FFI</abbr>";
    /// - <https://en.wikipedia.org/wiki/Queue_(abstract_data_type)>
    _ABBR_FIFO = "<abbr title='First-In, First-Out'>FIFO</abbr>."; // queue
    /// - <https://en.wikipedia.org/wiki/Stack_(abstract_data_type)>
    _ABBR_LIFO = "<abbr title='Last-In, First-Out'>LIFO</abbr>."; // stack
    /// - <https://en.wikipedia.org/wiki/Lookup_table>
    _ABBR_LUT  = "<abbr title='Look-up Table'>LUT</abbr>";
    /// - <https://en.wikipedia.org/wiki/Pseudorandom_number_generator>
    _ABBR_PRNG = "<abbr title='Pseudorandom number generator'>PRNG</abbr>";
        /// - <https://en.wikipedia.org/wiki/Linear_congruential_generator>
        _ABBR_LCG  = "<abbr title='Linear congruential generator'>LCG</abbr>";
        /// - <https://en.wikipedia.org/wiki/Permuted_congruential_generator>
        _ABBR_PCG  = "<abbr title='Permuted congruential generator'>PCG</abbr>";

    /* misc. root modules */

    _DOC_ZALL  = "<hr/>All crate items re-exported in a single flat namespace.\n\n
<br/>This flat view appears here as `zall`, and it is also mirrored at the
crate root through a hidden `all` module. Every item in this namespace
is already forwarded upward, since public modules automatically
re-export their contents to their parent.

The name `zall` ensures rustdoc search shows each item's original
full path rather than the earlier-sorted `all` re-export.";

    _DOC_ZALL_ = "All crate items re-exported, grouped by their root modules.\n\n
<br/>Each root module appears here and provides its own flat view of all its
public children. Together, they offer a structured alternative to the
fully flat `zall` namespace.

It is also mirrored at the crate root through a hidden `all_` module.";

    /* thematic modules tree */

    // 1 (the internal language of the library)
    _DOC_CODE            = "Reflective code synthesis.";
    _DOC_CODE_ANY        = "Dynamic typing and reflection.";
    _DOC_CODE_ERROR      = "Errors, backtraces and structured handling.";
    _DOC_CODE_MARKER     = "Marker types, traits, and macros.";
    _DOC_CODE_OPS        = "Overloadable operators.";
    _DOC_CODE_PANIC      = "Panic hooks, unwinding, and abort strategies.";
    _DOC_CODE_RESULT     = "Parameterized outcome and resolution types.";
    _DOC_CODE_UTIL       = "Utility macros and hint functions.";

    // 2 (structure without interpretation)
    _DOC_DATA              = "Data handling and manipulation.";
    _QUO_DATA              = "> How values are organized and accessed.";
    _DOC_DATA_ACCESS         = "Mechanisms of reachability and traversal.";
    _QUO_DATA_ACCESS         = "> Reachability without relocation.";
    _DOC_DATA_ACCESS_ADDRESS = "Addressability and directed reachability.";
    _DOC_DATA_ACCESS_ITER    = "Composable external iteration.";
    _DOC_DATA_CODEC         = "Data encoding and decoding abstractions.";
    _QUO_DATA_CODEC         = "> Structured, algorithmic transformations of data representations.";
    _DOC_DATA_CODEC_BIT     = "Bit-focused items.";
    _DOC_DATA_CODEC_CRYPTO  = "Cryptographic primitives for encryption, authentication, and hashing.";
    _DOC_DATA_CODEC_HASH    = "Generic hashing support.";
    _DOC_DATA_CODEC_HASH_CHECK = "Lightweight checksums for accidental corruption detection.";
    _DOC_DATA_ERROR         = "Data-related error types.";
    _DOC_DATA_ID            = "Identity abstractions for stable and contextual distinction.";
    _QUO_DATA_ID            = "> Distinction preserved across change.";
    _DOC_DATA_ID_KEY        = "Key-based storage implementations.";
    _DOC_DATA_ID_UID        = "Abstractions for producing and stabilizing unique identities.";
    _QUO_DATA_ID_UID        = "> Defines identity kinds and generation tools.";
    _DOC_DATA_LAYOUT        = "Structural arrangement of elements in memory or sequence.";
    _QUO_DATA_LAYOUT        = "> Determines how values are positioned and grouped.";
    _DOC_DATA_LAYOUT_ARRAY  = "Contiguous homogeneous storage with dimensional projections.";
    _DOC_DATA_LAYOUT_BUFFER = "Capacity-managed storage with explicit occupancy state.";
    _DOC_DATA_LAYOUT_DST    = "Dynamically-sized types stored without need of heap allocation.";
    _DOC_DATA_LAYOUT_QUEUE  = concat!["Homogeneous data structures that process elements in ",
        $crate::_ABBR_FIFO!(), " order."];
    _DOC_DATA_LAYOUT_STACK  = concat!["Homogeneous data structures that process elements in ",
        $crate::_ABBR_LIFO!(), " order."];
    _DOC_DATA_LAYOUT_TABLE  = "Tabular and heterogeneous data processing.";
    _DOC_DATA_SPACE         = "Spatial data structures.";
    _DOC_DATA_TOPOL         = "Relational topology over structured data.";
    _QUO_DATA_TOPOL         = "> Describes connectivity, adjacency, and ordered relations.";
    _DOC_DATA_TOPOL_LINKED  = "Homogeneous, sequentially accessed structures.";
    _DOC_DATA_TOPOL_NODE    = "Abstractions for structured relationships.";
    _DOC_DATA_TOPOL_SPATIAL = "Spatial adjacency and partitioning structures over indexed data.";
    _QUO_DATA_TOPOL_SPATIAL = "> Organizes locality, neighborhood, and region-based relations.";
    _DOC_DATA_VALUE         = "Enumerated data values and types, classified by size.";

    // 3 (the grammar of space)
    _DOC_GEOM              = "Geometric types, operations, and spatial constructs.";
    _DOC_GEOM_AFFINE       = "Structure of space under translation and linear combination.";
    _DOC_GEOM_AFFINE_FRAME = "Affine reference frames and coordinate systems.";
    _DOC_GEOM_AFFINE_MAP   = "Affine maps combining linear transformation and translation.";
    _DOC_GEOM_AFFINE_POINT = "Positions in affine space without metric or unit semantics.";
    _DOC_GEOM_AFFINE_TRANSFORM = "Semantic affine transformations applied to geometric entities.";
    _DOC_GEOM_DIR          = "Direction, orientation, and angular structure of space.";
    _DOC_GEOM_DIR_NAV      = "Spatial navigation and facing semantics.";
    _DOC_GEOM_FIG          = "Concrete geometric figures and objects.";
    _DOC_GEOM_FIG_CURVE    = "Curved geometric primitives and parametric paths.";
    _DOC_GEOM_FIG_LINE     = "Linear geometric primitives such as segments and rays.";
    _DOC_GEOM_FIG_POLY     = "Polygonal and polyhedral geometric figures.";
    _DOC_GEOM_FIG_SET      = "Collections of geometric figures for aggregation and derivation.";
    _DOC_GEOM_METRIC       = "Measurement of space: distances, extents, and magnitudes.";
    _DOC_GEOM_REL          = "Spatial predicates and semantic relations between geometric entities.";
    _DOC_GEOM_REL_ADJACENCY   = "Neighborhood and reachability relations between entities.";
    _DOC_GEOM_REL_CONTAINMENT = "Inside, outside, and inclusion relations.";
    _DOC_GEOM_REL_INCIDENCE   = "Incidence relations such as touching and crossing.";
    _DOC_GEOM_REL_ORDER       = "Ordering relations such as betweenness and sequence.";
    _DOC_GEOM_REL_SHARE       = "Relations of shared spatial support or overlap.";
    _DOC_GEOM_SPACE        = "Global organization, decomposition, and structure of space.";
    _DOC_GEOM_SPACE_FIELD  = "Scalar and vector quantities defined over space.";
    _DOC_GEOM_SPACE_GRID   = "Geometric grid systems using guides, modules, and spatial rhythm.";
    _DOC_GEOM_SPACE_LAYOUT = "Constraint-based spatial resolution.";
    _DOC_GEOM_SPACE_MOTION = "Geometric motion and change of space over time.";
    _DOC_GEOM_SPACE_PART   = "Spatial partitioning, subdivision, and tessellation.";
    _DOC_GEOM_SPACE_TOPOL  = "Topological properties of space such as connectivity and boundaries.";

    // 4 (structures of meaning, expression, and interpretation across domains)
    _DOC_LANG               = "Language structure and meaning across domains.";
    _DOC_LANG_DISC          = "Discourse and expression.";
    _DOC_LANG_DISC_RETHORIC = "Rethorical structures and stylistic devices.";
    _DOC_LANG_HUM           = "Human linguistics and language theory.";
    _DOC_LANG_HUM_ART       = "Artificial human languages.";
    _DOC_LANG_HUM_GRAM      = "Grammar and structural theory.";
    _DOC_LANG_HUM_I18N      = "Internationalization and localization support.";
    _DOC_LANG_HUM_NAT       = "Natural human languages.";
    _DOC_LANG_PROG          = "Programming languages.";
    _DOC_LANG_PROG_FFI      = concat![$crate::_ABBR_FFI!(), " bindings and interoperability."];
    _DOC_LANG_PROG_FFI_C    =
        "<a href='https://en.wikipedia.org/wiki/C_(programming_language)'>C</a> interfacing.";
    _DOC_LANG_REPR          = "Representation languages.";
    _DOC_LANG_SEM           = "Semantic relations, independent of form and execution.";

    // 5 (artifacts meant to be perceived)
    _DOC_MEDIA              = "Media representation, processing, and synthesis.";
    _DOC_MEDIA_AUDIO        = "Audio signal representation and processing.";
    _DOC_MEDIA_COLOR        = "Color models and chromatic systems.";
    _DOC_MEDIA_DRAW         = "Geometric drawing primitives and composition.";
    _DOC_MEDIA_FONT         = "Font data, metrics, and shaping systems.";
    _DOC_MEDIA_IMAGE        = "Raster image representation and processing.";
    _DOC_MEDIA_MOTION       = "Perceptual motion and temporal sequencing.";
    _DOC_MEDIA_PLATE        = "Retained, page-based document surfaces.";
    _DOC_MEDIA_PLATE_PDF    = "PDF document generation.";
    _DOC_MEDIA_VIDEO        = "Video stream representation and processing.";

    // 6 (formal systems of magnitude, order, and uncertainty)
    _DOC_NUM             = "Numerical types, structures, and operations.";
    _DOC_NUM_DOM         = "Numeric domains and value representations.";
    _DOC_NUM_DOM_REAL    = "Real-valued numeric domains and representations.";
    _DOC_NUM_DOM_REAL_FLOAT = "Floating point types and operations.";
    _DOC_NUM_DOM_INT     = "Integer types and operations.";
    _DOC_NUM_ERROR       = "Numeric-related error types.";
    _DOC_NUM_FIN         = "Finite and discrete numeric structures.";
    _DOC_NUM_FIN_BIT     = "Bitwise operations.";
    _DOC_NUM_FIN_LOGIC   = "Truth systems, reachability, constraints.";
    _DOC_NUM_FIN_ORD     = "Algorithms and structures based on relative ordering.";
    _DOC_NUM_GRAIN       = "Structural granularity and representation of numeric values.";
    _DOC_NUM_GRAIN_CAST  = "Casting between primitives.";
    _DOC_NUM_GRAIN_NICHE = "Specialized numeric types and behaviors.";
    _DOC_NUM_GRAIN_WIDE  = "Wide numeric types and parallel arithmetic.";
    _DOC_NUM_LIN         = "Linear algebraic structures and methods.";
    _DOC_NUM_PROB        = "Probability theory and statistical inference.";
    _DOC_NUM_PROB_RAND   = "Random number generation.";
    _DOC_NUM_PROB_RAND_PRNG  = concat!["Concrete ", $crate::_ABBR_PRNG!(), "s"];
    _DOC_NUM_PROB_RAND_NOISE = "Structured deterministic randomness.";
    _DOC_NUM_PROB_STATS  = "Descriptive statistics.";
    _DOC_NUM_QUANT       = "Quantification, measurement, and numerical relationships.";
    _DOC_NUM_QUANT_CONT  = "Continuity, calculus.";
    _DOC_NUM_QUANT_SCALE = "Scaling, remapping, and magnitude transformation of numeric values.";
    _DOC_NUM_SYMB        = "Symbolic numeric forms and manipulation.";

    // 7 (patterns of agency beyond the individual)
    _DOC_ORG         = "Coordination and structure of collective action.";
    _DOC_ORG_AGENT   = "Intentional entities capable of action and coordination.";
    _DOC_ORG_ECON    = "Exchange, incentives, and flows of value within collectives.";
    _DOC_ORG_GOV     = "Governance, authority, and rule-based coordination.";
    _DOC_ORG_INST    = "Enduring organizational forms and institutional structures.";
    _DOC_ORG_MORAL   = "Normative frameworks of responsibility, duty, and judgment.";
    _DOC_ORG_NORM    = "Shared social norms and informal behavioral expectations.";
    _DOC_ORG_ROLE    = "Roles, responsibilities, and positions within coordinated action.";

    // 8 (measured reality constrained by nature)
    _DOC_PHYS             = "Physical quantities, units, and models of the natural world.";
    _DOC_PHYS_ASTRO       = "Astronomy-related abstractions.";
    _DOC_PHYS_BIO         = "Biology-related abstractions.";
    _DOC_PHYS_CHEM        = "Chemistry-related abstractions.";
    _DOC_PHYS_ELEC        = "Electromagnetic-related abstractions.";
    _DOC_PHYS_HEAT        = "Thermodynamics and heat transfer.";
    _DOC_PHYS_GEO         = "Geophysics-related abstractions.";
    _DOC_PHYS_MECH        = "Mechanics-related abstractions.";
    _DOC_PHYS_OPTIC       = "Optics and light transport in piecewise media";
    _DOC_PHYS_TIME        = "Time and calendar types and operations.";
    _DOC_PHYS_TIME_SOURCE = "Time sources.";
    _DOC_PHYS_UNIT        = "Physical units of measure and unit prefixes.";
    _DOC_PHYS_WAVE        = "Wave primitives, wavelets.";

    // 9 (temporal structure and staging of program execution)
    _DOC_RUN               = "Temporal coordination and staging of a running system.";
    _QUO_RUN               = "> Where execution exists, progresses, and becomes concrete.";
    _DOC_RUN_CYCLE         = "Phases and transitions of a running system.";
    _DOC_RUN_TIME          = "Temporal structure and progression within a run.";
    _DOC_RUN_TIME_FRAME    = "The per-step temporal envelope and state snapshot.";
    _DOC_RUN_TIME_LOOP     = "Composed policies for advancing time within a run.";
    _DOC_RUN_REGIME        = "The committed configuration of a running environment.";
    _DOC_RUN_REGIME_CAP    = "Declared runtime capabilities available to a running system.";
    _DOC_RUN_STATE         = "The mutable state of a running system.";
    _DOC_RUN_STATE_CONTEXT = "Ambient, ephemeral state associated with the current run step.";
    _DOC_RUN_STATE_LOG     = "Sequential records of runtime state evolution.";
    _DOC_RUN_STATE_SCENE   = "A bounded runtime situation defining active state and transitions.";

    // 10 (the contract with the host)
    _DOC_SYS             = "System interfaces and hardware abstractions.";
    _DOC_SYS_ARCH        = "Architecture-specific intrinsics.";
    _DOC_SYS_DEVICE      = "Live system device interfaces.";
    _QUO_SYS_DEVICE      = "> Usable system devices exposed to running programs.";
    _DOC_SYS_DEVICE_AUDIO       = "Audio device backends and stream interfaces.";
    _DOC_SYS_DEVICE_DISPLAY     = "Display backends for windows, surfaces, and events.";
    _DOC_SYS_DEVICE_DISPLAY_X11 = "X11 display backend.";
    _DOC_SYS_ENV         = "Process environment inspection and manipulation.";
    _DOC_SYS_FS          = "Filesystem abstractions.";
    _DOC_SYS_FS_PATH     = "Cross-platform path manipulation.";
    _DOC_SYS_HW          = "Low-level hardware and driver-facing system interfaces.";
    _QUO_SYS_HW          = "> Hardware-facing system foundations.";
    _DOC_SYS_IO          = "I/O primitives and stream interfaces.";
    _DOC_SYS_LOG         = "Logging functionality.";
    _DOC_SYS_MEM         = "Memory primitives, layout contracts, and safe access foundations.";
    _QUO_SYS_MEM         = "> How bytes live, move, and are validated.";
    _DOC_SYS_MEM_ALLOC   = "Allocation strategies and ownership-backed storage abstractions.";
    _DOC_SYS_MEM_BOUND   = "Addressing, alignment, and movement constraints over memory.";
    _DOC_SYS_MEM_BOUND_PIN = "Types that pin data to a location in memory.";
    _DOC_SYS_MEM_BOUND_PTR = "Manual memory management via raw pointers.";
    _DOC_SYS_MEM_CELL    = "Shareable containers with interior mutability.";
    _DOC_SYS_MEM_LAYOUT  = "Memory layout, bit-validity, and representation invariants.";
    _DOC_SYS_MEM_SIZE    = "Memory size, bit width, and storage accounting.";
    _DOC_SYS_MEM_VIEW    = "Typed and byte-level views over memory.";
    _DOC_SYS_MEM_VIEW_BORROW = "Borrowed data and ownership-relaxed views.";
    _DOC_SYS_MEM_VIEW_SLICE  = "Contiguous views into memory.";
    _DOC_SYS_NET         = "Networking functionality.";
    _DOC_SYS_OS          = "Operating systems and supervisors.";
    _DOC_SYS_OS_BROWSER      = "Browser supervisory environment.";
    _DOC_SYS_OS_BROWSER_WEB  = "Web APIs interfacing.";
    _DOC_SYS_OS_FD       = "Unix-like file descriptors.";
    _DOC_SYS_OS_LINUX    = "Linux-specific definitions.";
    _DOC_SYS_OS_TERM     = "Terminal supervisory environment and ANSI/TTY interfacing.";
    _DOC_SYS_OS_WINDOWS  = "Windows-specific definitions.";

    // 11 (symbolic sequences with cultural weight)
    _DOC_TEXT            = "Text types and processing.";
    _DOC_TEXT_ASCII      = "ASCII strings and characters.";
    _DOC_TEXT_CHAR       = "Unicode scalar types and operations.";
    _DOC_TEXT_GRAPHEME   = concat!["Unicode", $crate::_ABBR_EGC!(), "s."];
    _DOC_TEXT_ERROR      = "Text-related error types.";
    _DOC_TEXT_FMT        = "String formatting.";
    _DOC_TEXT_LAYOUT     = "One-dimensional spatial semantics of text.";
    _QUO_TEXT_LAYOUT     = "> A negotiation between a symbolic sequence and an available extent.";
    _DOC_TEXT_PARSE      = "Unstructured string parsing.";
    _DOC_TEXT_STR        = "String types and related functionality.";

    // 12 (interaction structures, presentation state, and input semantics)
    _DOC_UI         = "Interactive surfaces, input semantics, and presentation state.";
    _QUO_UI         = "> How humans interact with what exists.";
    _DOC_UI_EVENT   = "User interface events-related functionality.";
    _DOC_UI_INTENT  = "Declarative user and device intent resolved against runtime capabilities.";
    _DOC_UI_LAYOUT  = "Interactive spatial arrangement and negotiation.";
    _DOC_UI_VIEW    = "Stateful interactive projections.";

    // 13 (practices, capacities, and meanings of lived life)
    _DOC_VITA       = "Lived practices of embodied beings.";
    _DOC_VITA_BODY  = "Embodied capability, health, and physical limits of living beings.";
    _DOC_VITA_CRAFT = "Practical, learned ways of shaping material reality to support life.";
    _DOC_VITA_HOME  = "Inhabited space, from dwelling to built and shared environments.";
    _DOC_VITA_LOVE  = "Relational life, from kinship and care to intimacy and community.";
    _DOC_VITA_MIND  = "Cognition, memory, meaning, and inner orientation of lived experience.";
    _DOC_VITA_PLAY  = "Expression, play, and shared enjoyment beyond necessity or survival.";

    // 14 (effort structured over time)
    _DOC_WORK             = "Computational work and its execution mechanics.";
    _DOC_WORK_ACTOR       = "Isolated units of agency that own state and communicate by message.";
    _DOC_WORK_FUTURE      = "Asynchronous work and continuations.";
    _DOC_WORK_FUTURE_COROUTINE = "Stackless coroutines with explicit scheduling and runners.";
    _DOC_WORK_PROCESS     = "Process-based execution of work.";
    _DOC_WORK_SYNC        = "Synchronization mechanisms for concurrent work.";
    _DOC_WORK_SYNC_ATOMIC = "Atomic types.";
    _DOC_WORK_SYNC_MPSC   = "Multi-producer, single-consumer channels.";
    _DOC_WORK_THREAD      = "Thread-based execution of work.";

    // (15) (support structures not part of the public scene)
    _DOC_YARD       = "Internal scaffolding and misc. machinery.";
    _QUO_YARD       = "> This space exists so the rest can be clean.";
    _DOC_YARD__DEP  = "Re-exported dependencies.";
}
