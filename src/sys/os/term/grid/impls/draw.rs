// devela::sys::os::term::grid::impls::draw
//
//! Grid-region drawing and transfer operations.
//

use crate::{Cmp, Position2, RegionS2, TermGrid, whilst};

#[inline] #[rustfmt::skip]
const fn clip_region(region: RegionS2<usize>, width: usize, height: usize)
    -> Option<(usize, usize, usize, usize)> {
    let [x, y] = region.pos.dim;
    let [w, h] = region.ext.dim;
    if x >= width || y >= height || w == 0 || h == 0 { return None; }
    let x1 = Cmp(x.saturating_add(w)).min(width);
    let y1 = Cmp(y.saturating_add(h)).min(height);
    Some((x, y, x1, y1))
}

/// # Region drawing
#[rustfmt::skip]
impl<E: Copy, S: AsRef<[E]> + AsMut<[E]>> TermGrid<E, S> {
    /// Fills the part of `region` lying within the grid.
    ///
    /// Regions extending beyond the grid are clipped.
    pub fn fill_region(&mut self, region: RegionS2<usize>, element: impl Into<E>) {
        let Some((x0, y0, x1, y1)) = clip_region(region, self.width(), self.height())
            else { return; };
        let element = element.into();
        whilst! { y in y0,..y1; {
            self.row_mut(y).unwrap()[x0..x1].fill(element);
        }}
    }
    /// Fills a clipped horizontal span.
    pub fn hline(&mut self, x: usize, y: usize, len: usize, element: impl Into<E>) {
        self.fill_region(RegionS2::from(((x, y), (len, 1))), element);
    }
    /// Fills a clipped vertical span.
    pub fn vline(&mut self, x: usize, y: usize, len: usize, element: impl Into<E>) {
        self.fill_region(RegionS2::from(((x, y), (1, len))), element);
    }
    /// Draws a clipped one-cell frame around `region`.
    pub fn frame(&mut self, region: RegionS2<usize>, element: impl Into<E>) {
        let [x, y] = region.pos.dim;
        let [w, h] = region.ext.dim;
        if w == 0 || h == 0 { return; }
        let element = element.into();
        self.hline(x, y, w, element);
        if h > 1 { self.hline(x, y.saturating_add(h - 1), w, element); }
        if h > 2 {
            let inner_y = y.saturating_add(1);
            self.vline(x, inner_y, h - 2, element);
            if w > 1 { self.vline( x.saturating_add(w - 1), inner_y, h - 2, element); }
        }
    }
}

/// # Grid transfer
#[rustfmt::skip]
impl<E: Copy, D> TermGrid<E, D> where D: AsRef<[E]> + AsMut<[E]> {
    /// Copies `source` into this grid at `destination`.
    ///
    /// Cells extending beyond the destination grid are clipped.
    pub fn blit_at<S>(&mut self, source: &TermGrid<E, S>, destination: Position2<usize>)
    where S: AsRef<[E]> {
        self.blit_region_at(source,
            RegionS2::from(((0, 0), (source.width(), source.height()))), destination);
    }
    /// Copies a source region into this grid at `destination`.
    ///
    /// Both the source region and destination grid are clipped.
    pub fn blit_region_at<S>(&mut self, source: &TermGrid<E, S>,
        source_region: RegionS2<usize>, destination: Position2<usize>) where S: AsRef<[E]> {
        let [src_x, src_y] = source_region.pos.dim;
        let [req_w, req_h] = source_region.ext.dim;
        let [dst_x, dst_y] = destination.dim;
        if src_x >= source.width() || src_y >= source.height()
            || dst_x >= self.width() || dst_y >= self.height() { return; }
        let width = req_w.min(source.width() - src_x).min(self.width() - dst_x);
        let height = req_h.min(source.height() - src_y).min(self.height() - dst_y);
        if width == 0 || height == 0 { return; }
        whilst! { y in 0..height; {
            let src = &source.row(src_y + y).unwrap()[src_x..src_x + width];
            let dst = &mut self.row_mut(dst_y + y).unwrap()[dst_x..dst_x + width];
            dst.copy_from_slice(src);
        }}
    }
}
