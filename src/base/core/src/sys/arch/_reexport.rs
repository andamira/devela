// devela_base_core::sys::arch::_reexport

use crate::{_TAG_CODE, _TAG_PLATFORM, _reexport};

_reexport! { rust: core::arch, location: "sys/arch", tag: _TAG_CODE!() _TAG_PLATFORM!(),
    doc: "Inline assembly.",
    asm
}
_reexport! { rust: core::arch, location: "sys/arch", tag: _TAG_CODE!() _TAG_PLATFORM!(),
    doc: "Module-level inline assembly.",
    global_asm
}
