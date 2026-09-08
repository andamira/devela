// devela/src/sys/net/_reexport_core.rs

#[allow(unused_imports)]
use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: core::net,
    location: "sys/net" => struct AddrParseError, tag: _tags!(network error),
    doc: "An error which can be returned when parsing an IP or socket address.",
    @AddrParseError as NetAddrParseError
}
