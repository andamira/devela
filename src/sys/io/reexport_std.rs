// devela::sys::io::reexport_std
//
//! Reexported items from `std`.
//

use crate::code::reexport;

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

/* io structs and enums */

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
    doc: "Wraps an in-memory buffer and provides it with a [`IoSeek`] implementation.",
    @Cursor as IoCursor
}
reexport! { rust: not(std)|std::io,
    doc: "The error type for I/O operations of [`IoRead`], [`IoWrite`], [`IoSeek`],
    and associated traits.",
    @Error as IoError
}
reexport! { rust: not(std)|std::io,
    doc: "A list specifying general categories of I/O error.",
    @ErrorKind as IoErrorKind
}
reexport! { rust: not(std)|std::io,
    doc: "Like `BufWriter`, but flushing whenever a newline (`0x0a`, `'\n'`) is detected.",
    @LineWriter as IoLineWriter
}
reexport! { rust: not(std)|std::io,
    doc: "A specialized [`Result`] type for I/O operations.",
    @Result as IoResult
}
reexport! { rust: not(std)|std::io,
    doc: "Reader adapter which limits the bytes read from an underlying reader.",
    @Take as IoTake
}

/* io functions */

reexport! { rust: not(std)|std::io,
    doc: "Copies the entire contents of a reader into a writer.",
    @copy as io_copy
}
reexport! { rust: std::io,
    doc: "Creates a value that is always at EOF for reads, and ignores all data written.",
    @empty as io_empty
}
reexport! { rust: std::io,
    doc: "Read all bytes from a reader into a new String",
    @read_to_string as io_read_to_string
}
reexport! { rust: std::io,
    doc: "Creates an instance of a reader that infinitely repeats one byte.",
    @repeat as io_repeat
}
reexport! { rust: std::io,
    doc: "Creates an instance of a writer which will successfully consume all data.",
    @sink as io_sink
}
reexport! { rust: std::io,
    doc: "Constructs a new handle to the standard error of the current process.",
    @stderr as io_stderr
}
reexport! { rust: std::io,
    doc: "Constructs a new handle to the standard input of the current process.",
    @stdin as io_stdin
}
reexport! { rust: std::io,
    doc: "Constructs a new handle to the standard output of the current process.",
    @stdout as io_stdout
}
