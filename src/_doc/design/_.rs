// devela/src/_doc/design/_.rs
//
//! Principles, structure, and API conventions.
#![doc = crate::_doc!(modules: crate::_doc; design: constitution)] // api, structure
#![doc = crate::_doc!(hr)]
//!
//! The library takes shape through many small decisions
//! that gradually form recurring patterns.
//!
//! This section develops the principles and structural ideas
//! that help those decisions remain coherent.
//

// /// Library API design.
// pub mod api {
//     #![doc = include_str!("./api.md")]
// }

/// Design principles, constraints, and enduring project posture.
pub mod constitution {
    #![doc = include_str!("./constitution.md")]
}

// /// # Library structure and source organization.
// pub mod structure {
//     #![doc = include_str!("./structure.md")]
// }
