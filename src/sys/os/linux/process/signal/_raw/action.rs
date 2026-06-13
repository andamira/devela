// devela/src/sys/os/linux/process/signal/_raw/action.rs
//
//! Defines [`LINUX_SIGACTION`].
//

use crate::c_size_t;
#[cfg(doc)]
use crate::{LINUX_SIGNAL, LinuxSigaction};

#[doc = crate::_tags!(linux signal abi)]
/// Linux flag constants for [`LinuxSigaction`].
#[doc = crate::_doc_meta!{location("sys/os/linux/process")}]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct LINUX_SIGACTION;

// 8 flags
impl LINUX_SIGACTION {
    /// If signum is [`SIGCHLD`], do not receive notification when child processes
    /// stop (i.e., when they receive one of [`SIGSTOP`], [`SIGTSTP`], [`SIGTTIN`],
    /// or [`SIGTTOU`]) or resume (i.e., they receive [`SIGCONT`])
    /// (see [`wait(2)`]).
    ///
    /// This flag is meaningful only when establishing a handler for `SIGCHLD`.
    ///
    /// [`SIGCHLD`]: LINUX_SIGNAL::SIGCHLD
    /// [`SIGSTOP`]: LINUX_SIGNAL::SIGSTOP
    /// [`SIGTSTP`]: LINUX_SIGNAL::SIGTSTP
    /// [`SIGTTIN`]: LINUX_SIGNAL::SIGTTIN
    /// [`SIGTTOU`]: LINUX_SIGNAL::SIGTTOU
    /// [`SIGCONT`]: LINUX_SIGNAL::SIGCONT
    /// [`wait(2)`]: https://man7.org/linux/man-pages/man2/wait.2.html
    pub const SA_NOCLDSTOP: c_size_t = 0x0000_0001;

    /// If signum is [`SIGCHLD`], do not transform children into
    /// zombies when they terminate.
    ///
    /// See also [`waitpid(2)`].
    ///
    /// This flag is meaningful only when establishing a handler for `SIGCHLD`,
    /// or when setting that signal's disposition to [`SIG_DFL`].
    ///
    /// (since Linux 2.6)
    ///
    /// [`SIGCHLD`]: LINUX_SIGNAL::SIGCHLD
    /// [`SIG_DFL`]: LinuxSigaction::SIG_DFL
    /// [`waitpid(2)`]: https://man7.org/linux/man-pages/man2/wait.2.html
    pub const SA_NOCLDWAIT: c_size_t = 0x0000_0002;

    /// If the [`SA_NOCLDWAIT`] flag is set when establishing a handler for
    /// [`SIGCHLD`], POSIX.1 leaves it unspecified whether a `SIGCHLD` signal is
    /// generated when a child process terminates.
    ///
    /// On Linux, a `SIGCHLD` signal is generated in this case; on some other
    /// implementations, it is not.
    ///
    /// Do not add the signal to the thread's signal mask while the handler is
    /// executing, unless the signal is specified in act.sa_mask.  Consequently,
    /// a further instance of the signal may be delivered to the thread while it
    /// is executing the handler.  This flag is meaningful only when establishing
    /// a signal handler.
    ///
    /// `SA_NOMASK` is an obsolete, nonstandard synonym for this flag.
    ///
    /// [`SIGCHLD`]: LINUX_SIGNAL::SIGCHLD
    /// [`SA_NOCLDWAIT`]: Self::SA_NOCLDWAIT
    pub const SA_NODEFER: c_size_t = 0x4000_0000;

    /// Call the signal handler on an alternate signal stack provided by
    /// [`sigaltstack(2)`].
    ///
    /// If an alternate stack is not available, the default stack will be used.
    /// This flag is meaningful only when establishing a signal handler.
    ///
    /// [`sigaltstack(2)`]: https://man7.org/linux/man-pages/man2/sigaltstack.2.html
    pub const SA_ONSTACK: c_size_t = 0x0800_0000;

    /// Restore the signal action to the default upon entry to the signal handler.
    ///
    /// This flag is meaningful only when establishing a signal handler.
    ///
    /// `SA_ONESHOT` is an obsolete, nonstandard synonym for this flag.
    pub const SA_RESETHAND: c_size_t = 0x8000_0000;

    /// Provide behavior compatible with BSD signal semantics by making certain
    /// system calls restartable across signals. This flag is meaningful only
    /// when establishing a signal handler.  See [`signal(7)`] for a discussion of
    /// system call restarting.
    ///
    /// [`signal(7)`]: https://man7.org/linux/man-pages/man7/signal.7.html
    pub const SA_RESTART: c_size_t = 0x1000_0000;

    /// Not intended for application use.
    ///
    /// This flag is used by C libraries to indicate that the `sa_restorer` field contains
    /// the address of a "signal trampoline".  See [`sigreturn(2)`] for more details.
    ///
    /// [`sigreturn(2)`]: https://man7.org/linux/man-pages/man2/sigreturn.2.html
    pub const SA_RESTORER: c_size_t = 0x0400_0000;

    /// The signal handler takes three arguments, not one.
    ///
    /// In this case, `sa_sigaction` should be set instead of `sa_handler`.
    ///
    /// This flag is meaningful only when establishing a signal handler.
    ///
    /// (since Linux 2.2)
    pub const SA_SIGINFO: c_size_t = 0x0000_0004;
}
