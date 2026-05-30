// devela::sys::os::linux::process::signal::_raw::signal
//
//! Defines [`LINUX_SIGNAL`].
//

use crate::c_int;

#[doc = crate::_tags!(linux signal abi)]
/// Linux standard signals constants.
#[doc = crate::_doc_meta!{location("sys/os/linux/process")}]
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
