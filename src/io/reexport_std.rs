// devela::io::reexport_std
//
//! Reexported items from `std`.
//

use crate::code::reexport;

/* io traits */

reexport! { rust: not(std)|std::io, local_module: "io",
    doc: "A type of `Read`er which has an internal buffer.",
    BufRead
}
reexport! { rust: not(std)|std::io, local_module: "io",
    doc: "Allows for reading bytes from a source.",
    Read
}
reexport! { rust: not(std)|std::io, local_module: "io",
    doc: "Provides a cursor which can be moved within a stream of bytes.",
    Seek
}
reexport! { rust: not(std)|std::io, local_module: "io",
    doc: "A trait for objects which are byte-oriented sinks.",
    Write
}

/* io structs and enums */

reexport! { rust: not(std)|std::io, local_module: "io",
    doc: "Wraps an in-memory buffer and provides it with a [`Seek`] implementation.",
    Cursor
}
reexport! { rust: not(std)|std::io, local_module: "io",
    doc: "The error type for I/O operations of [`Read`], [`Write`], [`Seek`],
    and associated traits.",
    @Error as IoError
}
reexport! { rust: not(std)|std::io, local_module: "io",
    doc: "A list specifying general categories of I/O error.",
    @ErrorKind as IoErrorKind
}
reexport! { rust: not(std)|std::io, local_module: "io",
    doc: "A specialized [`Result`] type for I/O operations.",
    @Result as IoResult
}

/* io functions */

reexport! { rust: not(std)|std::io, local_module: "io",
    doc: "Copies the entire contents of a reader into a writer.",
    @copy as io_copy
}

reexport! { rust: std::io, local_module: "io",
    doc: "Creates a value that is always at EOF for reads, and ignores all data written.",
    @empty as io_empty
}
reexport! { rust: std::io, local_module: "io",
    doc: "Read all bytes from a reader into a new String",
    @read_to_string as io_read_to_string
}
reexport! { rust: std::io, local_module: "io",
    doc: "Creates an instance of a reader that infinitely repeats one byte.",
    @repeat as io_repeat
}
reexport! { rust: std::io, local_module: "io",
    doc: "Creates an instance of a writer which will successfully consume all data.",
    @sink as io_sink
}
reexport! { rust: std::io, local_module: "io",
    doc: "Constructs a new handle to the standard error of the current process.",
    @stderr as io_stderr
}
reexport! { rust: std::io, local_module: "io",
    doc: "Constructs a new handle to the standard input of the current process.",
    @stdin as io_stdin
}
reexport! { rust: std::io, local_module: "io",
    doc: "Constructs a new handle to the standard output of the current process.",
    @stdout as io_stdout
}
