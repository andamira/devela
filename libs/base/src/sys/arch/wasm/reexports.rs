// devela_base::sys::arch::wasm::reexports
//
//!
//

#[cfg(target_arch = "wasm32")]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
crate::_reexport! { rust: core::arch::wasm32,
    doc: "WASM-specific 128-bit wide SIMD vector type.",
    @v128 as w_v128
}
