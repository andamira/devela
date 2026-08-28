// devela/src/data/layout/table/mod.rs
//
#![doc = crate::_DOC_DATA_LAYOUT_TABLE!()] // public
#![doc = crate::_doc!(modules: crate::data::layout; table)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Tables interpret two-dimensional array structure as rows and columns.
//! [`ArrayShape`] supplies dimensional extent, while [`ArrayLayout`] maps
//! logical cells to physical storage.
//!
//! This module defines tabular structure and access only. Cell semantics,
//! schema, persistence, and encoded representations remain separate concerns.
//!
//! [`ArrayShape`]: crate::ArrayShape
//! [`ArrayLayout`]: crate::ArrayLayout
//

#[cfg(test)]
mod _test;

mod define; // Table
mod backing;

mod coord;
mod layout;
mod shape;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            coord::{TableCoord, TableCoordIter},
            define::Table,
            layout::TableLayout,
            shape::TableShape,
        };
    }
}
