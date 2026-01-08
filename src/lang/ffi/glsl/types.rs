// devela::lang::ffi::glsl
//
//! GLSL types.
//
// TOC
// - data types definitions
// - impl_matrix_methods! macro
// - tests

#![allow(non_camel_case_types, missing_docs)]

pub use data_types::*;

#[rustfmt::skip]
mod data_types {
    /* scalars */

    #[doc = crate::_tags!(primitive)]
    /// A GLSL boolean.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    pub type g_bool = bool;

    #[doc = crate::_tags!(primitive num)]
    /// A GLSL single-precision floating-point number.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    pub type g_float = f32;
    #[doc = crate::_tags!(primitive num)]
    /// A GLSL double-precision floating-point number.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    pub type g_double = f64;

    #[doc = crate::_tags!(primitive num)]
    /// A GLSL signed 32-bit integer.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    pub type g_int = i32;
    #[doc = crate::_tags!(primitive num)]
    /// A GLSL unsigned 32-bit integer.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    pub type g_uint = u32;

    /* vectors */

    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `bvec2`, a vector of 2×[`g_bool`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct g_bvec2 { pub x: bool, pub y: bool }
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `bvec3`, a vector of 3×[`g_bool`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct g_bvec3 { pub x: bool, pub y: bool, pub z: bool }
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `bvec4`, a vector of 4×[`g_bool`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct g_bvec4 { pub x: bool, pub y: bool, pub z: bool, pub w: bool }

    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `vec2`, a vector of 2×[`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_vec2 { pub x: g_float, pub y: g_float }
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `vec3`, a vector of 3×[`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_vec3 { pub x: g_float, pub y: g_float, pub z: g_float }
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `vec4`, a vector of 4×[`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    pub struct g_vec4 { pub x: g_float, pub y: g_float, pub z: g_float, pub w: g_float }

    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `dvec2`, a vector of 2×[`g_double`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_dvec2 { pub x: g_double, pub y: g_double }
    /// Equivalent to GLSL `dvec3`, a vector of 3×[`g_double`]s.
    #[doc = crate::_tags!(geom)]
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_dvec3 { pub x: g_double, pub y: g_double, pub z: g_double }
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `dvec4`, a vector of 4×[`g_double`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_dvec4 { pub x: g_double, pub y: g_double, pub z: g_double, pub w: g_double }

    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `ivec2`, a vector of 2×[`g_int`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct g_ivec2 { pub x: g_int, pub y: g_int }
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `ivec3`, a vector of 3×[`g_int`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct g_ivec3 { pub x: g_int, pub y: g_int, pub z: g_int }
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `ivec4`, a vector of 4×[`g_int`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct g_ivec4 { pub x: g_int, pub y: g_int, pub z: g_int, pub w: g_int }

    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `ivec2`, a vector of 2×[`g_uint`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct g_uvec2 { pub x: g_uint, pub y: g_uint }
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `ivec3`, a vector of 3×[`g_uint`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct g_uvec3 { pub x: g_uint, pub y: g_uint, pub z: g_uint }
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `ivec4`, a vector of 4×[`g_uint`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct g_uvec4 { pub x: g_uint, pub y: g_uint, pub z: g_uint, pub w: g_uint }

    /* matrices */

    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `mat2`, a column-major 2×2 matrix of [`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_mat2(pub [g_float; 2 * 2]);
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `mat2x3`, a column-major 2×3 matrix of [`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_mat2x3(pub [g_float; 2 * 3]);
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `mat2x4`, a column-major 2×4 matrix of [`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_mat2x4(pub [g_float; 2 * 4]);

    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `mat3x2`, a column-major 3×2 matrix of [`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_mat3x2(pub [g_float; 3 * 2]);
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `mat3x3`, a column-major 3×3 matrix of [`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_mat3(pub [g_float; 3 * 3]);
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `mat3x4`, a column-major 3×4 matrix of [`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_mat3x4(pub [g_float; 3 * 4]);

    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `mat4x2`, a column-major 4×2 matrix of [`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_mat4x2(pub [g_float; 4 * 2]);
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `mat4x3`, a column-major 4×3 matrix of [`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_mat4x3(pub [g_float; 4 * 3]);
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `mat4x4`, a column-major 4×4 matrix of [`g_float`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_mat4(pub [g_float; 4 * 4]);

    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `dmat2`, a column-major 2×2 matrix of [`g_double`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_dmat2(pub [g_double; 2 * 2]);
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `dmat3`, a column-major 3×3 matrix of [`g_double`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_dmat3(pub [g_double; 3 * 3]);
    #[doc = crate::_tags!(geom)]
    /// Equivalent to GLSL `dmat4`, a column-major 4×4 matrix of [`g_double`]s.
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_dmat4(pub [g_double; 4 * 4]);

    /* custom types: vertices */

    #[doc = crate::_tags!(geom)]
    /// A convenient 2D GLSL vertex representation with [`g_float`]s (position + UV).
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_vertex2 { pub pos: g_vec2, pub uv: g_vec2 }

    #[doc = crate::_tags!(geom)]
    /// A convenient 3D GLSL vertex representation with [`g_float`]s (position + UV).
    #[doc = crate::_doc_location!("lang/ffi/glsl")]
    #[repr(C)] #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct g_vertex3 { pub pos: g_vec3, pub uv: g_vec2 }
}

macro_rules! impl_matrix_methods {
    ($($mat:ident, $inner:ty, $cols:expr, $rows:expr);* $(;)?) => {
        $(impl $mat {
            /// Retrieves the value at `(row, col)`, if valid.
            pub const fn get(&self, row: usize, col: usize) -> Option<$inner> {
                if let Some(idx) = self.to_index(row, col) { Some(self.0[idx]) } else { None }
            }
            /// Converts `(row, col)` into a linear index.
            pub const fn to_index(&self, row: usize, col: usize) -> Option<usize> {
                if row < $rows && col < $cols { Some(col * $rows + row) } else { None }
            }
            /// Converts a linear index back into `(row, col)`, if valid.
            pub const fn from_index(&self, index: usize) -> Option<(usize, usize)> {
                if index < self.0.len() { Some((index % $rows, index / $rows)) } else { None }
            }
        })*
    };
}
impl_matrix_methods!(
    g_mat2, g_float, 2, 2; g_mat2x3, g_float, 2, 3; g_mat2x4, g_float, 2, 4;
    g_mat3x2, g_float, 3, 2; g_mat3, g_float, 3, 3; g_mat3x4, g_float, 3, 4;
    g_mat4x2, g_float, 4, 2; g_mat4x3, g_float, 4, 3; g_mat4, g_float, 4, 4;
    g_dmat2, g_double, 2, 2; g_dmat3, g_double, 3, 3; g_dmat4, g_double, 4, 4;
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_methods() {
        let m = g_mat3([
            0.0, 1.0, 2.0, // Column 0
            3.0, 4.0, 5.0, // Column 1
            6.0, 7.0, 8.0, // Column 2
        ]);

        // Test `to_index`
        assert_eq!(m.to_index(0, 0), Some(0));
        assert_eq!(m.to_index(1, 2), Some(7));
        assert_eq!(m.to_index(2, 2), Some(8));
        assert_eq!(m.to_index(3, 2), None); // Out of bounds

        // Test `get`
        assert_eq!(m.get(1, 2), Some(7.0));
        assert_eq!(m.get(0, 0), Some(0.0));
        assert_eq!(m.get(2, 1), Some(5.0));
        assert_eq!(m.get(3, 1), None); // Out of bounds

        // Test `from_index`
        assert_eq!(m.from_index(0), Some((0, 0)));
        assert_eq!(m.from_index(7), Some((1, 2)));
        assert_eq!(m.from_index(8), Some((2, 2)));
        assert_eq!(m.from_index(9), None); // Out of bounds
    }
}
