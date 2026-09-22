//
//! Defines [`CanvasRaster`], [`CanvasRasterExt`].
//

use crate::{Canvas, Position2, RasterGrid, RasterLineIter, RegionS2, is, pos};

#[doc = crate::_tags!(image)]
/// A [`Canvas`] whose spatial units are canonical raster cells.
#[doc = crate::_doc_meta!{
    location("media/visual/draw", trait CanvasRaster),
}]
/// Implementing `CanvasRaster` declares that the canvas extent defines
/// a [`RasterGrid`] with `u32` cell coordinates.
///
/// Raster cells use an upper-left origin, with positive `x` extending rightward
/// and positive `y` extending downward.
///
/// This trait describes raster geometry only. It does not imply pixel storage,
/// byte layout, color encoding, readback, compositing, or presentation.
///
/// Rasterization algorithms may use signed `i64` lattice positions before
/// clipping and projecting them into the raster grid.
pub trait CanvasRaster: Canvas<Unit = u32> {}

#[doc = crate::_tags!(image)]
/// Extension methods for a canonical raster [`CanvasRaster`].
#[doc = crate::_doc_meta!{
    location("media/visual/draw", trait CanvasRasterExt),
}]
/// This trait is implemented for every [`CanvasRaster`].
///
/// Its methods derive canonical software-raster operations from the minimal [`Canvas`] contract,
/// without requiring or exposing any particular raster storage representation.
pub trait CanvasRasterExt: CanvasRaster {
    /// Returns the canonical raster grid covering this canvas.
    fn canvas_raster_grid(&self) -> RasterGrid {
        RasterGrid::new(self.canvas_extent())
    }

    /// Draws a canonical aliased one-cell-wide raster line.
    ///
    /// `start` and `end` are signed raster-lattice positions and may lie outside
    /// the canvas. The line is clipped to the canvas grid using [`RasterLineIter`].
    fn canvas_draw_line(
        &mut self,
        start: Position2<i64>,
        end: Position2<i64>,
        color: Self::Color,
    ) -> Result<(), Self::Error>
    where
        Self::Color: Copy,
    {
        let mut iter = RasterLineIter::new(self.canvas_raster_grid(), start, end);
        while let Some(element) = iter.next() {
            self.canvas_set_color(element.coord(), color)?;
        }
        Ok(())
    }

    /// Draws the outline of a raster region.
    fn canvas_draw_region(
        &mut self,
        region: RegionS2<u32>,
        color: Self::Color,
    ) -> Result<(), Self::Error>
    where
        Self::Color: Copy,
    {
        let (x, y) = (region.x() as i64, region.y() as i64);
        let (w, h) = (region.w() as i64, region.h() as i64);
        is! { w == 0 || h == 0, return Ok(()) }
        let (right, bottom) = (x + w - 1, y + h - 1);
        is! { h == 1, return self.canvas_draw_line(pos![x, y], pos![right, y], color) }
        is! { w == 1, return self.canvas_draw_line(pos![x, y], pos![x, bottom], color) }
        self.canvas_draw_line(pos![x, y], pos![right, y], color)?;
        self.canvas_draw_line(pos![right, y], pos![right, bottom], color)?;
        self.canvas_draw_line(pos![right, bottom], pos![x, bottom], color)?;
        self.canvas_draw_line(pos![x, bottom], pos![x, y], color)
    }
}

impl<C: CanvasRaster + ?Sized> CanvasRasterExt for C {}
