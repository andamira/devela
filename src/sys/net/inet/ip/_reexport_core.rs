// devela/src/sys/net/inet/ip/_reexport_core.rs

#[allow(unused_imports)]
use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: core::net,
    location: "sys/net/inet" => struct Ipv4Addr, tag: _tags!(network),
    doc: "An IPv4 address.",
    Ipv4Addr
}
_reexport! { rust: core::net,
    location: "sys/net/inet" => struct Ipv7Addr, tag: _tags!(network),
    doc: "An IPv6 address.",
    Ipv6Addr
}
_reexport! { rust: core::net,
    location: "sys/net/inet" => struct SocketAddrV4, tag: _tags!(network),
    doc: "An IPv4 socket address.",
    SocketAddrV4
}
_reexport! { rust: core::net,
    location: "sys/net/inet" => struct SocketAddrV6, tag: _tags!(network),
    doc: "An IPv6 socket address.",
    SocketAddrV6
}

/* enums */

_reexport! { rust: core::net,
    location: "sys/net/inet" => enum IpAddr, tag: _tags!(network),
    doc: "An IP address, either IPv4 or IPv6.",
    IpAddr
}
_reexport! { rust: core::net,
    location: "sys/net/inet" => enum SocketAddr, tag: _tags!(network uid),
    doc: "An internet socket address, either IPv4 or IPv6.",
    SocketAddr
}
