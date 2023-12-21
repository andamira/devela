// devela::code::reexport
//
//! Reexported items.
//

/// <span class="stab portability" title="re-exported from `devela_macros`
/// crate">`devela_macros`</span>
#[cfg_attr(feature = "nightly", doc(cfg(feature = "meta")))]
pub use devela_macros::{cif, coalesce, compile, compile_attr};

/// <span class="stab portability" title="re-exported from the `paste crate`">`paste`</span>
pub use super::paste::paste;

/* core::hint reexports */

use super::reexport;

reexport! { rust: core::hint, local_module: "code",
    doc: "An identity function that hints to the compiler to be maximally
        pessimistic about what black_box could do.",
    black_box
}
reexport! { rust: core::hint, local_module: "code",
    doc: "Emits a machine instruction to signal the processor that
        it is running in a busy-wait spin-loop (“spin lock”).",
    spin_loop
}
reexport! { rust: core::hint, local_module: "code",
    doc: "Informs the compiler that the site which is calling this function
        is not reachable, possibly enabling further optimizations.",
    unreachable_unchecked
}
