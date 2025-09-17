// devela_base_core::sys::net::reexports
//
//!
//

#[allow(unused_imports)]
use crate::{_TAG_ERROR, _TAG_ITERATOR, _reexport};

/* core structs */

_reexport! { rust: core::net,
    tag: _TAG_ERROR!(),
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
