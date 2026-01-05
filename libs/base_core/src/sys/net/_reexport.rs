// devela_base_core::sys::net::_reexport

#[allow(unused_imports)]
use crate::{_TAG_ERROR, _TAG_ITERATOR, _TAG_NETWORK, _TAG_UID, _reexport};

/* core structs */

_reexport! { rust: core::net, location: "sys/net", tag: _TAG_NETWORK!() _TAG_ERROR!(),
    doc: "An error which can be returned when parsing an IP address or a socket address.",
    AddrParseError
}
_reexport! { rust: core::net, location: "sys/net", tag: _TAG_NETWORK!(),
    doc: "An IPv4 address.",
    Ipv4Addr
}
_reexport! { rust: core::net, location: "sys/net", tag: _TAG_NETWORK!(),
    doc: "An IPv6 address.",
    Ipv6Addr
}
_reexport! { rust: core::net, location: "sys/net", tag: _TAG_NETWORK!(),
    doc: "An IPv4 socket address.",
    SocketAddrV4
}
_reexport! { rust: core::net, location: "sys/net", tag: _TAG_NETWORK!(),
    doc: "An IPv6 socket address.",
    SocketAddrV6
}

/* core enums */

_reexport! { rust: core::net, location: "sys/net", tag: _TAG_NETWORK!(),
    doc: "An IP address, either IPv4 or IPv6.",
    IpAddr
}
_reexport! { rust: core::net, location: "sys/net", tag: _TAG_NETWORK!() _TAG_UID!(),
    doc: "An internet socket address, either IPv4 or IPv6.",
    SocketAddr
}
