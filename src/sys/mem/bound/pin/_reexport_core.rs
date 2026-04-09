// devela::sys::mem::bound::pin::_reexport_core

use crate::{_reexport, _tags};

_reexport! { rust: core::pin, location: "sys/mem", tag: _tags!(code mem lifetime),
    doc: "Constructs a <code>[Pin]<[&mut] T></code>, by pinning a `value: T` locally.",
    pin
}

_reexport! { rust: core::pin, location: "sys/mem", tag: _tags!(lifetime mem guard),
    doc: "A pointer which pins its pointee in place.",
    Pin
}

// WAIT: [derive_coerce_pointee](https://github.com/rust-lang/rust/issues/123430)
// _reexport! { rust: core::pin,
//     doc: "A pointer or a wrapper for one, where unsizing can be performed on the pointee when it is pinned.",
//     PinCoerceUnsized
// }
