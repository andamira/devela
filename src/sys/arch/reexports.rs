// devela::sys::arch::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

reexport! { rust: core::arch,
    doc: "Inline assembly.",
    asm
}
reexport! { rust: core::arch,
    doc: "Module-level inline assembly.",
    global_asm
}

reexport! { rust: std::arch,
    doc: "Tests at *runtime* whether an `aarch64` feature is enabled on aarch64 platforms.",
    is_aarch64_feature_detected
}

reexport! { rust: std::arch,
    doc: "Tests at *runtime* whether a CPU feature is enabled on x86/x86-64 platforms.",
    is_x86_feature_detected
}
