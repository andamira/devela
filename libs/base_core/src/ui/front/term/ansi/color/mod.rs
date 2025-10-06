// devela::ui::front::term::ansi::color
//
//! ANSI codes related to color.
//
// TOC
// - enum AnsiColor
// - enum AnsiColor3
// - struct AnsiColor8
// - impl Ansi:
//   - 3-bit Color escape codes
//   - 8-bit Color escape codes
//   - 8-bit Grey escape codes
//   - 8-bit Palette escape codes
//   - RGB Color escape codes

use crate::{Ansi, AsciiDigits as AsciiD};

mod bit3;
mod bit8;
pub use {bit3::*, bit8::*};

/// Complete ANSI color selection
///
/// The size of this type is 32-bit.
// IMPROVE: use Rgb8 color type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnsiColor {
    /// No color (reset to default).
    None,
    /// Standard 3-bit colors.
    Dark(AnsiColor3),
    /// Bright 3-bit colors.
    Bright(AnsiColor3),
    /// 8-bit colors (256 colors).
    Palette(AnsiColor8),
    /// True color RGB
    Rgb([u8; 3]),
}

/// # Conversions
#[rustfmt::skip]
impl AnsiColor {
    const _CHECK_SIZE: () = const { assert![4 == size_of::<Self>(), "AnsiColor size != 4"] };

    ///
    pub const fn into_3(self) -> Option<AnsiColor3> {
        match self { Self::Dark(c) | Self::Bright(c) => Some(c), _ => None }
    }
    ///
    pub const fn into_8(self) -> Option<AnsiColor8> {
        match self { Self::Palette(c) => Some(c), _ => None }
    }
    ///
    pub const fn into_rgb(self) -> Option<[u8; 3]> {
        match self { Self::Rgb(c) => Some(c), _ => None }
    }
}

/// # Color constants
#[allow(missing_docs, non_upper_case_globals)]
impl AnsiColor {
    pub const Black: Self = Self::Dark(AnsiColor3::Black);
    pub const Red: Self = Self::Dark(AnsiColor3::Red);
    pub const Green: Self = Self::Dark(AnsiColor3::Green);
    pub const Yellow: Self = Self::Dark(AnsiColor3::Yellow);
    pub const Blue: Self = Self::Dark(AnsiColor3::Blue);
    pub const Magenta: Self = Self::Dark(AnsiColor3::Magenta);
    pub const Cyan: Self = Self::Dark(AnsiColor3::Cyan);
    pub const White: Self = Self::Dark(AnsiColor3::White);

    pub const BlackBright: Self = Self::Bright(AnsiColor3::Black);
    pub const RedBright: Self = Self::Bright(AnsiColor3::Red);
    pub const GreenBright: Self = Self::Bright(AnsiColor3::Green);
    pub const YellowBright: Self = Self::Bright(AnsiColor3::Yellow);
    pub const BlueBright: Self = Self::Bright(AnsiColor3::Blue);
    pub const MagentaBright: Self = Self::Bright(AnsiColor3::Magenta);
    pub const CyanBright: Self = Self::Bright(AnsiColor3::Cyan);
    pub const WhiteBright: Self = Self::Bright(AnsiColor3::White);

    /* constants: abbr */
    pub const K: Self = Self::Black;
    pub const R: Self = Self::Red;
    pub const G: Self = Self::Green;
    pub const Y: Self = Self::Yellow;
    pub const B: Self = Self::Blue;
    pub const M: Self = Self::Magenta;
    pub const C: Self = Self::Cyan;
    pub const W: Self = Self::White;
    pub const KB: Self = Self::BlackBright;
    pub const RB: Self = Self::RedBright;
    pub const GB: Self = Self::GreenBright;
    pub const YB: Self = Self::YellowBright;
    pub const BB: Self = Self::BlueBright;
    pub const MB: Self = Self::MagentaBright;
    pub const CB: Self = Self::CyanBright;
    pub const WB: Self = Self::WhiteBright;
}

// the bare color escape codes (private module)
mod C {
    use super::AnsiColor3 as A3;

    pub(super) const FG: u8 = b'3';
    pub(super) const BG: u8 = b'4';
    pub(super) const BRI_FG: u8 = b'9';
    pub(super) const BRI_BG: [u8; 2] = *b"10";
    //
    pub(super) const C8: [u8; 4] = *b"8;5;";
    pub(super) const RGB: [u8; 4] = *b"8;2;";
    //
    pub(super) const BLACK_FG: [u8; 2] = [FG, A3::Black.to_ascii()];
    pub(super) const RED_FG: [u8; 2] = [FG, A3::Red.to_ascii()];
    pub(super) const GREEN_FG: [u8; 2] = [FG, A3::Green.to_ascii()];
    pub(super) const YELLOW_FG: [u8; 2] = [FG, A3::Yellow.to_ascii()];
    pub(super) const BLUE_FG: [u8; 2] = [FG, A3::Blue.to_ascii()];
    pub(super) const MAGENTA_FG: [u8; 2] = [FG, A3::Magenta.to_ascii()];
    pub(super) const CYAN_FG: [u8; 2] = [FG, A3::Cyan.to_ascii()];
    pub(super) const WHITE_FG: [u8; 2] = [FG, A3::White.to_ascii()];

    pub(super) const BLACK_BG: [u8; 2] = [BG, A3::Black.to_ascii()];
    pub(super) const RED_BG: [u8; 2] = [BG, A3::Red.to_ascii()];
    pub(super) const GREEN_BG: [u8; 2] = [BG, A3::Green.to_ascii()];
    pub(super) const YELLOW_BG: [u8; 2] = [BG, A3::Yellow.to_ascii()];
    pub(super) const BLUE_BG: [u8; 2] = [BG, A3::Blue.to_ascii()];
    pub(super) const MAGENTA_BG: [u8; 2] = [BG, A3::Magenta.to_ascii()];
    pub(super) const CYAN_BG: [u8; 2] = [BG, A3::Cyan.to_ascii()];
    pub(super) const WHITE_BG: [u8; 2] = [BG, A3::White.to_ascii()];

    pub(super) const BRI_BLACK_FG: [u8; 2] = [BRI_FG, A3::Black.to_ascii()];
    pub(super) const BRI_RED_FG: [u8; 2] = [BRI_FG, A3::Red.to_ascii()];
    pub(super) const BRI_GREEN_FG: [u8; 2] = [BRI_FG, A3::Green.to_ascii()];
    pub(super) const BRI_YELLOW_FG: [u8; 2] = [BRI_FG, A3::Yellow.to_ascii()];
    pub(super) const BRI_BLUE_FG: [u8; 2] = [BRI_FG, A3::Blue.to_ascii()];
    pub(super) const BRI_MAGENTA_FG: [u8; 2] = [BRI_FG, A3::Magenta.to_ascii()];
    pub(super) const BRI_CYAN_FG: [u8; 2] = [BRI_FG, A3::Cyan.to_ascii()];
    pub(super) const BRI_WHITE_FG: [u8; 2] = [BRI_FG, A3::White.to_ascii()];

    pub(super) const BRI_BLACK_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A3::Black.to_ascii()];
    pub(super) const BRI_RED_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A3::Red.to_ascii()];
    pub(super) const BRI_GREEN_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A3::Green.to_ascii()];
    pub(super) const BRI_YELLOW_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A3::Yellow.to_ascii()];
    pub(super) const BRI_BLUE_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A3::Blue.to_ascii()];
    pub(super) const BRI_MAGENTA_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A3::Magenta.to_ascii()];
    pub(super) const BRI_CYAN_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A3::Cyan.to_ascii()];
    pub(super) const BRI_WHITE_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A3::White.to_ascii()];
}

/// # RGB Color escape codes
#[rustfmt::skip]
impl Ansi {
    /// Code to set the foreground color to `fg: [r, g, b]` values,
    /// and the background to `bg: [r, g, b]`.
    // \x1b[38;2;R;G;B;48;2;R;G;Bm
    #[must_use]
    pub const fn RGB(fg: [u8; 3], bg: [u8; 3]) -> [u8; 36] {
        const X: [u8; 4] = C::RGB;
        let [fR, fG, fB] = fg;
        let [bR, bG, bB] = bg;
        let [fR, fG, fB] = [AsciiD(fR).digits10(), AsciiD(fG).digits10(), AsciiD(fB).digits10()];
        let [bR, bG, bB] = [AsciiD(bR).digits10(), AsciiD(bG).digits10(), AsciiD(bB).digits10()];
        [
            b'\x1b', b'[', C::FG, X[0], X[1], X[2], X[3], // \x1b[38;2;
            fR[0], fR[1], fR[2], b';', fG[0], fG[1], fG[2], b';', fB[0], fB[1], fB[2], b';',
            C::BG, X[0], X[1], X[2], X[3], // 48;2;
            bR[0], bR[1], bR[2], b';', bG[0], bG[1], bG[2], b';', bB[0], bB[1], bB[2], b'm',
        ]
    }

    /// Code to set the foreground color to `fg: [r, g, b]` values.
    #[must_use]
    pub const fn RGB_FG(fg: [u8; 3]) -> [u8; 19] {
        const X: [u8; 4] = C::RGB;
        let [r, g, b] = fg;
        let [r, g, b] = [AsciiD(r).digits10(), AsciiD(g).digits10(), AsciiD(b).digits10()];
        [
            b'\x1b', b'[',
            C::FG, X[0], X[1], X[2], X[3],
            r[0], r[1], r[2], b';', g[0], g[1], g[2], b';', b[0], b[1], b[2],
            b'm'
        ]
    }

    /// Code to set the background color to `bg: [r, g, b]` values.
    #[must_use]
    pub const fn RGB_BG(bg: [u8; 3]) -> [u8; 19] {
        const X: [u8; 4] = C::RGB;
        let [r, g, b] = bg;
        let [r, g, b] = [AsciiD(r).digits10(), AsciiD(g).digits10(), AsciiD(b).digits10()];
        [
            b'\x1b', b'[',
            C::BG, X[0], X[1], X[2], X[3],
            r[0], r[1], r[2], b';', g[0], g[1], g[2], b';', b[0], b[1], b[2],
            b'm',
        ]
    }
}
