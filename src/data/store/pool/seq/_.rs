// devela/src/data/store/pool/seq/_.rs
//
//! Variable-length sequence pools.
//!
//! Sequences have generational identities while their contiguous cell spans
//! may be reclaimed or relocated. Cells preserve order but
//! have no independent stable identity.
//!
//! Logical sequence length is distinct from reserved span capacity. This allows
//! sequences to grow within existing reservations while making physical
//! fragmentation observable and explicitly manageable.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;
    #[cfg(any(test, feature = "_docs_examples"))]
    mod _example;

    mod define; // pool_seq!
    mod_ impls; // hidden macros for pool_seq! variants
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::define::pool_seq;
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
    _hidden {
        pub use super::impls::_hidden::*;
    }
}
