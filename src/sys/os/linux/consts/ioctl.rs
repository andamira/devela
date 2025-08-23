// devela::sys::os::linux::consts::ioctl
//
//! Linux `ioctl` constants.
//
// https://man7.org/linux/man-pages/man4/tty_ioctl.4.html

use crate::{c_int, c_ulong};

/// Linux `ioctl` constants.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct LINUX_IOCTL;

/// # Get and set terminal attributes
/// ---
impl LINUX_IOCTL {
    /// Get the current serial port settings.
    ///
    /// Argument: struct termios *argp
    pub const TCGETS: c_ulong = 0x5401;

    /// Set the current serial port settings.
    ///
    /// Argument: const struct termios *argp
    pub const TCSETS: c_ulong = 0x5402;

    /// Allow the output buffer to drain, and set the current serial port settings.
    ///
    /// Argument: const struct termios *argp
    pub const TCSETSW: c_ulong = 0x5403;

    /// Allow the output buffer to drain, discard pending input,
    /// and set the current serial port settings.
    ///
    /// Argument: const struct termios *argp
    pub const TCSETSF: c_ulong = 0x5404;

    // The following four ioctls, added in Linux 2.6.20, are just like TCGETS,
    // TCSETS, TCSETSW, TCSETSF, except that they take a struct termios2 *
    // instead of a struct termios *.

    /// Get the current serial port settings. (*termios2*)
    ///
    /// Argument: struct termios2 *argp
    pub const TCGETS2: c_ulong = 0x802c_542a;

    /// Set the current serial port settings. (*termios2*)
    ///
    /// Argument: const struct termios2 *argp
    pub const TCSETS2: c_ulong = 0x402c_542b;

    /// Allow the output buffer to drain, and set the current serial port settings.
    /// (*termios2*)
    ///
    /// Argument: const struct termios2 *argp
    pub const TCSETSW2: c_ulong = 0x402c_542c;

    /// Allow the output buffer to drain, discard pending input,
    /// and set the current serial port settings. (*termios2*)
    ///
    /// Argument: const struct termios2 *argp
    pub const TCSETSF2: c_ulong = 0x402c_542d;

    // The following four ioctls are just like TCGETS, TCSETS, TCSETSW, TCSETSF,
    // except that they take a struct termio * instead of a struct termios *.

    /// Get the current serial port settings. (*termio*)
    ///
    /// Argument: struct termio *argp
    pub const TCGETA: c_ulong = 0x5405;

    /// Set the current serial port settings. (*termio*)
    ///
    /// Argument: const struct termio *argp
    pub const TCSETA: c_ulong = 0x5406;

    /// Allow the output buffer to drain, and set the current serial port settings.
    /// (*termio*)
    ///
    /// Argument: const struct termio *argp
    pub const TCSETAW: c_ulong = 0x5407;

    /// Allow the output buffer to drain, discard pending input,
    /// and set the current serial port settings. (*termio*)
    ///
    /// Argument: const struct termio *argp
    pub const TCSETAF: c_ulong = 0x5408;
}

/// # Software flow control
/// ---
impl LINUX_IOCTL {
    /// Equivalent to tcflow(fd, arg).
    ///
    /// Argument: int arg
    ///
    /// See tcflow(3) for the argument values [`TCOOFF`][Self::TCOOFF],
    /// [`TCOON`][Self::TCOON], [`TCIOFF`][Self::TCIOFF], [`TCION`][Self::TCION].
    pub const TCXONC: c_ulong = 0x540A;

    /// Suspends output.
    pub const TCOOFF: c_int = 0;
    /// Resumes suspended output.
    pub const TCOON: c_int = 1;
    /// Sends a STOP character to the terminal, to stop the terminal from sending
    /// any further input.
    pub const TCIOFF: c_int = 2;
    /// Sends a START character to the terminal, to tell the terminal that it can
    /// resume sending input.
    pub const TCION: c_int = 3;
}

/// # Exclusive mode
/// ---
impl LINUX_IOCTL {
    /// Put the terminal into exclusive mode.
    ///
    /// No further open(2) operations on the terminal are permitted.  (They fail
    /// with EBUSY, except for a process with the `CAP_SYS_ADMIN` capability.)
    pub const TIOCEXCL: c_ulong = 0x540C;

    /// Disable exclusive mode.
    pub const TIOCNXCL: c_ulong = 0x540D;

    /// (since Linux 3.8) If the terminal is currently in exclusive mode, place
    /// a nonzero value in the location pointed to by argp; otherwise, place
    /// zero in *argp.
    pub const TIOCGEXCL: c_ulong = 0x8004_5440;
}

/// # Faking input
/// ---
impl LINUX_IOCTL {
    /// Insert the given byte in the input queue.
    ///
    /// Argument: const char *argp
    pub const TIOCSTI: c_ulong = 0x5412;
}

/// # Get and set window size
/// ---
impl LINUX_IOCTL {
    /// Get window size.
    ///
    /// Argument: struct winsize *argp
    pub const TIOCGWINSZ: c_ulong = 0x5413;

    /// Set window size.
    ///
    /// Argument: const struct winsize *argp
    pub const TIOCSWINSZ: c_ulong = 0x5414;
}

/// # Modem control
/// ---
impl LINUX_IOCTL {
    /// Get the status of modem bits.
    ///
    /// Argument: int *argp
    pub const TIOCMGET: c_ulong = 0x5415;

    /// Set the indicated modem bits.
    ///
    /// Argument: const int *argp
    pub const TIOCMBIS: c_ulong = 0x5416;

    /// Clear the indicated modem bits.
    ///
    /// Argument: const int *argp
    pub const TIOCMBIC: c_ulong = 0x5417;

    /// Set the status of modem bits.
    ///
    /// Argument: const int *argp
    pub const TIOCMSET: c_ulong = 0x5418;

    /// DSR (data set ready/line enable)
    pub const TIOCM_LE: c_int = 0x001;
    /// DTR (data terminal ready)
    pub const TIOCM_DTR: c_int = 0x002;
    /// RTS (request to send)
    pub const TIOCM_RTS: c_int = 0x004;
    /// Secondary TXD (transmit)
    pub const TIOCM_ST: c_int = 0x008;
    /// Secondary RXD (receive)
    pub const TIOCM_SR: c_int = 0x010;
    /// CTS (clear to send)
    pub const TIOCM_CTS: c_int = 0x020;
    /// DCD (data carrier detect)
    pub const TIOCM_CAR: c_int = 0x040;
    /// see [`TIOCM_CAR`][Self::TIOCM_CAR]
    pub const TIOCM_CD: c_int = Self::TIOCM_CAR;
    /// RNG (ring)
    pub const TIOCM_RNG: c_int = 0x080;
    /// see [`TIOCM_RNG`][Self::TIOCM_RNG]
    pub const TIOCM_RI: c_int = Self::TIOCM_RNG;
    /// DSR (data set ready)
    pub const TIOCM_DSR: c_int = 0x100;
}

/// # Marking a line as local
/// ---
impl LINUX_IOCTL {
    /// ("Get software carrier flag")
    ///
    /// Get the status of the `CLOCAL` flag in the `c_cflag` field of the termios
    /// structure.
    ///
    /// Argument: int *argp
    pub const TIOCGSOFTCAR: c_ulong = 0x5419;

    /// ("Set software carrier flag")
    ///
    /// Set the `CLOCAL` flag in the termios structure when `*argp` is nonzero, and
    /// clear it otherwise.
    ///
    /// Argument: const int *argp
    pub const TIOCSSOFTCAR: c_ulong = 0x541A;
}

/// # Buffer count and flushing
/// ---
impl LINUX_IOCTL {
    /// Get the number of bytes in the input buffer.
    pub const FIONREAD: c_ulong = 0x541B;

    /// Same as [`FIONREAD`][Self::FIONREAD].
    pub const TIOCINQ: c_ulong = Self::FIONREAD;

    /// Get the number of bytes in the output buffer.
    pub const TIOCOUTQ: c_ulong = 0x5411;

    /// See tcflush(3) for the argument values `TCIFLUSH`, `TCOFLUSH`, `TCIOFLUSH`.
    ///
    /// Equivalent to tcflush(fd, arg).
    pub const TCFLSH: c_ulong = 0x540B;

    /// Get line status register.
    ///
    /// Status register has `TIOCSER_TEMT` bit set when output buffer is empty and
    /// also hardware transmitter is physically empty.
    pub const TIOCSERGETLSR: c_ulong = 0x5459;
}

/// # Linux-specific
/// ---
impl LINUX_IOCTL {
    /// Linux-specific ioctls.
    ///
    /// The action of the following ioctls depends on the first byte in
    /// the struct pointed to by argp, referred to here as the subcode.
    /// These are legal only for the superuser or the owner of the
    /// current terminal.  Symbolic subcodes are available in
    /// <linux/tiocl.h> since Linux 2.5.71.
    ///
    /// See <https://man7.org/linux/man-pages/man2/ioctl_console.2.html>.
    pub const TIOCLINUX: c_ulong = 0x541C;
}

/// # Redirecting console output
/// ---
impl LINUX_IOCTL {
    /// Redirect output that would have gone to `/dev/console` or `/dev/tty0` to
    /// the given terminal.
    ///
    /// Argument: void
    ///
    /// If that was a pseudoterminal master, send it to the slave. Before Linux
    /// 2.6.10, anybody can do this as long as the output was not redirected
    /// yet; since Linux 2.6.10, only a process with the `CAP_SYS_ADMIN`
    /// capability may do this. If output was redirected already, then EBUSY is
    /// returned, but redirection can be stopped by using this ioctl with fd
    /// pointing at /dev/console or /dev/tty0.
    pub const TIOCCONS: c_ulong = 0x541D;
}

/// # Pseudoterminal ioctls
/// ---
impl LINUX_IOCTL {
    /// Enable (when *argp is nonzero) or disable packet mode.
    ///
    /// Argument: const int *argp
    ///
    /// Can be applied to the master side of a pseudoterminal only (and will
    /// return ENOTTY otherwise).
    ///
    /// In packet mode, each subsequent read(2) will return a packet that either
    /// contains a single nonzero control byte, or has a single byte containing
    /// zero ('\0') followed by data written on the slave side of the
    /// pseudoterminal.
    ///
    /// If the first byte is not TIOCPKT_DATA (0), it is an OR of one or more of
    /// the following bits:
    ///
    /// - `TIOCPKT_FLUSHREAD`    The read queue for the terminal is flushed.
    /// - `TIOCPKT_FLUSHWRITE`   The write queue for the terminal is flushed.
    /// - `TIOCPKT_STOP`         Output to the terminal is stopped.
    /// - `TIOCPKT_START`        Output to the terminal is restarted.
    /// - `TIOCPKT_DOSTOP`       The start and stop characters are ^S/^Q.
    /// - `TIOCPKT_NOSTOP`       The start and stop characters are not ^S/^Q.
    pub const TIOCPKT: c_ulong = 0x5420;

    /// Set (if *argp is nonzero) or remove (if *argp is zero) the lock on the
    /// pseudoterminal slave device.
    ///
    /// Argument: int *argp
    pub const TIOCSPTLCK: c_ulong = 0x4004_5431;

    /// Return the current packet mode setting in the integer pointed to by
    /// argp.
    ///
    /// Argument: const int *argp
    pub const TIOCGPKT: c_ulong = 0x8004_5438;

    /// Place the current lock state of the pseudoterminal slave device in the
    /// location pointed to by argp.
    ///
    /// Argument: int *argp
    pub const TIOCGPTLCK: c_ulong = 0x8004_5439;

    /// Open and return a new file descriptor that refers to the peer
    /// pseudoterminal slave device.
    ///
    /// Argument: int flags
    ///
    /// Given a file descriptor in fd that refers to a pseudoterminal master,
    /// open (with the given open(2)-style flags) and return a new file
    /// descriptor that refers to the peer pseudoterminal slave device.
    ///
    /// This operation can be performed regardless of whether the pathname of
    /// the slave device is accessible through the calling process's mount
    /// namespace.
    pub const TIOCGPTPEER: c_ulong = 0x5441;
}

/// # Controlling terminal
/// ---
impl LINUX_IOCTL {
    /// Make the given terminal the controlling terminal of the calling process.
    ///
    /// Argument: int arg
    ///
    /// The calling process must be a session leader and not have a controlling
    /// terminal already. For this case, arg should be specified as zero.
    ///
    /// If this terminal is already the controlling terminal of a different
    /// session group, then the ioctl fails with `EPERM`, unless the caller has
    /// the `CAP_SYS_ADMIN` capability and arg equals 1, in which case the
    /// terminal is stolen, and all processes that had it as controlling
    /// terminal lose it.
    pub const TIOCSCTTY: c_ulong = 0x540E;

    /// Give up this controlling terminal.
    ///
    /// Argument: void
    ///
    /// If the process was session leader, then send `SIGHUP` and `SIGCONT` to
    /// the foreground process group and all processes in the current session
    /// lose their controlling terminal.
    pub const TIOCNOTTY: c_ulong = 0x5422;
}

/// # Line discipline
/// ---
impl LINUX_IOCTL {
    /// Get the line discipline of the terminal.
    ///
    /// Argument: int *argp
    pub const TIOCSETD: c_ulong = 0x5423;

    /// Set the line discipline of the terminal.
    ///
    /// Argument: const int *argp
    pub const TIOCGETD: c_ulong = 0x5424;
}

/// # Sending a break
/// ---
impl LINUX_IOCTL {
    /// Equivalent to tcsendbreak(fd, arg).
    ///
    /// Argument: int arg
    ///
    /// If the terminal is using asynchronous serial data transmission, and arg is
    /// zero, then send a break (a stream of zero bits) for between 0.25 and 0.5
    /// seconds.  If the terminal is not using asynchronous serial data
    /// transmission, then either a break is sent, or the function returns without
    /// doing anything.  When arg is nonzero, nobody knows what will happen.
    pub const TCSBRK: c_ulong = 0x5409;

    /// So-called "POSIX version" of [`TCSBRK`][Self::TCSBRK].
    ///
    /// It treats nonzero arg as a time interval measured in deciseconds, and does
    /// nothing when the driver does not support breaks.
    ///
    /// Argument: int arg
    pub const TCSBRKP: c_ulong = 0x5425;

    /// Turn break on, that is, start sending zero bits.
    ///
    /// Argument: void
    pub const TIOCSBRK: c_ulong = 0x5427;

    /// Turn break off, that is, stop sending zero bits.
    ///
    /// Argument: void
    pub const TIOCCBRK: c_ulong = 0x5428;
}

/// Process group and session ID
/// ---
impl LINUX_IOCTL {
    /// Get the session ID of the given terminal.
    ///
    /// Argument: pid_t *argp
    ///
    /// This fails with the error `ENOTTY` if the terminal is not a master
    /// pseudoterminal and not our controlling terminal.
    pub const TIOCGSID: c_ulong = 0x5429;

    /// Get the process group ID of the foreground process group on this terminal.
    ///
    /// Argument: pid_t *argp
    pub const TIOCGPGRP: c_ulong = 0x540F;

    /// Set the foreground process group ID of this terminal.
    ///
    /// Argument: const pid_t *argp
    pub const TIOCSPGRP: c_ulong = 0x5410;
}

/// # Miscellaneous
/// ---
#[allow(missing_docs)]
impl LINUX_IOCTL {
    /// Set non-blocking I/O mode if the argument is non-zero.
    ///
    /// In non-blocking mode, read(2) or write(2) calls return -1 and set errno to
    /// `EAGAIN` immediately when no data is available.
    pub const FIONBIO: c_ulong = 0x5421;

    pub const TIOCGSERIAL: c_ulong = 0x541E;
    pub const TIOCSSERIAL: c_ulong = 0x541F;

    pub const TIOCGRS485: c_ulong = 0x542E;
    pub const TIOCSRS485: c_ulong = 0x542F;
    pub const TIOCGPTN: c_ulong = 0x8004_5430;
    pub const TIOCGDEV: c_ulong = 0x8004_5432;
    pub const TCGETX: c_ulong = 0x5432;
    pub const TCSETX: c_ulong = 0x5433;
    pub const TCSETXF: c_ulong = 0x5434;
    pub const TCSETXW: c_ulong = 0x5435;
    pub const TIOCSIG: c_ulong = 0x4004_5436;
    pub const TIOCVHANGUP: c_ulong = 0x5437;
    // pub const TIOCGISO7816: c_ulong = 0x8028_5442;
    // pub const TIOCSISO7816: c_ulong = 0xc028_5443;
    pub const FIONCLEX: c_ulong = 0x5450;
    pub const FIOCLEX: c_ulong = 0x5451;
    pub const FIOASYNC: c_ulong = 0x5452;
    pub const TIOCSERCONFIG: c_ulong = 0x5453;
    pub const TIOCSERGWILD: c_ulong = 0x5454;
    pub const TIOCSERSWILD: c_ulong = 0x5455;
    pub const TIOCGLCKTRMIOS: c_ulong = 0x5456;
    pub const TIOCSLCKTRMIOS: c_ulong = 0x5457;
    pub const TIOCSERGSTRUCT: c_ulong = 0x5458;
    pub const TIOCSERGETMULTI: c_ulong = 0x545A;
    pub const TIOCSERSETMULTI: c_ulong = 0x545B;
    pub const TIOCMIWAIT: c_ulong = 0x545C;
    pub const TIOCGICOUNT: c_ulong = 0x545D;
    pub const BLKIOMIN: c_ulong = 0x1278;
    pub const BLKIOOPT: c_ulong = 0x1279;
    pub const BLKSSZGET: c_ulong = 0x1268;
    pub const BLKPBSZGET: c_ulong = 0x127B;
}
