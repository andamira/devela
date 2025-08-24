// devela::sys::net::reexports
//
//!
//

#[allow(unused_imports)]
use crate::{_reexport, TAG_ERROR, TAG_ITERATOR};

/* std structs */

_reexport! { rust: std::net,
    tag: TAG_ITERATOR!(),
    doc: "An iterator that infinitely accepts connections on a [`TcpListener`].",
    @Incoming as TcpIncoming
}
_reexport! { rust: std::net,
    doc: "A TCP socket server, listening for connections.",
    TcpListener
}
_reexport! { rust: std::net,
    doc: "A TCP stream between a local and a remote socket.",
    TcpStream
}
_reexport! { rust: std::net,
    doc: "A UDP socket.",
    UdpSocket
}

/* std enums */

_reexport! { rust: std::net,
    doc: "Possible values which can be passed to the [`TcpStream::shutdown`] method.",
    @Shutdown as TcpShutdown
}

/* std traits */

_reexport! { rust: std::net,
    doc: "Objects which can be converted or resolved to one or more [`SocketAddr`] values.",
    ToSocketAddrs
}
