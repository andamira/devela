// devela/src/_doc/practice/_.rs
//
//! Patterns, techniques, and project tooling.
#![doc = crate::_doc!(modules: crate::_doc; practice: macros)] // tooling
#![doc = crate::_doc!(hr)]
//!
//! A codebase also develops ways of working
//! that are larger than any particular implementation.
//!
//! This section keeps the techniques, conventions, and tooling
//! that have proved useful across the project.
//

crate::mods_in! {
    /// # Concepts, patterns, and practical notes on Rust macros.
    pub mod_ macros;
}
// /// # Repository tooling and maintenance workflows.
// pub mod tooling {
//     #![doc = include_str!("./tooling.md")]
// }
