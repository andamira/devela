// devela::sys::os::linux::process::signal::flags
//
//! Defines [`LinuxSigactionFlags`].
//

#[cfg(doc)]
use crate::LinuxSigaction;
use crate::{LINUX_SIGACTION as A, c_size_t, set};

set! {
    #[doc = crate::_tags!(linux signal)]
    /// [`LinuxSigaction`] input flags.
    #[doc = crate::_doc_meta!{
        location("sys/os/term/session"),
        test_size_of(LinuxSigactionFlags = 1|8),
    }]
    pub struct LinuxSigactionFlags(u8) {
        /// Do not notify when child processes stop or resume.
        SA_NOCLDSTOP = 0;
        /// Do not create zombies when child processes terminate.
        SA_NOCLDWAIT = 1;
        /// Do not mask the signal while its handler is running.
        SA_NODEFER = 2;
        /// Run the handler on an alternate signal stack.
        SA_ONSTACK = 3;
        /// Reset the signal action to the default on handler entry.
        SA_RESETHAND = 4;
        /// Restart certain interrupted system calls.
        SA_RESTART = 5;
        /// Use the explicit Linux signal-restorer trampoline.
        SA_RESTORER = 6;
        /// Use a three-argument `SA_SIGINFO` handler.
        SA_SIGINFO = 7;
    }
    impl {
        /// Returns flags from the raw Linux `sa_flags` word.
        #[must_use]
        pub const fn from_c_size_t(bits: c_size_t) -> Self {
            Self::new()
                .with_if(bits & (A::SA_NOCLDSTOP as c_size_t) != 0, Self::SA_NOCLDSTOP)
                .with_if(bits & (A::SA_NOCLDWAIT as c_size_t) != 0, Self::SA_NOCLDWAIT)
                .with_if(bits & (A::SA_NODEFER as c_size_t) != 0, Self::SA_NODEFER)
                .with_if(bits & (A::SA_ONSTACK as c_size_t) != 0, Self::SA_ONSTACK)
                .with_if(bits & (A::SA_RESETHAND as c_size_t) != 0, Self::SA_RESETHAND)
                .with_if(bits & (A::SA_RESTART as c_size_t) != 0, Self::SA_RESTART)
                .with_if(bits & (A::SA_RESTORER as c_size_t) != 0, Self::SA_RESTORER)
                .with_if(bits & (A::SA_SIGINFO as c_size_t) != 0, Self::SA_SIGINFO)
        }
        /// Returns flags as a raw Linux `sa_flags` word.
        #[must_use]
        pub const fn as_c_size_t(self) -> c_size_t {
            let mut bits = 0;
            if self.has(Self::SA_NOCLDSTOP) { bits |= A::SA_NOCLDSTOP as c_size_t; }
            if self.has(Self::SA_NOCLDWAIT) { bits |= A::SA_NOCLDWAIT as c_size_t; }
            if self.has(Self::SA_NODEFER)   { bits |= A::SA_NODEFER as c_size_t; }
            if self.has(Self::SA_ONSTACK)   { bits |= A::SA_ONSTACK as c_size_t; }
            if self.has(Self::SA_RESETHAND) { bits |= A::SA_RESETHAND as c_size_t; }
            if self.has(Self::SA_RESTART)   { bits |= A::SA_RESTART as c_size_t; }
            if self.has(Self::SA_RESTORER)  { bits |= A::SA_RESTORER as c_size_t; }
            if self.has(Self::SA_SIGINFO)   { bits |= A::SA_SIGINFO as c_size_t; }
            bits
        }
        const _ASSERT_RAW_VALUES: () = {
            assert!(Self::SA_NOCLDSTOP.as_c_size_t() == A::SA_NOCLDSTOP);
            assert!(Self::SA_NOCLDWAIT.as_c_size_t() == A::SA_NOCLDWAIT);
            assert!(Self::SA_NODEFER.as_c_size_t()   == A::SA_NODEFER);
            assert!(Self::SA_ONSTACK.as_c_size_t()   == A::SA_ONSTACK);
            assert!(Self::SA_RESETHAND.as_c_size_t() == A::SA_RESETHAND);
            assert!(Self::SA_RESTART.as_c_size_t()   == A::SA_RESTART);
            assert!(Self::SA_RESTORER.as_c_size_t()  == A::SA_RESTORER);
            assert!(Self::SA_SIGINFO.as_c_size_t()   == A::SA_SIGINFO);
        };
    }
}
