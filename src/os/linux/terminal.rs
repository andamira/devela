// devela::os::linux::terminal
//
//! Linux terminal related items.
//

use super::{LinuxTerminalSize, LinuxTermios};
use crate::sync::atomic::{Atomic, AtomicOrdering};

/// State of the terminal saved globally, that can be restored from anywhere.
///
/// This allows to restore the initial terminal state from a panic handler. E.g.:
///
/// ```ignore
/// #[panic_handler]
/// fn panic(_info: &core::panic::PanicInfo) -> ! {
///     LinuxTerminal::restore_saved_state().unwrap();
/// }
/// ```
pub static LINUX_TERMINAL_STATE: Atomic<LinuxTermios> = Atomic::new(LinuxTermios::new());

/// Linux terminal manager.
#[derive(Debug, Default)]
pub struct LinuxTerminal;

impl Drop for LinuxTerminal {
    fn drop(&mut self) {
        // If we are here, this should work
        Self::restore_saved_state().unwrap()
    }
}

impl LinuxTerminal {
    /// Returns a new linux terminal configured in canonical (cooked) mode.
    ///
    /// It saves the initial terminal state in [`LINUX_TERMINAL_STATE`].
    #[inline]
    pub fn new() -> Result<Self, isize> {
        Self::save_state()?;
        Ok(Self)
    }

    /// Returns a new linux terminal configured in raw mode.
    ///
    /// *Raw* mode is a mode where the terminal's input is processed character
    /// by character, rather than line by line.
    ///
    /// It saves the initial terminal state in [`LINUX_TERMINAL_STATE`].
    #[inline]
    pub fn new_raw() -> Result<Self, isize> {
        Self::save_state()?;

        let new = Self::new()?;
        new.enable_raw_mode()?;
        Ok(new)
    }

    /// Saves the current terminal state into [`LINUX_TERMINAL_STATE`].
    pub fn save_state() -> Result<(), isize> {
        LINUX_TERMINAL_STATE.store(LinuxTermios::get_state()?, AtomicOrdering::Relaxed);
        Ok(())
    }

    /// Restores the current terminal state into [`LINUX_TERMINAL_STATE`].
    #[inline]
    pub fn restore_saved_state() -> Result<(), isize> {
        LinuxTermios::set_state(LINUX_TERMINAL_STATE.load(AtomicOrdering::Relaxed))
    }

    /// Returns `true` if we are in a terminal context.
    #[inline]
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        LinuxTermios::is_terminal()
    }

    /// Returns the terminal dimensions.
    #[inline]
    pub fn size(&self) -> Result<LinuxTerminalSize, isize> {
        LinuxTermios::get_winsize()
    }

    /// Enables raw mode.
    ///
    /// Raw mode is a way to configure the terminal so that it does not process or
    /// interpret any of the input but instead passes it directly to the program.
    #[inline]
    pub fn enable_raw_mode(&self) -> Result<(), isize> {
        LinuxTermios::enable_raw_mode()
    }

    /// Disables raw mode.
    #[inline]
    pub fn disable_raw_mode(&self) -> Result<(), isize> {
        LinuxTermios::disable_raw_mode()
    }
}
