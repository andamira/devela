// devela::sys::os::term::render::slice

use crate::{Ansi, AnsiColor, NotEnoughSpace, TermRenderer, whilst};

/// # Mutable byte-frame construction
impl<B: AsRef<[u8]> + AsMut<[u8]>> TermRenderer<B> {
    /// Clears the active frame without modifying the underlying bytes.
    pub const fn clear_buffer(&mut self) -> &mut Self {
        self.len = 0;
        self
    }

    /// Appends raw bytes to the active frame.
    ///
    /// Returns `NotEnoughSpace` if the bytes do not fit in the remaining storage.
    pub fn try_push_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, NotEnoughSpace> {
        let Some(end) = self.len.checked_add(bytes.len()) else {
            return Err(NotEnoughSpace(Some(bytes.len())));
        };
        if end > self.capacity() {
            return Err(NotEnoughSpace(Some(bytes.len())));
        }
        self.buf.as_mut()[self.len..end].copy_from_slice(bytes);
        self.len = end;
        Ok(self)
    }

    /// Appends a string to the active frame.
    pub fn try_push_str(&mut self, text: &str) -> Result<&mut Self, NotEnoughSpace> {
        self.try_push_bytes(text.as_bytes())
    }

    // /// Begins a full-screen frame.
    // pub fn try_begin_frame(&mut self) -> Result<&mut Self, NotEnoughSpace> {
    //     self.clear_buffer();
    //     self.try_cursor_hide()?;
    //     self.try_clear_screen()?;
    //     self.try_cursor_home()
    // }

    /// Finishes a full-screen frame.
    pub fn try_finish_frame(&mut self) -> Result<&mut Self, NotEnoughSpace> {
        self.try_format_reset()?;
        self.try_cursor_show()
    }
    /// Clears the whole terminal screen.
    pub fn try_clear_screen(&mut self) -> Result<&mut Self, NotEnoughSpace> {
        self.try_push_bytes(&Ansi::ERASE_SCREEN_B)
    }
    /// Clears the current terminal line.
    pub fn try_clear_line(&mut self) -> Result<&mut Self, NotEnoughSpace> {
        self.try_push_bytes(&Ansi::ERASE_LINE_B)
    }

    /// Moves the cursor to the terminal home position.
    pub fn try_cursor_home(&mut self) -> Result<&mut Self, NotEnoughSpace> {
        self.try_push_bytes(&Ansi::CURSOR_HOME_B)
    }
    /// Hides the cursor.
    pub fn try_cursor_hide(&mut self) -> Result<&mut Self, NotEnoughSpace> {
        self.try_push_bytes(&Ansi::CURSOR_INVISIBLE_B)
    }
    /// Shows the cursor.
    pub fn try_cursor_show(&mut self) -> Result<&mut Self, NotEnoughSpace> {
        self.try_push_bytes(&Ansi::CURSOR_VISIBLE_B)
    }
    /// Moves the cursor to one-based terminal coordinates.
    pub fn try_cursor_move_to(&mut self, col: u16, row: u16) -> Result<&mut Self, NotEnoughSpace> {
        let mut tmp = [0u8; 14];
        let seq = Ansi::CURSOR_MOVE_N_B(&mut tmp, col, row);
        self.try_push_bytes(seq)
    }
    /// Moves the cursor to zero-based grid coordinates.
    pub fn try_cursor_move_to0(&mut self, x: u16, y: u16) -> Result<&mut Self, NotEnoughSpace> {
        self.try_cursor_move_to(x.saturating_add(1), y.saturating_add(1))
    }

    /// Resets all terminal presentation attributes and colors to their defaults.
    pub fn try_format_reset(&mut self) -> Result<&mut Self, NotEnoughSpace> {
        self.try_push_bytes(&Ansi::RESET_B)
    }
    /// Enables bold style.
    pub fn try_style_bold(&mut self) -> Result<&mut Self, NotEnoughSpace> {
        self.try_push_bytes(&Ansi::BOLD_B)
    }

    /// Sets the foreground color.
    pub fn try_color_fg(&mut self, color: AnsiColor) -> Result<&mut Self, NotEnoughSpace> {
        match color {
            AnsiColor::None => Ok(self),
            AnsiColor::Default => self.try_push_bytes(&Ansi::DEFAULT_FG_B),
            AnsiColor::Dark(c) => self.try_push_bytes(&Ansi::COLOR_FG_B(c)),
            AnsiColor::Bright(c) => self.try_push_bytes(&Ansi::COLOR_FG_BRIGHT_B(c)),
            AnsiColor::Palette(c) => self.try_push_bytes(&Ansi::COLOR8_FG_B(c)),
            AnsiColor::Rgb(c) => self.try_push_bytes(&Ansi::RGB_FG_B(c)),
        }
    }
    /// Sets the background color.
    pub fn try_color_bg(&mut self, color: AnsiColor) -> Result<&mut Self, NotEnoughSpace> {
        match color {
            AnsiColor::None => Ok(self),
            AnsiColor::Default => self.try_push_bytes(&Ansi::DEFAULT_BG_B),
            AnsiColor::Dark(c) => self.try_push_bytes(&Ansi::COLOR_BG_B(c)),
            AnsiColor::Bright(c) => self.try_push_bytes(&Ansi::COLOR_BG_BRIGHT_B(c)),
            AnsiColor::Palette(c) => self.try_push_bytes(&Ansi::COLOR8_BG_B(c)),
            AnsiColor::Rgb(c) => self.try_push_bytes(&Ansi::RGB_BG_B(c)),
        }
    }

    /// Sets foreground and background colors.
    ///
    /// Argument order is `(fg, bg)`, matching ANSI code order.
    pub fn try_colors(
        &mut self,
        fg: AnsiColor,
        bg: AnsiColor,
    ) -> Result<&mut Self, NotEnoughSpace> {
        match (fg, bg) {
            (AnsiColor::None, AnsiColor::None) => Ok(self),
            (AnsiColor::Dark(fg), AnsiColor::Dark(bg)) => {
                self.try_push_bytes(&Ansi::COLORS_B(fg, bg))
            }
            (AnsiColor::Bright(fg), AnsiColor::Bright(bg)) => {
                self.try_push_bytes(&Ansi::COLORS_BRIGHT_B(fg, bg))
            }
            (AnsiColor::Bright(fg), AnsiColor::Dark(bg)) => {
                self.try_push_bytes(&Ansi::COLORS_BRIGHT_FG_B(fg, bg))
            }
            (AnsiColor::Dark(fg), AnsiColor::Bright(bg)) => {
                self.try_push_bytes(&Ansi::COLORS_BRIGHT_BG_B(fg, bg))
            }
            (AnsiColor::Palette(fg), AnsiColor::Palette(bg)) => {
                self.try_push_bytes(&Ansi::COLORS8_B(fg, bg))
            }
            (AnsiColor::Rgb(fg), AnsiColor::Rgb(bg)) => self.try_push_bytes(&Ansi::RGB_B(fg, bg)),
            (fg, bg) => {
                self.try_color_fg(fg)?;
                self.try_color_bg(bg)
            }
        }
    }

    /// Writes text at one-based terminal coordinates.
    pub fn try_text_at(
        &mut self,
        col: u16,
        row: u16,
        text: &str,
    ) -> Result<&mut Self, NotEnoughSpace> {
        self.try_cursor_move_to(col, row)?;
        self.try_push_str(text)
    }
    /// Writes text at zero-based terminal coordinates.
    ///
    /// This moves the cursor, then appends `text` as raw UTF-8 bytes.
    /// It does not clip, wrap, or account for display width.
    pub fn try_text_at0(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
    ) -> Result<&mut Self, NotEnoughSpace> {
        self.try_text_at(x.saturating_add(1), y.saturating_add(1), text)
    }

    /// Draws a horizontal repeated string from zero-based coordinates.
    ///
    /// `unit` is repeated `count` times as raw UTF-8 bytes.
    /// This does not account for display width.
    pub fn try_hline_at0(
        &mut self,
        x: u16,
        y: u16,
        unit: &str,
        count: usize,
    ) -> Result<&mut Self, NotEnoughSpace> {
        self.try_cursor_move_to0(x, y)?;
        whilst! { i in 0..count; { self.try_push_str(unit)?; }}
        Ok(self)
    }
    /// Draws a vertical repeated string from zero-based grid coordinates.
    ///
    /// `unit` is repeated `count` times as raw UTF-8 bytes.
    /// This does not account for display width.
    pub fn try_vline_at0(
        &mut self,
        x: u16,
        y: u16,
        unit: &str,
        count: usize,
    ) -> Result<&mut Self, NotEnoughSpace> {
        whilst! { i in 0..count; {
            let yi = y.saturating_add(i.min(u16::MAX as usize) as u16);
            self.try_cursor_move_to0(x, yi)?;
            self.try_push_str(unit)?;
        }}
        Ok(self)
    }
}
