// devela/src/code/hint/_reexport_core.rs
//
//! Reexported hints.
//

use crate::{_reexport, _tags};

/* `core::hint` functions */
// https://doc.rust-lang.org/stable/core/hint/

_reexport! { rust: core::hint,
    location: "code/hint" => macro assert_unchecked, tag: _tags!(assert),
    doc: "Makes a *soundness* promise to the compiler that the `cond`ition holds.", assert_unchecked
}
_reexport! { rust: core::hint,
    location: "code/hint" => macro black_box, tag: _tags!(code),
    doc: "Hints the compiler to be maximally pessimistic about what black_box could do.", black_box
}
_reexport! { rust: core::hint,
    location: "code/hint" => macro cold_path, tag: _tags!(code),
    doc: "Hints to the compiler that given path is cold, i.e., unlikely to be taken.", cold_path
}
_reexport! { rust: core::hint,
    location: "code/hint" => macro select_unpredictable, tag: _tags!(code),
    doc: "Hints the compiler that the `condition` is branch-unpredictable.", select_unpredictable
}
_reexport! { rust: core::hint,
    location: "code/hint" => macro spin_loop, tag: _tags!(code),
    doc: "Signals the processor that it is running in a busy-wait spin-loop.", spin_loop
}
_reexport! { rust: core::hint,
    location: "code/hint" => macro unreachable_unchecked, tag: _tags!(assert),
    doc: "Informs the compiler that the current calling site is not reachable.", unreachable_unchecked
}
