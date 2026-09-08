// devela/src/sys/net/transport/_reexport_std.rs

#[cfg(doc)]
use crate::SocketAddr;
#[allow(unused_imports)]
use crate::{_reexport, _tags};

/* traits */

_reexport! { rust: std::net,
    location: "sys/net" => enum ToSocketAddrs, tag: _tags!(network),
    doc: "Objects which can be converted or resolved to one or more [`SocketAddr`] values.",
    ToSocketAddrs
}
