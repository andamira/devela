// devela_base_core::code::panic::_reexport

use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: core::panic, location: "code/panic", tag: _tags!(code),
    doc: "Passed to `#[panic_handler]` in `no_std`, carries a formatted message.",
    PanicInfo
}
_reexport! { rust: core::panic, location: "code/panic", tag: _tags!(code),
    doc: "A struct containing information about the location of a panic.",
    @Location as PanicLocation
}
_reexport! { rust: core::panic, location: "code/panic", tag: _tags!(code assert),
    doc: "A simple wrapper around a type to assert that it is unwind safe.",
    @AssertUnwindSafe as PanicAssertUnwindSafe
}

/* traits */

_reexport! { rust: core::panic, location: "code/panic", tag: _tags!(code lifetime),
    doc: "A marker trait which represents a shared reference considered unwind safe.",
    @RefUnwindSafe as PanicRefUnwindSafe
    // RefUnwindSafe
}
_reexport! { rust: core::panic, location: "code/panic", tag: _tags!(code),
    doc: "A marker trait which represents \"panic safe\" types in Rust.",
    @UnwindSafe as PanicUnwindSafe
    // UnwindSafe
}

/* macros */

_reexport! { rust: core, location: "code/panic", tag: _tags!(code),
doc: "Indicates unfinished code.", todo }
_reexport! { rust: core, location: "code/panic", tag: _tags!(code),
doc: "Indicates unreachable code.", unreachable }
_reexport! { rust: core, location: "code/panic", tag: _tags!(code),
doc: "Indicates unimplemented code.", unimplemented }

// NOTE: the macro and the module have the same name
//
#[doc = crate::_tags!(code)]
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
/// Panics the current thread.
#[doc = crate::_doc_location!("code/panic")]
///
#[doc = "*Re-exported from [`core::panic`][macro@::core::panic]*."]
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
