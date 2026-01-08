// devela_base_core::sys::env::_reexport

use crate::{_reexport, _tags};

/* core macros */

_reexport! { rust: core, location: "sys/env", tag: _tags!(code),
    doc: "Optionally inspects an environment variable at compile time.",
    option_env
}

#[doc = _tags!(code)]
// NOTE: the macro and the module have the same name
//
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
/// Inspects an environment variable at compile time.
#[doc = crate::_doc_location!("sys/env")]
///
#[doc = "*Re-exported from [`core::env`][macro@::core::env]*."]
#[doc = "\n\n---"]
///
/// The reason of the `_` suffix is to avoid conflicting with Rust's prelude
/// when glob importing from this crate. Since this macro has the same name
/// as its sibling module `core::env`, in order to be able to re-export
/// only the macro we have to wrap it with our own.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! env_ { ($($tt:tt)*) => { core::env![$($tt)*] } }
#[doc(inline)]
pub use env_;
