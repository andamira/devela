// devela::sys::arch::reexports
//
//!
//

use crate::_reexport;

_reexport! { rust: core::arch,
    doc: "Inline assembly.",
    asm
}
_reexport! { rust: core::arch,
    doc: "Module-level inline assembly.",
    global_asm
}
