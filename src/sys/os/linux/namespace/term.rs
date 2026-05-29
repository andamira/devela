// devela::sys::os::linux::namespace::term

use crate::{Linux, LinuxResult as Result};
use crate::{LinuxTermios, ScopeGuard, TermSize};

pub(crate) type LinuxTermModeGuard = ScopeGuard<LinuxTermios, fn(LinuxTermios, &()), ()>;

/// # Terminal-related methods.
#[cfg(feature = "term")]
impl Linux {
    /// Returns `true` if we are in a terminal context.
    #[must_use]
    pub fn is_terminal() -> bool {
        LinuxTermios::is_terminal()
    }
    /// Returns the terminal dimensions.
    pub fn terminal_size() -> Result<TermSize> {
        LinuxTermios::read_window_size()
    }

    /// Enters a scoped event-oriented terminal input mode.
    pub fn scoped_event_mode() -> Result<LinuxTermModeGuard> {
        Self::scoped_termios_update(|state| state.make_event())
    }
    /// Enters a scoped raw terminal input mode.
    pub fn scoped_raw_mode() -> Result<LinuxTermModeGuard> {
        Self::scoped_termios_update(|state| state.make_raw())
    }

    /// Enters a scoped termios state update.
    ///
    /// Reads the current termios state, applies `f`, writes the modified state,
    /// and returns a guard that restores the original state on drop.
    pub fn scoped_termios_update(f: impl FnOnce(&mut LinuxTermios)) -> Result<LinuxTermModeGuard> {
        let initial_state = LinuxTermios::read_state()?;
        let mut active_state = initial_state;
        f(&mut active_state);
        active_state.write_state()?;
        Ok(ScopeGuard::with(initial_state, (), Self::restore_linux_termios))
    }

    /// Enables raw mode.
    ///
    /// Raw mode is a way to configure the terminal so that it does not process or
    /// interpret any of the input but instead passes it directly to the program.
    pub fn enable_raw_mode() -> Result<()> {
        LinuxTermios::enable_raw_mode()
    }
    /// Disables raw mode.
    pub fn reset_cooked_mode() -> Result<()> {
        LinuxTermios::reset_cooked_mode()
    }

    #[allow(
        clippy::trivially_copy_pass_by_ref,
        reason = "matches ScopeGuard's FnOnce(T, &S) callback signature"
    )]
    fn restore_linux_termios(state: LinuxTermios, (): &()) {
        let _ = state.write_state();
    }
}
