// devela/src/sys/os/term/backend/linux/restore.rs
//
//! Defines (`TermLinuxRestore`, `TermLinuxRestoreFlags`).
//

use crate::{Ansi, Debug, TermLineMode, TermMode};
use crate::{Linux, LinuxResult, LinuxTermModeGuard};

crate::set! {
    /// Terminal escape-sequence restoration flags.
    ///
    /// These flags only track ANSI/reporting/presentation cleanup.
    /// Termios state is restored separately by the stored line-discipline guard.
    struct TermLinuxRestoreFlags(u16) {
        RESET_STYLE              = 0;
        SHOW_CURSOR              = 1;
        LEAVE_ALT_SCREEN         = 2;
        DISABLE_BRACKETED_PASTE  = 3;
        DISABLE_FOCUS_EVENTS     = 4;

        DISABLE_MOUSE            = 5;
        DISABLE_MOUSE_DRAG       = 6;
        DISABLE_MOUSE_MOTION     = 7;
        DISABLE_MOUSE_SGR        = 8;
        DISABLE_MOUSE_SGR_PIXELS = 9;

        DISABLE_SYNC_UPDATE      = 10;
        CLEAR_ON_DROP            = 11;
    }
}

#[doc = crate::_tags!(term linux guard)]
/// Linux terminal session restore payload.
#[doc = crate::_doc_meta!{
    location("sys/os/term"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TermLinuxRestore = 48|384),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TermLinuxRestore = 56|448),
}]
///
/// This is the synchronization point between [`TermMode`] and Linux-specific
/// terminal state changes. It applies termios line mode changes and ANSI
/// reporting/presentation requests, recording the corresponding restoration
/// work as each step succeeds.
#[derive(Debug)]
pub struct TermLinuxRestore {
    guard: Option<LinuxTermModeGuard>,
    flags: TermLinuxRestoreFlags,
}
impl TermLinuxRestore {
    pub(crate) fn none() -> Self {
        Self { guard: None, flags: TermLinuxRestoreFlags::new() }
    }
    pub(crate) fn raw() -> LinuxResult<Self> {
        Self::enter(TermMode::raw())
    }
    pub(crate) fn enter(mode: TermMode) -> LinuxResult<Self> {
        let mut restore = Self::none();

        /* line discipline */
        match mode.line_mode() {
            TermLineMode::Line => {}
            TermLineMode::Event => {
                restore.guard = Some(Linux::scoped_event_mode()?);
            }
            TermLineMode::Raw => {
                restore.guard = Some(Linux::scoped_termios_update(|state| {
                    state.make_raw();

                    if mode.has_processed_output() {
                        state.set_output_processing(true);
                    }
                })?);
            }
        }

        /* presentation */
        if mode.has_alt_screen() {
            Linux::print_bytes(&Ansi::ENABLE_ALT_SCREEN_B)?;
            restore.flags = restore.flags.with_leave_alt_screen();
        }
        if mode.has_hide_cursor() {
            Linux::print_bytes(&Ansi::CURSOR_INVISIBLE_B)?;
            restore.flags = restore.flags.with_show_cursor();
        }

        /* input reporting */
        if mode.has_bracketed_paste() {
            Linux::print_bytes(&Ansi::ENABLE_BRACKETED_PASTE_B)?;
            restore.flags = restore.flags.with_disable_bracketed_paste();
        }
        if mode.has_focus_events() {
            Linux::print_bytes(&Ansi::ENABLE_FOCUS_EVENTS_B)?;
            restore.flags = restore.flags.with_disable_focus_events();
        }

        /* mouse: enable encoding before reporting */
        // with fallback:
        if mode.has_mouse_sgr() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_SGR_B)?;
            restore.flags = restore.flags.with_disable_mouse_sgr();
        }
        if mode.has_mouse_sgr_pixels() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_SGR_PIXELS_B)?;
            restore.flags = restore.flags.with_disable_mouse_sgr_pixels();
        }

        if mode.has_mouse_motion() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_MOTION_B)?;
            restore.flags = restore.flags.with_disable_mouse_motion();
        } else if mode.has_mouse_drag() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_DRAG_B)?;
            restore.flags = restore.flags.with_disable_mouse_drag();
        } else if mode.has_mouse() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_B)?;
            restore.flags = restore.flags.with_disable_mouse();
        }

        /* screen clearing */
        if mode.has_clear_on_enter() {
            Linux::print_bytes(&Ansi::ERASE_SCREEN_B)?;
            Linux::print_bytes(&Ansi::CURSOR_HOME_B)?;
        }
        if mode.has_clear_on_drop() {
            restore.flags = restore.flags.with_clear_on_drop();
        }

        /* output transaction */
        if mode.has_sync_update() {
            Linux::print_bytes(&Ansi::ENABLE_SYNC_UPDATE_B)?;
            restore.flags = restore.flags.with_disable_sync_update();
        }

        Ok(restore)
    }
}
impl Drop for TermLinuxRestore {
    fn drop(&mut self) {
        /* style reset */
        if self.flags.has_reset_style() {
            let _ = Linux::print_bytes(&Ansi::RESET_B);
        }

        /* input reporting */
        if self.flags.has_disable_bracketed_paste() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_BRACKETED_PASTE_B);
        }
        if self.flags.has_disable_focus_events() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_FOCUS_EVENTS_B);
        }

        /* mouse: disable reporting before encoding */
        if self.flags.has_disable_mouse_motion() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_MOTION_B);
        }
        if self.flags.has_disable_mouse_drag() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_DRAG_B);
        }
        if self.flags.has_disable_mouse() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_B);
        }

        if self.flags.has_disable_mouse_sgr_pixels() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_SGR_PIXELS_B);
        }
        if self.flags.has_disable_mouse_sgr() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_SGR_B);
        }

        /* screen clearing */
        if self.flags.has_clear_on_drop() {
            let _ = Linux::print_bytes(&Ansi::ERASE_SCREEN_B);
            let _ = Linux::print_bytes(&Ansi::CURSOR_HOME_B);
        }
        /* presentation */
        if self.flags.has_show_cursor() {
            let _ = Linux::print_bytes(&Ansi::CURSOR_VISIBLE_B);
        }
        if self.flags.has_leave_alt_screen() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_ALT_SCREEN_B);
        }
        /* output transaction */
        if self.flags.has_disable_sync_update() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_SYNC_UPDATE_B);
        }
        // `guard` drops after this and restores termios.
    }
}
