// devela/src/sys/os/term/grid/impls/text.rs
//
//! Text writing operations for [`TermGrid`].
//

use crate::{Position2, TermGrid, TermPen, Termel};

/// # Text drawing
#[rustfmt::skip]
impl<S: Copy, C: Copy, D> TermGrid<Termel<char, S, C>, D>
where D: AsRef<[Termel<char, S, C>]> + AsMut<[Termel<char, S, C>]> {
    /// Writes width-one Unicode scalars horizontally at `position`.
    ///
    /// Writing stops at the right grid edge. No wrapping, line-break handling,
    /// grapheme segmentation, or terminal-width calculation is performed.
    ///
    /// Returns the number of cells written.
    pub fn write_str_at(&mut self, position: Position2<usize>, text: &str, pen: TermPen<S, C>)
        -> usize {
        let [x, y] = position.dim;
        if x >= self.width() || y >= self.height() { return 0; }
        let row = self.row_mut(y).unwrap();
        let available = row.len() - x;
        let mut written = 0;
        for ch in text.chars().take(available) {
            if matches!(ch, '\r' | '\n') { break; }
            row[x + written] = pen.termel(ch);
            written += 1;
        }
        written
    }

    /// Writes width-one Unicode scalars horizontally at `(x, y)`.
    ///
    /// Returns the number of cells written.
    pub fn write_str_xy(&mut self, x: usize, y: usize, text: &str, pen: TermPen<S, C>) -> usize {
        self.write_str_at(Position2::new([x, y]), text, pen)
    }
}
