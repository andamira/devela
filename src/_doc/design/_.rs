// devela/src/_doc/design/_.rs
//
//! Principles, structure, and API conventions.
#![doc = crate::_doc!(modules: crate::_doc; design: principles, structure)] // api
#![doc = crate::_doc!(hr)]
//!
//! The library takes shape through many small decisions that gradually
//! form recurring patterns.
//!
//! This section gathers the principles behind
//! them, the structure they produce across the library, and the
//! conventions that give its public APIs a consistent shape.
//

// /// Library API design.
// pub mod api {
//     #![doc = include_str!("./api.md")]
// }

/// Design principles, constraints, and enduring project posture.
pub mod principles {
    #![doc = include_str!("./principles.md")]
}

/// # Library structure and source organization.
pub mod structure {
    #![doc = include_str!("./structure.md")]
}
