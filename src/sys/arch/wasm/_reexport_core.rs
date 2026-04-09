// devela::sys::arch::wasm::_reexport_core

#[allow(unused_imports, reason = "wasm target-gate")]
use crate::{_reexport, _tags};

#[cfg(target_arch = "wasm32")]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
crate::_reexport! { rust: core::arch::wasm32, location: "sys/arch",
    tag: _tags!(platform mem num),
    doc: "WASM-specific 128-bit wide SIMD vector type.",
    @v128 as w_v128
}
