// devela::sys::os::linux::terminal
//
//! Linux terminal related items.
//

use super::{LinuxTerminalSize, LinuxTermios};

#[cfg(all(feature = "dep_atomic", feature = "dep_bytemuck"))]
use crate::_dep::atomic::{Atomic, Ordering as AtomicOrdering};

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
///
/// # Features
/// Makes use of [`atomic`] and [`bytemuck`] dependencies to save the
/// terminal state in an [`Atomic`].
///
/// [`atomic`]: crate::_dep::atomic
/// [`bytemuck`]: crate::_dep::bytemuck
#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(all(feature = "dep_atomic", feature = "dep_bytemuck")))
)]
#[cfg(all(feature = "dep_atomic", feature = "dep_bytemuck"))]
pub static LINUX_TERMINAL_STATE: Atomic<LinuxTermios> = Atomic::new(LinuxTermios::new());

/// Linux terminal manager.
///
/// # Features
/// With `atomic` and `bytemuck` enabled,
/// the terminal state is saved in [`LINUX_TERMINAL_STATE`] and restored on drop.
#[allow(rustdoc::broken_intra_doc_links, reason = "LINUX_TERMINAL_STATE")]
#[derive(Debug, Default)]
pub struct LinuxTerminal;

#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(all(feature = "dep_atomic", feature = "dep_bytemuck")))
)]
#[cfg(all(feature = "dep_atomic", feature = "dep_bytemuck"))]
impl Drop for LinuxTerminal {
    fn drop(&mut self) {
        // If we are here, this should work
        Self::restore_saved_state().unwrap();
    }
}

#[allow(rustdoc::broken_intra_doc_links, reason = "LINUX_TERMINAL_STATE")]
impl LinuxTerminal {
    /// Returns a new linux terminal configured in canonical (cooked) mode.
    ///
    /// # Features
    /// With `atomic` and `bytemuck` enabled,
    /// it saves the initial terminal state in [`LINUX_TERMINAL_STATE`].
    pub fn new() -> Result<Self, isize> {
        #[cfg(all(feature = "dep_atomic", feature = "dep_bytemuck"))]
        Self::save_state()?;
        Ok(Self)
    }

    /// Returns a new linux terminal configured in raw mode.
    ///
    /// *Raw* mode is a mode where the terminal's input is processed character
    /// by character, rather than line by line.
    ///
    /// # Features
    /// With `atomic` and `bytemuck` enabled,
    /// it saves the initial terminal state in [`LINUX_TERMINAL_STATE`].
    pub fn new_raw() -> Result<Self, isize> {
        #[cfg(all(feature = "dep_atomic", feature = "dep_bytemuck"))]
        Self::save_state()?;

        let new = Self::new()?;
        new.enable_raw_mode()?;
        Ok(new)
    }

    /// Saves the current terminal state into [`LINUX_TERMINAL_STATE`].
    #[cfg_attr(
        feature = "nightly_doc",
        doc(cfg(all(feature = "dep_bytemuck", feature = "dep_atomic")))
    )]
    #[cfg(all(feature = "dep_atomic", feature = "dep_bytemuck"))]
    pub fn save_state() -> Result<(), isize> {
        LINUX_TERMINAL_STATE.store(LinuxTermios::get_state()?, AtomicOrdering::Relaxed);
        Ok(())
    }

    /// Restores the current terminal state into [`LINUX_TERMINAL_STATE`].
    #[cfg_attr(
        feature = "nightly_doc",
        doc(cfg(all(feature = "dep_bytemuck", feature = "dep_atomic")))
    )]
    #[cfg(all(feature = "dep_atomic", feature = "dep_bytemuck"))]
    pub fn restore_saved_state() -> Result<(), isize> {
        LinuxTermios::set_state(LINUX_TERMINAL_STATE.load(AtomicOrdering::Relaxed))
    }

    /// Returns `true` if we are in a terminal context.
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        LinuxTermios::is_terminal()
    }

    /// Returns the terminal dimensions.
    pub fn size(&self) -> Result<LinuxTerminalSize, isize> {
        LinuxTermios::get_winsize()
    }

    /// Enables raw mode.
    ///
    /// Raw mode is a way to configure the terminal so that it does not process or
    /// interpret any of the input but instead passes it directly to the program.
    pub fn enable_raw_mode(&self) -> Result<(), isize> {
        LinuxTermios::enable_raw_mode()
    }

    /// Disables raw mode.
    pub fn disable_raw_mode(&self) -> Result<(), isize> {
        LinuxTermios::disable_raw_mode()
    }
}
