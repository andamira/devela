// devela_base_core::work::sync::atomic::reexports
//
//! Re-exports items from core, [portable-atomic](https://docs.rs/portable-atomic),
//! and the [`Atomic`] type from the [atomic](https://docs.rs/atomic) crate.
//

use crate::{_reexport, TAG_ATOMIC};

/* from `core` */

// enums
_reexport! { rust: core::sync::atomic,
    tag: TAG_ATOMIC!(),
    doc: "Atomic memory ordering.",
    @Ordering as AtomicOrdering
}

// functions
_reexport! { rust: core::sync::atomic,
    tag: TAG_ATOMIC!(),
    doc: "An atomic fence.",
    @fence as atomic_fence
}
_reexport! { rust: core::sync::atomic,
    tag: TAG_ATOMIC!(),
    doc: "A compiler memory fence.",
    @compiler_fence as atomic_compiler_fence
}
