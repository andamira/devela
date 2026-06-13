// devela/src/sys/os/linux/io/term/_raw/input.rs

use crate::c_uint;

#[doc = crate::_tags!(internal linux term)]
/// [`LinuxTermios`][crate::LinuxTermios] input flags.
#[derive(Debug)]
pub(crate) struct LINUX_TERMIOS_IFLAG;

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
