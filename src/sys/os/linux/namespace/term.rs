// devela::sys::os::linux::namespace::term

use crate::{Linux, LinuxResult as Result};
use crate::{LinuxTermios, ScopeGuard, TermSize};

pub(crate) type LinuxRawModeGuard = ScopeGuard<LinuxTermios, fn(LinuxTermios, &()), ()>;

/// # Terminal-related methods.
#[rustfmt::skip]
#[cfg(feature = "term")]
impl Linux {
    /// Returns `true` if we are in a terminal context.
    #[must_use]
    pub fn is_terminal() -> bool { LinuxTermios::is_terminal() }

    /// Returns the terminal dimensions.
    pub fn terminal_size() -> Result<TermSize> { LinuxTermios::get_winsize() }

    /// Enables raw mode.
    ///
    /// Raw mode is a way to configure the terminal so that it does not process or
    /// interpret any of the input but instead passes it directly to the program.
    pub fn enable_raw_mode() -> Result<()> { LinuxTermios::enable_raw_mode() }

    /// Disables raw mode.
    pub fn disable_raw_mode() -> Result<()> { LinuxTermios::disable_raw_mode() }

    /// Enables raw mode and returns a `ScopeGuard` that restores the original state on drop.
    pub fn scoped_raw_mode() -> Result<LinuxRawModeGuard> {
        let initial_state = LinuxTermios::get_state()?;
        LinuxTermios::enable_raw_mode()?;
        Ok(ScopeGuard::with(initial_state, (), Self::restore_linux_termios))
    }
    fn restore_linux_termios(state: LinuxTermios, _: &()) {
        let _ = LinuxTermios::set_state(state);
    }
}
