// devela/examples/ui/web/main.rs
//
//! Renders a backend-neutral UI output to an HTML canvas.
//

#![no_std]

use devela::{Lunit, UiDensity, UiDraw, UiOutputView, UiRect, WebCanvasUi};

devela::set_panic_handler![web];

/* UI adapter */

const CANVAS_UI: WebCanvasUi<'static> = WebCanvasUi::new("#canvas_ui", UiDensity::ONE);

/* demonstration style */

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum DemoStyle {
    Background,
    Panel,
    Border,
    Text,
    Accent,
}

/// Resolves the demonstration's semantic styles into its dark theme.
const fn demo_dark_rgba(style: DemoStyle) -> [u8; 4] {
    match style {
        DemoStyle::Background => [18, 20, 24, 255],
        DemoStyle::Panel => [34, 38, 46, 255],
        DemoStyle::Border => [91, 101, 119, 255],
        DemoStyle::Text => [226, 230, 238, 255],
        DemoStyle::Accent => [92, 156, 255, 255],
    }
}

/* demonstration output */

type DemoDraw = UiDraw<DemoStyle>;

const fn rect(x: i32, y: i32, width: i32, height: i32) -> UiRect {
    UiRect::from_xy_wh(Lunit::px(x), Lunit::px(y), Lunit::px(width), Lunit::px(height))
}

static DEMO_DRAWS: [DemoDraw; 8] = [
    // Canvas background.
    UiDraw::rect_fill(rect(0, 0, 500, 260), DemoStyle::Background),
    // Panel.
    UiDraw::rect_fill(rect(20, 20, 460, 220), DemoStyle::Panel),
    UiDraw::rect_stroke(rect(20, 20, 460, 220), Lunit::px(2), DemoStyle::Border),
    // Heading.
    UiDraw::text(rect(40, 42, 400, 24), "devela UI", DemoStyle::Text),
    UiDraw::text(
        rect(40, 70, 400, 24),
        "UiOutputView -> WebCanvasUi -> Canvas 2D",
        DemoStyle::Text,
    ),
    // Button-like composition.
    UiDraw::rect_fill(rect(40, 112, 140, 42), DemoStyle::Accent),
    UiDraw::rect_stroke(rect(40, 112, 140, 42), Lunit::px(2), DemoStyle::Border),
    UiDraw::text(rect(68, 126, 100, 18), "Continue", DemoStyle::Text),
];

fn demo_output() -> UiOutputView<'static, DemoStyle> {
    UiOutputView::from_slices(&[], &[], &DEMO_DRAWS)
}

/// Renders the demonstration output.
fn draw() {
    let output = demo_output();

    CANVAS_UI.render_with(output.draw_list(), |style| demo_dark_rgba(*style));
}

/// Initializes and renders the web UI demonstration.
#[unsafe(no_mangle)]
pub extern "C" fn main() {
    draw();
}
