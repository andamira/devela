// devela/src/run/app/control.rs
//
//! Defines [`AppControl`].
//

use crate::is;

crate::enumset! {
    #[doc = crate::_tags!(runtime event)]
    /// External control notice directed at an application.
    #[doc = crate::_doc_meta!{ location("run/app/control") }]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum AppControl(
        #[doc = crate::_tags!(runtime signal set)]
        /// A set of application control notices.
        #[doc = crate::_doc_meta!{ location("run/app/control") }]
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
    impl enum {
        /// All application control notices, in declaration order.
        pub const ALL: [Self; Self::VARIANTS] = [
            Self::Interrupt,
            Self::Quit,
            Self::Terminate,
            Self::Hangup,
            Self::Suspend,
            Self::Resume,
            Self::Close,
            Self::Logout,
            Self::Shutdown,
        ];
    }
    impl set {
        /// Calls `f` for each control notice in this set.
        pub fn for_each(self, mut f: impl FnMut(AppControl)) {
            for control in AppControl::ALL {
                is! { control.is_in(self), f(control) }
            }
        }
    }
}

#[cfg(feature = "_linux_abi")]
mod impl_linux {
    use crate::{
        AppControl, AppControlSet, AtomicOrdering::SeqCst, AtomicPtr, Linux, LinuxSigactionFlags,
        LinuxSignal, LinuxSignalSet, Ptr, c_int, transmute, whilst,
    };

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
    impl Linux {
        /// Registers a handler for portable application control notices.
        pub fn app_control_handler(
            handler: fn(AppControl),
            controls: AppControlSet,
            flags: impl Into<LinuxSigactionFlags>,
        ) {
            static HANDLER: AtomicPtr<()> = AtomicPtr::new(Ptr::null_mut());

            fn adapter(sig: c_int) {
                let Some(control) =
                    LinuxSignal::from_c_int(sig).and_then(LinuxSignal::to_app_control)
                else {
                    return;
                };
                let handler = HANDLER.load(SeqCst);
                if !handler.is_null() {
                    // SAFETY: stored from a `fn(AppControl)` and checked for null.
                    let handler: fn(AppControl) = unsafe { transmute(handler) };
                    handler(control);
                }
            }
            HANDLER.store(handler as *mut (), SeqCst);
            Self::sig_handler(adapter, controls.to_linux_signals(), flags.into());
        }
    }
}
