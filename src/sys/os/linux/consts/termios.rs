// devela::sys::os::linux::consts::termios
//
//! linux [`termios`] flags
//!
//! [`termios`]: https://man7.org/linux/man-pages/man3/termios.3.html
//
// - https://man7.org/linux/man-pages/man0/termios.h.0p.html
// - https://man7.org/linux/man-pages/man1/stty.1.html
//

#![allow(non_camel_case_types, clippy::zero_prefixed_literal)]

use core::ffi::c_uint;

/// [`LinuxTermios`][crate::LinuxTermios] input flags.
#[derive(Debug)]
pub struct LINUX_TERMIOS_IFLAG;

/// [`LinuxTermios`][crate::LinuxTermios] output flags.
#[derive(Debug)]
pub struct LINUX_TERMIOS_OFLAG;

/// [`LinuxTermios`][crate::LinuxTermios] control flags.
#[derive(Debug)]
pub struct LINUX_TERMIOS_CFLAG;

/// [`LinuxTermios`][crate::LinuxTermios] local flags.
#[derive(Debug)]
pub struct LINUX_TERMIOS_LFLAG;

impl LINUX_TERMIOS_IFLAG {
    /// Ignore BREAK condition on input.
    pub const IGNBRK: c_uint = 0_000_001;

    /// Signal interrupt on BREAK.
    /// If [`IGNBRK`](Self::IGNBRK) is set, a BREAK is ignored.
    ///
    /// If it is not set but `BRKINT` is set, then a BREAK causes the input and
    /// output queues to be flushed, and if the terminal is the controlling
    /// terminal of a foreground process group, it will cause a `SIGINT` to be
    /// sent to this foreground process group.
    ///
    /// When neither `IGNBRK` nor `BRKINT` are set, a BREAK reads as a null byte
    /// ('\0'), except when [`PARMRK`](Self::PARMRK) is set, in which case it
    /// reads as the sequence `\377` `\0` `\0`.
    pub const BRKINT: c_uint = 0_000_002;

    /// Ignore framing errors and parity errors.
    pub const IGNPAR: c_uint = 0_000_004;

    /// Mark parity and framing errors.
    ///
    /// If this bit is set, input bytes with parity or framing
    /// errors are marked when passed to the program.
    ///
    /// This bit is meaningful only when [`INPCK`](Self::INPCK) is set and
    /// [`IGNPAR`](Self::IGNPAR) is not set.
    ///
    /// The way erroneous bytes are marked is with two preceding
    /// bytes, `\377` and `\0`.
    ///
    /// Thus, the program actually reads three bytes for one erroneous byte
    /// received from the terminal.
    ///
    /// If a valid byte has the value `\377`, and [`ISTRIP`](Self::ISTRIP) is
    /// not set, the program might confuse it with the prefix that marks a
    /// parity error.
    ///
    /// Therefore, a valid byte `\377` is passed to the program as two bytes,
    /// `\377` `\377`, in this case.
    ///
    /// If neither `IGNPAR` nor `PARMRK` is set, read a character with a parity
    /// error or framing error as `\0`.
    pub const PARMRK: c_uint = 0_000_010;

    /// Enable input parity checking.
    pub const INPCK: c_uint = 0_000_020;

    /// Strip off eighth bit.
    pub const ISTRIP: c_uint = 0_000_040;

    /// Translate `NL` to `CR` on input.
    pub const INLCR: c_uint = 0_000_100;

    /// Ignore `CR` (carriage return) on input.
    pub const IGNCR: c_uint = 0_000_200;

    /// Translate CR to NL (carriage return to newline) on input
    /// (unless [`IGNCR`](Self::IGNCR) is set).
    pub const ICRNL: c_uint = 0_000_400;

    /// Map uppercase characters to lowercase on input (not in POSIX).
    pub const IUCLC: c_uint = 0_001_000;

    /// Enable XON/XOFF flow control on output.
    pub const IXON: c_uint = 0_002_000;

    /// (XSI) Typing any character will restart stopped output.
    ///
    /// (The default is to allow just the START character to
    /// restart output.)
    pub const IXANY: c_uint = 0_004_000;

    /// Enable XON/XOFF flow control on input.
    pub const IXOFF: c_uint = 0_010_000;

    /// Ring bell when input queue is full (not in POSIX).
    ///
    /// Linux does not implement this bit, and acts as if it is always set.
    pub const IMAXBEL: c_uint = 0_020_000;

    /// Input is UTF8. (not in POSIX)
    ///
    /// This allows character-erase to be correctly performed in cooked mode.
    ///
    /// (since Linux 2.6.4)
    pub const IUTF8: c_uint = 0_040_000;
}

impl LINUX_TERMIOS_OFLAG {
    /// Enable implementation-defined output processing.
    pub const OPOST: c_uint = 0_000_001;

    /// Map lowercase characters to uppercase on output. (not in POSIX)
    pub const OLCUC: c_uint = 0_000_002;

    /// (XSI) Map NL to CR-NL on output.
    pub const ONLCR: c_uint = 0_000_004;

    /// Map CR to NL on output.
    pub const OCRNL: c_uint = 0_000_010;

    /// Don't output CR at column 0.
    pub const ONOCR: c_uint = 0_000_020;

    /// The `NL` character is assumed to do the carriage-return
    /// function; the kernel's idea of the current column is set
    /// to 0 after both `NL` and `CR`.
    pub const ONLRET: c_uint = 0_000_040;

    /// Send fill characters for a delay, rather than using a timed delay.
    pub const OFILL: c_uint = 0_000_100;

    /// Fill character is ASCII `DEL` (0177).
    ///
    /// If unset, fill character is ASCII `NUL` ('\0'). (Not implemented on Linux.)
    pub const OFDEL: c_uint = 0_000_200;

    /// Newline delay mask.
    /// Values are [`NL0`](Self::NL0) and [`NL1`](Self::NL1).
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE` or `_XOPEN_SOURCE`]
    pub const NLDLY: c_uint = 0_000_400;
    /// `NLDLY`: Newline type 0_.
    pub const NL0: c_uint = 0_000_000;
    /// `NLDLY`: Newline type 1.
    pub const NL1: c_uint = 0_000_400;

    /// Carriage return delay mask. Values are [`CR0`](Self::CR0),
    /// [`CR1`](Self::CR1), [`CR2`](Self::CR2), or [`CR3`](Self::CR3).
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE` or `_XOPEN_SOURCE`]
    pub const CRDLY: c_uint = 0_003_000;
    /// `CRDLY`: Carriage-return delay type 0.
    pub const CR0: c_uint = 0_000_000;
    /// `CRDLY`: Carriage-return delay type 1.
    pub const CR1: c_uint = 0_001_000;
    /// `CRDLY`: Carriage-return delay type 2.
    pub const CR2: c_uint = 0_002_000;
    /// `CRDLY`: Carriage-return delay type 3.
    pub const CR3: c_uint = 0_003_000;

    /// Horizontal tab delay mask.
    ///
    /// Values are [`TAB0`](Self::TAB0), [`TAB1`](Self::TAB1),
    /// [`TAB2`](Self::TAB2), [`TAB3`](Self::TAB3)
    /// (or `XTABS`, but see the BUGS section).
    ///
    /// A value of `TAB3`, that is, `XTABS`, expands tabs to spaces
    /// (with tab stops every eight columns).
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE` or `_XOPEN_SOURCE`]
    ///
    /// ## BUGS
    /// On the Alpha architecture before Linux 4.16 (and glibc before
    /// glibc 2.28), the `XTABS` value was different from `TAB3` and it was
    /// ignored by the `N_TTY` line discipline code of the terminal driver
    /// as a result (because as it wasn't part of the `TABDLY` mask).
    pub const TABDLY: c_uint = 0_014000;
    /// `TABDLY`: Horizontal-tab delay type 0.
    pub const TAB0: c_uint = 0_000_000;
    /// `TABDLY`: Horizontal-tab delay type 1.
    pub const TAB1: c_uint = 0_004_000;
    /// `TABDLY`: Horizontal-tab delay type 2.
    pub const TAB2: c_uint = 0_010_000;
    /// `TABDLY`: Horizontal-tab delay type 3.
    pub const TAB3: c_uint = 0_014_000;

    /// Backspace delay mask.
    /// Values are [`BS0`](Self::BS0) or [`BS1`](Self::BS1).
    ///
    /// (Has never been implemented.)
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE` or `_XOPEN_SOURCE`]
    pub const BSDLY: c_uint = 0_020_000;
    /// Backspace-delay type 0.
    pub const BS0: c_uint = 0_000_000;
    /// Backspace-delay type 1.
    pub const BS1: c_uint = 0_020_000;

    /// Vertical tab delay mask.
    /// Values are [`VT0`](Self::VT0) or [`VT1`](Self::VT1).
    pub const VTDLY: c_uint = 0_040_000;
    /// `VTDLY`: Vertical-tab delay type 0.
    pub const VT0: c_uint = 0_000_000;
    /// `VTDLY`: Vertical-tab delay type 1.
    pub const VT1: c_uint = 0_040_000;

    /// Form feed delay mask.
    /// Values are [`FF0`](Self::FF0) or [`FF1`](Self::FF1).
    ///
    /// [requires `_BSD_SOURCE` or `_SVID_SOURCE` or `_XOPEN_SOURCE`]
    pub const FFDLY: c_uint = 0_100_000;
    /// `FFDLY`: Form-feed delay type 0.
    pub const FF0: c_uint = 0_000_000;
    /// `FFDLY`: Form-feed delay type 1.
    pub const FF1: c_uint = 0_100_000;

    /// See: [`TAB3`](Self::TAB3)
    pub const XTABS: c_uint = 0_014_000;
}

impl LINUX_TERMIOS_CFLAG {
    /// Character size mask.
    /// Values are [`CS5`](Self::CS5), [`CS6`](Self::CS5), [`CS7`](Self::CS5),
    /// or [`CS8`](Self::CS5).
    pub const CSIZE: c_uint = 0_000_060;
    /// `CSIZE`: Character size mask type 5.
    pub const CS5: c_uint = 0_000_000;
    /// `CSIZE`: Character size mask type 6.
    pub const CS6: c_uint = 0_000_020;
    /// `CSIZE`: Character size mask type 7.
    pub const CS7: c_uint = 0_000_040;
    /// `CSIZE`: Character size mask type 8.
    pub const CS8: c_uint = 0_000_060;

    /// Set two stop bits, rather than one.
    pub const CSTOPB: c_uint = 0_000_100;

    /// Enable receiver.
    pub const CREAD: c_uint = 0_000_200;

    /// Enable parity generation on output and parity checking for input.
    pub const PARENB: c_uint = 0_000_400;

    /// If set, then parity for input and output is odd;
    /// otherwise even parity is used.
    pub const PARODD: c_uint = 0_001_000;

    /// Lower modem control lines after last process closes the
    /// device (hang up).
    pub const HUPCL: c_uint = 0_002_000;

    /// Ignore modem control lines.
    pub const CLOCAL: c_uint = 0_004_000;

    // (not in POSIX) Baud speed mask (4+1 bits).
    // [requires _BSD_SOURCE or _SVID_SOURCE]
    // CBAUD

    // (not in POSIX) Extra baud speed mask (1 bit), included in
    // CBAUD.  [requires _BSD_SOURCE or _SVID_SOURCE]
    //
    // (POSIX says that the baud speed is stored in the termios
    // structure without specifying where precisely, and provides
    // cfgetispeed() and cfsetispeed() for getting at it.  Some
    // systems use bits selected by CBAUD in c_cflag, other
    // systems use separate fields, for example, sg_ispeed and
    // sg_ospeed.)
    // CBAUDEX

    // (not in POSIX) Block output from a noncurrent shell layer.
    // For use by shl (shell layers).  (Not implemented on
    // Linux.)
    // LOBLK

    // (not in POSIX) Mask for input speeds.  The values for the
    // CIBAUD bits are the same as the values for the CBAUD bits,
    // shifted left IBSHIFT bits.  [requires _BSD_SOURCE or
    // _SVID_SOURCE] (Not implemented in glibc, supported on
    // Linux via TCGET* and TCSET* ioctls; see ioctl_tty(2))
    // CIBAUD

    // (not in POSIX) Use "stick" (mark/space) parity (supported
    // on certain serial devices): if PARODD is set, the parity
    // bit is always 1; if PARODD is not set, then the parity bit
    // is always 0.  [requires _BSD_SOURCE or _SVID_SOURCE]
    // CMSPAR

    // (not in POSIX) Enable RTS/CTS (hardware) flow control.
    // [requires _BSD_SOURCE or _SVID_SOURCE]
    // CRTSCTS
}

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
