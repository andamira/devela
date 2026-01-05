// devela_base_core::sys::arch::wasm::_reexport

#[allow(unused_imports, reason = "wasm target-gate")]
use crate::{_TAG_MEM, _TAG_NUM, _TAG_PLATFORM, _reexport};

#[cfg(target_arch = "wasm32")]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
crate::_reexport! { rust: core::arch::wasm32, location: "sys/arch",
    tag: _TAG_PLATFORM!() _TAG_MEM!() _TAG_NUM!(),
    doc: "WASM-specific 128-bit wide SIMD vector type.",
    @v128 as w_v128
}
