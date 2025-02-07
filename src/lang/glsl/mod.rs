// devela::lang::glsl
//
//! <a href="https://es.wikipedia.org/wiki/GLSL"><abbr title="OpenGL Shading Language">
//! GLSL</abbr></a> interfacing.
//!
//! - <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)>
//!
//! Matrices are column-major order, and they implement a few methods for correctly indexing.
//

mod types;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::types::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
