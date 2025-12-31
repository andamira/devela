// devela::sys::net::reexports
//
//!
//

#[allow(unused_imports)]
use crate::{_TAG_ERROR, _TAG_ITERATOR, _TAG_NETWORK, _reexport};

/* std structs */

_reexport! { rust: std::net,
    tag: _TAG_NETWORK!() _TAG_ITERATOR!(),
    doc: "An iterator that infinitely accepts connections on a [`TcpListener`].",
    @Incoming as TcpIncoming
}
_reexport! { rust: std::net,
    tag: _TAG_NETWORK!(),
    doc: "A TCP socket server, listening for connections.",
    TcpListener
}
_reexport! { rust: std::net,
    tag: _TAG_NETWORK!(),
    doc: "A TCP stream between a local and a remote socket.",
    TcpStream
}
_reexport! { rust: std::net,
    tag: _TAG_NETWORK!(),
    doc: "A UDP socket.",
    UdpSocket
}

/* std enums */

_reexport! { rust: std::net,
    tag: _TAG_NETWORK!(),
    doc: "Possible values which can be passed to the [`TcpStream::shutdown`] method.",
    @Shutdown as TcpShutdown
}

/* std traits */

_reexport! { rust: std::net,
    tag: _TAG_NETWORK!(),
    doc: "Objects which can be converted or resolved to one or more [`SocketAddr`] values.",
    ToSocketAddrs
}
