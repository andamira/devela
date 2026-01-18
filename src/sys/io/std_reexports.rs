// devela::sys::io::std_reexports
//
//! Reexported items from `std`.
//

use crate::{_TAG_ERROR, _TAG_ERROR_COMPOSITE, _TAG_IO, _TAG_RESULT, _reexport};

/* io traits */

_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A type of `Read`er which has an internal buffer.",
    @BufRead as IoBufRead
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Allows for reading bytes from a source.",
    @Read as IoRead
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Provides a cursor which can be moved within a stream of bytes.",
    @Seek as IoSeek
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A trait for objects which are byte-oriented sinks.",
    @Write as IoWrite
}

/* io structs */

_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Adds buffering to any reader.",
    @BufReader as IoBufReader
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Wraps a writer and buffers its output.",
    @BufWriter as IoBufWriter
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "An iterator over `u8` values of a reader.",
    @Bytes as IoBytes
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Adapter to chain together two readers.",
    @Chain as IoChain
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Wraps an in-memory buffer and provides it with an [`IoSeek`] implementation.",
    @Cursor as IoCursor
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Ignores any data written via [`IoWrite`], and read via [`IoRead`].",
    @Empty as IoEmpty
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!() _TAG_ERROR_COMPOSITE!(),
    doc: "Error type for [`IoRead`], [`IoWrite`], [`IoSeek`] operations.",
    @Error as IoError
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!() _TAG_ERROR!(),
    doc: "An error returned by [`IoBufWriter::into_inner`]",
    @IntoInnerError as IoIntoInnerError
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Read end of an anonymous pipe.",
    @PipeReader as IoPipeReader
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Write end of an anonymous pipe.",
    @PipeWriter as IoPipeWriter
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A buffer type used with `IoWrite::write_vectored`.",
    IoSlice
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A buffer type used with `IoRead::read_vectored`.",
    IoSliceMut
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Like `BufWriter`, but flushing whenever a newline (`0x0a`, `'\n'`) is detected.",
    @LineWriter as IoLineWriter
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "An iterator over the lines of an instance of [`IoBufRead`].",
    @Lines as IoLines
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A reader which yields one byte over and over and over and over and over and…",
    @Repeat as IoRepeat
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!() _TAG_RESULT!(),
    doc: "A specialized [`Result`] type for I/O operations.",
    @Result as IoResult
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A writer which will move data into the void.",
    @Sink as IoSink
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "An iterator over the contents of an instance of BufRead split on a particular byte.",
    @Split as IoSplit
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A handle to the standard error stream of a process.",
    Stderr
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A locked reference to the [`Stderr`] handle.",
    StderrLock
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A handle to the standard input stream of a process.",
    Stdin
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A locked reference to the [`Stdin`] handle.",
    StdinLock
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A handle to the global standard output stream of the current process.",
    Stdout
}
_reexport! { rust: std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "A locked reference to the [`Stdout`] handle.",
    StdoutLock
}
_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!(),
    doc: "Reader adapter which limits the bytes read from an underlying reader.",
    @Take as IoTake
}
// @WriterPanicked as IoWriterPanicked

/* io enums */

_reexport! { rust: not(std)|std::io, location: "sys/io", tag: _TAG_IO!() _TAG_ERROR_COMPOSITE!(),
    doc: "A list specifying general categories of I/O error.",
    @ErrorKind as IoErrorKind
}
// @SeekFrom as IoSeekFrom
