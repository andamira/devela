// devela/src/sys/os/term/ui/cell.rs
//
//! Defines [`TermCellUi`].
//

use crate::{RegionS2, TermColor, TermColors, TermGrid, Termel, is};
use crate::{UiCellMetric, UiDrawKind, UiDrawListView, UiRound};

#[doc = crate::_tags!(term ui)]
/// A cell-form UI adapter for terminal grids.
#[doc = crate::_doc_meta! {
    location("sys/os/term"),
    test_size_of(TermCellUi = 8|64),
}]
///
/// Projects backend-neutral UI drawing records into a [`TermGrid`] of
/// [`Termel<char>`] values.
///
/// Logical UI geometry is converted into discrete terminal cells using the
/// configured [`UiCellMetric`].
///
/// The style resolver returns a terminal-cell template. Rectangle operations
/// use its character and appearance, while text operations replace its
/// character with the corresponding text scalar.
///
/// Transparent foreground and background colors preserve the previously drawn
/// cell colors. Other unresolved composition modes remain available for later
/// processing.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TermCellUi {
    metric: UiCellMetric,
}

#[rustfmt::skip]
impl TermCellUi {
    /// Constructs a terminal-cell UI adapter.
    pub const fn new(metric: UiCellMetric) -> Self {
        Self { metric }
    }
    /// Returns the logical cell metric.
    pub const fn metric(&self) -> UiCellMetric { self.metric }
}

impl TermCellUi {
    /// Renders a painter-ordered UI draw list into `grid`.
    pub fn render_with<S, T, D>(
        &self,
        draws: &UiDrawListView<'_, S, T>,
        grid: &mut TermGrid<Termel<char>, D>,
        mut resolve: impl FnMut(&S) -> Termel<char>,
    ) where
        T: AsRef<str>,
        D: AsRef<[Termel<char>]> + AsMut<[Termel<char>]>,
    {
        for draw in draws.iter() {
            match draw.kind() {
                UiDrawKind::RectFill { style } => {
                    let rect = self.metric.rect_to_cells(draw.rect(), UiRound::Outward);
                    render_fill(grid, rect, resolve(style));
                }
                UiDrawKind::RectStroke { width, style } => {
                    let rect = self.metric.rect_to_cells(draw.rect(), UiRound::Outward);
                    let cols = self.metric.x_to_col(*width, UiRound::Ceil);
                    let rows = self.metric.y_to_row(*width, UiRound::Ceil);
                    render_inner_stroke(grid, rect, cols, rows, resolve(style));
                }
                UiDrawKind::Text { text, style } => {
                    let rect = self.metric.rect_to_cells(draw.rect(), UiRound::Nearest);
                    render_text(grid, rect, text.as_ref(), resolve(style));
                }
            }
        }
    }
}

/* private helpers */

type CellRect = RegionS2<i32>;

fn render_fill<D>(grid: &mut TermGrid<Termel<char>, D>, rect: CellRect, cell: Termel<char>)
where
    D: AsRef<[Termel<char>]> + AsMut<[Termel<char>]>,
{
    paint_region(grid, rect, cell);
}

/// Renders a stroke entirely inside `rect`.
///
/// Horizontal and vertical thicknesses may differ because terminal cells need
/// not represent square logical regions.
fn render_inner_stroke<D>(
    grid: &mut TermGrid<Termel<char>, D>,
    rect: CellRect,
    cols: i32,
    rows: i32,
    cell: Termel<char>,
) where
    D: AsRef<[Termel<char>]> + AsMut<[Termel<char>]>,
{
    let (x, y, w, h) = (rect.x(), rect.y(), rect.w(), rect.h());
    is! { cols <= 0 || rows <= 0 || w <= 0 || h <= 0, return }
    let double_cols = cols.saturating_mul(2);
    let double_rows = rows.saturating_mul(2);
    if double_cols >= w || double_rows >= h {
        paint_region(grid, rect, cell);
        return;
    }
    let (middle_y, middle_h) = (y.saturating_add(rows), h.saturating_sub(double_rows));
    let bottom_y = y.saturating_add(h.saturating_sub(rows));
    let right_x = x.saturating_add(w.saturating_sub(cols));
    // Horizontal strips
    paint_region(grid, CellRect::from_xy_wh(x, y, w, rows), cell);
    paint_region(grid, CellRect::from_xy_wh(x, bottom_y, w, rows), cell);
    // Vertical strips, excluding already-painted corners
    paint_region(grid, CellRect::from_xy_wh(x, middle_y, cols, middle_h), cell);
    paint_region(grid, CellRect::from_xy_wh(right_x, middle_y, cols, middle_h), cell);
}

/// Renders one unwrapped width-one text run.
///
/// Text is clipped to the rectangle and grid. Line-break characters terminate
/// the run.
fn render_text<D>(
    grid: &mut TermGrid<Termel<char>, D>,
    rect: CellRect,
    text: &str,
    template: Termel<char>,
) where
    D: AsRef<[Termel<char>]> + AsMut<[Termel<char>]>,
{
    is! { rect.w() <= 0 || rect.h() <= 0, return }
    let y = rect.y();
    is! { y < 0 || y as usize >= grid.height(), return }
    let max_cols = rect.w() as usize;
    for (column, ch) in text.chars().enumerate() {
        is! { column >= max_cols || matches!(ch, '\r' | '\n'), break }
        let x = rect.x().saturating_add(column as i32);
        is! { x < 0 || x as usize >= grid.width(), continue }
        let (x, y) = (x as usize, y as usize);
        let under = grid.get_xy_copy(x, y).unwrap();
        let over = template.with_value(ch);
        let _ = grid.set_xy(x, y, compose_cell(under, over));
    }
}

/// Paints `cell` over every visible position in `rect`.
fn paint_region<D>(grid: &mut TermGrid<Termel<char>, D>, rect: CellRect, cell: Termel<char>)
where
    D: AsRef<[Termel<char>]> + AsMut<[Termel<char>]>,
{
    let Some(region) = clip_region(rect, grid.width(), grid.height()) else {
        return;
    };
    // Fully opaque cells need no read-modify-write composition.
    if cell.fg().is_opaque() && cell.bg().is_opaque() {
        grid.fill_region(region, cell);
        return;
    }
    let (x0, y0) = (region.x(), region.y());
    let (x1, y1) = (x0 + region.w(), y0 + region.h());
    for y in y0..y1 {
        for x in x0..x1 {
            let under = grid.get_xy_copy(x, y).unwrap();
            let _ = grid.set_xy(x, y, compose_cell(under, cell));
        }
    }
}

/// Composes one terminal cell over another.
///
/// Transparent colors preserve their corresponding underlying color.
/// Character and style come from the upper cell.
fn compose_cell(under: Termel<char>, over: Termel<char>) -> Termel<char> {
    let colors =
        TermColors::new(compose_color(under.fg(), over.fg()), compose_color(under.bg(), over.bg()));
    Termel::from_value(*over.value(), *over.style(), colors)
}
const fn compose_color(under: TermColor, over: TermColor) -> TermColor {
    if over.is_transparent() { under } else { over }
}

/// Clips a signed cell rectangle to a grid and converts it to unsigned cells.
fn clip_region(rect: CellRect, width: usize, height: usize) -> Option<RegionS2<usize>> {
    is! { rect.w() <= 0 || rect.h() <= 0, return None }
    let x0 = rect.x().max(0) as usize;
    let y0 = rect.y().max(0) as usize;
    let x1 = rect.x().saturating_add(rect.w()).max(0) as usize;
    let y1 = rect.y().saturating_add(rect.h()).max(0) as usize;
    let x1 = x1.min(width);
    let y1 = y1.min(height);
    is! { x0 >= x1 || y0 >= y1, return None }
    Some(RegionS2::from_xy_wh(x0, y0, x1 - x0, y1 - y0))
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{Extent2, Lunit, TermColorMode, TermStyle, UiDraw, UiOutputView, UiRect};

    type Cell = Termel<char>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Style {
        Fill,
        Stroke,
        Text,
    }

    const FILL_BG: TermColor = TermColor::rgb([10, 20, 30]);
    const TEXT_FG: TermColor = TermColor::rgb([220, 230, 240]);
    const TRANSPARENT: TermColor = TermColor::default_with_mode(TermColorMode::Transparent);

    const fn rect(x: i32, y: i32, width: i32, height: i32) -> UiRect {
        UiRect::from_xy_wh(Lunit::px(x), Lunit::px(y), Lunit::px(width), Lunit::px(height))
    }
    const fn resolve(style: Style) -> Cell {
        match style {
            Style::Fill => Termel::from_value(
                'F',
                TermStyle::new(),
                TermColors::new(TermColor::DEFAULT, FILL_BG),
            ),
            Style::Stroke => Termel::from_value('S', TermStyle::new(), TermColors::DEFAULT),
            Style::Text => {
                Termel::from_value(' ', TermStyle::new(), TermColors::new(TEXT_FG, TRANSPARENT))
            }
        }
    }

    #[test]
    fn projects_non_square_cell_metrics() {
        const W: usize = 5;
        const H: usize = 4;
        let mut grid =
            TermGrid::new([Cell::plain_const('.'); W * H], Extent2::new([W, H])).unwrap();
        let ui = TermCellUi::new(UiCellMetric::from_px(2, 3).unwrap());
        let draws: [UiDraw<Style>; 1] = [UiDraw::rect_fill(rect(2, 3, 4, 6), Style::Fill)];
        let output = UiOutputView::from_slices(&[], &[], &draws);
        ui.render_with(output.draw_list(), &mut grid, |style| resolve(*style));
        // Logical (2, 3, 4, 6) becomes cells (1, 1, 2, 2).
        assert_eq!(*grid.get_xy(1, 1).unwrap().value(), 'F',);
        assert_eq!(*grid.get_xy(2, 2).unwrap().value(), 'F',);
        assert_eq!(*grid.get_xy(0, 0).unwrap().value(), '.',);
        assert_eq!(*grid.get_xy(3, 2).unwrap().value(), '.',);
    }
    #[test]
    fn stroke_remains_inside_its_rectangle() {
        const W: usize = 6;
        const H: usize = 6;
        let mut grid =
            TermGrid::new([Cell::plain_const('.'); W * H], Extent2::new([W, H])).unwrap();
        let ui = TermCellUi::new(UiCellMetric::ONE);
        let draws: [UiDraw<Style>; 1] =
            [UiDraw::rect_stroke(rect(1, 1, 4, 4), Lunit::px(1), Style::Stroke)];
        let output = UiOutputView::from_slices(&[], &[], &draws);
        ui.render_with(output.draw_list(), &mut grid, |style| resolve(*style));
        for y in 0..H {
            for x in 0..W {
                let value = *grid.get_xy(x, y).unwrap().value();
                let on_edge = ((x == 1 || x == 4) && (1..=4).contains(&y))
                    || ((y == 1 || y == 4) && (1..=4).contains(&x));
                assert_eq!(value, if on_edge { 'S' } else { '.' }, "unexpected cell at ({x}, {y})",);
            }
        }
        // The inner region remains untouched.
        assert_eq!(*grid.get_xy(2, 2).unwrap().value(), '.',);
    }
    #[test]
    fn transparent_text_preserves_the_background() {
        const W: usize = 4;
        const H: usize = 2;
        let mut grid =
            TermGrid::new([Cell::plain_const('.'); W * H], Extent2::new([W, H])).unwrap();
        let ui = TermCellUi::new(UiCellMetric::ONE);
        let draws: [UiDraw<Style>; 2] = [
            UiDraw::rect_fill(rect(0, 0, 4, 2), Style::Fill),
            UiDraw::text(rect(1, 0, 2, 1), "A", Style::Text),
        ];
        let output = UiOutputView::from_slices(&[], &[], &draws);
        ui.render_with(output.draw_list(), &mut grid, |style| resolve(*style));
        let cell = grid.get_xy_copy(1, 0).unwrap();
        assert_eq!(*cell.value(), 'A');
        assert_eq!(cell.fg(), TEXT_FG);
        assert_eq!(cell.bg(), FILL_BG);
        // Composition has produced terminal-renderable opaque colors.
        assert!(cell.fg().is_opaque());
        assert!(cell.bg().is_opaque());
    }
    #[test]
    fn text_is_clipped_to_rect_and_grid() {
        const W: usize = 4;
        const H: usize = 2;
        let mut grid =
            TermGrid::new([Cell::plain_const('.'); W * H], Extent2::new([W, H])).unwrap();
        let ui = TermCellUi::new(UiCellMetric::ONE);
        let draws: [UiDraw<Style>; 2] = [
            // Rectangle clipping permits only A and B
            UiDraw::text(rect(1, 0, 2, 1), "ABCD", Style::Text),
            // Grid clipping permits only X
            UiDraw::text(rect(3, 1, 3, 1), "XYZ", Style::Text),
        ];
        let output = UiOutputView::from_slices(&[], &[], &draws);
        ui.render_with(output.draw_list(), &mut grid, |style| resolve(*style));
        assert_eq!(
            [
                *grid.get_xy(0, 0).unwrap().value(),
                *grid.get_xy(1, 0).unwrap().value(),
                *grid.get_xy(2, 0).unwrap().value(),
                *grid.get_xy(3, 0).unwrap().value(),
            ],
            ['.', 'A', 'B', '.'],
        );
        assert_eq!(
            [
                *grid.get_xy(0, 1).unwrap().value(),
                *grid.get_xy(1, 1).unwrap().value(),
                *grid.get_xy(2, 1).unwrap().value(),
                *grid.get_xy(3, 1).unwrap().value(),
            ],
            ['.', '.', '.', 'X'],
        );
    }
}
