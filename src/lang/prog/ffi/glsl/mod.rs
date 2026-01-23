// devela::lang::prog::ffi::glsl
//
//! <a href="https://es.wikipedia.org/wiki/GLSL"><abbr title="OpenGL Shading Language">
//! GLSL</abbr></a> interfacing.
//!
//! - <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)>
//!
//! Matrices are column-major order, and they implement a few methods for correctly indexing.
//

// mod impls; // WIP

// mod shaders; // WIP
mod types; // g_*

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // shaders::*,
            types::*,
        };
    }
}
