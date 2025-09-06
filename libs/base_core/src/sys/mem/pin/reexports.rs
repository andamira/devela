// devela_base_core::sys::mem::pin::reexports
//
//!
//

use crate::_reexport;

_reexport! { rust: core::pin,
    doc: "Constructs a <code>[Pin]<[&mut] T></code>, by pinning a `value: T` locally.",
    pin
}
_reexport! { rust: core::pin,
    doc: "A pointer which pins its pointee in place.",
    Pin
}
// WAIT: [derive_coerce_pointee](https://github.com/rust-lang/rust/issues/123430)
// _reexport! { rust: core::pin,
//     doc: "A pointer or a wrapper for one, where unsizing can be performed on the pointee when it is pinned.",
//     PinCoerceUnsized
// }
