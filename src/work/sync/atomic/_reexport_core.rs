// devela::work::sync::atomic::_reexport
//
//! Re-exports items from core, [portable-atomic](https://docs.rs/portable-atomic),
//! and the [`Atomic`] type from the [atomic](https://docs.rs/atomic) crate.
//

use crate::{_reexport, _tags};

// enums
_reexport! { rust: core::sync::atomic, location: "work/sync/atomic",
    tag: _tags!(concurrency atomic),
    doc: "Atomic memory ordering.",
    @Ordering as AtomicOrdering
}

// functions
_reexport! { rust: core::sync::atomic, location: "work/sync/atomic",
    tag: _tags!(concurrency atomic),
    doc: "An atomic fence.",
    @fence as atomic_fence
}
_reexport! { rust: core::sync::atomic, location: "work/sync/atomic",
    tag: _tags!(concurrency atomic),
    doc: "A compiler memory fence.",
    @compiler_fence as atomic_compiler_fence
}
