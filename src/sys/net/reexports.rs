// devela::sys::net::reexports
//
//! Reexported items from `core`.
//

#[allow(unused_imports)]
use crate::{_reexport, TAG_ERROR, TAG_ITERATOR};

/* core structs */

_reexport! { rust: core::net,
    tag: TAG_ERROR!(),
    doc: "An error which can be returned when parsing an IP address or a socket address.",
    AddrParseError
}
_reexport! { rust: core::net,
    doc: "An IPv4 address.",
    Ipv4Addr
}
_reexport! { rust: core::net,
    doc: "An IPv6 address.",
    Ipv6Addr
}
_reexport! { rust: core::net,
    doc: "An IPv4 socket address.",
    SocketAddrV4
}
_reexport! { rust: core::net,
    doc: "An IPv6 socket address.",
    SocketAddrV6
}

/* core enums */

_reexport! { rust: core::net,
    doc: "An IP address, either IPv4 or IPv6.",
    IpAddr
}
_reexport! { rust: core::net,
    doc: "An internet socket address, either IPv4 or IPv6.",
    SocketAddr
}

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
