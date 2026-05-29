// devela::sys::os::linux::io::term::_raw::output

#![allow(dead_code, non_camel_case_types, clippy::zero_prefixed_literal)]

use crate::c_uint;

#[doc = crate::_tags!(internal linux term)]
/// [`LinuxTermios`][crate::LinuxTermios] output flags.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
#[derive(Debug)]
pub(crate) struct LINUX_TERMIOS_OFLAG;

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
