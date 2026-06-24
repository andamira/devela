// devela/src/sys/os/term/backend/linux/signal.rs
//
//! Implements signal handling methods for [`TermLinux`].
//

use crate::{AppControl, AppControlSet, Linux, LinuxSignalSet, TermLinux};
use crate::{AtomicBool, AtomicOrdering, AtomicU16, c_int};
#[cfg(doc)]
use crate::{EventKind::Control, EventWindow::Resized};

static PENDING_APP_CONTROLS: AtomicU16 = AtomicU16::new(0);
static PENDING_RESIZE: AtomicBool = AtomicBool::new(false);

/// # Signal handling
impl TermLinux {
    /// Registers Linux signals used by this terminal event source.
    ///
    /// This enables application control notices from `controls` and terminal resize
    /// notices from `SIGWINCH`. Pending notices are drained into this backend's event
    /// queue as `EventKind::`[`Control`] and `EventWindow::`[`Resized`].
    ///
    /// This installs process-wide signal handlers. Registering another handler for
    /// the same Linux signals may replace this path.
    pub fn listen_signals(&mut self, controls: AppControlSet) {
        self.listen_app_controls(controls);
        self.listen_resize();
    }

    /// Registers application control notices as terminal events.
    ///
    /// This installs process-wide Linux signal handlers for the corresponding
    /// controls. Pending notices are drained into this backend's event queue as
    /// `EventKind::`[`Control`].
    ///
    /// Registering another handler for the same Linux signals may replace this path.
    pub fn listen_app_controls(&mut self, controls: AppControlSet) {
        Self::install_app_controls(controls);
    }

    /// Registers terminal resize notices as window resize events.
    ///
    /// This installs a process-wide `SIGWINCH` handler. Pending resize notices are
    /// drained into this backend's event queue as `EventWindow::`[`Resized`].
    ///
    /// Registering another handler for `SIGWINCH` may replace this path.
    pub fn listen_resize(&mut self) {
        Self::install_resize();
    }
}

// Internal helpers
impl TermLinux {
    /* app controls */

    fn install_app_controls(controls: AppControlSet) {
        Linux::app_control_handler(Self::app_control_handler, controls, None);
    }
    fn app_control_handler(control: AppControl) {
        let bit = control.to_set().bits();
        PENDING_APP_CONTROLS.fetch_or(bit, AtomicOrdering::SeqCst);
    }
    pub(super) fn take_app_controls() -> AppControlSet {
        AppControlSet::from_bits(PENDING_APP_CONTROLS.swap(0, AtomicOrdering::SeqCst))
    }

    /* resize */

    fn install_resize() {
        Linux::sig_handler(Self::resize_handler, LinuxSignalSet::SIGWINCH, None);
    }
    fn resize_handler(_sig: c_int) {
        PENDING_RESIZE.store(true, AtomicOrdering::SeqCst);
    }
    pub(super) fn take_resize() -> bool {
        PENDING_RESIZE.swap(false, AtomicOrdering::SeqCst)
    }
}
