// devela::sys::os::linux::io::term::flags
//
//!
//

use crate::{
    LINUX_TERMIOS_CFLAG as C, LINUX_TERMIOS_IFLAG as I, LINUX_TERMIOS_LFLAG as L,
    LINUX_TERMIOS_OFLAG as O,
};
use crate::{c_uint, set};
set! {
    #[doc = crate::_tags!(linux term)]
    /// [`LinuxTermios`][crate::LinuxTermios] input flags.
    #[doc = crate::_doc_meta!{
        location("sys/os/term/session"),
        test_size_of(LinuxTermiosInputFlags = 2|16),
    }]
    ///
    pub struct LinuxTermiosInputFlags(u16) {
        /// Ignore BREAK condition on input.
        IGNBRK  = 0;
        /// Signal interrupt on BREAK.
        BRKINT  = 1;
        /// Ignore framing errors and parity errors.
        IGNPAR  = 2;
        /// Mark parity and framing errors.
        PARMRK  = 3;
        /// Enable input parity checking.
        INPCK   = 4;
        /// Strip off eighth bit.
        ISTRIP  = 5;
        /// Translate `NL` to `CR` on input.
        INLCR   = 6;
        /// Ignore `CR` on input.
        IGNCR   = 7;
        /// Translate `CR` to `NL` on input.
        ICRNL   = 8;
        /// Map uppercase characters to lowercase on input.
        IUCLC   = 9;
        /// Enable XON/XOFF flow control on output.
        IXON    = 10;
        /// Any character restarts stopped output.
        IXANY   = 11;
        /// Enable XON/XOFF flow control on input.
        IXOFF   = 12;
        /// Ring bell when input queue is full.
        IMAXBEL = 13;
        /// Input is UTF-8.
        IUTF8   = 14;
    }
    impl {
        /// Returns flags from the raw Linux `c_iflag` word.
        #[must_use]
        pub const fn from_c_uint(bits: c_uint) -> Self {
            Self::from_bits(bits as u16)
        }
        /// Returns flags as a raw Linux `c_iflag` word.
        #[must_use]
        pub const fn as_c_uint(self) -> c_uint {
            self.bits() as c_uint
        }
        const _ASSERT_RAW_VALUES: () = {
            assert!(Self::IGNBRK.as_c_uint()  == I::IGNBRK);
            assert!(Self::BRKINT.as_c_uint()  == I::BRKINT);
            assert!(Self::IGNPAR.as_c_uint()  == I::IGNPAR);
            assert!(Self::PARMRK.as_c_uint()  == I::PARMRK);
            assert!(Self::INPCK.as_c_uint()   == I::INPCK);
            assert!(Self::ISTRIP.as_c_uint()  == I::ISTRIP);
            assert!(Self::INLCR.as_c_uint()   == I::INLCR);
            assert!(Self::IGNCR.as_c_uint()   == I::IGNCR);
            assert!(Self::ICRNL.as_c_uint()   == I::ICRNL);
            assert!(Self::IUCLC.as_c_uint()   == I::IUCLC);
            assert!(Self::IXON.as_c_uint()    == I::IXON);
            assert!(Self::IXANY.as_c_uint()   == I::IXANY);
            assert!(Self::IXOFF.as_c_uint()   == I::IXOFF);
            assert!(Self::IMAXBEL.as_c_uint() == I::IMAXBEL);
            assert!(Self::IUTF8.as_c_uint()   == I::IUTF8);
        };
    }
}
set! {
    #[doc = crate::_tags!(linux term)]
    /// [`LinuxTermios`][crate::LinuxTermios] output flags.
    #[doc = crate::_doc_meta!{
        location("sys/os/term/session"),
        test_size_of(LinuxTermiosOutputFlags = 4|32),
    }]
    ///
    /// NOTE: Some constants in this group are masked delay fields,
    /// not independent boolean flags. Prefer field-specific helpers
    /// when they exist instead of combining delay values manually.
    pub struct LinuxTermiosOutputFlags(u32) {
        /// Enable implementation-defined output processing.
        OPOST  = 0;
        /// Map lowercase characters to uppercase on output.
        OLCUC  = 1;
        /// Map `NL` to `CR-NL` on output.
        ONLCR  = 2;
        /// Map `CR` to `NL` on output.
        OCRNL  = 3;
        /// Don't output `CR` at column 0.
        ONOCR  = 4;
        /// `NL` performs carriage-return.
        ONLRET = 5;
        /// Send fill characters for a delay.
        OFILL  = 6;
        /// Fill character is ASCII `DEL`.
        OFDEL  = 7;

        /// Newline delay mask.
        NLDLY = 8;
        /// Newline delay type 1.
        NL1   = 8;

        /// Carriage-return delay mask.
        CRDLY = 9..=10;
        /// Carriage-return delay type 1.
        CR1   = 9;
        /// Carriage-return delay type 2.
        CR2   = 10;
        /// Carriage-return delay type 3.
        CR3   = 9..=10;

        /// Horizontal-tab delay mask.
        TABDLY = 11..=12;
        /// Horizontal-tab delay type 1.
        TAB1   = 11;
        /// Horizontal-tab delay type 2.
        TAB2   = 12;
        /// Horizontal-tab delay type 3.
        TAB3   = 11..=12;

        /// Backspace delay mask.
        BSDLY = 13;
        /// Backspace delay type 1.
        BS1   = 13;

        /// Vertical-tab delay mask.
        VTDLY = 14;
        /// Vertical-tab delay type 1.
        VT1   = 14;

        /// Form-feed delay mask.
        FFDLY = 15;
        /// Form-feed delay type 1.
        FF1   = 15;

        /// Expands tabs to spaces. Same value as [`TAB3`](Self::TAB3).
        XTABS = 11..=12;
    }
    impl {
        /// Newline delay type 0.
        pub const NL0: Self = Self::new();
        /// Carriage-return delay type 0.
        pub const CR0: Self = Self::new();
        /// Horizontal-tab delay type 0.
        pub const TAB0: Self = Self::new();
        /// Backspace delay type 0.
        pub const BS0: Self = Self::new();
        /// Vertical-tab delay type 0.
        pub const VT0: Self = Self::new();
        /// Form-feed delay type 0.
        pub const FF0: Self = Self::new();

        /// Returns flags from the raw Linux `c_oflag` word.
        #[must_use]
        pub const fn from_c_uint(bits: c_uint) -> Self {
            Self::from_bits(bits)
        }
        /// Returns flags as a raw Linux `c_oflag` word.
        #[must_use]
        pub const fn as_c_uint(self) -> c_uint {
            self.bits()
        }
        const _ASSERT_RAW_VALUES: () = {
            assert!(Self::OPOST.as_c_uint()  == O::OPOST);
            assert!(Self::OLCUC.as_c_uint()  == O::OLCUC);
            assert!(Self::ONLCR.as_c_uint()  == O::ONLCR);
            assert!(Self::OCRNL.as_c_uint()  == O::OCRNL);
            assert!(Self::ONOCR.as_c_uint()  == O::ONOCR);
            assert!(Self::ONLRET.as_c_uint() == O::ONLRET);
            assert!(Self::OFILL.as_c_uint()  == O::OFILL);
            assert!(Self::OFDEL.as_c_uint()  == O::OFDEL);

            assert!(Self::NLDLY.as_c_uint() == O::NLDLY);
            assert!(Self::NL0.as_c_uint()   == O::NL0);
            assert!(Self::NL1.as_c_uint()   == O::NL1);

            assert!(Self::CRDLY.as_c_uint() == O::CRDLY);
            assert!(Self::CR0.as_c_uint()   == O::CR0);
            assert!(Self::CR1.as_c_uint()   == O::CR1);
            assert!(Self::CR2.as_c_uint()   == O::CR2);
            assert!(Self::CR3.as_c_uint()   == O::CR3);

            assert!(Self::TABDLY.as_c_uint() == O::TABDLY);
            assert!(Self::TAB0.as_c_uint()   == O::TAB0);
            assert!(Self::TAB1.as_c_uint()   == O::TAB1);
            assert!(Self::TAB2.as_c_uint()   == O::TAB2);
            assert!(Self::TAB3.as_c_uint()   == O::TAB3);
            assert!(Self::XTABS.as_c_uint()  == O::XTABS);

            assert!(Self::BSDLY.as_c_uint() == O::BSDLY);
            assert!(Self::BS0.as_c_uint()   == O::BS0);
            assert!(Self::BS1.as_c_uint()   == O::BS1);

            assert!(Self::VTDLY.as_c_uint() == O::VTDLY);
            assert!(Self::VT0.as_c_uint()   == O::VT0);
            assert!(Self::VT1.as_c_uint()   == O::VT1);

            assert!(Self::FFDLY.as_c_uint() == O::FFDLY);
            assert!(Self::FF0.as_c_uint()   == O::FF0);
            assert!(Self::FF1.as_c_uint()   == O::FF1);
        };
    }
}
set! {
    #[doc = crate::_tags!(linux term)]
    /// [`LinuxTermios`][crate::LinuxTermios] control flags.
    #[doc = crate::_doc_meta!{
        location("sys/os/linux/io/term"),
        test_size_of(LinuxTermiosControlFlags = 2|16),
    }]
    /// NOTE: `CSIZE`/`CS5`/`CS6`/`CS7`/`CS8` are a masked character-size field,
    /// not independent flags. Prefer [`LinuxTermios::set_char_size`].
    pub struct LinuxTermiosControlFlags(u16) {
        /// Character size mask.
        CSIZE  = 4..=5;
        /// Character size 6 bits.
        CS6    = 4;
        /// Character size 7 bits.
        CS7    = 5;
        /// Character size 8 bits.
        CS8    = 4..=5;

        /// Set two stop bits, rather than one.
        CSTOPB = 6;
        /// Enable receiver.
        CREAD  = 7;
        /// Enable parity generation and checking.
        PARENB = 8;
        /// Odd parity instead of even parity.
        PARODD = 9;
        /// Hang up on last close.
        HUPCL  = 10;
        /// Ignore modem control lines.
        CLOCAL = 11;
    }
    impl {
        /// Character size 5 bits.
        pub const CS5: Self = Self::new();

        /// Returns flags from the raw Linux `c_cflag` word.
        #[must_use]
        pub const fn from_c_uint(bits: c_uint) -> Self {
            Self::from_bits(bits as u16)
        }
        /// Returns flags as a raw Linux `c_cflag` word.
        #[must_use]
        pub const fn as_c_uint(self) -> c_uint {
            self.bits() as c_uint
        }
        const _ASSERT_RAW_VALUES: () = {
            assert!(Self::CSIZE.as_c_uint()  == C::CSIZE);
            assert!(Self::CS5.as_c_uint()    == C::CS5);
            assert!(Self::CS6.as_c_uint()    == C::CS6);
            assert!(Self::CS7.as_c_uint()    == C::CS7);
            assert!(Self::CS8.as_c_uint()    == C::CS8);
            assert!(Self::CSTOPB.as_c_uint() == C::CSTOPB);
            assert!(Self::CREAD.as_c_uint()  == C::CREAD);
            assert!(Self::PARENB.as_c_uint() == C::PARENB);
            assert!(Self::PARODD.as_c_uint() == C::PARODD);
            assert!(Self::HUPCL.as_c_uint()  == C::HUPCL);
            assert!(Self::CLOCAL.as_c_uint() == C::CLOCAL);
        };
    }
}
set! {
    #[doc = crate::_tags!(linux term)]
    /// [`LinuxTermios`][crate::LinuxTermios] local flags.
    #[doc = crate::_doc_meta!{
        location("sys/os/linux/io/term"),
        test_size_of(LinuxTermiosLocalFlags = 4|32),
    }]
    pub struct LinuxTermiosLocalFlags(u32) {
        /// Enable signals.
        ISIG    = 0;
        /// Enable canonical mode.
        ICANON  = 1;
        /// Uppercase-only terminal compatibility mode.
        XCASE   = 2;
        /// Echo input characters.
        ECHO    = 3;
        /// Echo erase processing.
        ECHOE   = 4;
        /// Echo kill processing.
        ECHOK   = 5;
        /// Echo newline.
        ECHONL  = 6;
        /// Disable flushing when generating signals.
        NOFLSH  = 7;
        /// Send `SIGTTOU` for background output.
        TOSTOP  = 8;
        /// Echo control characters as `^X`.
        ECHOCTL = 9;
        /// Visual erase mode.
        ECHOPRT = 10;
        /// Echo line kill by erasing each character.
        ECHOKE  = 11;
        /// Output is being flushed.
        FLUSHO  = 12;
        /// Pending input reprint.
        PENDING = 14;
        /// Enable implementation-defined input processing.
        IEXTEN  = 15;
        /// External processing.
        EXTPROC = 16;
    }
    impl {
        /// Returns flags from the raw Linux `c_lflag` word.
        #[must_use]
        pub const fn from_c_uint(bits: c_uint) -> Self {
            Self::from_bits(bits)
        }
        /// Returns flags as a raw Linux `c_lflag` word.
        #[must_use]
        pub const fn as_c_uint(self) -> c_uint {
            self.bits()
        }
        const _ASSERT_RAW_VALUES: () = {
            assert!(Self::ISIG.as_c_uint()    == L::ISIG);
            assert!(Self::ICANON.as_c_uint()  == L::ICANON);
            assert!(Self::XCASE.as_c_uint()   == L::XCASE);
            assert!(Self::ECHO.as_c_uint()    == L::ECHO);
            assert!(Self::ECHOE.as_c_uint()   == L::ECHOE);
            assert!(Self::ECHOK.as_c_uint()   == L::ECHOK);
            assert!(Self::ECHONL.as_c_uint()  == L::ECHONL);
            assert!(Self::NOFLSH.as_c_uint()  == L::NOFLSH);
            assert!(Self::TOSTOP.as_c_uint()  == L::TOSTOP);
            assert!(Self::ECHOCTL.as_c_uint() == L::ECHOCTL);
            assert!(Self::ECHOPRT.as_c_uint() == L::ECHOPRT);
            assert!(Self::ECHOKE.as_c_uint()  == L::ECHOKE);
            assert!(Self::FLUSHO.as_c_uint()  == L::FLUSHO);
            assert!(Self::PENDING.as_c_uint() == L::PENDING);
            assert!(Self::IEXTEN.as_c_uint()  == L::IEXTEN);
            assert!(Self::EXTPROC.as_c_uint() == L::EXTPROC);
        };
    }
}
