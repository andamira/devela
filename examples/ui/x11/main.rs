// devela/examples/ui/x11/main.rs
//
//! Renders a backend-neutral UI output to an X11 pixel surface.
//

use devela::{EventWindow, FONT_BIT_3_5, Key, is};
use devela::{Lunit, UiDensity, UiDraw, UiOutputView, UiRect};
use devela::{XError, XFrontend, XSurfaceUi};

/* output geometry */

const WIDTH: u16 = 500;
const HEIGHT: u16 = 260;

const X_UI: XSurfaceUi = XSurfaceUi::new(UiDensity::ONE, 2);

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
    /// Resolves the demonstration's semantic styles into its dark theme.
    const fn dark(self) -> [u8; 4] {
        match self {
            Self::Background => [18, 20, 24, 255],
            Self::Panel => [34, 38, 46, 255],
            Self::Border => [91, 101, 119, 255],
            Self::Text => [226, 230, 238, 255],
            Self::Accent => [92, 156, 255, 255],
        }
    }
}

/* demonstration output */

type DemoDraw = UiDraw<DemoStyle>;

const fn rect(x: i32, y: i32, width: i32, height: i32) -> UiRect {
    UiRect::from_xy_wh(Lunit::px(x), Lunit::px(y), Lunit::px(width), Lunit::px(height))
}

static DEMO_DRAWS: [DemoDraw; 8] = [
    // Pixel-surface background.
    UiDraw::rect_fill(rect(0, 0, 500, 260), DemoStyle::Background),
    // Panel.
    UiDraw::rect_fill(rect(20, 20, 460, 220), DemoStyle::Panel),
    UiDraw::rect_stroke(rect(20, 20, 460, 220), Lunit::px(2), DemoStyle::Border),
    // Heading.
    UiDraw::text(rect(40, 42, 400, 24), "devela UI", DemoStyle::Text),
    UiDraw::text(
        rect(40, 70, 400, 24),
        "UiOutputView -> XSurfaceUi -> XSurfaceFrame",
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

/* presentation */

fn draw(front: &mut XFrontend) -> Result<(), XError> {
    let output = demo_output();
    let depth = front.display().depth();
    front.with_surface_frame(WIDTH, HEIGHT, depth, true, |surface| {
        X_UI.render_with(output.draw_list(), surface, &FONT_BIT_3_5, |style| style.dark())
    })
}

fn main() -> Result<(), XError> {
    let mut front = XFrontend::open(100, 100, WIDTH, HEIGHT)?;
    loop {
        let event = front.wait_event();
        is! { event.is_none(), break }

        if let Some(window) = event.some_window() {
            match window {
                EventWindow::RedrawRequested => draw(&mut front)?,
                EventWindow::CloseRequested => break,
                _ => {}
            }
        } else if let Some(key) = event.some_key()
            && key.physical == Key::Q
        {
            break;
        }
    }
    Ok(())
}
