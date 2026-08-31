// devela/src/data/word/_.rs
//
#![doc = crate::_DOC_DATA_WORD!()] // public
#![doc = crate::_doc!(modules: crate::data; word)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//! Words are copyable representational atoms with one canonical raw form.
//!
//! [`WordTry`] identifies a type exactly with an admitted subset of its raw
//! representation domain. Peeling into that representation is always possible,
//! while reconstruction may reject raw values outside the admitted subset.
//!
//! [`Word`] is the total case: every value of the raw representation is
//! admitted, establishing an exact correspondence between the two domains.
//!
//! Canonical reconstruction is distinct from other ways of constructing a
//! value. Domain-specific constructors may accept broader inputs and project,
//! clamp, wrap, quantize, reserve, or otherwise normalize them when that is
//! useful to the abstraction. Such operations do not weaken the word contract:
//! [`WordTry::try_from_raw`] must reconstruct admitted raw representations
//! exactly and reject, rather than normalize, non-admitted ones.
//!
//! This is a value-level representational contract, not a memory-layout or
//! serialization guarantee. A word and its raw representation need not have
//! identical Rust layouts, and reconstruction concerns representation validity
//! rather than parsing, storage lookup, resource resolution, or other
//! contextual state.
//!
//! [`word!`] generates these interfaces for transparent newtypes and for
//! explicitly supplied representation lenses.
//!
//! Here, *word* denotes a small self-contained copyable representational atom;
//! it does not specifically mean a target-machine word.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod macros; // word!
    mod traits; // Word, WordTry
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            macros::word,
            traits::{Word, WordTry},
        };
    }
}
