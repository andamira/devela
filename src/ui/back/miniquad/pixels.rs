// devela::ui::back::miniquad::pixels
//
//! Defines [`MiniquadPixels`].
//
// TOC
// - MiniquadPixels
// - mod shader

use crate::{
    Box, MiniquadEventHandlerExt, MiniquadWindow, Vec, format_buf, g_vec2, g_vertex2, is, miniquad,
    vec_ as vec,
};
use ::miniquad::{
    Bindings, BufferLayout, EventHandler, FilterMode, MipmapFilterMode, Pipeline, PipelineParams,
    RenderingBackend, TextureFormat, TextureId, TextureParams, VertexAttribute, VertexFormat,
};

#[doc = crate::_TAG_IMAGE!()]
#[doc = crate::_TAG_RUNTIME!()]
/// Draws a single fullscreen quad textured by a pixel buffer.
#[doc = crate::_doc_location!("ui/back/miniquad")]
pub struct MiniquadPixels {
    ctx: Option<Box<dyn RenderingBackend>>,
    pipeline: Option<Pipeline>,
    bindings: Option<Bindings>,
    texture: Option<TextureId>,

    /* pixel buffer options */
    ///
    pub pixels: Vec<u8>,
    width: u32,
    height: u32,
    viewport: (f32, f32, f32, f32), // x, y, w, h IMPROVE: use Extent2d
    //
    interpolation: bool,
    maintain_aspect_ratio: bool,
}

impl core::fmt::Debug for MiniquadPixels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut buf = [0u8; 20];

        f.debug_struct("MiniquadPixels")
            // .field("ctx", "…")
            // .field("pipeline", "…")
            // .field("bindings", "…")
            // .field("texture", "…")
            .field("pixels", &format_buf![&mut buf, "{}B", self.pixels.len()].unwrap())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("interpolation", &self.interpolation)
            .field("maintain_aspect_ratio", &self.maintain_aspect_ratio)
            .finish()
    }
}

impl MiniquadPixels {
    /// Returns an uninitialized pixel-? stage with the given buffer size.
    ///
    /// By default it maintains the aspect ratio, and doesn't interpolate.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            ctx: None,
            pipeline: None,
            bindings: None,
            texture: None,
            /* pixel buffer */ // IMPROVE use a specialized struct
            pixels: vec![0; width as usize * height as usize * 4],
            width,
            height,
            viewport: (0.0, 0.0, width as f32, height as f32),
            interpolation: false,
            maintain_aspect_ratio: true,
        }
    }

    /// Returns the current viewport size
    // IMPROVE: Use Extent2d
    pub fn viewport(&self) -> (f32, f32, f32, f32) {
        self.viewport
    }

    /// Initialize the pixel buffer stage.
    ///
    /// This method performs the following steps:
    /// - Creates a new rendering backend context.
    /// - Defines a fullscreen quad in normalized device coordinates.
    ///   - The vertex shader will flip the y-axis to match the pixel buffer’s top-left origin.
    ///   - If `maintain_aspect_ratio == true` it will be letterboxed to maintain proportion,
    ///     otherwise it will be rendered covering the entire screen from (-1, -1) to (1, 1).
    /// - Creates a texture from the pixel data with the specified size, and filtering mode.
    /// - Compiles the vertex and fragment shaders.
    /// - Sets up the rendering pipeline.
    /// - Returns the initialized `MiniquadPixels` instance.
    pub fn init(mut self) -> Self {
        let mut ctx: Box<dyn RenderingBackend> = MiniquadWindow::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: [g_vertex2; 4] = [
            g_vertex2 { pos : g_vec2 { x: -1.0, y: -1.0 }, uv: g_vec2 { x: 0., y: 0. } },
            g_vertex2 { pos : g_vec2 { x:  1.0, y: -1.0 }, uv: g_vec2 { x: 1., y: 0. } },
            g_vertex2 { pos : g_vec2 { x:  1.0, y:  1.0 }, uv: g_vec2 { x: 1., y: 1. } },
            g_vertex2 { pos : g_vec2 { x: -1.0, y:  1.0 }, uv: g_vec2 { x: 0., y: 1. } },
        ];
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let (vbuf, ibuf) = miniquad![new_vertices_indices(ctx, Immutable, &vertices, &indices)];

        let interp = is![self.interpolation; FilterMode::Linear; FilterMode::Nearest];

        let texture = ctx.new_render_texture(TextureParams {
            width: self.width,
            height: self.height,
            format: TextureFormat::RGBA8,
            mag_filter: interp,
            min_filter: interp,
            ..Default::default()
        });

        let bindings = miniquad![bindings(vec![vbuf], ibuf, vec![texture])];
        let shader = miniquad![new_shader(ctx, VERTEX, FRAGMENT, METAL, shader_meta())].unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        self.ctx = Some(ctx);
        self.pipeline = Some(pipeline);
        self.bindings = Some(bindings);
        self.texture = Some(texture);
        self
    }
}

#[rustfmt::skip]
impl MiniquadEventHandlerExt for MiniquadPixels {
    fn init(self) -> Self { self.init() }
    fn interpolation(&self) -> bool { self.interpolation }
    fn set_interpolation(&mut self, set: bool) {
        self.interpolation = set;
        if let Some(ctx) = &mut self.ctx {
            if let Some(texture) = self.texture {
                let f = is![set; FilterMode::Linear; FilterMode::Nearest];
                ctx.texture_set_filter(texture, f,  MipmapFilterMode::None);
            }
        }
    }
    fn maintain_aspect_ratio(&self) -> bool { self.maintain_aspect_ratio }
    fn set_maintain_aspect_ratio(&mut self, set: bool) { self.maintain_aspect_ratio = set; }
}
impl EventHandler for MiniquadPixels {
    fn update(&mut self) {}
    fn draw(&mut self) {
        if self.maintain_aspect_ratio {
            let ctx = self.ctx.as_mut().unwrap();

            let (sw, sh) = MiniquadWindow::get_size();
            let (fw, fh) = (self.width as f32, self.height as f32);

            let desired_ratio = fw / fh;
            let current_ratio = sw / sh;
            let scale = if current_ratio > desired_ratio {
                sh / fh // letterbox horizontally
            } else {
                sw / fw // letterbox vertically
            };

            let vp_w = (scale * fw).round();
            let vp_h = (scale * fh).round();
            // center within the window
            let vp_x = ((sw - vp_w) / 2.0).round();
            let vp_y = ((sh - vp_h) / 2.0).round();

            self.viewport = (vp_x, vp_y, vp_w, vp_h);

            // Update the texture with the pixel data
            ctx.texture_update(self.texture.unwrap(), &self.pixels);
            // Begin rendering the default pass.
            ctx.begin_default_pass(Default::default());

            // Apply our viewport that maintains the aspect ratio.
            ctx.apply_viewport(vp_x as i32, vp_y as i32, vp_w as i32, vp_h as i32);

            // Apply the pipeline and bindings.
            ctx.apply_pipeline(&self.pipeline.unwrap());
            ctx.apply_bindings(self.bindings.as_ref().unwrap());
            // Draw the vertices.
            ctx.draw(0, 6, 1);

            // restore the viewport
            ctx.apply_viewport(0, 0, sw as i32, sh as i32);

            ctx.end_render_pass();
            ctx.commit_frame();
        } else {
            let ctx = self.ctx.as_mut().unwrap();

            let (sw, sh) = MiniquadWindow::get_size();
            // let (fw, fh) = (self.width as f32, self.height as f32); // MAYBE, CHECK
            self.viewport = (0.0, 0.0, sw, sh);

            // Update the texture with the pixel data
            ctx.texture_update(self.texture.unwrap(), &self.pixels);
            // Begin rendering the default pass
            ctx.begin_default_pass(Default::default());

            // Apply the pipeline and bindings
            ctx.apply_pipeline(&self.pipeline.unwrap());
            ctx.apply_bindings(self.bindings.as_ref().unwrap());
            // Draw the vertices
            ctx.draw(0, 6, 1);

            ctx.end_render_pass();
            ctx.commit_frame();
        }
    }
}

use shader::{FRAGMENT, METAL, VERTEX, shader_meta};
mod shader {
    use crate::{ToString, vec_ as vec};
    use ::miniquad::{ShaderMeta, UniformBlockLayout};

    // Returns the shader metadata, such as the names of the images and uniforms it uses.
    pub fn shader_meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }

    // Define the vertex shader, for transforming the vertices into screen coordinates.
    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;
    uniform vec2 offset;
    varying lowp vec2 texcoord;
    void main() {
        gl_Position = vec4(in_pos + offset, 0, 1);
        // Flip y axis: convert texture coordinates from bottom-left to top-left origin
        texcoord = vec2(in_uv.x, 1.0 - in_uv.y);
        // texcoord = in_uv; // no flipping
    }"#;

    // Define the fragment shader, for computing the color of each pixel.
    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;
    uniform sampler2D tex;
    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    // Define the Metal shader, translated from VERTEX and FRAGMENT.
    pub const METAL: &str = r#"#include <metal_stdlib>
    using namespace metal;

    struct VertexIn {
        float2 in_pos [[attribute(0)]];
        float2 in_uv  [[attribute(1)]];
    };
    struct Uniforms {
        float2 offset;
    };
    struct VertexOut {
        float4 position [[position]];
        float2 texcoord;
    };

    vertex VertexOut vertex_main(VertexIn in [[stage_in]],
                                 constant Uniforms &uniforms [[buffer(1)]])
    {
        VertexOut out;
        float2 pos = in.in_pos + uniforms.offset;
        out.position = float4(pos, 0.0, 1.0);
        // Flip y axis: convert texture coordinates from bottom-left to top-left origin
        out.texcoord = float2(in.in_uv.x, 1.0 - in.in_uv.y);
        // out.texcoord = in.in_uv; // no flipping
        return out;
    }

    fragment float4 fragment_main(VertexOut in [[stage_in]],
                                  texture2d<float> tex [[texture(0)]],
                                  sampler samp [[sampler(0)]])
    {
        return tex.sample(samp, in.texcoord);
    }
    "#;
}
