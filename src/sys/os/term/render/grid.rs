// devela::sys::os::term::render::grid
//
//! Terminal-grid encoding extensions for [`TermRenderer`].
//
// TOC
// - impl grid rendering
// - impl terminal formatting
// - impl character encoding
// - impl helpers
// - TermRenderState
// - tests

use crate::NotEnoughSpace;
use crate::{Ansi, AnsiColor, TermColors};
use crate::{TermGrid, TermGridError, TermRenderer, TermStyle, Termel};
use crate::{charu, is, unwrap, whilst};

type GridResult<T> = Result<T, TermGridError>;

/// # Grid rendering
#[rustfmt::skip]
impl<B: AsRef<[u8]> + AsMut<[u8]>> TermRenderer<B> {
    /// Renders a width-one character grid at the terminal origin.
    ///
    /// Equivalent to [`Self::try_render_grid_at`] at `(0, 0)`.
    ///
    /// # Errors
    /// Returns an error if the byte-frame storage is insufficient,
    /// the grid exceeds the terminal coordinate representation,
    /// or a color requires composition before ANSI encoding.
    ///
    /// Bytes encoded before an error remain in the active frame,
    /// which may end with non-default appearance state.
    #[rustfmt::skip]
    pub fn try_render_grid<S>(&mut self, grid: &TermGrid<Termel<char>, S>)
        -> GridResult<&mut Self> where S: AsRef<[Termel<char>]> {
        self.try_render_grid_at(grid, 0, 0)
    }

    /// Renders a width-one character grid at a zero-based terminal position.
    ///
    /// Each row is positioned explicitly, avoiding implicit wrapping at the right
    /// terminal margin. Adjacent cells reuse unchanged styles and colors. Terminal
    /// style is reset before and after the grid.
    ///
    /// Every character is currently treated as occupying one terminal column.
    ///
    /// # Errors
    /// Returns an error if the byte-frame storage is insufficient,
    /// the positioned grid exceeds the terminal coordinate representation,
    /// or a color requires composition before ANSI encoding.
    ///
    /// Bytes encoded before an error remain in the active frame,
    /// which may end with non-default appearance state.
    pub fn try_render_grid_at<S>(&mut self, grid: &TermGrid<Termel<char>, S>, col: u16, row: u16)
        -> GridResult<&mut Self> where S: AsRef<[Termel<char>]> {
        is! { grid.is_empty(), return Ok(self) }
        let last_col = grid.width().saturating_sub(1);
        let last_row = grid.height().saturating_sub(1);
        const MAX_COORD0: usize = u16::MAX as usize - 1;
        if col as usize + last_col > MAX_COORD0 || row as usize + last_row > MAX_COORD0 {
            return Err(TermGridError::RenderPositionOverflow); // guarantee in-range-rows exists
        }
        let mut state = TermRenderState::default();
        self.try_format_reset()?;
        whilst! { y in 0..grid.height(); {
            self.try_cursor_move_to0(col, row + y as u16)?;
            let row_cells = unwrap![some_guaranteed_or_ub grid.row(y)]; // all in-range-rows exist
            whilst! { x in 0..row_cells.len(); {
                let cell = row_cells[x];
                let (style, colors) = (*cell.style(), *cell.colors());
                if style != state.style {
                    self.try_style_transition(state.style, style)?;
                    state.style = style;
                }
                if colors != state.colors {
                    let (fg, bg) = Self::resolve_ansi_colors(colors)?;
                    self.try_colors(fg, bg)?;
                    state.colors = colors;
                }
                self.try_push_char(*cell.textel().value())?;
            }}
        }}
        self.try_format_reset()?;
        Ok(self)
    }
}
/// # Terminal formatting
#[rustfmt::skip]
impl<B: AsRef<[u8]> + AsMut<[u8]>> TermRenderer<B> {
    /// Enables every style contained in `style`.
    ///
    /// Existing styles not present in `style` remain unchanged.
    pub fn try_style_enable(&mut self, style: TermStyle) -> Result<&mut Self, NotEnoughSpace> {
        is![style.contains(TermStyle::BOLD), self.try_push_bytes(&Ansi::BOLD_B)?;];
        is![style.contains(TermStyle::ITALIC), self.try_push_bytes(&Ansi::ITALIC_B)?;];
        is![style.contains(TermStyle::UNDERLINE), self.try_push_bytes(&Ansi::UNDERLINE_B)?;];
        is![style.contains(TermStyle::DIM), self.try_push_bytes(&Ansi::DIM_B)?;];
        is![style.contains(TermStyle::BLINK), self.try_push_bytes(&Ansi::BLINK_B)?;];
        is![style.contains(TermStyle::INVERSE), self.try_push_bytes(&Ansi::INVERSE_B)?;];
        is![style.contains(TermStyle::HIDDEN), self.try_push_bytes(&Ansi::HIDDEN_B)?;];
        is![style.contains(TermStyle::CROSSED), self.try_push_bytes(&Ansi::CROSSED_B)?;];
        Ok(self)
    }
    /// Transitions the terminal from `from` styles to `to` styles.
    ///
    /// Bold and dim are handled together
    /// because their ANSI reset code disables both intensity styles.
    pub fn try_style_transition(&mut self, from: TermStyle, to: TermStyle)
        -> Result<&mut Self, NotEnoughSpace> {
        if from == to { return Ok(self); }
        let removed = from.difference(to);
        let added = to.difference(from);
        let intensity = TermStyle::BOLD | TermStyle::DIM;
        let intensity_removed = removed.intersects(intensity);
        if intensity_removed {
            self.try_push_bytes(&Ansi::BOLD_OFF_B)?;
            is![to.contains(TermStyle::BOLD), self.try_push_bytes(&Ansi::BOLD_B)?;];
            is![to.contains(TermStyle::DIM), self.try_push_bytes(&Ansi::DIM_B)?;];
        }
        is![removed.contains(TermStyle::ITALIC), self.try_push_bytes(&Ansi::ITALIC_OFF_B)?;];
        is![removed.contains(TermStyle::UNDERLINE), self.try_push_bytes(&Ansi::UNDERLINE_OFF_B)?;];
        is![removed.contains(TermStyle::BLINK), self.try_push_bytes(&Ansi::BLINK_OFF_B)?;];
        is![removed.contains(TermStyle::INVERSE), self.try_push_bytes(&Ansi::INVERSE_OFF_B)?;];
        is![removed.contains(TermStyle::HIDDEN), self.try_push_bytes(&Ansi::HIDDEN_OFF_B)?;];
        is![removed.contains(TermStyle::CROSSED), self.try_push_bytes(&Ansi::CROSSED_OFF_B)?;];
        let added = if intensity_removed { added.without(intensity) } else { added };
        self.try_style_enable(added)
    }
}

/// # Character encoding
impl<B: AsRef<[u8]> + AsMut<[u8]>> TermRenderer<B> {
    /// Appends a Rust Unicode scalar encoded as UTF-8.
    pub fn try_push_char(&mut self, ch: char) -> Result<&mut Self, NotEnoughSpace> {
        let mut buf = [0u8; 4];
        self.try_push_str(ch.encode_utf8(&mut buf))
    }
    /// Appends a [`charu`] Unicode scalar encoded as UTF-8.
    pub fn try_push_charu(&mut self, ch: charu) -> Result<&mut Self, NotEnoughSpace> {
        let mut buf = [0u8; 4];
        self.try_push_str(ch.as_str_into(&mut buf))
    }
    // …
}

/* helpers */
impl<B: AsRef<[u8]> + AsMut<[u8]>> TermRenderer<B> {
    /// Resolves paired terminal colors into directly encodable ANSI colors.
    fn resolve_ansi_colors(colors: TermColors) -> GridResult<(AnsiColor, AnsiColor)> {
        let fg = colors.fg().to_ansi().ok_or(TermGridError::unresolved_color(true, colors.fg()))?;
        let bg =
            colors.bg().to_ansi().ok_or(TermGridError::unresolved_color(false, colors.bg()))?;
        Ok((fg, bg))
    }
}

/// Terminal appearance state retained while encoding adjacent grid cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TermRenderState {
    style: TermStyle,
    colors: TermColors,
}
// Must match the state established by `TermRenderer::try_format_reset()`.
impl Default for TermRenderState {
    fn default() -> Self {
        Self {
            style: TermStyle::new(),
            colors: TermColors::DEFAULT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TermColor, TermColorMode, ext};

    #[test]
    fn style_transition_same_writes_nothing() {
        let mut r = TermRenderer::from_buf([0u8; 64], 8, 1);
        r.try_style_transition(TermStyle::BOLD, TermStyle::BOLD).unwrap();
        assert_eq!(r.buffered(), b"");
    }
    #[test]
    fn style_transition_none_to_styles() {
        let mut r = TermRenderer::from_buf([0u8; 64], 8, 1);
        r.try_style_transition(TermStyle::new(), TermStyle::BOLD | TermStyle::UNDERLINE)
            .unwrap();
        assert_eq!(r.buffered(), b"\x1b[1m\x1b[4m");
    }
    #[test]
    fn style_transition_removes_independent_style() {
        let mut r = TermRenderer::from_buf([0u8; 64], 8, 1);
        r.try_style_transition(TermStyle::BOLD | TermStyle::ITALIC, TermStyle::BOLD).unwrap();
        assert_eq!(r.buffered(), b"\x1b[23m");
    }
    #[test]
    fn style_transition_bold_to_dim() {
        let mut r = TermRenderer::from_buf([0u8; 64], 8, 1);
        r.try_style_transition(TermStyle::BOLD, TermStyle::DIM).unwrap();
        assert_eq!(r.buffered(), b"\x1b[22m\x1b[2m");
    }
    #[test]
    fn style_transition_bold_dim_to_bold() {
        let mut r = TermRenderer::from_buf([0u8; 64], 8, 1);
        r.try_style_transition(TermStyle::BOLD | TermStyle::DIM, TermStyle::BOLD).unwrap();
        assert_eq!(r.buffered(), b"\x1b[22m\x1b[1m");
    }
    #[test]
    fn push_chars_encode_utf8() {
        let mut r = TermRenderer::from_buf([0u8; 16], 8, 1);
        r.try_push_char('A').unwrap();
        r.try_push_char('λ').unwrap();
        r.try_push_char('🦀').unwrap();
        assert_eq!(r.buffered(), "Aλ🦀".as_bytes());
    }
    #[test]
    fn empty_grid_writes_nothing() {
        let grid = TermGrid::new([Termel::plain_const(' '); 0], ext!(0usize, 0usize)).unwrap();
        let mut r = TermRenderer::from_buf([0u8; 64], 8, 1);
        r.try_render_grid(&grid).unwrap();
        assert_eq!(r.buffered(), b"");
    }
    #[test]
    fn positioned_grid_places_each_row_explicitly() {
        let cells = [
            Termel::plain_const('A'),
            Termel::plain_const('B'),
            Termel::plain_const('C'),
            Termel::plain_const('D'),
        ];
        let grid = TermGrid::new(cells, ext!(2usize, 2usize)).unwrap();
        let mut r = TermRenderer::from_buf([0u8; 128], 8, 4);
        r.try_render_grid_at(&grid, 3, 1).unwrap();
        assert_eq!(r.buffered(), b"\x1b[0m\x1b[2;4HAB\x1b[3;4HCD\x1b[0m");
    }
    #[test]
    fn grid_reuses_appearance() {
        let colors = TermColors::new(TermColor::indexed(2), TermColor::indexed(0));
        let cells = [
            Termel::from_value('A', TermStyle::BOLD, colors),
            Termel::from_value('B', TermStyle::BOLD, colors),
        ];
        let grid = TermGrid::new(cells, ext!(2usize, 1usize)).unwrap();
        let mut r = TermRenderer::from_buf([0u8; 128], 8, 1);
        r.try_render_grid(&grid).unwrap();
        let bytes = r.buffered();
        assert_eq!(bytes.windows(4).filter(|w| *w == b"\x1b[1m").count(), 1);
        assert!(bytes.ends_with(b"AB\x1b[0m"));
    }
    #[test]
    fn positioned_grid_rejects_coordinate_overflow() {
        let grid = TermGrid::new([Termel::plain_const('X')], ext!(1usize, 1usize)).unwrap();
        let mut r = TermRenderer::from_buf([0u8; 64], 8, 1);
        let err = r.try_render_grid_at(&grid, u16::MAX, 0).unwrap_err();
        assert_eq!(err, TermGridError::RenderPositionOverflow);
        assert!(r.buffered().is_empty());
    }
    #[test]
    fn unresolved_color_is_reported() {
        let colors = TermColors::new(
            TermColor::DEFAULT.with_color_mode(TermColorMode::Transparent),
            TermColor::DEFAULT,
        );
        let grid = TermGrid::new(
            [Termel::from_value('X', TermStyle::new(), colors)],
            ext!(1usize, 1usize),
        )
        .unwrap();
        let mut r = TermRenderer::from_buf([0u8; 128], 8, 1);
        let err = r.try_render_grid(&grid).unwrap_err();
        assert_eq!(err, TermGridError::UnresolvedColor { foreground: true, color: colors.fg() });
    }
}
