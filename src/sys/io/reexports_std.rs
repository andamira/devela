// devela::sys::io::reexports_std
//
//! Reexported items from `std`.
//

use crate::reexport;

/* io traits */

reexport! { rust: not(std)|std::io,
    doc: "A type of `Read`er which has an internal buffer.",
    @BufRead as IoBufRead
}
reexport! { rust: not(std)|std::io,
    doc: "Allows for reading bytes from a source.",
    @Read as IoRead
}
reexport! { rust: not(std)|std::io,
    doc: "Provides a cursor which can be moved within a stream of bytes.",
    @Seek as IoSeek
}
reexport! { rust: not(std)|std::io,
    doc: "A trait for objects which are byte-oriented sinks.",
    @Write as IoWrite
}

/* io structs */

reexport! { rust: not(std)|std::io,
    doc: "Adds buffering to any reader.",
    @BufReader as IoBufReader
}
reexport! { rust: not(std)|std::io,
    doc: "Wraps a writer and buffers its output.",
    @BufWriter as IoBufWriter
}
reexport! { rust: not(std)|std::io,
    doc: "An iterator over `u8` values of a reader.",
    @Bytes as IoBytes
}
reexport! { rust: not(std)|std::io,
    doc: "Adapter to chain together two readers.",
    @Chain as IoChain
}
reexport! { rust: not(std)|std::io,
    doc: "Wraps an in-memory buffer and provides it with an [`IoSeek`] implementation.",
    @Cursor as IoCursor
}
reexport! { rust: std::io,
    doc: "Ignores any data written via [`IoWrite`], and read via [`IoRead`].",
    @Empty as IoEmpty
}
reexport! { rust: not(std)|std::io,
    tag: crate::TAG_ERROR_COMPOSITE!(),
    doc: "Error type for [`IoRead`], [`IoWrite`], [`IoSeek`] operations.",
    @Error as IoError
}
reexport! { rust: std::io,
    tag: crate::TAG_ERROR!(),
    doc: "An error returned by [`IoBufWriter::into_inner`]",
    @IntoInnerError as IoIntoInnerError
}
reexport! { rust: std::io,
    doc: "A buffer type used with `IoWrite::write_vectored`.",
    IoSlice
}
reexport! { rust: std::io,
    doc: "A buffer type used with `IoRead::read_vectored`.",
    IoSliceMut
}
reexport! { rust: not(std)|std::io,
    doc: "Like `BufWriter`, but flushing whenever a newline (`0x0a`, `'\n'`) is detected.",
    @LineWriter as IoLineWriter
}
reexport! { rust: std::io,
    doc: "An iterator over the lines of an instance of [`IoBufRead`].",
    @Lines as IoLines
}
reexport! { rust: std::io,
    doc: "A reader which yields one byte over and over and over and over and over and…",
    @Repeat as IoRepeat
}
reexport! { rust: not(std)|std::io,
    tag: crate::TAG_RESULT!(),
    doc: "A specialized [`Result`] type for I/O operations.",
    @Result as IoResult
}
reexport! { rust: std::io,
    doc: "A writer which will move data into the void.",
    @Sink as IoSink
}
reexport! { rust: std::io,
    doc: "An iterator over the contents of an instance of BufRead split on a particular byte.",
    @Split as IoSplit
}
reexport! { rust: std::io,
    doc: "A handle to the standard error stream of a process.",
    Stderr
}
reexport! { rust: std::io,
    doc: "A locked reference to the [`Stderr`] handle.",
    StderrLock
}
reexport! { rust: std::io,
    doc: "A handle to the standard input stream of a process.",
    Stdin
}
reexport! { rust: std::io,
    doc: "A locked reference to the [`Stdin`] handle.",
    StdinLock
}
reexport! { rust: std::io,
    doc: "A handle to the global standard output stream of the current process.",
    Stdout
}
reexport! { rust: std::io,
    doc: "A locked reference to the [`Stdout`] handle.",
    StdoutLock
}
reexport! { rust: not(std)|std::io,
    doc: "Reader adapter which limits the bytes read from an underlying reader.",
    @Take as IoTake
}
// @WriterPanicked as IoWriterPanicked

/* io enums */

reexport! { rust: not(std)|std::io,
    tag: crate::TAG_ERROR_COMPOSITE!(),
    doc: "A list specifying general categories of I/O error.",
    @ErrorKind as IoErrorKind
}
// @SeekFrom as IoSeekFrom
