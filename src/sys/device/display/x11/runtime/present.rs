// devela/src/sys/device/display/x11/runtime/present.rs
//
//! Defines [`XPresent`], (`XCopyLayout`, `XPresenter`) [`XRasterRenderer`].
//

use crate::{Boundary1d, RasterViewBytes, is};
use crate::{Event, RunFrame, RunPresent, RunRender};
use crate::{
    XDisplay, XError, XFrameCtx, XImageMode, XImageStore, XSurface, XSurfaceFrame, XWindow,
};

#[doc = crate::_tags!(unix runtime)]
/// Borrowed byte-backed presentation artifact for X11.
#[doc = crate::_doc_meta!{
    location("sys/device/display/x11"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: XPresent<'_> = 24|192),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: XPresent<'_> = 40|320),
}]
/// This carries an already encoded pixel raster together with the storage
/// metadata needed to adapt its scanlines to the active X11 image layout.
///
/// It does not perform color-model, channel-order, transfer, alpha,
/// or endianness conversion. The source pixels must already use
/// an encoding compatible with the active X11 visual.
#[derive(Debug)]
pub struct XPresent<'a> {
    width: u16,
    height: u16,

    /// Logical pixel depth.
    depth: u8,
    /// Stored source bytes occupied by one pixel.
    bytes_per_pixel: usize,
    /// Stored source bytes between consecutive row starts.
    bytes_per_line: usize,
    /// Logical boundary represented by the first stored row.
    row_start: Boundary1d,

    bytes: &'a [u8],
    clear_redraw: bool,
}
#[rustfmt::skip]
impl<'a> XPresent<'a> {
    const fn _new(width: u16, height: u16, depth: u8, bytes_per_pixel: usize, bytes_per_line: usize,
        row_start: Boundary1d, bytes: &'a [u8], clear_redraw: bool) -> Self {
        Self {
            width, height, depth, bytes_per_pixel, bytes_per_line, row_start, bytes, clear_redraw
        }
    }
    /// Returns whether each source row contains no trailing padding.
    pub const fn is_tight_rows(&self) -> bool {
        matches!(
            (self.width as usize).checked_mul(self.bytes_per_pixel),
            Some(row_bytes) if row_bytes == self.bytes_per_line
        )
    }
}

#[doc = crate::_tags!(unix runtime)]
/// Validated plan for copying raster rows into an X11 surface.
#[doc = crate::_doc_meta!{location("sys/device/display/x11")}]
///
/// This separates meaningful pixel bytes from source and destination row
/// padding. Construction verifies compatible depth and stored pixel width,
/// valid strides, representable lengths, and sufficient source storage.
///
/// Row orientation is kept by [`XPresent`] and applied while copying.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct XCopyLayout {
    row_bytes: usize,
    src_bytes_per_line: usize,
    dst_bytes_per_line: usize,
    height: usize,
    dst_required: usize,
}
impl XCopyLayout {
    fn new(
        input: &XPresent<'_>,
        dst_depth: u8,
        dst_bits_per_pixel: u8,
        dst_bytes_per_line: usize,
    ) -> Result<Self, XError> {
        if input.depth != dst_depth {
            return Err(XError::Other("raster depth does not match the X11 display depth"));
        }
        if dst_bits_per_pixel == 0 || dst_bits_per_pixel % 8 != 0 {
            return Err(XError::Other("X11 display pixel storage is not byte-aligned"));
        }
        let dst_bytes_per_pixel = usize::from(dst_bits_per_pixel / 8);
        if input.bytes_per_pixel != dst_bytes_per_pixel {
            return Err(XError::Other(
                "raster stored pixel width does not match the X11 display format",
            ));
        }
        let width = usize::from(input.width);
        let height = usize::from(input.height);
        let row_bytes = width
            .checked_mul(dst_bytes_per_pixel)
            .ok_or(XError::Other("X11 raster row byte length overflow"))?;
        if input.bytes_per_line < row_bytes {
            return Err(XError::Other("raster row stride is shorter than its stored pixel row"));
        }
        if dst_bytes_per_line < row_bytes {
            return Err(XError::Other("X11 row stride is shorter than its stored pixel row"));
        }
        // Like RasterLayout::min_len_bytes(): final-row padding is optional.
        let src_required = if width == 0 || height == 0 {
            0
        } else {
            (height - 1)
                .checked_mul(input.bytes_per_line)
                .and_then(|n| n.checked_add(row_bytes))
                .ok_or(XError::Other("raster byte length overflow"))?
        };
        if input.bytes.len() < src_required {
            return Err(XError::Other("raster byte storage is too short"));
        }
        let dst_required = dst_bytes_per_line
            .checked_mul(height)
            .ok_or(XError::Other("X11 surface byte length overflow"))?;
        Ok(Self {
            row_bytes,
            src_bytes_per_line: input.bytes_per_line,
            dst_bytes_per_line,
            height,
            dst_required,
        })
    }
}

#[doc = crate::_tags!(unix runtime)]
/// A minimal X11 presenter for byte-backed image artifacts.
#[doc = crate::_doc_meta!{location("sys/device/display/x11")}]
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
                #[cfg(ffi_xcb_shm··)]
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
        row_bytes: usize,
        height: usize,
        row_start: Boundary1d,
    ) {
        for dst_y in 0..height {
            let src_y = match row_start {
                Boundary1d::Upper => dst_y,
                Boundary1d::Lower => height - 1 - dst_y,
            };
            let dst_start = dst_y * dst_bytes_per_line;
            let src_start = src_y * src_bytes_per_line;
            let dst_row = &mut dst[dst_start..dst_start + row_bytes];
            let src_row = &src[src_start..src_start + row_bytes];
            dst_row.copy_from_slice(src_row);
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
        let bits_per_pixel = display.bits_per_pixel();
        Ok(XSurfaceFrame::_new(surface, bytes_per_line, bits_per_pixel))
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
        let dst_bytes_per_line = usize::try_from(ctx.display.bytes_per_line(input.width))
            .map_err(|_| XError::Other("X11 row stride exceeds usize"))?;
        let layout = XCopyLayout::new(
            &input,
            ctx.display.depth(),
            ctx.display.bits_per_pixel(),
            dst_bytes_per_line,
        )?;
        let surface = self.ensure_surface(ctx.display, input.width, input.height, input.depth)?;
        {
            let dst = surface.bytes_mut();
            if dst.len() < layout.dst_required {
                return Err(XError::Other("X11 surface storage is too short"));
            }
            let dst = &mut dst[..layout.dst_required];
            // Whole-buffer fast path. It is valid only when source rows are
            // upper-first and both layouts have identical strides.
            if input.row_start == Boundary1d::Upper
                && layout.src_bytes_per_line == layout.dst_bytes_per_line
                && input.bytes.len() >= layout.dst_required
            {
                dst.copy_from_slice(&input.bytes[..layout.dst_required]);
            } else {
                Self::copy_rows(
                    dst,
                    layout.dst_bytes_per_line,
                    input.bytes,
                    layout.src_bytes_per_line,
                    layout.row_bytes,
                    layout.height,
                    input.row_start,
                );
            }
        }
        surface.present(ctx.display, ctx.window)?;
        is! { input.clear_redraw, ctx.window.clear_redraw(ctx.display) }
        ctx.display.flush();
        Ok(())
    }
}

#[doc = crate::_tags!(unix runtime)]
/// Projects a byte-backed X11 image scene into a borrowed presentation artifact.
#[doc = crate::_doc_meta!{location("sys/device/display/x11")}]
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

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn xpresent_tightness_uses_stored_width() {
        let bytes = [0_u8; 24];

        let present = XPresent::_new(3, 2, 24, 4, 12, Boundary1d::Upper, &bytes, true);

        assert!(present.is_tight_rows());
    }

    #[test]
    fn copy_rows_reverses_lower_first_storage() {
        // Two 2-byte rows with two bytes of source padding per row.
        //
        // Stored first: lower logical row.
        // Stored next:  upper logical row.
        let src = [1, 2, 0, 0, 3, 4, 0, 0];
        let mut dst = [0_u8; 4];

        XPresenter::copy_rows(&mut dst, 2, &src, 4, 2, 2, Boundary1d::Lower);

        assert_eq!(dst, [3, 4, 1, 2,]);
    }

    #[test]
    fn copy_layout_rejects_stored_width_mismatch() {
        let bytes = [0_u8; 12];

        let input = XPresent::_new(
            2,
            2,
            24,
            3, // RGB8 storage
            6,
            Boundary1d::Upper,
            &bytes,
            true,
        );

        // Typical X11 root format: depth 24, stored bpp 32.
        assert!(XCopyLayout::new(&input, 24, 32, 8).is_err());
    }
}
