// devela/src/media/visual/color/rgb/mod.rs
//
//! RGB color space
//

mod define; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]
mod impls;

crate::mods_out! { // _mods
    _mods {
        pub use super::define::*;
    }
}
