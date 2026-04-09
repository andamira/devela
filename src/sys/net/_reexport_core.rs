// devela::sys::net::_reexport_core

#[allow(unused_imports)]
use crate::{_reexport, _tags};

/* core structs */

_reexport! { rust: core::net, location: "sys/net", tag: _tags!(network error),
    doc: "An error which can be returned when parsing an IP or socket address.",
    AddrParseError
}
_reexport! { rust: core::net, location: "sys/net", tag: _tags!(network),
    doc: "An IPv4 address.",
    Ipv4Addr
}
_reexport! { rust: core::net, location: "sys/net", tag: _tags!(network),
    doc: "An IPv6 address.",
    Ipv6Addr
}
_reexport! { rust: core::net, location: "sys/net", tag: _tags!(network),
    doc: "An IPv4 socket address.",
    SocketAddrV4
}
_reexport! { rust: core::net, location: "sys/net", tag: _tags!(network),
    doc: "An IPv6 socket address.",
    SocketAddrV6
}

/* core enums */

_reexport! { rust: core::net, location: "sys/net", tag: _tags!(network),
    doc: "An IP address, either IPv4 or IPv6.",
    IpAddr
}
_reexport! { rust: core::net, location: "sys/net", tag: _tags!(network uid),
    doc: "An internet socket address, either IPv4 or IPv6.",
    SocketAddr
}
