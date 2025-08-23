// devela::sys::env::reexports
//
//! Reexported items from `core`.
//

use crate::_reexport;

/* std items */

_reexport! { rust: std::env,
    tag: crate::TAG_ITERATOR!(),
    doc: "Iterates over the arguments of a process, yielding them as [`String`][crate::String]s.",
    @Args as IterArgs,
}
_reexport! { rust: std::env,
    tag: crate::TAG_ITERATOR!(),
    doc: "Iterates over the arguments of a process, yielding them as
[`OsString`][crate::OsString]s.",
    @ArgsOs as IterArgsOs,
}
_reexport! { rust: std::env,
    tag: crate::TAG_ITERATOR!(),
    doc: "An iterator over a snapshot of the environment variables of this process.",
    @Vars as IterVars,
}
_reexport! { rust: std::env,
    tag: crate::TAG_ITERATOR!(),
    doc: "An iterator over a snapshot of the environment variables of this process.",
    @VarsOs as IterVarsOs,
}
_reexport! { rust: std::env,
    tag: crate::TAG_ERROR!(),
    doc: "Returned from [`Env::var`][crate::Env#method.var].",
    VarError
}
_reexport! { rust: std::env,
    tag: crate::TAG_ITERATOR!(),
    doc: "An iterator that splits an environment variable into paths.",
    @SplitPaths as IterSplitPaths,
}
_reexport! { rust: std::env,
    tag: crate::TAG_ERROR!(),
    doc: "Returned from [Env::join_paths][crate::Env#method.join_paths].",
    JoinPathsError,
}

/* core macros */

_reexport! { rust: core,
    doc: "Optionally inspects an environment variable at compile time.",
    option_env
}

// NOTE: the macro and the module have the same name
//
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
/// Inspects an environment variable at compile time.
///
#[doc = "*Re-exported from [`core::env`][macro@crate::_core::env]*."]
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
