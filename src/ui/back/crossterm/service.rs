// devela::ui::back::crossterm::service
//
//! Defines [`CrosstermService`].
//
// ISSUES
// - WAIT: [add Event::Terminate](https://github.com/crossterm-rs/crossterm/issues/554)
// - WAIT: [support ctrl+z + fg](https://github.com/crossterm-rs/crossterm/issues/494)
//
// TODO
// - window refresh, render
//
// WAIT:publish: try_read: https://github.com/crossterm-rs/crossterm/issues/972

use ::crossterm::{event, execute, terminal};

use crate::{
    Io, IoResult, UiCap, UiCapImage, /* Event, EventSource, */ UiCapInput,
    /* Window, */ UiCapWindow, UiService,
};

#[doc = crate::_tags!(ui runtime platform)]
/// `crossterm`'s UI backend service.
#[doc = crate::_doc_location!("ui/back/crossterm")]
//
// https://docs.rs/crossterm/latest/crossterm/terminal/index.html
#[derive(Debug)]
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

#[rustfmt::skip]
impl CrosstermService {
    /// Creates a new `CrosstermService`.
    pub fn new() -> IoResult<Self> {
        Ok(Self { /* raw_mode: false */ })
    }

    /// Tells whether the raw mode is enabled.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.is_raw_mode_enabled.html
    pub fn is_raw_mode(&self) -> IoResult<bool> { terminal::is_raw_mode_enabled() }

    /// Enables the raw mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.enable_raw_mode.html
    pub fn enable_raw_mode(&self) -> IoResult<()> { terminal::enable_raw_mode() }

    /// Disables the raw mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.disable_raw_mode.html
    pub fn disable_raw_mode(&self) -> IoResult<()> { terminal::disable_raw_mode() }

    /// Switches to the alternate screen.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/struct.EnterAlternateScreen.html
    pub fn enter_alternate_screen(&self) -> IoResult<()> {
        Ok(execute!(Io::stdout(), terminal::EnterAlternateScreen)?)
    }

    /// Switches back to the main screen.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/struct.LeaveAlternateScreen.html
    pub fn leave_alternate_screen(&self) -> IoResult<()> {
        Ok(execute!(Io::stdout(), terminal::EnterAlternateScreen)?)
    }

    /// Enables receiving mouse events.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.EnableMouseCapture.html
    pub fn enable_mouse(&mut self) -> IoResult<()> {
        Ok(execute!(Io::stdout(), event::EnableMouseCapture)?)
    }
    /// Disables receiving mouse events.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.DisableMouseCapture.html
    pub fn disable_mouse(&mut self) -> IoResult<()> {
        Ok(execute!(Io::stdout(), event::DisableMouseCapture)?)
    }

    // TODO
    // /// Enables bracketed paste mode.
    // //
    // // https://docs.rs/crossterm/latest/crossterm/event/struct.EnableBracketedPaste.html
    // pub fn enable_bracketed_paste(&self) -> IoResult<()> {
    //     Ok(execute!(Io::stdout(), event::EnableBracketedPaste)?)
    // }
    //
    // /// Disables bracketed paste mode.
    // //
    // // https://docs.rs/crossterm/latest/crossterm/event/struct.DisableBracketedPaste.html
    // pub fn disable_bracketed_paste(&self) -> IoResult<()> {
    //     Ok(execute!(Io::stdout(), event::DisableBracketedPaste)?)
    // }

    /// Enables focus change mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.EnableFocusChange.html
    pub fn enable_focus_change(&self) -> IoResult<()> {
        Ok(execute!(Io::stdout(), event::EnableFocusChange)?)
    }

    /// Disables focus change mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.DisableFocusChange.html
    pub fn disable_focus_change(&self) -> IoResult<()> {
        Ok(execute!(Io::stdout(), event::DisableFocusChange)?)
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
