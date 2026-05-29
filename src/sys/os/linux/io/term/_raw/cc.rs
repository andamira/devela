// devela::sys::os::linux::io::term::_raw::cc

use crate::c_uchar;

#[doc = crate::_tags!(linux term)]
/// [`LinuxTermios`][crate::LinuxTermios] special characters symbolic indices.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
#[derive(Debug)]
pub struct LINUX_TERMIOS_CC;

impl LINUX_TERMIOS_CC {
    /// Interrupt character (INTR).  Send a SIGINT signal.
    ///
    /// (003, ETX, Ctrl-C, or also 0177, DEL, rubout)
    ///
    /// Recognized when ISIG is set, and then not passed as input.
    pub const VINTR: c_uchar = 0;

    /// (034, FS, Ctrl-\) Quit character (QUIT).
    ///
    /// Send SIGQUIT signal.
    ///
    /// Recognized when ISIG is set, and then not passed as input.
    pub const VQUIT: c_uchar = 1;

    /// Erase character (ERASE).
    ///
    /// This erases the previous not-yet-erased character,
    /// but does not erase past EOF or beginning-of- line.
    ///
    /// (0177, DEL, rubout, or 010, BS, Ctrl-H, or also #)
    ///
    /// Recognized when ICANON is set, and then not passed as input.
    pub const VERASE: c_uchar = 2;

    /// Kill character (KILL).
    ///
    /// This erases the input since the last EOF or beginning-of-line.
    ///
    /// (025, NAK, Ctrl-U, or Ctrl-X, or also @)
    ///
    /// Recognized when ICANON is set, and then not passed as input.
    pub const VKILL: c_uchar = 3;

    /// End-of-file character (EOF).
    ///
    /// More precisely: this character causes the pending tty buffer to be sent
    /// to the waiting user program without waiting for end-of-line.
    ///
    /// If it is the first character of the line, the read(2) in the user
    /// program returns 0, which signifies end- of-file.
    ///
    /// (004, EOT, Ctrl-D)
    ///
    /// Recognized when ICANON is set, and then not passed as input.
    pub const VEOF: c_uchar = 4;

    /// Timeout in deciseconds for noncanonical read (TIME).
    pub const VTIME: c_uchar = 5;

    /// Minimum number of characters for noncanonical read (MIN).
    pub const VMIN: c_uchar = 6;

    /// Switch character (SWTCH).
    ///
    /// (not in POSIX; not supported under Linux; 0, NUL)
    ///
    /// Used in System V to switch shells in shell layers, a predecessor to shell job control.
    pub const VSWTC: c_uchar = 7;
    // pub const VSWTCH : c_uchar = 7;

    /// Start character (START).
    ///
    /// (021, DC1, Ctrl-Q)
    ///
    /// Restarts output stopped by the Stop character.
    ///
    /// Recognized when IXON is set, and then not passed as input.
    pub const VSTART: c_uchar = 8;

    /// Stop character (STOP).
    ///
    /// (023, DC3, Ctrl-S)
    /// Stop output until Start character typed.
    ///
    /// Recognized when IXON is set, and then not passed as input.
    pub const VSTOP: c_uchar = 9;

    /// Suspend character (SUSP).
    ///
    /// Send SIGTSTP signal.
    ///
    /// (032, SUB, Ctrl-Z)
    ///
    /// Recognized when ISIG is set, and then not passed as input.
    pub const VSUSP: c_uchar = 10;

    /// Additional end-of-line character (EOL).
    ///
    /// (0, NUL)
    ///
    /// Recognized when ICANON is set.
    pub const VEOL: c_uchar = 11;

    /// Reprint unread characters (REPRINT).
    ///
    /// (not in POSIX; 022, DC2, Ctrl-R)
    ///
    /// Recognized when ICANON and IEXTEN are set, and then not passed as input.
    pub const VREPRINT: c_uchar = 12;

    /// (not in POSIX; not supported under Linux; 017, SI, Ctrl-O)
    /// Toggle: start/stop discarding pending output.  Recognized
    /// when IEXTEN is set, and then not passed as input.
    pub const VDISCARD: c_uchar = 13;

    /// Word erase (WERASE).
    ///
    /// (not in POSIX; 027, ETB, Ctrl-W)
    ///
    /// Recognized when ICANON and IEXTEN are set, and then not passed as input.
    pub const VWERASE: c_uchar = 14;

    /// Literal next (LNEXT).
    ///
    /// Quotes the next input character, depriving it of a possible special meaning.
    ///
    /// (not in POSIX; 026, SYN, Ctrl-V)
    ///
    /// Recognized when IEXTEN is set, and then not passed as input.
    pub const VLNEXT: c_uchar = 15;

    /// Yet another end-of-line character (EOL2).
    ///
    /// (not in POSIX; 0, NUL)
    ///
    /// Recognized when ICANON is set.
    pub const VEOL2: c_uchar = 16;

    // Status character (STATUS).
    //
    // Display status information at terminal, including state of
    // foreground process and amount of CPU time it has consumed.
    // Also sends a SIGINFO signal (not supported on Linux) to the
    // foreground process group.
    //
    // (not in POSIX; not supported under Linux; status request: 024, DC4, Ctrl-T).
}
