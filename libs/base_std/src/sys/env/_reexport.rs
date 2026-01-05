// devela_base_std::sys::env::reexports
//
//!
//

use crate::{_TAG_ERROR, _TAG_ITERATOR, _TAG_PLATFORM, _reexport};

_reexport! { rust: std::env, location: "sys/env", tag: _TAG_ITERATOR!() _TAG_PLATFORM!(),
    doc: "Iterates over the arguments of a process, yielding them as [`String`][crate::String]s.",
    @Args as IterArgs,
}
_reexport! { rust: std::env, location: "sys/env", tag: _TAG_ITERATOR!() _TAG_PLATFORM!(),
    doc: "Iterates over the arguments of a process, yielding them as
[`OsString`][crate::OsString]s.",
    @ArgsOs as IterArgsOs,
}
_reexport! { rust: std::env, location: "sys/env", tag: _TAG_ITERATOR!() _TAG_PLATFORM!(),
    doc: "An iterator over a snapshot of the environment variables of this process.",
    @Vars as IterVars,
}
_reexport! { rust: std::env, location: "sys/env", tag: _TAG_ITERATOR!() _TAG_PLATFORM!(),
    doc: "An iterator over a snapshot of the environment variables of this process.",
    @VarsOs as IterVarsOs,
}

_reexport! { rust: std::env, location: "sys/env", tag: _TAG_ITERATOR!() _TAG_PLATFORM!(),
    doc: "An iterator that splits an environment variable into paths.",
    @SplitPaths as IterSplitPaths,
}

_reexport! { rust: std::env, location: "sys/env", tag: _TAG_ERROR!() _TAG_PLATFORM!(),
    doc: "Returned from [Env::join_paths][crate::Env#method.join_paths].",
    JoinPathsError,
}
_reexport! { rust: std::env, location: "sys/env", tag: _TAG_ERROR!() _TAG_PLATFORM!(),
    doc: "Returned from [`Env::var`][crate::Env#method.var].",
    VarError
}
