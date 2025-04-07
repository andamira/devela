// devela::media::color::rgb
//
//! RGB color space
//

mod definitions; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]
mod impls;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::definitions::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
