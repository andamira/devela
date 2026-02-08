// devela_base_core::sys::mem::bound::pin::_reexport

use crate::{_TAG_CODE, _TAG_GUARD, _TAG_LIFETIME, _TAG_MEM, _reexport};

_reexport! { rust: core::pin, location: "sys/mem", tag: _TAG_CODE!() _TAG_MEM!() _TAG_LIFETIME!(),
    doc: "Constructs a <code>[Pin]<[&mut] T></code>, by pinning a `value: T` locally.",
    pin
}

_reexport! { rust: core::pin, location: "sys/mem", tag: _TAG_LIFETIME!() _TAG_MEM!() _TAG_GUARD!(),
    doc: "A pointer which pins its pointee in place.",
    Pin
}

// WAIT: [derive_coerce_pointee](https://github.com/rust-lang/rust/issues/123430)
// _reexport! { rust: core::pin,
//     doc: "A pointer or a wrapper for one, where unsizing can be performed on the pointee when it is pinned.",
//     PinCoerceUnsized
// }
