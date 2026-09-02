// devela/src/lang/prog/ffi/glsl/_.rs
//
//! <a href="https://es.wikipedia.org/wiki/GLSL"><abbr title="OpenGL Shading Language">
//! GLSL</abbr></a> interfacing.
//!
//! - <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)>
//!
//! Matrices are column-major order, and they implement a few methods for correctly indexing.
//

crate::mods_in! {
    // mod impls; // WIP

    // mod shaders; // WIP
    mod types; // g_*
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // shaders::*,
            types::*,
        };
    }
}
