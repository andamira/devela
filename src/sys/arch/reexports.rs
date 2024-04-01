// devela::sys::arch::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

/* `core` re-exports */

reexport! { rust: core::arch,
    doc: "Inline assembly.",
    asm
}
reexport! { rust: core::arch,
    doc: "Module-level inline assembly.",
    global_asm
}

/* `std` re-exports */

reexport! { rust: std::arch,
    doc: "Tests at *runtime* whether an `aarch64` feature is enabled.",
    @is_aarch64_feature_detected as detect_aarch64
}
reexport! { rust: std::arch,
    doc: "Tests at *runtime* whether an `x86/x86-64` feature is enabled.",
    @is_x86_feature_detected as detect_x86
}
