// devela::ui::back::miniquad::namespace
//
//! Defines the [`miniquad!`] macro.
//

#[cfg(doc)]
use ::miniquad::{
    Bindings, BufferId, BufferSource, BufferType, BufferUsage, RenderingBackend, ShaderId,
    ShaderMeta,
};

/// A macro for namespaced [`miniquad`][::miniquad] operations.
///
/// # Arms
/// - [`new_buffer`] wrappers:
///   - new_vertices([`$ctx`], [`$usage`], `&[vertices]`) -> [`BufferId`];
///   - new_indices([`$ctx`], [`$usage`], `&[indices]`) -> [`BufferId`];
///   - new_vertices_indices([`$ctx`], [`$usage`], `&[vertices]`, `&[indices]`)
///     -> ([`BufferId`], [`BufferId`]);
/// - [`new_shader`] wrappers:
///   - new_shader([`$ctx`], `&str`, `&str`, `&str`, [`$meta`]) -> [`ShaderId`];
///   - new_shader_glsl([`$ctx`], `&str`, `&str`, [`$meta`]) -> [`ShaderId`];
///   - new_shader_msl([`$ctx`], `&str`, [`$meta`]) -> [`ShaderId`];
/// - [`Bindings`] wrappers:
///   - bindings(`Vec<`[`BufferId`]`>`, [`BufferId`], `Vec<`[`ShaderId`]`>`) -> [`Bindings`];
///
/// [`new_buffer`]: ::miniquad::RenderingBackend::new_buffer
/// [`$ctx`]: ::miniquad::RenderingBackend
/// [`$usage`]: ::miniquad::BufferUsage
/// [`new_shader`]: ::miniquad::RenderingBackend::new_shader
/// [`$meta`]: ::miniquad::ShaderMeta
#[doc(hidden)]
#[macro_export]
macro_rules! _miniquad {
    (
    /* buffers */

    // Creates a vertex buffer resource object.
    new_vertices($ctx:expr, $(BufferUsage::)?$usage:ident, $vertices:expr)) => { $crate::paste! {
        // https://docs.rs/miniquad/latest/miniquad/graphics/enum.BufferType.html
        // https://docs.rs/miniquad/latest/miniquad/graphics/enum.BufferUsage.html
        // https://docs.rs/miniquad/latest/miniquad/graphics/enum.BufferSource.html
        // https://docs.rs/miniquad/latest/miniquad/graphics/struct.BufferId.html
        $ctx.new_buffer(
            ::miniquad::BufferType::VertexBuffer,
            ::miniquad::BufferUsage:: [<$usage:camel>],
            ::miniquad::BufferSource::slice($vertices),
        )
    }};
    (
    // Creates an index buffer resource object.
    new_indices($ctx:expr, $usage:ident, $indices:expr)) => { $crate::paste! {
        $ctx.new_buffer(
            ::miniquad::BufferType::IndexBuffer,
            ::miniquad::BufferUsage:: [<$usage:camel>],
            ::miniquad::BufferSource::slice($indices),
        )
    }};
    (
    // Creates both a vertex and index buffer resource objects.
    new_vertices_indices($ctx:expr, $usage:ident, $vertices:expr, $indices:expr)) => {
        (
            $crate::miniquad![new_vertices($ctx, $usage, $vertices)],
            $crate::miniquad![new_indices($ctx, $usage, $indices)],
        )
    };
    (
    /* shaders */

    // Creates a OpenGL shader buffer resource object.
    new_shader_glsl($ctx:expr, $vertex:expr, $fragment:expr, $meta:expr)) => {
        // https://docs.rs/miniquad/latest/miniquad/graphics/trait.RenderingBackend.html#tymethod.new_shader
        // https://docs.rs/miniquad/latest/miniquad/graphics/enum.ShaderSource.html
        // https://docs.rs/miniquad/latest/miniquad/graphics/struct.ShaderMeta.html
        // https://docs.rs/miniquad/latest/miniquad/graphics/struct.ShaderId.html
        // https://docs.rs/miniquad/latest/miniquad/graphics/enum.ShaderError.html
        // https://docs.rs/miniquad/latest/miniquad/graphics/enum.ShaderType.html
        $ctx.new_shader(
            ::miniquad::ShaderSource::Glsl { vertex: $vertex, fragment: $fragment },
            $meta
        )
    };
    (
    // Creates a Metal shader buffer resource object.
    new_shader_msl($ctx:expr, $msl:expr, $meta:expr)) => {
        $ctx.new_shader(::miniquad::ShaderSource::Msl { program: $msl }, $meta)
    };
    (
    // Creates a Glsl or Metal shader buffer resource object, depending on the backend.
    new_shader($ctx:expr, $vertex:expr, $fragment:expr, $msl:expr, $meta:expr)) => {
        match $ctx.info().backend {
           ::miniquad::Backend::OpenGl =>
               $crate::miniquad![new_shader_glsl($ctx, $vertex, $fragment, $meta)],
           ::miniquad::Backend::Metal =>
               $crate::miniquad![new_shader_msl($ctx, $msl, $meta)],
        }
    };
    (
    /* bindings */

    // Constructs Bindings.
    bindings($vertex_buffers:expr, $index_buffer:expr, $images:expr) ) => {
        // https://docs.rs/miniquad/latest/miniquad/graphics/struct.Bindings.html
        ::miniquad::Bindings {
            vertex_buffers: $vertex_buffers, index_buffer: $index_buffer, images: $images
        }
    };
}
#[doc(inline)]
pub use _miniquad as miniquad;
