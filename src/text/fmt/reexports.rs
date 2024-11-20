// devela::text::fmt::reexports
//
//!
//

use crate::code::reexport;

/* macros */

reexport! { rust: core,
    doc: "Constructs parameters for the other string-formatting macros.",
    format_args
}
reexport! { rust: alloc,
    doc: "Creates a String using interpolation of runtime expressions.",
    format
}
reexport! { rust: core,
    doc: "Writes formatted data into a buffer.",
    write
}
reexport! { rust: core,
    doc: "Writes formatted data into a buffer, with a newline appended.",
    writeln
}

/* functions */

reexport! { rust: alloc::fmt,
    doc: "Takes an [`FmtArguments`] struct and returns the resulting formatted string.",
    @format as fmt_format
}
reexport! { rust: core::fmt,
    doc: "Takes an output stream, and an [`FmtArguments`] struct.",
    @write as fmt_write
}

/* traits */

reexport! { rust: core::fmt,
    doc: "`b` formatting.",
    Binary
}
reexport! { rust: core::fmt,
    doc: "`?` formatting.",
    Debug
}
reexport! { rust: core::fmt,
    doc: "Format trait for an empty format, `{}`.",
    Display
}
reexport! { rust: core::fmt,
    doc: "`e` formatting.",
    LowerExp
}
reexport! { rust: core::fmt,
    doc: "`x` formatting.",
    LowerHex
}
reexport! { rust: core::fmt,
    doc: "`o` formatting.",
    Octal
}
reexport! { rust: core::fmt,
    doc: "`p` formatting.",
    Pointer
}
reexport! { rust: core::fmt,
    doc: "`E` formatting.",
    UpperExp
}
reexport! { rust: core::fmt,
    doc: "`X` formatting.",
    UpperHex
}
reexport! { rust: core::fmt,
    doc: "Writing or formatting into Unicode-accepting buffers or streams.",
    @Write as TextWrite
}

/* enums */

reexport! { rust: core::fmt,
    doc: "Possible alignments returned by `Formatter::align`.",
    @Alignment as FmtAlignment
}

/* aliases */

// NOTE it defines T = ()
reexport! { rust: core::fmt,
    doc: "The type returned by formatter methods.",
    @Result as FmtResult
}

/* structs */

reexport! { rust: core::fmt,
    doc: "Represents a safely precompiled version of a format string and its arguments.",
    @Arguments as FmtArguments
}
reexport! { rust: core::fmt,
    doc: "A struct to help with [`Debug`] implementations.",
    DebugList
}
reexport! { rust: core::fmt,
    doc: "A struct to help with [`Debug`] implementations.",
    DebugMap
}
reexport! { rust: core::fmt,
    doc: "A struct to help with [`Debug`] implementations.",
    DebugSet
}
reexport! { rust: core::fmt,
    doc: "A struct to help with [`Debug`] implementations.",
    DebugStruct
}
reexport! { rust: core::fmt,
    doc: "A struct to help with [`Debug`] implementations.",
    DebugTuple
}
reexport! { rust: core::fmt,
    doc: "The error type which is returned from formatting a message into a stream.",
    @Error as FmtError
}
reexport! { rust: core::fmt,
    doc: "Configuration for formatting.",
    Formatter
}
