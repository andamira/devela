// devela::sys::os::term::ansi::namespace::color

use crate::{__ansi_consts, Ansi, AnsiColor3, AnsiColor8, Cmp, Digits};

// the bare color escape codes
mod C {
    pub(crate) const FG: u8 = b'3';
    pub(crate) const BG: u8 = b'4';
    pub(crate) const BRI_FG: u8 = b'9';
    pub(crate) const BRI_BG: [u8; 2] = *b"10";
    pub(crate) const C8: [u8; 4] = *b"8;5;";
    pub(crate) const RGB: [u8; 4] = *b"8;2;";
}

/// # Default color escape codes
///
/// Resets foreground or background color to the terminal default.
#[rustfmt::skip]
impl Ansi { __ansi_consts! {
    /// Code to reset the background color to the terminal default.
    pub const DEFAULT_BG: [u8; 5] = "\x1b[49m", *b"\x1b[49m";
    /// Code to reset the foreground color to the terminal default.
    pub const DEFAULT_FG: [u8; 5] = "\x1b[39m", *b"\x1b[39m";
}}

/// # 3-bit Color escape codes
#[rustfmt::skip]
impl Ansi {
    __ansi_consts! {
        /// Code to set the the foreground `color`.
        pub const fn COLOR_FG(color: AnsiColor3) -> [u8; 5] {
            [b'\x1b', b'[', C::FG, color.to_ascii(), b'm']
        }
        /// Code to set the the foreground `color` bright.
        pub const fn COLOR_FG_BRIGHT(color: AnsiColor3) -> [u8; 5] {
            [b'\x1b', b'[', C::BRI_FG, color.to_ascii(), b'm']
        }

        /// Code to set the the background `color`.
        pub const fn COLOR_BG(color: AnsiColor3) -> [u8; 5] {
            [b'\x1b', b'[', C::BG, color.to_ascii(), b'm']
        }
        /// Code to set the the background `color` bright.
        pub const fn COLOR_BG_BRIGHT(color: AnsiColor3) -> [u8; 6] {
            [b'\x1b', b'[', C::BRI_BG[0], C::BRI_BG[1], color.to_ascii(), b'm']
        }

        /// Code to set the foreground color to `fg` and the background to `bg`.
        pub const fn COLORS(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 8] {
            [ b'\x1b', b'[', C::FG, fg.to_ascii(), b';', C::BG, bg.to_ascii(), b'm' ]
        }
        /// Code to set the foreground color to bright `fg` and the background to bright `bg`.
        pub const fn COLORS_BRIGHT(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 9] {
            [
                b'\x1b', b'[',
                C::BRI_FG, fg.to_ascii(), b';', C::BRI_BG[0], C::BRI_BG[1], bg.to_ascii(),
                b'm',
            ]
        }

        /// Code to set the foreground color to bright `fg` and the background to `bg`.
        pub const fn COLORS_BRIGHT_FG(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 8] {
            [ b'\x1b', b'[', C::BRI_FG, fg.to_ascii(), b';', C::BG, bg.to_ascii(), b'm' ]
        }

        /// Code to set the foreground color to `fg` and the background to bright `bg`.
        pub const fn COLORS_BRIGHT_BG(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 9] {
            [
                b'\x1b', b'[',
                C::FG, fg.to_ascii(), b';', C::BRI_BG[0], C::BRI_BG[1], bg.to_ascii(),
                b'm',
            ]
        }
    }
}
// # 3-bit Color escape codes constants
#[rustfmt::skip]
impl Ansi {
    __ansi_consts! {
        /// Code to set the foreground color to black.
        pub const BLACK: [u8; 5] = "\x1b[30m", *b"\x1b[30m";
        /// Code to set the foreground color to red.
        pub const RED: [u8; 5] = "\x1b[31m", *b"\x1b[31m";
        /// Code to set the foreground color to green.
        pub const GREEN: [u8; 5] = "\x1b[32m", *b"\x1b[32m";
        /// Code to set the foreground color to yellow.
        pub const YELLOW: [u8; 5] = "\x1b[33m", *b"\x1b[33m";
        /// Code to set the foreground color to blue.
        pub const BLUE: [u8; 5] = "\x1b[34m", *b"\x1b[34m";
        /// Code to set the foreground color to magenta.
        pub const MAGENTA: [u8; 5] = "\x1b[35m", *b"\x1b[35m";
        /// Code to set the foreground color to cyan.
        pub const CYAN: [u8; 5] = "\x1b[36m", *b"\x1b[36m";
        /// Code to set the foreground color to white.
        pub const WHITE: [u8; 5] = "\x1b[37m", *b"\x1b[37m";

        /// Code to set the background color to black.
        pub const BLACK_BG: [u8; 5] = "\x1b[40m", *b"\x1b[40m";
        /// Code to set the background color to red.
        pub const RED_BG: [u8; 5] = "\x1b[41m", *b"\x1b[41m";
        /// Code to set the background color to green.
        pub const GREEN_BG: [u8; 5] = "\x1b[42m", *b"\x1b[42m";
        /// Code to set the background color to yellow.
        pub const YELLOW_BG: [u8; 5] = "\x1b[43m", *b"\x1b[43m";
        /// Code to set the background color to blue.
        pub const BLUE_BG: [u8; 5] = "\x1b[44m", *b"\x1b[44m";
        /// Code to set the background color to magenta.
        pub const MAGENTA_BG: [u8; 5] = "\x1b[45m", *b"\x1b[45m";
        /// Code to set the background color to cyan.
        pub const CYAN_BG: [u8; 5] = "\x1b[46m", *b"\x1b[46m";
        /// Code to set the background color to white.
        pub const WHITE_BG: [u8; 5] = "\x1b[47m", *b"\x1b[47m";

        /// Code to set the foreground color to bright black.
        pub const BRIGHT_BLACK: [u8; 5] = "\x1b[90m", *b"\x1b[90m";
        /// Code to set the foreground color to bright red.
        pub const BRIGHT_RED: [u8; 5] = "\x1b[91m", *b"\x1b[91m";
        /// Code to set the foreground color to bright green.
        pub const BRIGHT_GREEN: [u8; 5] = "\x1b[92m", *b"\x1b[92m";
        /// Code to set the foreground color to bright yellow.
        pub const BRIGHT_YELLOW: [u8; 5] = "\x1b[93m", *b"\x1b[93m";
        /// Code to set the foreground color to bright blue.
        pub const BRIGHT_BLUE: [u8; 5] = "\x1b[94m", *b"\x1b[94m";
        /// Code to set the foreground color to bright magenta.
        pub const BRIGHT_MAGENTA: [u8; 5] = "\x1b[95m", *b"\x1b[95m";
        /// Code to set the foreground color to bright cyan.
        pub const BRIGHT_CYAN: [u8; 5] = "\x1b[96m", *b"\x1b[96m";
        /// Code to set the foreground color to bright white.
        pub const BRIGHT_WHITE: [u8; 5] = "\x1b[97m", *b"\x1b[97m";

        /// Code to set the background color to bright black.
        pub const BRIGHT_BLACK_BG: [u8; 6] = "\x1b[100m", *b"\x1b[100m";
        /// Code to set the background color to bright red.
        pub const BRIGHT_RED_BG: [u8; 6] = "\x1b[101m", *b"\x1b[101m";
        /// Code to set the background color to bright green.
        pub const BRIGHT_GREEN_BG: [u8; 6] = "\x1b[102m", *b"\x1b[102m";
        /// Code to set the background color to bright yellow.
        pub const BRIGHT_YELLOW_BG: [u8; 6] = "\x1b[103m", *b"\x1b[103m";
        /// Code to set the background color to bright blue.
        pub const BRIGHT_BLUE_BG: [u8; 6] = "\x1b[104m", *b"\x1b[104m";
        /// Code to set the background color to bright magenta.
        pub const BRIGHT_MAGENTA_BG: [u8; 6] = "\x1b[105m", *b"\x1b[105m";
        /// Code to set the background color to bright cyan.
        pub const BRIGHT_CYAN_BG: [u8; 6] = "\x1b[106m", *b"\x1b[106m";
        /// Code to set the background color to bright white.
        pub const BRIGHT_WHITE_BG: [u8; 6] = "\x1b[107m", *b"\x1b[107m";
    }
}

/// # 8-bit Color escape codes
#[rustfmt::skip]
impl Ansi {
    __ansi_consts! {
        /// Code to set the foreground color to `fg` and the background to `bg`.
        pub const fn COLORS8(fg: AnsiColor8, bg: AnsiColor8) -> [u8; 19] {
            const X: [u8; 4] = C::C8;
            let cf = fg.to_ascii();
            let cb = bg.to_ascii();
            [
                b'\x1b', b'[',
                C::FG, X[0], X[1], X[2], X[3], cf[0], cf[1], cf[2],
                C::BG, X[0], X[1], X[2], X[3], cb[0], cb[1], cb[2],
                b'm',
            ]
        }

        /// Code to set the foreground color to `fg`.
        pub const fn COLOR8_FG(fg: AnsiColor8) -> [u8; 11] {
            const X: [u8; 4] = C::C8;
            let c = fg.to_ascii();
            [ b'\x1b', b'[', C::FG, X[0], X[1], X[2], X[3], c[0], c[1], c[2], b'm' ]
        }

        /// Code to set the background color to `bg`.
        pub const fn COLOR8_BG(bg: AnsiColor8) -> [u8; 11] {
            const X: [u8; 4] = C::C8;
            let c = bg.to_ascii();
            [ b'\x1b', b'[', C::BG, X[0], X[1], X[2], X[3], c[0], c[1], c[2], b'm' ]
        }
    }
}

/// # 8-bit Grey escape codes
#[rustfmt::skip]
impl Ansi {
    __ansi_consts! {
        /// Code to set the foreground and background to 24-point grayscale.
        ///
        /// A value of 0 is almost black, and 24 (or more) is almost white.
        pub const fn GRAY(fg: u8, bg: u8) -> [u8; 19] {
            const X: [u8; 4] = C::C8;
            let cf = Digits(Cmp(fg).min(23)).digits10();
            let cb = Digits(Cmp(bg).min(23)).digits10();
            [
                b'\x1b', b'[',
                C::FG, X[0], X[1], X[2], X[3], cf[0], cf[1], cf[2],
                C::BG, X[0], X[1], X[2], X[3], cb[0], cb[1], cb[2],
                b'm',
            ]
        }
    }
}
// # 8-bit Grey constants
#[rustfmt::skip]
impl Ansi {
    // /// ANSI gray foreground 0/23 8-bit color (4% white, 96% black).
    // pub const bGRAY0: [u8; 11] = @ Ansi::bCOLOR8_FG(AnsiColor8::gray_wrap(0));
    __ansi_consts! {
        /// ANSI gray foreground 0/23 8-bit color (4% white, 96% black).
        pub const GRAY0: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(0));
        /// ANSI gray foreground 1/23 8-bit color (8% white, 92% black).
        pub const GRAY1: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(1));
        /// ANSI gray foreground 2/23 8-bit color (12% white, 88% black).
        pub const GRAY2: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(2));
        /// ANSI gray foreground 3/23 8-bit color (16% white, 84% black).
        pub const GRAY3: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(3));
        /// ANSI gray foreground 4/23 8-bit color (20% white, 80% black).
        pub const GRAY4: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(4));
        /// ANSI gray foreground 5/23 8-bit color (24% white, 76% black).
        pub const GRAY5: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(5));
        /// ANSI gray foreground 6/23 8-bit color (28% white, 72% black).
        pub const GRAY6: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(6));
        /// ANSI gray foreground 7/23 8-bit color (32% white, 68% black).
        pub const GRAY7: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(7));
        /// ANSI gray foreground 8/23 8-bit color (36% white, 64% black).
        pub const GRAY8: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(8));
        /// ANSI gray foreground 9/23 8-bit color (40% white, 60% black).
        pub const GRAY9: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(9));
        /// ANSI gray foreground 10/23 8-bit color (44% white, 56% black).
        pub const GRAY10: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(10));
        /// ANSI gray foreground 11/23 8-bit color (48% white, 52% black).
        pub const GRAY11: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(11));
        /// ANSI gray foreground 12/23 8-bit color (52% white, 48% black).
        pub const GRAY12: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(12));
        /// ANSI gray foreground 13/23 8-bit color (56% white, 44% black).
        pub const GRAY13: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(13));
        /// ANSI gray foreground 14/23 8-bit color (60% white, 40% black).
        pub const GRAY14: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(14));
        /// ANSI gray foreground 15/23 8-bit color (64% white, 36% black).
        pub const GRAY15: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(15));
        /// ANSI gray foreground 16/23 8-bit color (68% white, 32% black).
        pub const GRAY16: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(16));
        /// ANSI gray foreground 17/23 8-bit color (72% white, 28% black).
        pub const GRAY17: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(17));
        /// ANSI gray foreground 18/23 8-bit color (76% white, 24% black).
        pub const GRAY18: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(18));
        /// ANSI gray foreground 19/23 8-bit color (80% white, 20% black).
        pub const GRAY19: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(19));
        /// ANSI gray foreground 20/23 8-bit color (84% white, 16% black).
        pub const GRAY20: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(20));
        /// ANSI gray foreground 21/23 8-bit color (88% white, 12% black).
        pub const GRAY21: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(21));
        /// ANSI gray foreground 22/23 8-bit color (92% white, 8% black).
        pub const GRAY22: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(22));
        /// ANSI gray foreground 23/23 8-bit color (96% white, 4% black).
        pub const GRAY23: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(23));

        /// ANSI gray background 0/23 8-bit color (4% white, 96% black).
        pub const GRAY0_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(0));
        /// ANSI gray background 1/23 8-bit color (8% white, 92% black).
        pub const GRAY1_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(1));
        /// ANSI gray background 2/23 8-bit color (12% white, 88% black).
        pub const GRAY2_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(2));
        /// ANSI gray background 3/23 8-bit color (16% white, 84% black).
        pub const GRAY3_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(3));
        /// ANSI gray background 4/23 8-bit color (20% white, 80% black).
        pub const GRAY4_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(4));
        /// ANSI gray background 5/23 8-bit color (24% white, 76% black).
        pub const GRAY5_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(5));
        /// ANSI gray background 6/23 8-bit color (28% white, 72% black).
        pub const GRAY6_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(6));
        /// ANSI gray background 7/23 8-bit color (32% white, 68% black).
        pub const GRAY7_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(7));
        /// ANSI gray background 8/23 8-bit color (36% white, 64% black).
        pub const GRAY8_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(8));
        /// ANSI gray background 9/23 8-bit color (40% white, 60% black).
        pub const GRAY9_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(9));
        /// ANSI gray background 10/23 8-bit color (44% white, 56% black).
        pub const GRAY10_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(10));
        /// ANSI gray background 11/23 8-bit color (48% white, 52% black).
        pub const GRAY11_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(11));
        /// ANSI gray background 12/23 8-bit color (52% white, 48% black).
        pub const GRAY12_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(12));
        /// ANSI gray background 13/23 8-bit color (56% white, 44% black).
        pub const GRAY13_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(13));
        /// ANSI gray background 14/23 8-bit color (60% white, 40% black).
        pub const GRAY14_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(14));
        /// ANSI gray background 15/23 8-bit color (64% white, 36% black).
        pub const GRAY15_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(15));
        /// ANSI gray background 16/23 8-bit color (68% white, 32% black).
        pub const GRAY16_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(16));
        /// ANSI gray background 17/23 8-bit color (72% white, 28% black).
        pub const GRAY17_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(17));
        /// ANSI gray background 18/23 8-bit color (76% white, 24% black).
        pub const GRAY18_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(18));
        /// ANSI gray background 19/23 8-bit color (80% white, 20% black).
        pub const GRAY19_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(19));
        /// ANSI gray background 20/23 8-bit color (84% white, 16% black).
        pub const GRAY20_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(20));
        /// ANSI gray background 21/23 8-bit color (88% white, 12% black).
        pub const GRAY21_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(21));
        /// ANSI gray background 22/23 8-bit color (92% white, 8% black).
        pub const GRAY22_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(22));
        /// ANSI gray background 23/23 8-bit color (96% white, 4% black).
        pub const GRAY23_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(23));
    }
}

/// # 8-bit palette colors
#[rustfmt::skip]
impl Ansi {
    __ansi_consts! {
        /// Sets the given palette color. (OSC 4)
        // \x1b]4;{index};rgb:{rr:02x}/{gg:02x}/{bb:02x}\x07
        pub const fn SET_PALETTE(index: u8, color: [u8; 3]) -> [u8; 21] {
            let i = Digits(index).digits10();
            let [r, g, b] = color;
            let [r, g, b] = [Digits(r).digits16(), Digits(g).digits16(), Digits(b).digits16()];
            [
                b'\x1b', b']', b'4', b';', i[0], i[1], i[2], b';',
                b'r', b'g', b'b', b':',
                r[0], r[1], b'/', g[0], g[1], b'/', b[0], b[1], b'\x07'
            ]
        }
        /// Resets the given palette color to the default one. (OSC 104)
        pub const fn RESET_PALETTE(index: u8) -> [u8; 10] {
            let i = Digits(index).digits10();
            [b'\x1b', b']', b'1', b'0', b'4', b';', i[0], i[1], i[2], b'\x07']
        }
    }
    __ansi_consts! {
        /// Resets all the palette colors to the default ones. (OSC 104)
        pub const RESET_PALETTE_ALL: [u8; 6] = "\x1b]104\x07", *b"\x1b]104\x07";
    }
}

/// # RGB Color escape codes
#[rustfmt::skip]
impl Ansi { __ansi_consts! {
    /// Code to set the foreground color to `fg: [r, g, b]` values,
    /// and the background to `bg: [r, g, b]`.
    // \x1b[38;2;R;G;B;48;2;R;G;Bm
    pub const fn RGB(fg: [u8; 3], bg: [u8; 3]) -> [u8; 36] {
        const X: [u8; 4] = C::RGB;
        let [fR, fG, fB] = fg;
        let [bR, bG, bB] = bg;
        let [fR, fG, fB] = [Digits(fR).digits10(), Digits(fG).digits10(), Digits(fB).digits10()];
        let [bR, bG, bB] = [Digits(bR).digits10(), Digits(bG).digits10(), Digits(bB).digits10()];
        [
            b'\x1b', b'[', C::FG, X[0], X[1], X[2], X[3], // \x1b[38;2;
            fR[0], fR[1], fR[2], b';', fG[0], fG[1], fG[2], b';', fB[0], fB[1], fB[2], b';',
            C::BG, X[0], X[1], X[2], X[3], // 48;2;
            bR[0], bR[1], bR[2], b';', bG[0], bG[1], bG[2], b';', bB[0], bB[1], bB[2], b'm',
        ]
    }
    /// Code to set the foreground color to `fg: [r, g, b]` values.
    pub const fn RGB_FG(fg: [u8; 3]) -> [u8; 19] {
        const X: [u8; 4] = C::RGB;
        let [r, g, b] = fg;
        let [r, g, b] = [Digits(r).digits10(), Digits(g).digits10(), Digits(b).digits10()];
        [
            b'\x1b', b'[',
            C::FG, X[0], X[1], X[2], X[3],
            r[0], r[1], r[2], b';', g[0], g[1], g[2], b';', b[0], b[1], b[2],
            b'm'
        ]
    }
    /// Code to set the background color to `bg: [r, g, b]` values.
    pub const fn RGB_BG(bg: [u8; 3]) -> [u8; 19] {
        const X: [u8; 4] = C::RGB;
        let [r, g, b] = bg;
        let [r, g, b] = [Digits(r).digits10(), Digits(g).digits10(), Digits(b).digits10()];
        [
            b'\x1b', b'[',
            C::BG, X[0], X[1], X[2], X[3],
            r[0], r[1], r[2], b';', g[0], g[1], g[2], b';', b[0], b[1], b[2],
            b'm',
        ]
    }
}}
