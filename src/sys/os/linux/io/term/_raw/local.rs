// devela::sys::os::linux::io::term::_raw::local
//
#![allow(dead_code, non_camel_case_types, clippy::zero_prefixed_literal)]

use crate::c_uint;

#[doc = crate::_tags!(internal linux term)]
/// [`LinuxTermios`][crate::LinuxTermios] local flags.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
#[derive(Debug)]
pub(crate) struct LINUX_TERMIOS_LFLAG;

impl LINUX_TERMIOS_LFLAG {
    /// Enable signals.
    ///
    /// When any of the characters `INTR`, `QUIT`, `SUSP`, or `DSUSP` are
    /// received, generate the corresponding signal.
    pub const ISIG: c_uint = 0_000_001;

    /// Enable canonical mode (erase and kill processing).
    pub const ICANON: c_uint = 0_000_002;

    /// If ICANON is also set, terminal is uppercase only.
    /// (not in POSIX; not supported under Linux)
    ///
    /// Input is converted to lowercase, except for characters preceded by `\`.
    ///
    /// On output, uppercase characters are preceded by `\` and lowercase characters
    /// are converted to uppercase.
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE` or `_XOPEN_SOURCE`]
    pub const XCASE: c_uint = 0_000_004;

    /// Echo input characters.
    pub const ECHO: c_uint = 0_000_010;

    /// If [`ICANON`](Self::ICANON) is also set, the `ERASE` character erases
    /// the preceding input character, and `WERASE` erases the preceding word.
    pub const ECHOE: c_uint = 0_000_020;

    /// If [`ICANON`](Self::ICANON) is also set, the `KILL` character
    /// erases the current line.
    pub const ECHOK: c_uint = 0_000_040;

    /// If [`ICANON`](Self::ICANON) is also set, echo the `NL` character
    /// even if [`ECHO`](Self::ECHO) is not set.
    pub const ECHONL: c_uint = 0_000_100;

    /// Disable flushing the input and output queues when
    /// generating signals for the `INT`, `QUIT`, and `SUSP` characters.
    pub const NOFLSH: c_uint = 0_000_200;

    /// Send the `SIGTTOU` signal to the process group of a background process
    /// which tries to write to its controlling terminal.
    pub const TOSTOP: c_uint = 0_000_400;

    /// If [`ECHO`](Self::ECHO) is also set, terminal special characters
    /// other than `TAB`, `NL`, `START`, and `STOP` are echoed as `^X`,
    /// where `X` is the character with ASCII code `0x40` greater than
    /// the special character.
    ///
    /// (not in POSIX)
    ///
    /// For example, character `0x08` (BS) is echoed as `^H`.
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE`]
    pub const ECHOCTL: c_uint = 0_001_000;

    /// If [`ICANON`](Self::ICANON) and [`ECHO`](Self::ECHO) are also set,
    /// characters are printed as they are being erased.
    ///
    /// (not in POSIX)
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE`]
    pub const ECHOPRT: c_uint = 0_002_000;

    /// If [`ICANON`](Self::ICANON) is also set, `KILL` is echoed by erasing
    /// each character on the line, as specified by [`ECHOE`](Self::ECHOE)
    /// and [`ECHOPRT`](Self::ECHOPRT).
    ///
    /// (not in POSIX)
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE`]
    pub const ECHOKE: c_uint = 0_004_000;

    // /// Echo only when a process is reading.
    // ///
    // /// (not in POSIX) (Not implemented on Linux)
    // pub const DEFECHO: c_uint = ?;

    /// Output is being flushed.
    ///
    /// This flag is toggled by typing the `DISCARD` character.
    ///
    /// (not in POSIX; not supported under Linux)
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE`]
    pub const FLUSHO: c_uint = 0_010_000;

    /// All characters in the input queue are reprinted
    /// when the next character is read.
    ///
    /// (not in POSIX; not supported under Linux)
    ///
    /// (bash(1) handles typeahead this way.)
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE`]
    pub const PENDING: c_uint = 0_040_000;

    /// Enable implementation-defined input processing.
    ///
    /// This flag, as well as [`ICANON`](Self::ICANON) must be enabled for
    /// the special characters `EOL2`, `LNEXT`, `REPRINT`, `WERASE` to be
    /// interpreted, and for the [`IUCLC`](LINUX_TERMIOS_IFLAG::IUCLC) flag
    /// to be effective.
    pub const IEXTEN: c_uint = 0_100_000;

    /// enable "LINEMODE"; useful with high latency links
    pub const EXTPROC: c_uint = 0_200_000;
}
