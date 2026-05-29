// devela::sys::os::linux::io::term::_raw::control

use crate::c_uint;

#[doc = crate::_tags!(internal linux term)]
/// [`LinuxTermios`][crate::LinuxTermios] control flags.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
#[derive(Debug)]
pub(crate) struct LINUX_TERMIOS_CFLAG;

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
