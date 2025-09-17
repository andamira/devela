// devela::ui::front::term::size
//
//! Defines [`TermSize`].
//

#[doc = crate::_TAG_FFI!()]
/// The size of the terminal.
///
/// ## Used by
/// - `LinuxTermios`.
#[must_use]
#[repr(C)] // field order matters!
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TermSize {
    /// Rows of cells.
    pub rows: u16,
    /// Columns of cells.
    pub cols: u16,
    /// Horizontal pixels.
    pub x: u16,
    /// Vertical pixels.
    pub y: u16,
}

impl TermSize {
    /// Returns a new `TermSize`.
    pub const fn new(rows_cols: (u16, u16), xy: (u16, u16)) -> Self {
        TermSize {
            rows: rows_cols.0,
            cols: rows_cols.1,
            x: xy.0,
            y: xy.1,
        }
    }

    /// Returns the number of `(horizontal, vertical)` pixels.
    #[must_use]
    pub const fn pixels(self) -> (u16, u16) {
        (self.x, self.y)
    }
    /// Returns the number of `(columns, rows)` of cells.
    #[must_use]
    pub const fn cells(self) -> (u16, u16) {
        (self.cols, self.rows)
    }
    /// Returns the number of pixels per cell `(horizontal, vertical)`.
    #[must_use]
    pub const fn pixels_per_cell(self) -> (u16, u16) {
        (self.x / self.cols, self.y / self.rows)
    }
}
