//
//! Reexported hints.
//

#[cfg(doc)]
use crate::Arch;
use crate::{_reexport, _tags};

/* `core::hint` functions */
// https://doc.rust-lang.org/stable/core/hint/

_reexport! { rust: core::hint,
    location: "code/hint" => fn assert_unchecked, tag: _tags!(assert),
    doc: "Makes a *soundness* promise to the compiler that the `cond`ition holds.", assert_unchecked
}
_reexport! { rust: core::hint,
    location: "code/hint" => fn black_box, tag: _tags!(code),
    doc: "Hints the compiler to be maximally pessimistic about what black_box could do.", black_box
}
_reexport! { rust: core::hint,
    location: "code/hint" => fn cold_path, tag: _tags!(code),
    doc: "Hints to the compiler that given path is cold, i.e., unlikely to be taken.", cold_path
}
_reexport! { rust: core::hint,
    location: "code/hint" => fn select_unpredictable, tag: _tags!(code),
    doc: "Hints the compiler that the `condition` is branch-unpredictable.", select_unpredictable
}
_reexport! { rust: core::hint,
    location: "code/hint" => fn spin_loop, tag: _tags!(code),
    doc: "Signals the processor that it is running in a busy-wait spin-loop.\n\n
This is a best-effort hint and may do nothing. In current Rust implementations
it emits an architecture-specific instruction on x86/x86_64, RISC-V, AArch64,
LoongArch, and supported ARM configurations. On AVR, MSP430, wasm32, Xtensa,
and other unsupported architectures, the fallback is empty and may emit no
instruction at all.\n\n
It must not be used as a timing or delay primitive.\n\n
See also [`Arch::relax`] for a busy-wait step with an instruction-emitting
fallback, and [`Arch::nop`] for executing an actual no-operation instruction.",
    spin_loop
}
_reexport! { rust: core::hint,
    location: "code/hint" => fn unreachable_unchecked, tag: _tags!(assert),
    doc: "Informs the compiler that the current calling site is not reachable.", unreachable_unchecked
}
