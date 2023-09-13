// devela::os::terminal::color
//
//! ANSI colors
//

/// ANSI color codes.
pub struct AnsiColor;

pub(super) const FG: u8 = b'3';
pub(super) const BG: u8 = b'4';
pub(super) const BRIGHT_FG: u8 = b'9';
pub(super) const BRIGHT_BG: [u8; 2] = [b'1', b'0'];
//
pub(super) const BLACK: u8 = b'0';
pub(super) const RED: u8 = b'1';
pub(super) const GREEN: u8 = b'2';
pub(super) const YELLOW: u8 = b'3';
pub(super) const BLUE: u8 = b'4';
pub(super) const MAGENTA: u8 = b'5';
pub(super) const CYAN: u8 = b'6';
pub(super) const WHITE: u8 = b'7';

/// # 4-bit colors
impl AnsiColor {
    pub const BLACK_FG: &'static [u8; 2] = &[FG, BLACK];
    pub const RED_FG: &'static [u8; 2] = &[FG, RED];
    pub const GREEN_FG: &'static [u8; 2] = &[FG, GREEN];
    pub const YELLOW_FG: &'static [u8; 2] = &[FG, YELLOW];
    pub const BLUE_FG: &'static [u8; 2] = &[FG, BLUE];
    pub const MAGENTA_FG: &'static [u8; 2] = &[FG, MAGENTA];
    pub const CYAN_FG: &'static [u8; 2] = &[FG, CYAN];
    pub const WHITE_FG: &'static [u8; 2] = &[FG, WHITE];

    pub const BLACK_BG: &'static [u8; 2] = &[BG, BLACK];
    pub const RED_BG: &'static [u8; 2] = &[BG, RED];
    pub const GREEN_BG: &'static [u8; 2] = &[BG, GREEN];
    pub const YELLOW_BG: &'static [u8; 2] = &[BG, YELLOW];
    pub const BLUE_BG: &'static [u8; 2] = &[BG, BLUE];
    pub const MAGENTA_BG: &'static [u8; 2] = &[BG, MAGENTA];
    pub const CYAN_BG: &'static [u8; 2] = &[BG, CYAN];
    pub const WHITE_BG: &'static [u8; 2] = &[BG, WHITE];

    pub const BRIGHT_BLACK_FG: &'static [u8; 2] = &[BRIGHT_FG, BLACK];
    pub const BRIGHT_RED_FG: &'static [u8; 2] = &[BRIGHT_FG, RED];
    pub const BRIGHT_GREEN_FG: &'static [u8; 2] = &[BRIGHT_FG, GREEN];
    pub const BRIGHT_YELLOW_FG: &'static [u8; 2] = &[BRIGHT_FG, YELLOW];
    pub const BRIGHT_BLUE_FG: &'static [u8; 2] = &[BRIGHT_FG, BLUE];
    pub const BRIGHT_MAGENTA_FG: &'static [u8; 2] = &[BRIGHT_FG, MAGENTA];
    pub const BRIGHT_CYAN_FG: &'static [u8; 2] = &[BRIGHT_FG, CYAN];
    pub const BRIGHT_WHITE_FG: &'static [u8; 2] = &[BRIGHT_FG, WHITE];

    pub const BRIGHT_BLACK_BG: &'static [u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], BLACK];
    pub const BRIGHT_RED_BG: &'static [u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], RED];
    pub const BRIGHT_GREEN_BG: &'static [u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], GREEN];
    pub const BRIGHT_YELLOW_BG: &'static [u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], YELLOW];
    pub const BRIGHT_BLUE_BG: &'static [u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], BLUE];
    pub const BRIGHT_MAGENTA_BG: &'static [u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], MAGENTA];
    pub const BRIGHT_CYAN_BG: &'static [u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], CYAN];
    pub const BRIGHT_WHITE_BG: &'static [u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], WHITE];
}
