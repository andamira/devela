// devela/src/data/codec/pack/compress/_.rs
//
//! Size-reducing codecs.
//

crate::mods_in! {
    mod mode; // CompressionMode

    // mod rle; // WIP Run-length encoding and similar techniques

    // #[cfg(feature = "alloc")]
    // mod lempel_ziv; // WIP
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            mode::CompressionMode,
            // rle::*,
        };
        // #[cfg(feature = "alloc")]
        // pub use super::lempel_ziv::*;
    }
}
