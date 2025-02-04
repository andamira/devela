// devela::ui::service::crossterm::service
//
//! Defines [`CrosstermService`].
//
// ISSUES
// - WAIT: [add Event::Terminate](https://github.com/crossterm-rs/crossterm/issues/554)
// - WAIT: [support ctrl+z + fg](https://github.com/crossterm-rs/crossterm/issues/494)
//
// TODO
// - window refresh, render

use ::crossterm::{event, execute, terminal};

// use core::time::Duration;
use std::io;

use crate::{
    IoError, UiCap, UiCapImage, /* Event, EventSource, */ UiCapInput,
    /* Window, */ UiCapWindow, UiService,
};

/// `crossterm`'s UI service.
//
// https://docs.rs/crossterm/latest/crossterm/terminal/index.html
pub struct CrosstermService;
// { raw_mode: bool, }

// TODO
// impl Drop for CrosstermService {
//     fn drop(&mut self)  {
//         if self.raw_mode {
//             self.set_raw_mode(false);
//         }
//     }
// }

impl CrosstermService {
    /// Creates a new `CrosstermService`.
    pub fn new() -> Result<Self, IoError> {
        Ok(Self { /* raw_mode: false */ })
    }

    /// Tells whether the raw mode is enabled.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.is_raw_mode_enabled.html
    #[inline]
    pub fn is_raw_mode(&self) -> Result<bool, IoError> {
        terminal::is_raw_mode_enabled()
    }

    /// Enables the raw mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.enable_raw_mode.html
    #[inline]
    pub fn enable_raw_mode(&self) -> Result<(), IoError> {
        terminal::enable_raw_mode()
    }

    /// Disables the raw mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.disable_raw_mode.html
    #[inline]
    pub fn disable_raw_mode(&self) -> Result<(), IoError> {
        terminal::disable_raw_mode()
    }

    /// Switches to the alternate screen.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/struct.EnterAlternateScreen.html
    pub fn enter_alternate_screen(&self) -> Result<(), IoError> {
        Ok(execute!(io::stdout(), terminal::EnterAlternateScreen)?)
    }

    /// Switches back to the main screen.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/struct.LeaveAlternateScreen.html
    pub fn leave_alternate_screen(&self) -> Result<(), IoError> {
        Ok(execute!(io::stdout(), terminal::EnterAlternateScreen)?)
    }

    /// Enables receiving mouse events.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.EnableMouseCapture.html
    pub fn enable_mouse(&mut self) -> Result<(), IoError> {
        Ok(execute!(io::stdout(), event::EnableMouseCapture)?)
    }
    /// Disables receiving mouse events.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.DisableMouseCapture.html
    pub fn disable_mouse(&mut self) -> Result<(), IoError> {
        Ok(execute!(io::stdout(), event::DisableMouseCapture)?)
    }

    // TODO
    // /// Enables bracketed paste mode.
    // //
    // // https://docs.rs/crossterm/latest/crossterm/event/struct.EnableBracketedPaste.html
    // pub fn enable_bracketed_paste(&self) -> Result<(), IoError> {
    //     Ok(execute!(io::stdout(), event::EnableBracketedPaste)?)
    // }
    //
    // /// Disables bracketed paste mode.
    // //
    // // https://docs.rs/crossterm/latest/crossterm/event/struct.DisableBracketedPaste.html
    // pub fn disable_bracketed_paste(&self) -> Result<(), IoError> {
    //     Ok(execute!(io::stdout(), event::DisableBracketedPaste)?)
    // }

    /// Enables focus change mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.EnableFocusChange.html
    pub fn enable_focus_change(&self) -> Result<(), IoError> {
        Ok(execute!(io::stdout(), event::EnableFocusChange)?)
    }

    /// Disables focus change mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.DisableFocusChange.html
    pub fn disable_focus_change(&self) -> Result<(), IoError> {
        Ok(execute!(io::stdout(), event::DisableFocusChange)?)
    }
}

impl UiService for CrosstermService {
    fn capabilities(&self) -> UiCap {
        let image = Some(UiCapImage {
            rgb: true,
            // palette_change: false,
            // palette_size: ::crossterm::style::available_color_count(),
            ..Default::default()
        });

        let input = Some(UiCapInput { keyboard: true, mouse: true, ..Default::default() });

        // let text_grid = Some(UiCapTextGridCap {
        //     // we don't unknown
        //     cell_size: None,
        //     // https://github.com/crossterm-rs/crossterm/issues/166
        //     // custom_cell_size: false,
        //     // // https://github.com/crossterm-rs/crossterm/issues/677
        //     // unicode: true,
        //     // ..Default::default()
        // });

        let window = Some(UiCapWindow { multi: false });

        UiCap {
            image,
            input,
            // text_grid,
            window,
            ..Default::default()
        }
    }

    fn version(&self) -> (u32, u32, u32) {
        (0, 28, 1)
    }
}
