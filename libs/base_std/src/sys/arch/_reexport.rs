// devela::sys::arch::_reexport

use crate::{_TAG_PLATFORM, _reexport};

_reexport! { rust: std::arch, location: "sys/arch", tag: _TAG_PLATFORM!(),
    doc: "Tests at *runtime* whether an `aarch64` feature is enabled.",
    @is_aarch64_feature_detected as detect_aarch64
}
_reexport! { rust: std::arch, location: "sys/arch", tag: _TAG_PLATFORM!(),
    doc: "Tests at *runtime* whether an `x86/x86-64` feature is enabled.",
    @is_x86_feature_detected as detect_x86
}
