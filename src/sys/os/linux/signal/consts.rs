// devela::sys::os::linux::signal::consts
//
//! Signal numbers and action flags.
//!
//! Defines Linux signal identifiers and `sigaction`-related constants
//! used to configure signal behavior.
//!
//! Defines [`LINUX_SIGACTION`], [`LINUX_SIGNAL`].
//

#![allow(non_camel_case_types)]

use crate::c_int;

#[doc = crate::_tags!(linux)]
/// Linux flag constants for [`LinuxSigaction`][crate::sys::os::linux::LinuxSigaction].
#[doc = crate::_doc_location!("sys/os/linux")]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct LINUX_SIGACTION;

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
    pub const SA_NOCLDSTOP: usize = 0x0000_0001;

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
    /// [`SIG_DFL`]: crate::sys::os::linux::LinuxSigaction::SIG_DFL
    /// [`waitpid(2)`]: https://man7.org/linux/man-pages/man2/wait.2.html
    pub const SA_NOCLDWAIT: usize = 0x0000_0002;

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
    pub const SA_NODEFER: usize = 0x4000_0000;

    /// Call the signal handler on an alternate signal stack provided by
    /// [`sigaltstack(2)`].
    ///
    /// If an alternate stack is not available, the default stack will be used.
    /// This flag is meaningful only when establishing a signal handler.
    ///
    /// [`sigaltstack(2)`]: https://man7.org/linux/man-pages/man2/sigaltstack.2.html
    pub const SA_ONSTACK: usize = 0x0800_0000;

    /// Restore the signal action to the default upon entry to the signal handler.
    ///
    /// This flag is meaningful only when establishing a signal handler.
    ///
    /// `SA_ONESHOT` is an obsolete, nonstandard synonym for this flag.
    pub const SA_RESETHAND: usize = 0x8000_0000;

    /// Provide behavior compatible with BSD signal semantics by making certain
    /// system calls restartable across signals. This flag is meaningful only
    /// when establishing a signal handler.  See [`signal(7)`] for a discussion of
    /// system call restarting.
    ///
    /// [`signal(7)`]: https://man7.org/linux/man-pages/man7/signal.7.html
    pub const SA_RESTART: usize = 0x1000_0000;

    /// Not intended for application use.
    ///
    /// This flag is used by C libraries to indicate that the `sa_restorer` field contains
    /// the address of a "signal trampoline".  See [`sigreturn(2)`] for more details.
    ///
    /// [`sigreturn(2)`]: https://man7.org/linux/man-pages/man2/sigreturn.2.html
    pub const SA_RESTORER: usize = 0x0400_0000;

    /// The signal handler takes three arguments, not one.
    ///
    /// In this case, `sa_sigaction` should be set instead of `sa_handler`.
    ///
    /// This flag is meaningful only when establishing a signal handler.
    ///
    /// (since Linux 2.2)
    pub const SA_SIGINFO: usize = 0x0000_0004;
}

#[doc = crate::_tags!(linux)]
/// Linux standard signals constants.
#[doc = crate::_doc_location!("sys/os/linux")]
///
/// Each signal has a current disposition, which determines how the process
/// behaves when it is delivered the signal.
///
/// - `Term`:   Default action is to terminate the process.
/// - `Ign`:    Default action is to ignore the signal.
/// - `Core`:   Default action is to terminate the process and dump core (see core(5)).
/// - `Stop`:   Default action is to stop the process.
/// - `Cont`:   Default action is to continue the process if it is currently stopped.
///
/// The signals [`SIGKILL`] and [`SIGSTOP`] cannot be caught, blocked, or ignored.
///
/// # Info
/// - <https://man7.org/linux/man-pages/man7/signal.7.html>
///
/// [`SIGKILL`]: Self::SIGKILL
/// [`SIGSTOP`]: Self::SIGSTOP
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct LINUX_SIGNAL;

// 30 different signals
impl LINUX_SIGNAL {
    /// Hangup detected on controlling terminal or death of controlling process.
    ///
    /// Default action: `Term`.
    pub const SIGHUP: c_int = 1_i32;

    /// Interrupt from keyboard.
    ///
    /// Default action: `Term`.
    pub const SIGINT: c_int = 2;

    /// Quit from keyboard.
    ///
    /// Default action: `Core`.
    pub const SIGQUIT: c_int = 3;

    /// Illegal Instruction
    ///
    /// Default action: `Core`.
    pub const SIGILL: c_int = 4;

    /// Trace/breakpoint trap.
    ///
    /// Default action: `Core`.
    pub const SIGTRAP: c_int = 5;

    /// Abort signal from [`abort(3)`].
    ///
    /// Default action: `Core`.
    ///
    /// [`abort(3)`]: https://man7.org/linux/man-pages/man3/abort.3.html
    pub const SIGABRT: c_int = 6;

    /// IOT trap. A synonym for [`SIGABRT`][Self::SIGABRT].
    ///
    /// Default action: `Core`.
    pub const SIGIOT: c_int = Self::SIGABRT;

    /// Bus error (bad memory access)
    ///
    /// Default action: `Core`.
    pub const SIGBUS: c_int = 7;

    /// Floating-point exception.
    ///
    /// Default action: `Core`.
    pub const SIGFPE: c_int = 8;

    /// Kill signal.
    ///
    /// Default action: `Term`.
    pub const SIGKILL: c_int = 9;

    /// User-defined signal 1.
    ///
    /// Default action: `Term`.
    pub const SIGUSR1: c_int = 10;

    /// Invalid memory reference.
    ///
    /// Default action: `Core`.
    pub const SIGSEGV: c_int = 11;

    /// User-defined signal 2.
    ///
    /// Default action: `Term`.
    pub const SIGUSR2: c_int = 12;

    /// Broken pipe: write to pipe with no readers; see [`pipe(7)`].
    ///
    /// Default action: `Term`.
    ///
    /// [`pipe(7)`]: https://man7.org/linux/man-pages/man7/pipe.7.html
    pub const SIGPIPE: c_int = 13;

    /// Timer signal from [`alarm(2)`].
    ///
    /// Default action: `Term`.
    ///
    /// [`alarm(2)`]: https://man7.org/linux/man-pages/man2/alarm.2.html
    pub const SIGALRM: c_int = 14;

    /// Termination signal.
    ///
    /// Default action: `Term`.
    pub const SIGTERM: c_int = 15;

    /// Stack fault on coprocessor (unused).
    ///
    /// Default action: `Term`.
    pub const SIGSTKFLT: c_int = 16;

    /// Child stopped or terminated.
    ///
    /// Default action: `Ign`.
    pub const SIGCHLD: c_int = 17;

    /// A synonym for [`SIGCHLD`][Self::SIGCHLD].
    pub const SIGCLD: c_int = Self::SIGCHLD;

    /// Continue if stopped.
    ///
    /// Default action: `Cont`.
    pub const SIGCONT: c_int = 18;

    /// Stop process.
    ///
    /// Default action: `Stop`.
    pub const SIGSTOP: c_int = 19;

    /// Stop typed at terminal.
    ///
    /// Default action: `Stop`.
    pub const SIGTSTP: c_int = 20;

    /// Terminal input for background process.
    ///
    /// Default action: `Stop`.
    pub const SIGTTIN: c_int = 21;

    /// Terminal output for background process.
    ///
    /// Default action: `Stop`.
    pub const SIGTTOU: c_int = 22;

    /// Urgent condition on socket (4.2BSD).
    ///
    /// Default action: `Ign`.
    pub const SIGURG: c_int = 23;

    /// CPU time limit exceeded (4.2BSD); see [`setrlimit(2)`].
    ///
    /// Default action: `Core`.
    ///
    /// [`setrlimit(2)`]: https://man7.org/linux/man-pages/man2/setrlimit.2.html
    pub const SIGXCPU: c_int = 24;

    /// File size limit exceeded (4.2BSD); see [`setrlimit(2)`].
    ///
    /// Default action: `Core`.
    ///
    /// [`setrlimit(2)`]: https://man7.org/linux/man-pages/man2/setrlimit.2.html
    pub const SIGXFSZ: c_int = 25;

    /// Virtual alarm clock (4.2BSD).
    ///
    /// Default action: `Term`.
    pub const SIGVTALRM: c_int = 26;

    /// Profiling timer expired.
    ///
    /// Default action: `Term`.
    pub const SIGPROF: c_int = 27;

    /// Window resize signal (4.3BSD, Sun).
    ///
    /// Default action: `Ign`.
    pub const SIGWINCH: c_int = 28;

    /// I/O now possible (4.2BSD).
    ///
    /// Default action: `Term`.
    pub const SIGIO: c_int = 29;

    /// Pollable event (Sys V); synonym for [`SIGIO`][Self::SIGIO].
    pub const SIGPOLL: c_int = Self::SIGIO;

    /// Power failure (System V).
    ///
    /// Default action: `Term`.
    pub const SIGPWR: c_int = 30;

    /// A synonym for [`SIGPWR`][Self::SIGPWR].
    pub const SIGINFO: c_int = Self::SIGPWR;

    /// Bad system call (SVr4); see also [`seccomp(2)`].
    ///
    /// Default action: `Core`.
    ///
    /// [`seccomp(2)`]: https://man7.org/linux/man-pages/man2/seccomp.2.html
    pub const SIGSYS: c_int = 31;

    /// Synonymous with [`SIGSYS`][Self::SIGSYS].
    pub const SIGUNUSED: c_int = Self::SIGSYS;

    // /// Emulator trap.
    // ///
    // /// Default action: `Term`.
    // pub const SIGEMT: c_int = ?;

    // /// File lock lost (unused).
    // ///
    // /// Default action: `Term`.
    // pub const SIGLOST: c_int = ?
}
