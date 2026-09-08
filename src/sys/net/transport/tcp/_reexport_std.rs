// devela/src/sys/net/transport/tcp/_reexport_std.rs

#[allow(unused_imports)]
use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: std::net,
    location: "sys/net" => struct TcpListener, tag: _tags!(network),
    doc: "A TCP socket server, listening for connections.",
    TcpListener
}
_reexport! { rust: std::net,
    location: "sys/net" => struct TcpStream, tag: _tags!(network),
    doc: "A TCP stream between a local and a remote socket.",
    TcpStream
}

/* enums */

_reexport! { rust: std::net,
    location: "sys/net" => enum TcpShutdown, tag: _tags!(network),
    doc: "Possible values which can be passed to the [`TcpStream::shutdown`] method.",
    @Shutdown as TcpShutdown
}
