// devela::sys::device::display::x11::present
//
//! Defines [`XPresent`], [`XPresenter`] [`XRasterRender`].
//

use crate::RasterViewBytes;
use crate::is;
use crate::{Event, RunFrame, RunPresent, RunRender};
use crate::{
    XDisplay, XError, XFrameCtx, XImageMode, XImageStore, XSurface, XSurfaceFrame, XWindow,
};

#[doc = crate::_tags!(unix runtime)]
/// Borrowed byte-backed presentation artifact for X11.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// This is a simple artifact consumed by a private `XPresenter` to upload image
/// bytes to the target window and optionally clear its redraw flag.
#[derive(Debug)]
pub struct XPresent<'a> {
    width: u16,
    height: u16,
    depth: u8,
    bytes: &'a [u8],
    bytes_per_line: u32, // currenly used for validation only
    clear_redraw: bool,
}
#[rustfmt::skip]
impl<'a> XPresent<'a> {
    const fn _new(width: u16, height: u16, depth: u8, bytes: &'a [u8], bytes_per_line: u32,
        clear_redraw: bool) -> Self {
        Self { width, height, depth, bytes, bytes_per_line, clear_redraw }
    }
    /// Returns whether rows are tightly packed for the current depth.
    pub const fn is_tight_rows(&self) -> bool {
        let bits = self.width as u32 * self.depth as u32;
        bits.div_ceil(8) == self.bytes_per_line
    }
}

#[doc = crate::_tags!(unix runtime)]
/// A minimal X11 presenter for byte-backed image artifacts.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// `XPresenter` consumes [`XPresent`] artifacts
/// and applies them to the current X11 frame context.
#[derive(Debug)]
pub(crate) struct XPresenter {
    mode: XImageMode,
    surface: Option<XSurface>,
}
impl XPresenter {
    pub const fn new(mode: XImageMode) -> Self {
        Self { mode, surface: None }
    }
    pub const fn mode(&self) -> XImageMode {
        self.mode
    }
    pub const fn active_mode(&self) -> Option<XImageMode> {
        if let Some(ref s) = self.surface { Some(s.mode()) } else { None }
    }
    fn ensure_surface(
        &mut self,
        display: &XDisplay,
        width: u16,
        height: u16,
        depth: u8,
    ) -> Result<&mut XSurface, XError> {
        let needs_new = match &self.surface {
            None => true,
            Some(s) => s.size() != (width, height) || s.depth() != depth,
        };
        if needs_new {
            let surface = match self.mode {
                XImageMode::Auto => XSurface::new(display, width, height, depth)?,
                XImageMode::Cpu => XSurface::new_cpu(display, width, height, depth)?,
                XImageMode::Shm => XSurface::new_shm(display, width, height, depth)?,
            };
            self.surface = Some(surface);
        }
        Ok(self.surface.as_mut().unwrap())
    }
    fn copy_rows(
        dst: &mut [u8],
        dst_bytes_per_line: usize,
        src: &[u8],
        src_bytes_per_line: usize,
        height: u16,
    ) {
        let h = height as usize;
        for y in 0..h {
            let ds = y * dst_bytes_per_line;
            let ss = y * src_bytes_per_line;
            let de = ds + dst_bytes_per_line.min(src_bytes_per_line);
            let se = ss + dst_bytes_per_line.min(src_bytes_per_line);
            dst[ds..de].copy_from_slice(&src[ss..se]);
        }
    }
    pub(crate) fn surface_frame<'a>(
        &'a mut self,
        display: &XDisplay,
        width: u16,
        height: u16,
        depth: u8,
    ) -> Result<XSurfaceFrame<'a>, XError> {
        let surface = self.ensure_surface(display, width, height, depth)?;
        let bytes_per_line = display.bytes_per_line(width);
        Ok(XSurfaceFrame::_new(surface, bytes_per_line))
    }
    pub(crate) fn present_surface(
        &mut self,
        display: &mut XDisplay,
        window: &mut XWindow,
        clear_redraw: bool,
    ) -> Result<(), XError> {
        let surface = self.surface.as_mut().ok_or(XError::Other("no surface in XPresenter"))?;
        surface.present(display, window)?;
        is! { clear_redraw, window.clear_redraw(display) }
        display.flush();
        Ok(())
    }
}
impl<'ctx> RunPresent<Event, XFrameCtx<'ctx>> for XPresenter {
    type Input<'a>
        = XPresent<'a>
    where
        Self: 'a;
    type Output = ();
    type Error = XError;
    fn run_present<'a>(
        &'a mut self,
        frame: &mut RunFrame<'a, Event, XFrameCtx<'ctx>>,
        input: Self::Input<'a>,
    ) -> Result<Self::Output, Self::Error> {
        let ctx = frame.context_mut();
        let surface = self.ensure_surface(ctx.display, input.width, input.height, input.depth)?;
        let dst_bpl = ctx.display.bytes_per_line(input.width) as usize;
        let src_bpl = input.bytes_per_line as usize;
        let dst = surface.bytes_mut();
        if dst_bpl == src_bpl && dst.len() == input.bytes.len() {
            dst.copy_from_slice(input.bytes); // fast-path
        } else {
            Self::copy_rows(dst, dst_bpl, input.bytes, src_bpl, input.height);
        }
        surface.present(ctx.display, ctx.window)?;
        is! { input.clear_redraw, ctx.window.clear_redraw(ctx.display) }
        ctx.display.flush();
        Ok(())
    }
}

#[doc = crate::_tags!(unix runtime)]
/// Projects a byte-backed X11 image scene into a borrowed presentation artifact.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// It borrows image bytes from the scene and packages them
/// as an [`XPresent`] for an `XPresenter` to upload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct XRasterRenderer {
    /// Whether to clear the window redraw flag after presentation.
    pub clear_redraw: bool,
}
impl XRasterRenderer {
    /// Creates a new X11 byte-image renderer.
    pub const fn new(clear_redraw: bool) -> Self {
        Self { clear_redraw }
    }
}
impl Default for XRasterRenderer {
    fn default() -> Self {
        Self::new(true)
    }
}
#[rustfmt::skip]
impl<S: RasterViewBytes + ?Sized, E, C> RunRender<S, E, C> for XRasterRenderer {
    type Error = XError;
    type Output<'a> = XPresent<'a> where Self: 'a, S: 'a, E: 'a;
    fn run_render<'a>(&'a mut self, _frame: &mut RunFrame<'a, E, C>, scene: &'a S)
        -> Result<Self::Output<'a>, Self::Error> {
        let [width, height] = scene.raster_extent_bytes().dim;
        Ok(XPresent::_new(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            scene.raster_depth(),
            scene.raster_bytes(),
            scene.raster_bytes_per_line() as u32,
            self.clear_redraw,
        ))
    }
}
