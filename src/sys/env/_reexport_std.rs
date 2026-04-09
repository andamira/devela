// devela::sys::env::reexports_std

use crate::{_reexport, _tags};

_reexport! { rust: std::env, location: "sys/env", tag: _tags!(iterator platform),
    doc: "Iterates over the arguments of a process, yielding them as [`String`][crate::String]s.",
    @Args as IterArgs,
}
_reexport! { rust: std::env, location: "sys/env", tag: _tags!(iterator platform),
    doc: "Iterates over the arguments of a process, yielding them as
[`OsString`][crate::OsString]s.",
    @ArgsOs as IterArgsOs,
}
_reexport! { rust: std::env, location: "sys/env", tag: _tags!(iterator platform),
    doc: "An iterator over a snapshot of the environment variables of this process.",
    @Vars as IterVars,
}
_reexport! { rust: std::env, location: "sys/env", tag: _tags!(iterator platform),
    doc: "An iterator over a snapshot of the environment variables of this process.",
    @VarsOs as IterVarsOs,
}

_reexport! { rust: std::env, location: "sys/env", tag: _tags!(iterator platform),
    doc: "An iterator that splits an environment variable into paths.",
    @SplitPaths as IterSplitPaths,
}

_reexport! { rust: std::env, location: "sys/env", tag: _tags!(error platform),
    doc: "Returned from [`Env::join_paths`][crate::Env#method.join_paths].",
    JoinPathsError,
}
_reexport! { rust: std::env, location: "sys/env", tag: _tags!(error platform),
    doc: "Returned from [`Env::var`][crate::Env#method.var].",
    VarError
}
