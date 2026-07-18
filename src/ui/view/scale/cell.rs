// devela/ui/view/scale/cell.rs
//
//! Defines [`UiCellMetric`].
//

use crate::UiRound::{self, Ceil, Floor, Inward, Nearest, Outward};
use crate::{Cmp, Lunit, RegionS2, UiNum, UiRect};

#[doc = crate::_tags!(ui layout quant)]
/// Cell size in logical UI layout space.
#[doc = crate::_doc_meta! {
    location("ui/view/scale"),
    test_size_of(UiCellMetric = 8|64),
}]
/// Defines the logical width and height represented by one discrete output cell.
///
/// A cell need not be square. Terminal cells, tile grids, text grids, and other
/// cell-oriented projections may use different horizontal and vertical measures.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiCellMetric {
    width: Lunit,
    height: Lunit,
}
crate::_impl_init![Self::ONE => UiCellMetric];
impl Default for UiCellMetric {
    fn default() -> Self {
        Self::ONE
    }
}
#[rustfmt::skip]
impl UiCellMetric {
    /// One logical pixel per cell in each dimension.
    pub const ONE: Self = Self::new_unchecked(Lunit::px(1), Lunit::px(1));

    const fn new_unchecked(width: Lunit, height: Lunit) -> Self { Self { width, height } }

    /* constructors */

    /// Constructs a metric from positive logical cell dimensions.
    ///
    /// Returns `None` if either dimension is zero or negative.
    pub const fn new(width: Lunit, height: Lunit) -> Option<Self> {
        if width.quanta() <= 0 || height.quanta() <= 0 { None }
        else { Some(Self { width, height }) }
    }
    /// Constructs a metric from whole logical-pixel dimensions.
    ///
    /// Returns `None` if either dimension is zero or negative.
    pub const fn from_px(width: i32, height: i32) -> Option<Self> {
        Self::new(Lunit::px(width), Lunit::px(height))
    }

    /* queries */

    /// Returns the logical width represented by one cell.
    pub const fn width(self) -> Lunit { self.width }

    /// Returns the logical height represented by one cell.
    pub const fn height(self) -> Lunit { self.height }

    /* scalar projection */

    /// Projects a logical horizontal coordinate to a cell column.
    ///
    /// `Inward` and `Outward` are rectangle policies; for scalar projection
    /// they behave like `Nearest`.
    pub const fn x_to_col(self, x: Lunit, round: UiRound) -> i32 {
        UiNum::round_div_scalar_i64_to_i32(x.quanta() as i64, self.width.quanta() as i64, round)
    }

    /// Projects a logical vertical coordinate to a cell row.
    ///
    /// `Inward` and `Outward` are rectangle policies; for scalar projection
    /// they behave like `Nearest`.
    pub const fn y_to_row(self, y: Lunit, round: UiRound) -> i32 {
        UiNum::round_div_scalar_i64_to_i32(y.quanta() as i64, self.height.quanta() as i64, round)
    }
    /// Projects a cell column back into logical horizontal space.
    pub const fn col_to_x(self, col: i32) -> Lunit {
        Lunit::new_saturated_up((col as i64).saturating_mul(self.width.quanta() as i64))
    }
    /// Projects a cell row back into logical vertical space.
    pub const fn row_to_y(self, row: i32) -> Lunit {
        Lunit::new_saturated_up((row as i64).saturating_mul(self.height.quanta() as i64))
    }

    /* rectangle projection */

    /// Projects a logical UI rectangle to a discrete cell rectangle.
    pub const fn rect_to_cells(self, rect: UiRect, round: UiRound) -> RegionS2<i32> {
        match round {
            Outward => {
                let x0 = self.x_to_col(rect.left(), Floor);
                let y0 = self.y_to_row(rect.top(), Floor);
                let x1 = self.x_to_col(rect.right(), Ceil);
                let y1 = self.y_to_row(rect.bottom(), Ceil);
                RegionS2::from_xy_wh(x0, y0,
                    Cmp(x1.saturating_sub(x0)).max(0), Cmp(y1.saturating_sub(y0)).max(0))
            }
            Inward => {
                let x0 = self.x_to_col(rect.left(), Ceil);
                let y0 = self.y_to_row(rect.top(), Ceil);
                let x1 = self.x_to_col(rect.right(), Floor);
                let y1 = self.y_to_row(rect.bottom(), Floor);
                RegionS2::from_xy_wh(x0, y0,
                    Cmp(x1.saturating_sub(x0)).max(0), Cmp(y1.saturating_sub(y0)).max(0))
            }
            Floor | Ceil | Nearest => RegionS2::from_xy_wh(
                self.x_to_col(rect.x(), round),
                self.y_to_row(rect.y(), round),
                Cmp(self.x_to_col(rect.w(), round)).max(0),
                Cmp(self.y_to_row(rect.h(), round)).max(0),
            ),
        }
    }
    /// Projects a discrete cell rectangle back into logical UI space.
    pub const fn cells_to_rect(self, cells: RegionS2<i32>) -> UiRect {
        UiRect::from_xy_wh(
            self.col_to_x(cells.x()),
            self.row_to_y(cells.y()),
            self.col_to_x(cells.w()).max_zero(),
            self.row_to_y(cells.h()).max_zero(),
        )
    }
}
#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn rejects_nonpositive_dimensions() {
        assert!(UiCellMetric::new(Lunit::ZERO, Lunit::px(16)).is_none());
        assert!(UiCellMetric::new(Lunit::px(8), Lunit::ZERO).is_none());
        assert!(UiCellMetric::new(Lunit::px(-8), Lunit::px(16)).is_none());
    }
    #[test]
    fn projects_independent_cell_dimensions() {
        let metric = UiCellMetric::from_px(8, 16).unwrap();
        assert_eq!(metric.x_to_col(Lunit::px(24), UiRound::Nearest), 3,);
        assert_eq!(metric.y_to_row(Lunit::px(48), UiRound::Nearest), 3,);
    }
    #[test]
    fn outward_projection_covers_partial_cells() {
        let metric = UiCellMetric::from_px(8, 16).unwrap();
        let rect = UiRect::from_xy_wh(Lunit::px(4), Lunit::px(8), Lunit::px(8), Lunit::px(16));
        let cells = metric.rect_to_cells(rect, UiRound::Outward);
        assert_eq!(cells, RegionS2::from_xy_wh(0, 0, 2, 2));
    }
    #[test]
    fn cell_rectangle_roundtrips_exactly() {
        let metric = UiCellMetric::from_px(8, 16).unwrap();
        let cells = RegionS2::from_xy_wh(2, 3, 10, 4);
        assert_eq!(metric.rect_to_cells(metric.cells_to_rect(cells), UiRound::Nearest,), cells,);
    }
}
