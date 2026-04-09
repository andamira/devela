// devela::sys::arch::_reexport_core

use crate::{_reexport, _tags};

_reexport! { rust: core::arch, location: "sys/arch", tag: _tags!(code platform),
    doc: "Inline assembly.",
    asm
}
_reexport! { rust: core::arch, location: "sys/arch", tag: _tags!(code platform),
    doc: "Module-level inline assembly.",
    global_asm
}
