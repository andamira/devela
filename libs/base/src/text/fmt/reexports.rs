// devela::text::fmt::reexports
//
//!
//

use crate::{TAG_FMT, TAG_RESULT};

/* macros */

// TODO
// pub use core::format_ags;
pub use core::write;
pub use core::writeln;
// reexport! { rust: core,
//     tag: TAG_FMT!(),
//     doc: "Constructs parameters for the other string-formatting macros.",
//     format_args
// }
// reexport! { rust: core,
//     tag: TAG_FMT!(),
//     doc: "Writes formatted data into a buffer.",
//     write
// }
// reexport! { rust: core,
//     tag: TAG_FMT!(),
//     doc: "Writes formatted data into a buffer, with a newline appended.",
//     writeln
// }

/* traits */

// TODO
pub use core::fmt::Binary;
pub use core::fmt::Debug;
pub use core::fmt::Display;
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "`b` formatting.",
//     Binary
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "`?` formatting.",
//     Debug
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "Format trait for an empty format, `{}`.",
//     Display
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "`e` formatting.",
//     LowerExp
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "`x` formatting.",
//     LowerHex
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "`o` formatting.",
//     Octal
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "`p` formatting.",
//     Pointer
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "`E` formatting.",
//     UpperExp
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "`X` formatting.",
//     UpperHex
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "Writing or formatting into Unicode-accepting buffers or streams.",
//     @Write as FmtWrite
// }

/* enums */

// TODO
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "Possible alignments returned by `Formatter::align`.",
//     @Alignment as FmtAlignment
// }

/* aliases */

#[doc = TAG_RESULT!()]
#[doc = TAG_FMT!()]
/// The type returned by formatter methods.
///
/// Note that this is not the same as [`core::fmt::Result`], since this one
/// doesn't hardcode the returned type to `()`.
pub type FmtResult<T> = Result<T, FmtError>;

/* structs */

pub use core::fmt::Error as FmtError;
pub use core::fmt::Formatter;
// TODO
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "Represents a safely precompiled version of a format string and its arguments.",
//     @Arguments as FmtArguments
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "A struct to help with [`Debug`] implementations.",
//     DebugList
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "A struct to help with [`Debug`] implementations.",
//     DebugMap
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "A struct to help with [`Debug`] implementations.",
//     DebugSet
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "A struct to help with [`Debug`] implementations.",
//     DebugStruct
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "A struct to help with [`Debug`] implementations.",
//     DebugTuple
// }
// reexport! { rust: core::fmt,
//     tag: concat![TAG_ERROR!(), TAG_FMT!()],
//     doc: "The error type which is returned from formatting a message into a stream.",
//     @Error as FmtError
// }
// reexport! { rust: core::fmt,
//     tag: TAG_FMT!(),
//     doc: "Configuration for formatting.",
//     Formatter
// }
