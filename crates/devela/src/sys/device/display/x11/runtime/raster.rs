//
//! Defines [`XRasterRenderer`].
//

use crate::{RasterViewBytes, RunFrame, RunRender, XError, XPresent};

#[doc = crate::_tags!(unix runtime)]
/// Projects a byte-backed raster scene into an X11 presentation artifact.
#[doc = crate::_doc_meta!{
    location("sys/device/display/x11", struct XRasterRenderer),
}]
/// The source storage is borrowed and packaged as an [`XPresent`] for the
/// X11 presenter. No color-model or channel-encoding conversion is performed;
/// the source pixels must already be compatible with the active X11 visual.
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

    fn run_render<'a>(
        &'a mut self,
        _frame: &mut RunFrame<'a, E, C>,
        scene: &'a S,
    ) -> Result<Self::Output<'a>, Self::Error> {
        let [width, height] = scene.raster_extent_bytes().dim;
        let width = u16::try_from(width)
            .map_err(|_| XError::Other("raster width exceeds the X11 u16 range"))?;
        let height = u16::try_from(height)
            .map_err(|_| XError::Other("raster height exceeds the X11 u16 range"))?;
        Ok(XPresent::_new(
            width,
            height,
            scene.raster_depth(),
            scene.raster_bytes_per_pixel_bytes(),
            scene.raster_bytes_per_line(),
            scene.raster_row_start_bytes(),
            scene.raster_bytes(),
            self.clear_redraw,
        ))
    }
}
