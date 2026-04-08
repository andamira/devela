// devela::sys::net::_reexport_std

#[allow(unused_imports)]
use crate::{_reexport, _tags};

/* std structs */

_reexport! { rust: std::net, location: "sys/net", tag: _tags!(network iterator),
    doc: "An iterator that infinitely accepts connections on a [`TcpListener`].",
    @Incoming as TcpIncoming
}
_reexport! { rust: std::net, location: "sys/net", tag: _tags!(network),
    doc: "A TCP socket server, listening for connections.",
    TcpListener
}
_reexport! { rust: std::net, location: "sys/net", tag: _tags!(network),
    doc: "A TCP stream between a local and a remote socket.",
    TcpStream
}
_reexport! { rust: std::net, location: "sys/net", tag: _tags!(network),
    doc: "A UDP socket.",
    UdpSocket
}

/* std enums */

_reexport! { rust: std::net, location: "sys/net", tag: _tags!(network),
    doc: "Possible values which can be passed to the [`TcpStream::shutdown`] method.",
    @Shutdown as TcpShutdown
}

/* std traits */

_reexport! { rust: std::net, location: "sys/net", tag: _tags!(network),
    doc: "Objects which can be converted or resolved to one or more [`SocketAddr`] values.",
    ToSocketAddrs
}
