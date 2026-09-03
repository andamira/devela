// devela/src/media/visual/color/rgb/_.rs
//
//! RGB color space
//

crate::mods_in! {
    mod define; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]
    mod_ impls;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::*;
    }
}
