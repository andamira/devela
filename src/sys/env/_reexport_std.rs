// devela::sys::env::reexports_std

use crate::{_reexport, _tags};

_reexport! { rust: std::env, location: "sys/env", tag: _tags!(error platform),
    doc: "Returned from [`Env::join_paths`][crate::Env::join_paths].",
    JoinPathsError,
}
_reexport! { rust: std::env, location: "sys/env", tag: _tags!(error platform),
    doc: "Returned from [`Env::var`][crate::Env::var].",
    VarError
}
