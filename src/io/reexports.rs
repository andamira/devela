// devela::io
//
//! Reexported items from `std`.
//

use crate::code::reexport;

/* io traits */

reexport! { rust: "std"|std::io, local_module: "io",
    doc: "A type of `Read`er which has an internal buffer.",
    BufRead
}
reexport! { rust: "std"|std::io, local_module: "io",
    doc: "Allows for reading bytes from a source.",
    Read
}
reexport! { rust: "std"|std::io, local_module: "io",
    doc: "Provides a cursor which can be moved within a stream of bytes.",
    Seek
}
reexport! { rust: "std"|std::io, local_module: "io",
    doc: "A trait for objects which are byte-oriented sinks.",
    Write
}

/* io structs */

reexport! { rust: "std"|std::io, local_module: "io",
    doc: "Wraps an in-memory buffer and provides it with a [`Seek`] implementation.",
    Cursor
}

reexport! { rust: "std"|std::io, local_module: "io",
    doc: "Wraps an in-memory buffer and provides it with a [`Seek`] implementation.",
    @Error as IoError
}
reexport! { rust: "std"|std::io, local_module: "io",
    doc: "A specialized [`Result`] type for I/O operations.",
    @Result as IoResult
}
