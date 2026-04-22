// devela::sys::device::display::x11::present
//
//! Defines [`XPresent`], [`XPresenter`] [`XRasterRender`].
//

use crate::RasterView;
use crate::is;
use crate::{Event, RunFrame, RunPresent, RunRender};
use crate::{XError, XFrameCtx};

#[doc = crate::_tags!(unix runtime)]
/// Borrowed pixel presentation artifact for X11.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// This is a simple artifact consumed by [`XPresenter`] to upload a `u32`
/// pixel buffer to the target window and optionally clear its redraw flag.
pub struct XPresent<'a> {
    width: u16,
    height: u16,
    depth: u8,
    pixels: &'a [u32],
    clear_redraw: bool,
}

#[doc = crate::_tags!(unix runtime)]
/// A minimal X11 presenter for pixel-buffer artifacts.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// `XPresenter` consumes [`XPresent`] artifacts
/// and applies them to the current X11 frame context.
#[derive(Debug)]
pub struct XPresenter;
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
        ctx.window.put_image_u32(input.width, input.height, input.depth, input.pixels);
        is! { input.clear_redraw, ctx.window.clear_redraw(ctx.display) }
        ctx.display.flush();
        Ok(())
    }
}

///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct XRasterRenderer {
    ///
    pub depth: u8,
    ///
    pub clear_redraw: bool,
}
impl XRasterRenderer {
    ///
    pub const fn new(depth: u8, clear_redraw: bool) -> Self {
        Self { depth, clear_redraw }
    }
}
impl Default for XRasterRenderer {
    fn default() -> Self {
        Self::new(24, true)
    }
}
impl<S: ?Sized, E, C> RunRender<S, E, C> for XRasterRenderer
where
    S: RasterView<Sample = u32>,
{
    type Output<'a>
        = XPresent<'a>
    where
        Self: 'a,
        S: 'a,
        E: 'a;

    type Error = XError;

    fn run_render<'a>(
        &'a mut self,
        _frame: &mut RunFrame<'a, E, C>,
        scene: &'a S,
    ) -> Result<Self::Output<'a>, Self::Error> {
        let [width, height] = scene.raster_extent().dim;
        Ok(XPresent {
            width: width.try_into().unwrap(), // IMPROVE error path
            height: height.try_into().unwrap(),
            depth: self.depth,
            pixels: scene.raster_samples(),
            clear_redraw: self.clear_redraw,
        })
    }
}
