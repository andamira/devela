// devela/src/sys/arch/wasm/_reexport_core.rs

#[allow(unused_imports, reason = "wasm target-gate")]
use crate::{_reexport, _tags};

#[cfg(target_arch = "wasm32")]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
crate::_reexport! { rust: core::arch::wasm32,
    location: "sys/arch" => struct w_v128,
    tag: _tags!(platform mem num),
    doc: "WASM-specific 128-bit wide SIMD vector type.",
    @v128 as w_v128
}

// doc-shim
#[cfg(all(doc, not(target_arch = "wasm32")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
#[doc = crate::_tags!(platform mem num)]
/// WASM-specific 128-bit wide SIMD vector type.
#[doc = crate::_doc_meta! {
    location("sys/arch", struct w_v128),
    origin(rust core::arch::wasm32; renamed(v128 as w_v128)),
}]
pub struct w_v128([u8; 16]);
