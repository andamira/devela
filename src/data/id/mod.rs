// devela/src/data/id/mod.rs
//
#![doc = crate::_DOC_DATA_ID!()] // public
#![doc = crate::_doc!(modules: crate::data; id)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! It answers *which thing* is being referred to, independently of where it
//! is stored, how it is reached, or how it is related to other values.
//!
//! An identity may be generated, physically anchored, translated from an
//! external identifier, or meaningful only within a particular store.
//!
//! - [`handle_gen!`] defines index-and-generation handles for reusable slots.
//! - [`handle_span!`] defines offset-and-length handles for contiguous regions.
//! - [`id_seq!`] produces sequential identifiers.
//! - [`IdPin`] and [`IdPinBox`] anchor identity to stable memory locations.
//! - [`IdRegistry`] maps foreign identifiers into compact local identities.
//

mod handle; // handle!, HandleExample
mod uid; // IdPin

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            handle::*,
            uid::_all::*,
        };
    }
}
