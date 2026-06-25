// devela/src/run/app/control.rs
//
//! Defines [`AppControl`].
//

crate::enumset! {
    #[doc = crate::_tags!(runtime event)]
    /// External control notice directed at an application.
    #[doc = crate::_doc_meta!{
        location("run/app"),
        test_size_of(AppControl = 1|8; niche Option),

    }]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum AppControl(
        #[doc = crate::_tags!(runtime signal set)]
        /// A set of application control notices.
        #[doc = crate::_doc_meta!{
            location("run/app"),
            test_size_of(AppControlSet = 2|16),
        }]
        pub AppControlSet: u16
    ) {
        /// User requested interruption, usually `Ctrl+C`.
        Interrupt,
        /// User requested a stronger quit, usually `Ctrl+\` on Unix terminals.
        Quit,
        /// Polite termination request from the environment.
        Terminate,
        /// Session, terminal, or controlling process disappeared.
        Hangup,
        /// Application/session was asked to suspend.
        Suspend,
        /// Application/session resumed after suspension.
        Resume,
        /// Application/session is being closed.
        Close,
        /// User session is logging out.
        Logout,
        /// System is shutting down.
        Shutdown,
    }
}

#[cfg(feature = "_linux_abi")]
mod impl_linux {
    use crate::{AppControl, AppControlSet, LinuxSignal, LinuxSignalSet, whilst};

    impl AppControl {
        /// Returns the Linux signal normally used to deliver this control notice.
        #[must_use]
        pub const fn to_linux_signal(self) -> Option<LinuxSignal> {
            match self {
                Self::Interrupt => Some(LinuxSignal::SIGINT),
                Self::Quit => Some(LinuxSignal::SIGQUIT),
                Self::Terminate => Some(LinuxSignal::SIGTERM),
                Self::Hangup => Some(LinuxSignal::SIGHUP),
                Self::Suspend => Some(LinuxSignal::SIGTSTP),
                Self::Resume => Some(LinuxSignal::SIGCONT),
                /* unmapped on linux */
                // Self::Close => None,
                // Self::Logout => None,
                // Self::Shutdown => None,
                _ => None,
            }
        }
    }
    impl AppControlSet {
        /// Returns the Linux signals needed to receive these app controls.
        #[must_use]
        pub const fn to_linux_signals(self) -> LinuxSignalSet {
            let mut out = LinuxSignalSet::new();
            whilst! { i in 0..AppControl::VARIANTS; {
                let control = AppControl::ALL[i];
                if control.is_in(self) {
                    if let Some(signal) = control.to_linux_signal() {
                        out = out.with(signal.to_set());
                    }
                }
            }}
            out
        }
    }
    impl LinuxSignal {
        /// Returns the portable application control notice represented by this signal.
        #[must_use]
        pub const fn to_app_control(self) -> Option<AppControl> {
            match self {
                Self::SIGINT => Some(AppControl::Interrupt),
                Self::SIGQUIT => Some(AppControl::Quit),
                Self::SIGTERM => Some(AppControl::Terminate),
                Self::SIGHUP => Some(AppControl::Hangup),
                Self::SIGTSTP => Some(AppControl::Suspend),
                Self::SIGCONT => Some(AppControl::Resume),
                /* mapped to EventWindow */
                // Self::SIGWINCH => None,
                _ => None,
            }
        }
    }
}
