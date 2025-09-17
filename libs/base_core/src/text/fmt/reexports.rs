// devela_base_core::text::fmt::reexports
//
//!
//

use crate::{_TAG_ERROR, _TAG_FMT, _TAG_RESULT, _reexport};

/* macros */

_reexport! { rust: core,
    tag: _TAG_FMT!(),
    doc: "Constructs parameters for the other string-formatting macros.",
    format_args
}
_reexport! { rust: core,
    tag: _TAG_FMT!(),
    doc: "Writes formatted data into a buffer.",
    write
}
_reexport! { rust: core,
    tag: _TAG_FMT!(),
    doc: "Writes formatted data into a buffer, with a newline appended.",
    writeln
}

/* traits */

_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "`b` formatting.",
    Binary
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "`?` formatting.",
    Debug
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "Format trait for an empty format, `{}`.",
    Display
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "`e` formatting.",
    LowerExp
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "`x` formatting.",
    LowerHex
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "`o` formatting.",
    Octal
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "`p` formatting.",
    Pointer
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "`E` formatting.",
    UpperExp
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "`X` formatting.",
    UpperHex
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "Writing or formatting into Unicode-accepting buffers or streams.",
    @Write as FmtWrite
}

/* enums */

_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "Possible alignments returned by `Formatter::align`.",
    @Alignment as FmtAlignment
}

/* aliases */

#[doc = _TAG_RESULT!()]
#[doc = _TAG_FMT!()]
/// The type returned by formatter methods.
///
/// Note that this is not the same as [`core::fmt::Result`], since this one
/// doesn't hardcode the returned type to `()`.
pub type FmtResult<T> = Result<T, FmtError>;

/* structs */

_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "Represents a safely precompiled version of a format string and its arguments.",
    @Arguments as FmtArguments
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "A struct to help with [`Debug`] implementations.",
    DebugList
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "A struct to help with [`Debug`] implementations.",
    DebugMap
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "A struct to help with [`Debug`] implementations.",
    DebugSet
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "A struct to help with [`Debug`] implementations.",
    DebugStruct
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "A struct to help with [`Debug`] implementations.",
    DebugTuple
}
_reexport! { rust: core::fmt,
    tag: concat![_TAG_FMT!(), _TAG_ERROR!()],
    doc: "The error type which is returned from formatting a message into a stream.",
    @Error as FmtError
}
_reexport! { rust: core::fmt,
    tag: _TAG_FMT!(),
    doc: "Configuration for formatting.",
    Formatter
}
