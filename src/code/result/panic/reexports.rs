// devela::code::result::panic::reexports
//
//! Reexported items.
//

use crate::reexport;

/* structs */

reexport! { rust: core::panic,
    doc: "Passed to `#[panic_handler]` in `no_std`, where panics always carry a formatted message.",
    PanicInfo
}
reexport! { rust: std::panic,
    doc: "Passed to `std::panic::set_hook` in `std`, where panics can have arbitrary payloads.",
    PanicHookInfo
}
reexport! { rust: core::panic,
    doc: "A struct containing information about the location of a panic.",
    @Location as PanicLocation
}
reexport! { rust: core::panic,
    doc: "A simple wrapper around a type to assert that it is unwind safe.",
    @AssertUnwindSafe as PanicAssertUnwindSafe
}

/* traits */

reexport! { rust: core::panic,
    doc: "A marker trait which represents a shared reference considered unwind safe.",
    @RefUnwindSafe as PanicRefUnwindSafe
    // RefUnwindSafe
}
reexport! { rust: core::panic,
    doc: "A marker trait which represents “panic safe” types in Rust.",
    @UnwindSafe as PanicUnwindSafe
    // UnwindSafe
}

/* macros */

reexport! { rust: core, doc: "Indicates unfinished code.", todo }
reexport! { rust: core, doc: "Indicates unreachable code.", unreachable }
reexport! { rust: core, doc: "Indicates unimplemented code.", unimplemented }

// NOTE: the macro and the module have the same name
//
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
/// Panics the current thread.
///
#[doc = "*Re-exported from [`core::panic`][macro@crate::_core::panic]*."]
#[doc = "\n\n---"]
///
/// The reason of the `_` suffix is to avoid conflicting with the Rust's prelude
/// when glob importing from this crate. Since this macro has the same name
/// as its sibling module `core::panic`, in order to be able to re-export
/// only the macro we have to wrap it with our own.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! panic_ { ($($tt:tt)*) => { core::panic![$($tt)*] } }
#[doc(inline)]
pub use panic_;
