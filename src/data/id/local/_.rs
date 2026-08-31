// devela/src/data/id/local/_.rs
//
#![doc = crate::_DOC_DATA_ID_LOCAL!()] // public
#![doc = crate::_doc!(modules: crate::data::id; local)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Identity in this module is scoped by a local execution, allocation,
//! registry, or type context rather than by a portable external format.
//!
//! - [`IdPin`] and [`IdPinBox`] anchor identity to stable memory locations.
//! - [`IdRegistry`] maps external identities into compact local identities.
//! - [`id_seq!`] allocates sequential identities within a local generator.
//! - [`TypeResource`] distinguishes otherwise compatible IDs by resource type.
//

crate::mods_in! {
    #[cfg(any(test, feature = "_docs_examples"))]
    mod _example; // IdSeqU64Example

    mod pin; // IdPin
    #[cfg(feature = "alloc")]
    mod pin_box; // IdPinBox

    mod registry; // IdRegistry
    mod seq; // id_seq!
    mod type_resource; // zero-cost type-safe resource markers
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            pin::IdPin,
            registry::IdRegistry,
            seq::id_seq,
            type_resource::{TypeResource, TypeResourced, type_resource},
        };
        #[cfg(feature = "alloc")]
        pub use super::pin_box::IdPinBox;

        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::IdSeqU64Example;
    }
}
