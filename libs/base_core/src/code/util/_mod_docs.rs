// devela_base_core::code::util::_docs
//
//! Docs headings for modules across crates and misc. abbreviations.
//

#![allow(missing_docs, reason = "hidden internals for the workspace")]

crate::CONST! { hidden macro_export,
/* templates:
#![doc = crate::_DOC_X!()]
    _DOC_ = "";
*/

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

    _DOC_ZALL  = "All crate items re-exported in a single flat namespace.\n\n
This flat view appears here as `zall`, and it is also mirrored at the
crate root through a hidden `all` module. Every item in this namespace
is already forwarded upward, since public modules automatically
re-export their contents to their parent.

The name `zall` ensures rustdoc search shows each item's original
full path rather than the earlier-sorted `all` re-export.";

    _DOC_ZALL_ = "All crate items re-exported, grouped by their root modules.\n\n
Each root module appears here and provides its own flat view of all its
public children. Together, they offer a structured alternative to the
fully flat `zall` namespace.

It is also mirrored at the crate root through a hidden `all_` module.";

    _DOC_CODE            = "Reflective code synthesis.";
    _DOC_CODE_ANY        = "Dynamic typing and reflection.";
    _DOC_CODE_ERROR      = "Errors, backtraces and structured handling.";
    _DOC_CODE_MARKER     = "Marker types, traits, and macros.";
    _DOC_CODE_PANIC      = "Panic hooks, unwinding, and abort strategies.";
    _DOC_CODE_RESULT     = "Parameterized outcome and resolution types.";
    _DOC_CODE_UTIL       = "Utility macros and hint functions.";

    _DOC_DATA            = "Data handling and manipulation.";
    _DOC_DATA_ADDRESS    = "Addressability and directed reachability.";
    _DOC_DATA_CODEC      = "Data encoding and decoding abstractions.";
    _DOC_DATA_CODEC_HASH = "Generic hashing support.";
    _DOC_DATA_CODEC_HASH_CHECK = "Lightweight checksums for accidental corruption detection.";
    _DOC_DATA_ITER       = "Composable external iteration.";
    _DOC_DATA_KEY        = "Key-based storage implementations.";
    _DOC_DATA_LIST       = "Sequential collections implementations.";
    _DOC_DATA_LIST_ARRAY = "Homogeneous, sequentially allocated, random-access structures.";
    _DOC_DATA_LIST_LINK  = "Homogeneous, sequentially accessed structures.";
    _DOC_DATA_LIST_QUEUE = concat!["Homogeneous data structures that process elements in ",
        $crate::_ABBR_FIFO!(), " order."];
    _DOC_DATA_LIST_STACK = concat!["Homogeneous data structures that process elements in ",
        $crate::_ABBR_LIFO!(), " order."];
    _DOC_DATA_UID        = "Abstractions for producing and stabilizing unique identities.";

    _DOC_GAME = "Game-development and interactive applications.";

    _DOC_LANG = concat!["Language facilities, ",
        $crate::_ABBR_DSL!(), "s and ", $crate::_ABBR_FFI!(),"s."];
    _DOC_LANG_FFI = concat![$crate::_ABBR_FFI!(),
        "bindings and interoperability.\n\nBridges to external languages and platforms."];
    _DOC_LANG_FFI_C =
        "<a href='https://en.wikipedia.org/wiki/C_(programming_language)'>C</a> interfacing.";

    _DOC_MEDIA      = "Multimedia functionality.";

    _DOC_NUM             = "Numerical types and operations.";
    _DOC_NUM_FLOAT       = "Floating point types and operations.";
    _DOC_NUM_GEOM        = "Geometric types, operations, and spatial constructs.";
    _DOC_NUM_GEOM_METRIC = "Geometric measurement and spatial relationships.";
    _DOC_NUM_GEOM_DIR    = "Direction, orientation, and spatial symmetry.";
    _DOC_NUM_INT         = "Integer types and operations.";
    _DOC_NUM_NICHE       = "Specialized numeric types and behaviors.";
    _DOC_NUM_ORD         = "Algorithms and structures based on relative ordering.";
    _DOC_NUM_QUANT       = "Quantification and numerical relationships.";
    _DOC_NUM_RAND        = "Random number generation.";
    _DOC_NUM_WIDE        = "Wide numeric types and parallel arithmetic.";

    _DOC_PHYS            = "Physical units and measurement.";
    _DOC_PHYS_TIME       = "Time and calendar types and operations.";

    _DOC_SYS             = "System interfaces and hardware abstractions.";
    _DOC_SYS_ARCH        = "Architecture-specific intrinsics.";
    _DOC_SYS_ENV         = "Process environment inspection and manipulation.";
    _DOC_SYS_FS          = "Filesystem abstractions.";
    _DOC_SYS_FS_PATH     = "Cross-platform path manipulation.";
    _DOC_SYS_HW          = "Hardware interfaces and device I/O.";
    _DOC_SYS_IO          = "I/O primitives and stream interfaces.";
    _DOC_SYS_LOG         = "Logging functionality.";
    _DOC_SYS_MEM         = "Memory management.";
    _DOC_SYS_MEM_ALLOC   = "Memory allocation.";
    _DOC_SYS_MEM_BORROW  = "Borrowed data.";
    _DOC_SYS_MEM_CELL    = "Shareable mutable containers.";
    _DOC_SYS_MEM_PIN     = "Types that pin data to a location in memory.";
    _DOC_SYS_MEM_PTR     = "Manually memory management via raw pointers.";
    _DOC_SYS_NET         = "Networking functionality.";
    _DOC_SYS_OS          = "Operating systems and supervisors.";
    _DOC_SYS_OS_FD       = "Unix-like file descriptors.";
    _DOC_SYS_SOUND       = "Sound systems.";

    _DOC_TEXT            = "Text types and processing.";
    _DOC_TEXT_ASCII      = "ASCII strings and characters.";
    _DOC_TEXT_CHAR       = "Unicode scalar types and operations.";
    _DOC_TEXT_EGC        = concat!["Unicode", $crate::_ABBR_EGC!(), "."];
    _DOC_TEXT_FMT        = "String formatting.";
    _DOC_TEXT_PARSE      = "Unstructured string parsing.";
    _DOC_TEXT_STR        = "String types and related functionality.";

    _DOC_UI              = "User interface functionality.";

    _DOC_WORK             = "Work management and concurrency.";
    _DOC_WORK_FUTURE      = "Asynchronous execution.";
    _DOC_WORK_PROCESS     = "Native process management.";
    _DOC_WORK_SYNC        = "Synchronization primitives.";
    _DOC_WORK_SYNC_ATOMIC = "Atomic types.";
    _DOC_WORK_SYNC_MPSC   = "Multi-producer, single-consumer channels.";
    _DOC_WORK_THREAD      = "Native threads.";
}
