// devela::lang::wasm::reexports
//
//! Reexported items from `core`.
//

use crate::reexport;

reexport! { rust: core::arch::wasm32,
    doc: "WASM-specific 128-bit wide SIMD vector type.",
    @v128 as w_v128
}
