// devela/src/data/id/handle/_.rs
//
#![doc = crate::_DOC_DATA_ID_HANDLE!()] // public
#![doc = crate::_doc!(modules: crate::data::id; handle)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! A handle is a compact, copyable token carrying the
//! components needed by a resolver to refer to something.
//!
//! The handle carries information; the resolver supplies meaning and validity.
//!
//! Handle fields are resolver components. Names such as *index*, *offset*,
//! *length*, or *generation* suggest common roles, but the raw representation
//! does not by itself establish bounds, occupancy, scope, kind, or liveness.
//!
//! Constructing a handle only establishes that its components are representable.
//! The resolving context determines whether it currently refers to a valid value.
//!
//! - [`handle!`] defines compact handles from arbitrary numeric components.
//! - [`handle_span!`] defines offset-and-length handles for contiguous spans.
//! - [`handle_gen!`] defines index-and-generation handles for reusable locations.
//!
//! In generational handles, a *generation* identifies the current incarnation of a
//! reusable location. It is distinct from a general revision, version, or timestamp.
//

crate::mods_in! {
    #[cfg(any(test, doctest, feature = "_docs_examples"))]
    mod _example; // HandleSpanExample

    mod define; // handle!
    mod generation; // handle_gen!
    mod span; // handle_span!
}
crate::mods_out! { // _mods, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            define::handle,
            generation::handle_gen,
            span::handle_span,
        };
        #[cfg(any(test, doctest, feature = "_docs_examples"))]
        pub use super::_example::{HandleExample, HandleGenExample, HandleSpanExample};
    }
    _hidden {
        #[cfg(any(test, doctest, feature = "_docs_examples"))]
        pub use super::_example::{
            _DOC_HANDLE_METHODS,
            _DOC_HANDLE_INDEX_METHODS,
            _DOC_HANDLE_GEN_METHODS,
            _DOC_HANDLE_SPAN_METHODS,
        };
    }
}
