// devela_base_core::media::visual::color::rgb
//
//! RGB color space
//

mod definitions; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]
mod impls;

crate::structural_mods! { // _mods
    _mods {
        pub use super::definitions::*;
    }
}
