// devela/src/data/layout/table/mod.rs
//
#![doc = crate::_DOC_DATA_LAYOUT_TABLE!()] // public
#![doc = crate::_doc!(modules: crate::data::layout; table)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Tables distinguish two logical axes as rows and columns while delegating
//! general dimensional shape and affine storage mapping to
//! [`ArrayShape`] and [`ArrayLayout`].
//!
//! This module describes tabular structure only. Cell value semantics,
//! schema, persistence, and encoded representations are independent concerns.
//!
//! [`ArrayShape`]: crate::ArrayShape
//! [`ArrayLayout`]: crate::ArrayLayout
//

#[cfg(test)]
mod _test;

// mod define; //
// mod backing; //

mod coord;
mod layout;
mod shape;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            coord::{TableCoord, TableCoordIter},
            layout::TableLayout,
            shape::TableShape,
        };
    }
}
