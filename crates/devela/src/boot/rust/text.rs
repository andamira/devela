use crate::{_reexport, _tags};

pub use {fmt::*, parse::*, scalar::*, str::*};

pub(crate) mod fmt {
    use super::*;

    /* aliases */

    #[doc = crate::_tags!(fmt result)]
    /// The result of a formatting operation.
    #[doc = crate::_doc_meta!{
        location("text/fmt", type FmtResult),
    }]
    /// Equivalent to [`core::fmt::Result`] when `T` is `()`,
    /// while allowing other successful return types.
    pub type FmtResult<T = ()> = Result<T, FmtError>;

    /* macros */

    _reexport! { rust: alloc,
        location: "text/fmt" => macro format, tag: _tags!(fmt construction),
        doc: "Creates a String using interpolation of runtime expressions.",
        format
    }
    _reexport! { rust: core,
        location: "text/fmt" => macro format_args, tag: _tags!(fmt),
        doc: "Constructs parameters for the other string-formatting macros.",
        format_args
    }
    _reexport! { rust: core,
        location: "text/fmt" => macro write, tag: _tags!(fmt),
        doc: "Writes formatted data into a buffer.",
        write
    }
    _reexport! { rust: core,
        location: "text/fmt" => macro writeln, tag: _tags!(fmt),
        doc: "Writes formatted data into a buffer, with a newline appended.",
        writeln
    }

    /* traits */

    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait Binary, tag: _tags!(fmt num),
        doc: "`b` formatting.",
        Binary
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait Debug, tag: _tags!(fmt debug),
        doc: "`?` formatting. (Derivable)",
        Debug
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait Display, tag: _tags!(fmt),
        doc: "Format trait for an empty format, `{}`.",
        Display
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait LowerExp, tag: _tags!(fmt num),
        doc: "`e` formatting.",
        LowerExp
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait LowerHex, tag: _tags!(fmt num),
        doc: "`x` formatting.",
        LowerHex
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait Octal, tag: _tags!(fmt num),
        doc: "`o` formatting.",
        Octal
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait Pointer, tag: _tags!(fmt),
        doc: "`p` formatting.",
        Pointer
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait UpperExp, tag: _tags!(fmt num),
        doc: "`E` formatting.",
        UpperExp
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait UpperHex, tag: _tags!(fmt num),
        doc: "`X` formatting.",
        UpperHex
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait FmtWrite, tag: _tags!(fmt),
        doc: "Writing or formatting into Unicode-accepting buffers or streams.",
        @Write as FmtWrite
    }

    /* enums */

    _reexport! { rust: core::fmt,
        location: "text/fmt" => trait FmtAlignment, tag: _tags!(fmt),
        doc: "Possible alignments returned by `Formatter::align`.",
        @Alignment as FmtAlignment
    }

    /* structs */

    _reexport! { rust: core::fmt,
        location: "text/fmt" => struct FmtArguments, tag: _tags!(fmt),
        doc: "Represents a safely precompiled version of a format string and its arguments.",
        @Arguments as FmtArguments
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => struct DebugList, tag: _tags!(fmt debug),
        doc: "A struct to help with [`Debug`] implementations.",
        DebugList
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => struct DebugMap, tag: _tags!(fmt debug),
        doc: "A struct to help with [`Debug`] implementations.",
        DebugMap
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => struct DebugSet, tag: _tags!(fmt debug),
        doc: "A struct to help with [`Debug`] implementations.",
        DebugSet
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => struct DebugStruct, tag: _tags!(fmt debug),
        doc: "A struct to help with [`Debug`] implementations.",
        DebugStruct
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => struct DebugTuple, tag: _tags!(fmt debug),
        doc: "A struct to help with [`Debug`] implementations.",
        DebugTuple
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => struct FmtError, tag: _tags!(fmt error),
        doc: "The error type which is returned from formatting a message into a stream.",
        @Error as FmtError
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => struct FmtFromFn, tag: _tags!(fmt),
        doc: "Implements [`Debug`] and [`Display`] via the provided closure.",
        @FromFn as FmtFromFn
    }
    _reexport! { rust: core::fmt,
        location: "text/fmt" => struct Formatter, tag: _tags!(fmt),
        doc: "Configuration for formatting.",
        Formatter
    }
}

pub(crate) mod parse {
    use super::*;

    _reexport! { rust: core::num,
        location: "text/parse" => struct ParseIntError, tag: _tags!(text parser num error),
        doc: "An error which can be returned when parsing an integer.",
        ParseIntError
    }
    _reexport! { rust: core::num,
        location: "text/parse" => struct ParseFloatError, tag: _tags!(text parser num error),
        doc: "An error which can be returned when parsing an float.",
        ParseFloatError
    }

    // NOTE: Utf8Error not re-exported. See `InvalidUtf8` instead.
    // _reexport! { rust: core::str,
    //     location: "text/parse" => struct Utf8Error, tag: _tags!(text parser error),
    //     doc: "An error which can occur when interpreting a sequence of `u8` as a string.",
    //     Utf8Error
    // }

    /* enums */

    _reexport! { rust: core::num,
        location: "text/parse" => enum ParseIntErrorKind, tag: _tags!(text parser num error),
        doc: "Kinds of errors that can cause parsing an integer to fail.",
        @IntErrorKind as ParseIntErrorKind
    }
}

pub(crate) mod scalar {
    #[doc = crate::_tags!(primitive text)]
    /// <span class="stab portability" title="re-exported from rust's `core`">`core`</span>
    /// A 32-bit [Unicode scalar][scalar].
    #[doc = crate::_doc_meta!{
        location("text/unicode/scalar", type char),
        test_size_of(char = 4; niche Option),
    }]
    /// It can represent each and every scalar.
    ///
    /// See also: [`char7`], [`char8`], [`char16`].
    ///
    /// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
    /// [`char7`]: crate::char7
    /// [`char8`]: crate::char8
    /// [`char16`]: crate::char16
    ///
    /// ---
    /// ---
    // NOTE: this reexport type is not recognized implicity by rustdoc, is it a BUG?
    // TODO: minimal example and the search/make an ISSUE in rust-repo
    #[allow(non_camel_case_types)]
    pub type char = ::core::primitive::char;

    // NOTE: replaced by `CharIter`
    // _reexport! { rust: core::str,
    //     tag: crate::tags!(text iterator),
    //     doc: "An iterator over the [`char`][prim@char]s of a string slice.",
    //     @Chars as IterChars
    // }
}

pub(crate) mod str {
    use super::*;

    _reexport! { rust: alloc::string,
        location: "text/str" => struct FromUtf8Error, tag: _tags!(text error),
        doc: "A possible error value when converting a String from a UTF-8 byte vector.",
        FromUtf8Error
    }
    _reexport! { rust: std::ffi,
        location: "text/str" => struct OsStr, tag: _tags!(text lifetime platform),
        doc: "Borrowed reference to an OS string (See [`OsString`]).",
        OsStr
    }
    _reexport! { rust: std::ffi,
        location: "text/str" => struct OsString, tag: _tags!(text platform),
        doc: "A type for owned, mutable native strings, interconvertible with Rust strings.",
        OsString
    }
    _reexport! { rust: alloc::string,
        location: "text/str" => struct String, tag: _tags!(text),
        doc: "A UTF-8–encoded, growable string.",
        String
    }

    /* traits  */

    _reexport! { rust: core::str,
        location: "text/str" => trait FromStr, tag: _tags!(text lifetime),
        doc: "Parse a value from a string.",
        FromStr
    }
    _reexport! { rust: alloc::string,
        location: "text/str" => trait ToString, tag: _tags!(text value),
        doc: "A trait for converting a value to a [`String`].",
        ToString
    }
}
