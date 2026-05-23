// devela::data::codec::pack::compress
//
//! Size-reducing codecs.
//

mod mode; // CompressionMode

// mod rle; // WIP Run-length encoding and similar techniques

// #[cfg(feature = "alloc")]
// mod lempel_ziv; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            mode::*,
            // rle::*,
        };
        // #[cfg(feature = "alloc")]
        // pub use super::lempel_ziv::*;
    }
}
