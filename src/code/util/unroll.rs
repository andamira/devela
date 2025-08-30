// devela::code::util::unroll
//
//! Re-exports the code-generated [`unroll!`] macro.
//

#[cfg_attr(nightly_doc, doc(cfg(feature = "_unroll")))]
mod codegen {
    include!(concat!(env!("OUT_DIR"), "/build/unroll.rs")); // `unroll!` macro
}

crate::structural_mods! { // _mods
    _mods {
        pub use super::codegen::*;
    }
}
