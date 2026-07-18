// devela/examples/ui/term/main.rs
//
//! Renders a backend-neutral UI output to a terminal cell grid.
//

use devela::{Error, ansi, ext};
use devela::{Lunit, TermCellUi, UiCellMetric, UiDraw, UiOutputView, UiRect};
use devela::{TermColor, TermColorMode, TermColors, TermGrid, TermRenderer, TermStyle, Termel};

/* output geometry */

const COLS: usize = 50;
const ROWS: usize = 13;

type Cell = Termel<char>;
// type Grid = TermGrid<Cell, [Cell; COLS * ROWS]>;

const CELL_METRIC: UiCellMetric = UiCellMetric::from_px(10, 20).unwrap();
const TERM_UI: TermCellUi = TermCellUi::new(CELL_METRIC);

/* demonstration style */

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum DemoStyle {
    Background,
    Panel,
    Border,
    Text,
    Accent,
}
impl DemoStyle {
    /// Resolves the demonstration's semantic styles into terminal cells.
    const fn dark(self) -> Cell {
        const S: TermStyle = TermStyle::new();
        type Colors = TermColors;
        const WHITE_BLUE: TermColor = TermColor::rgb([226, 230, 238]);
        const LIGHT_BLUE: TermColor = TermColor::rgb([92, 156, 255]);
        const DARK_BLUE: TermColor = TermColor::rgb([91, 101, 119]);
        const BLACK_BLUE1: TermColor = TermColor::rgb([18, 20, 24]);
        const BLACK_BLUE2: TermColor = TermColor::rgb([34, 38, 46]);
        const TRANSPARENT: TermColor = TermColor::default_with_mode(TermColorMode::Transparent);
        match self {
            Self::Background => Termel::from_value(' ', S, Colors::new(WHITE_BLUE, BLACK_BLUE1)),
            Self::Panel => Termel::from_value(' ', S, Colors::new(WHITE_BLUE, BLACK_BLUE2)),
            Self::Border => Termel::from_value(' ', S, Colors::new(DARK_BLUE, DARK_BLUE)),
            Self::Text => Termel::from_value(' ', S, Colors::new(WHITE_BLUE, TRANSPARENT)),
            Self::Accent => Termel::from_value(' ', S, Colors::new(BLACK_BLUE1, LIGHT_BLUE)),
        }
    }
}

/* demonstration output */

type DemoDraw = UiDraw<DemoStyle>;

const fn rect(x: i32, y: i32, width: i32, height: i32) -> UiRect {
    UiRect::from_xy_wh(Lunit::px(x), Lunit::px(y), Lunit::px(width), Lunit::px(height))
}

static DEMO_DRAWS: [DemoDraw; 8] = [
    // Complete logical output background
    UiDraw::rect_fill(rect(0, 0, 500, 260), DemoStyle::Background),
    // Main panel
    UiDraw::rect_fill(rect(20, 20, 460, 220), DemoStyle::Panel),
    UiDraw::rect_stroke(rect(20, 20, 460, 220), Lunit::px(2), DemoStyle::Border),
    // Heading
    UiDraw::text(rect(40, 42, 400, 24), "devela UI", DemoStyle::Text),
    UiDraw::text(rect(40, 70, 400, 24), "UiOutputView -> TermCellUi -> TermGrid", DemoStyle::Text),
    // Button-like composition
    UiDraw::rect_fill(rect(40, 112, 140, 42), DemoStyle::Accent),
    UiDraw::rect_stroke(rect(40, 112, 140, 42), Lunit::px(2), DemoStyle::Border),
    UiDraw::text(rect(68, 126, 100, 18), "Continue", DemoStyle::Text),
];

fn demo_output() -> UiOutputView<'static, DemoStyle> {
    UiOutputView::from_slices(&[], &[], &DEMO_DRAWS)
}

/* presentation */

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = TermGrid::new([Cell::plain_const(' '); COLS * ROWS], ext![COLS, ROWS])?;
    let output = demo_output();
    TERM_UI.render_with(output.draw_list(), &mut grid, |style| style.dark());
    let mut renderer = TermRenderer::from_buf([0_u8; 8192], COLS as u16, ROWS as u16);
    renderer.try_push_bytes(ansi![b: cursor_invisible, erase_screen, cursor_home])?;
    renderer.try_render_grid(&grid)?;
    renderer.try_push_bytes(ansi![b: reset, cursor_visible])?;
    renderer.try_push_bytes(b"\r\n")?;
    renderer.present()?;
    Ok(())
}
