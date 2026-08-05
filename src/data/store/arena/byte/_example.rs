// devela/src/data/store/arena/byte/_example.rs
//
//! Defines [`ArenaBytesExample`], [`ArenaBytesHandleExample`], [`ArenaBytesMarkExample`].
//

use crate::arena_bytes;

#[cfg(any(test, feature = "_docs_examples"))]
arena_bytes! {
    [
        offset: u8;
    ]

    #[doc = crate::_tags!(example data_structure)]
    /// An example memory arena.
    ///
    /// Generated with [`arena_bytes!`].
    pub ArenaBytesExample;

    #[doc = crate::_tags!(example uid)]
    /// An example handle into [`ArenaBytesExample`].
    ///
    /// Generated with [`arena_bytes!`] and [`handle_span!`][crate::handle_span].
    pub ArenaBytesHandleExample;

    #[doc = crate::_tags!(example state)]
    /// An example memory arena mark.
    ///
    /// Generated with [`arena_bytes!`].
    pub ArenaBytesMarkExample;
}
