// devela::code::util::unroll
//
//! Re-exports the code-generated [`unroll!`] macro.
//

#[cfg_attr(nightly_doc, doc(cfg(feature = "_unroll")))]
mod codegen {
    include!(concat!(env!("OUT_DIR"), "/build/unroll.rs")); // `unroll!` macro
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::codegen::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
