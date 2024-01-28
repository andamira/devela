// devela::error::panic
//
//! Panic support, extends
//! `std::`[`panic`][mod@std::panic].
//

// re-export private sub-modules
#[allow(unused)]
pub use reexports::*;

pub(crate) mod all {
    pub use super::reexports::*;
}

mod reexports {
    use crate::code::reexport;

    /* structs */

    reexport! { rust: core::panic, local_module: "error",
        doc: "A struct providing information about a panic.",
        PanicInfo
    }
    reexport! { rust: core::panic, local_module: "error",
        doc: "A struct containing information about the location of a panic.",
        @Location as PanicLocation
    }
    reexport! { rust: core::panic, local_module: "error",
        doc: "A simple wrapper around a type to assert that it is unwind safe.",
        @AssertUnwindSafe as PanicAssertUnwindSafe
        // AssertUnwindSafe
    }

    /* traits */

    reexport! { rust: core::panic, local_module: "error",
        doc: "A marker trait representing types where a shared reference is considered unwind safe.",
        @RefUnwindSafe as PanicRefUnwindSafe
        // RefUnwindSafe
    }
    reexport! { rust: core::panic, local_module: "error",
        doc: "A marker trait which represents “panic safe” types in Rust.",
        @UnwindSafe as PanicUnwindSafe
        // UnwindSafe
    }

    /* functions */

    reexport! { rust: std::panic, local_module: "error",
        doc: "Invokes a closure, capturing the cause of an unwinding panic if one occurs.",
        @catch_unwind as panic_catch
    }
    reexport! { rust: std::panic, local_module: "error",
        doc: "Panic the current thread with the given message as the panic payload.",
        panic_any
    }
    reexport! { rust: std::panic, local_module: "error",
        doc: "Triggers a panic without invoking the panic hook.",
        @resume_unwind as panic_resume
    }
    reexport! { rust: std::panic, local_module: "error",
        doc: "Registers a custom panic hook, replacing the previously registered hook.",
        @set_hook as panic_set_hook
    }
    reexport! { rust: std::panic, local_module: "error",
        doc: "Unregisters the current panic hook and returns it,
        registering the default hook in its place",
        @take_hook as panic_unset_hook
    }

    /* macros */

    reexport! { rust: core, local_module: "error", doc: "Indicates unfinished code.", todo }
    reexport! { rust: core, local_module: "error", doc: "Indicates unreachable code.", unreachable }
    reexport! { rust: core, local_module: "error",
    doc: "Indicates unimplemented code.", unimplemented }

    // NOTE: the macro and the module have the same name :(
    //
    /// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
    /// Panics the current thread.
    ///
    #[doc = "*Re-exported from [`core::panic`][macro@panic]*."]
    #[doc = "\n\n---"]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "error")))]
    #[macro_export]
    macro_rules! panic { ($($tt:tt)*) => { core::panic![$($tt)*] } }
    pub use panic;
}
