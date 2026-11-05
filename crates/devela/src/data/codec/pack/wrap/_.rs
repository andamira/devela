//
//! Structured wrappers and chunked containers.
//

crate::mods_in! {
    #[cfg(feature = "riff")]
    mod_ riff; // Resource Interchange File Format
    // mod png_chunk; // MAYBE
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "riff")]
        pub use super::riff::_all::*;
        // pub use super::png_chunk::*;
    }
}
