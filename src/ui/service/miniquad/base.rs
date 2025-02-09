// devela::ui::service::miniquad::base
//
//! Defines [`MiniquadPixelBuf`].
//
// TOC
// - MiniquadPixelBuf
// - mod shader

use crate::{
    format_buf, g_vec2, g_vertex2, iif, vec_ as vec, Box, MiniquadEventHandler,
    MiniquadEventHandlerExt, MiniquadWindow, Vec,
};
use ::miniquad::{
    Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, FilterMode, MipmapFilterMode,
    Pipeline, PipelineParams, RenderingBackend, ShaderSource, TextureFormat, TextureId,
    TextureParams, VertexAttribute, VertexFormat,
};

/// Draws a single fullscreen quad textured by a pixel buffer.
pub struct MiniquadPixelBuf {
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

impl core::fmt::Debug for MiniquadPixelBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut buf = [0u8; 20];

        f.debug_struct("MiniquadPixelBuf")
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

impl MiniquadPixelBuf {
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
    /// - Returns the initialized `MiniquadPixelBuf` instance.
    pub fn init(mut self) -> Self {
        let mut ctx: Box<dyn RenderingBackend> = MiniquadWindow::new_rendering_backend();

        // Define vertices for the stage.
        #[rustfmt::skip]
        let vertices: [g_vertex2; 4] = [
            g_vertex2 { pos : g_vec2 { x: -1.0, y: -1.0 }, uv: g_vec2 { x: 0., y: 0. } },
            g_vertex2 { pos : g_vec2 { x:  1.0, y: -1.0 }, uv: g_vec2 { x: 1., y: 0. } },
            g_vertex2 { pos : g_vec2 { x:  1.0, y:  1.0 }, uv: g_vec2 { x: 1., y: 1. } },
            g_vertex2 { pos : g_vec2 { x: -1.0, y:  1.0 }, uv: g_vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        // Define indices for the vertices.
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        // Create a new texture from the pixel data.
        let interp = iif![self.interpolation; FilterMode::Linear; FilterMode::Nearest];

        let texture = ctx.new_render_texture(TextureParams {
            width: self.width,
            height: self.height,
            format: TextureFormat::RGBA8,
            mag_filter: interp,
            min_filter: interp,
            ..Default::default()
        });

        // Define bindings for the vertex, the index buffers and the texture.
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        // Create a new shader.
        let shader = ctx
            .new_shader(
                ShaderSource::Glsl { vertex: shader::VERTEX, fragment: shader::FRAGMENT },
                shader::meta(),
            )
            .unwrap();

        // Create a new pipeline.
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
impl MiniquadEventHandlerExt for MiniquadPixelBuf {
    fn init(self) -> Self { self.init() }
    fn interpolation(&self) -> bool { self.interpolation }
    fn set_interpolation(&mut self, set: bool) {
        self.interpolation = set;
        if let Some(ctx) = &mut self.ctx {
            if let Some(texture) = self.texture {
                let f = iif![set; FilterMode::Linear; FilterMode::Nearest];
                ctx.texture_set_filter(texture, f,  MipmapFilterMode::None);
            }
        }
    }
    fn maintain_aspect_ratio(&self) -> bool { self.maintain_aspect_ratio }
    fn set_maintain_aspect_ratio(&mut self, set: bool) { self.maintain_aspect_ratio = set; }
}
impl MiniquadEventHandler for MiniquadPixelBuf {
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

mod shader {
    use crate::{vec_ as vec, ToString};
    use ::miniquad::{ShaderMeta, UniformBlockLayout};

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

    // Returns the shader metadata, such as the names of the images and uniforms it uses.
    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
