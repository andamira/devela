// devela_base_core::work::sync::atomic::_reexport
//
//! Re-exports items from core, [portable-atomic](https://docs.rs/portable-atomic),
//! and the [`Atomic`] type from the [atomic](https://docs.rs/atomic) crate.
//

use crate::{_TAG_ATOMIC, _TAG_CONCURRENCY, _reexport};

// enums
_reexport! { rust: core::sync::atomic, location: "work/sync/atomic",
    tag: _TAG_CONCURRENCY!() _TAG_ATOMIC!(),
    doc: "Atomic memory ordering.",
    @Ordering as AtomicOrdering
}

// functions
_reexport! { rust: core::sync::atomic, location: "work/sync/atomic",
    tag: _TAG_CONCURRENCY!() _TAG_ATOMIC!(),
    doc: "An atomic fence.",
    @fence as atomic_fence
}
_reexport! { rust: core::sync::atomic, location: "work/sync/atomic",
    tag: _TAG_CONCURRENCY!() _TAG_ATOMIC!(),
    doc: "A compiler memory fence.",
    @compiler_fence as atomic_compiler_fence
}
