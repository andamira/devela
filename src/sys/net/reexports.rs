// devela::sys::net::reexports
//
//! Reexported items from `core`.
//

#[allow(unused_imports)]
use crate::{TAG_ERROR, TAG_ITERATOR, reexport};

/* core structs */

reexport! { rust: core::net,
    tag: TAG_ERROR!(),
    doc: "An error which can be returned when parsing an IP address or a socket address.",
    AddrParseError
}
reexport! { rust: core::net,
    doc: "An IPv4 address.",
    Ipv4Addr
}
reexport! { rust: core::net,
    doc: "An IPv6 address.",
    Ipv6Addr
}
reexport! { rust: core::net,
    doc: "An IPv4 socket address.",
    SocketAddrV4
}
reexport! { rust: core::net,
    doc: "An IPv6 socket address.",
    SocketAddrV6
}

/* core enums */

reexport! { rust: core::net,
    doc: "An IP address, either IPv4 or IPv6.",
    IpAddr
}
reexport! { rust: core::net,
    doc: "An internet socket address, either IPv4 or IPv6.",
    SocketAddr
}

/* std structs */

reexport! { rust: std::net,
    tag: TAG_ITERATOR!(),
    doc: "An iterator that infinitely accepts connections on a [`TcpListener`].",
    @Incoming as TcpIncoming
}
reexport! { rust: std::net,
    doc: "A TCP socket server, listening for connections.",
    TcpListener
}
reexport! { rust: std::net,
    doc: "A TCP stream between a local and a remote socket.",
    TcpStream
}
reexport! { rust: std::net,
    doc: "A UDP socket.",
    UdpSocket
}

/* std enums */

reexport! { rust: std::net,
    doc: "Possible values which can be passed to the [`TcpStream::shutdown`] method.",
    @Shutdown as TcpShutdown
}

/* std traits */

reexport! { rust: std::net,
    doc: "Objects which can be converted or resolved to one or more [`SocketAddr`] values.",
    ToSocketAddrs
}
