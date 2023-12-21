// devela::result::panic
//
//! Panic support, extends
//! `std::`[`panic`][mod@std::panic].
//

// re-export private sub-modules
pub use reexports::*;

pub(crate) mod all {
    pub use super::reexports::*;
}

mod reexports {
    use crate::code::reexport;

    /* structs */

    reexport! { rust: core::panic, local_module: "result",
        doc: "A struct providing information about a panic.",
        PanicInfo
    }
    reexport! { rust: core::panic, local_module: "result",
        doc: "A struct containing information about the location of a panic.",
        @Location as PanicLocation
    }
    reexport! { rust: core::panic, local_module: "result",
        doc: "A simple wrapper around a type to assert that it is unwind safe.",
        @AssertUnwindSafe as PanicAssertUnwindSafe
        // AssertUnwindSafe
    }

    /* traits */

    reexport! { rust: core::panic, local_module: "result",
        doc: "A marker trait representing types where a shared reference is considered unwind safe.",
        @RefUnwindSafe as PanicRefUnwindSafe
        // RefUnwindSafe
    }
    reexport! { rust: core::panic, local_module: "result",
        doc: "A marker trait which represents “panic safe” types in Rust.",
        @UnwindSafe as PanicUnwindSafe
        // UnwindSafe
    }

    /* functions */

    reexport! { rust: "std"|std::panic, local_module: "result",
        doc: "Invokes a closure, capturing the cause of an unwinding panic if one occurs.",
        @catch_unwind as panic_catch
    }
    reexport! { rust: "std"|std::panic, local_module: "result",
        doc: "Panic the current thread with the given message as the panic payload.",
        panic_any
    }
    reexport! { rust: "std"|std::panic, local_module: "result",
        doc: "Triggers a panic without invoking the panic hook.",
        @resume_unwind as panic_resume
    }
    reexport! { rust: "std"|std::panic, local_module: "result",
        doc: "Registers a custom panic hook, replacing the previously registered hook.",
        @set_hook as panic_set_hook
    }
    reexport! { rust: "std"|std::panic, local_module: "result",
        doc: "Unregisters the current panic hook and returns it,
        registering the default hook in its place",
        @take_hook as panic_unset_hook
    }
}
