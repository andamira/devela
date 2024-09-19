// devela::sys::env::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

reexport! { rust: core,
    doc: "Optionally inspects an environment variable at compile time.",
    option_env
}

// NOTE: the macro and the module have the same name
//
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
/// Inspects an environment variable at compile time.
///
#[doc = "*Re-exported from [`core::env`][macro@crate::_dep::_core::env]*."]
#[doc = "\n\n---"]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! env { ($($tt:tt)*) => { core::env![$($tt)*] } }
#[doc(inline)]
pub use env;
