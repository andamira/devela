// devela/src/data/id/local/_.rs
//
#![doc = crate::_DOC_DATA_ID_LOCAL!()] // public
#![doc = crate::_doc!(modules: crate::data::id; local)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Local identities are meaningful within a context
//! that establishes and bounds their uniqueness.
//!
//! The context may be a stable memory location,
//! a registry, a sequential allocator, or a resource type.
//!
//! Local identities rely on that surrounding context to determine
//! what they distinguish and for how long the distinction remains valid.
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
