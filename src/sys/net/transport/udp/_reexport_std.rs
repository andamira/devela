#[allow(unused_imports)]
use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: std::net,
    location: "sys/net" => struct UdpSocket, tag: _tags!(network),
    doc: "A UDP socket.",
    UdpSocket
}
