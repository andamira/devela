// devela::sys::os::linux::namespace
//
//! Defines the [`Linux`] namespace.
//

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
use crate::{
    c_uint, iif, transmute, AtomicOrdering, AtomicPtr, Duration, LinuxSigaction, LinuxSigset,
    LinuxTimespec, Ordering, Ptr, _core::str::from_utf8_unchecked, LINUX_ERRNO as ERRNO,
    LINUX_FILENO as FILENO, LINUX_SIGACTION as SIGACTION,
};

#[doc = crate::TAG_NAMESPACE!()]
/// Linux-related operations.
///
/// # Features
/// All the methods depend on the features `linux` and `unsafe_syscall`.
///
/// # Methods
/// - [read](#read-related-methods)
/// - [write](#write-related-methods)
/// - [thread](#thread-related-methods)
/// - [signal](#signaling-related-methods)
/// - [random](#randomness-related-methods)
/// - [syscalls](#system-calls)
pub struct Linux;

/// # Read-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Gets a single byte from *stdin*.
    ///
    /// This function makes use of the [`Linux::sys_read`] syscall to read a byte.
    ///
    /// # Error Handling
    /// If the read fails, it prints an error message and exits with status code 11.
    pub fn get_byte() -> u8 {
        let mut c = 0;
        loop {
            let n = unsafe { Linux::sys_read(FILENO::STDIN, &mut c as *mut u8, 1) };
            if n < 0 { Linux::print("read failed"); unsafe { Linux::sys_exit(11) }; }
            if n == 1 { break; }
        }
        c
    }

    /// Pauses execution until receiving from *stdin* any `char` in the `list`.
    ///
    /// # Error Handling
    /// If the read fails, it prints an error message and exits with status code 11.
    pub fn pause_until_char(list: &[char]) {
        loop { if list.contains(&Linux::get_dirty_char()) { break; } }
    }

    /// Gets a single `char` from *stdin*,
    /// or `None` if the bytes are not valid UTF-8.
    ///
    /// # Error Handling
    /// If the read fails, it prints an error message and exits with status code 11.
    pub fn get_char() -> Option<char> {
        let bytes = Linux::get_utf8_bytes()?;
        let s = unsafe { from_utf8_unchecked(&bytes) };
        Some(s.chars().next().unwrap())
    }

    /// Gets either a single `char` from *stdin*, or the replacement character.
    ///
    /// If the bytes received doesn't form a valid unicode scalar then the
    /// [replacement character (�)][char::REPLACEMENT_CHARACTER] will be returned.
    ///
    /// # Error Handling
    /// If the read fails, it prints an error message and exits with status code 11.
    pub fn get_dirty_char() -> char {
        match Linux::get_utf8_bytes() {
            Some(bytes) => {
                let s = unsafe { from_utf8_unchecked(&bytes) };
                s.chars().next().unwrap()
            }
            None => char::REPLACEMENT_CHARACTER,
        }
    }

    /// Gets a UTF-8 encoded byte sequence from *stdin* representing a `char`.
    ///
    /// Returns `None` if the bytes does not form a valid unicode scalar.
    ///
    /// # Error Handling
    /// If the read fails, it prints an error message and exits with status code 11.
    pub fn get_utf8_bytes() -> Option<[u8; 4]> {
        let mut bytes = [0u8; 4];
        let len;
        // read the first byte to determine the length of the character
        bytes[0] = Linux::get_byte();
        if bytes[0] & 0x80 == 0 { return Some([bytes[0], 0, 0, 0]); } // return ASCII immediately
        else if bytes[0] & 0xE0 == 0xC0 { len = 2; }
        else if bytes[0] & 0xF0 == 0xE0 { len = 3; }
        else if bytes[0] & 0xF8 == 0xF0 { len = 4; }
        else { return None; } // Not a valid first byte of a UTF-8 character
        // read the remaining bytes of the character
        for i in 1..len {
            bytes[i as usize] = Linux::get_byte();
            if bytes[i as usize] & 0xC0 != 0x80 { return None; } // Not a valid continuation byte
        }
        Some(bytes)
    }

    /// Prompts the user for a string from *stdin*, backed by a `buffer`.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut name_buffer = [0_u8; 32];
    /// let name: &str = Linux::prompt::<32>("Enter your name: ", &mut name_buffer);
    /// ```
    ///
    /// # Error Handling
    /// - If the write fails, it prints an error message and exits with status code 10.
    /// - If the read fails, it prints an error message and exits with status code 11.
    pub fn prompt<'input, const CAP: usize>(
        text: &str,
        buffer: &'input mut [u8; CAP],
    ) -> &'input str { Linux::print(text); Linux::get_line(buffer) }

    /// Gets a string from *stdin* backed by a `buffer`, until a newline.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut buf = [0_u8; 32];
    /// let name: &str = Linux::get_line::<32>(&mut buf);
    /// ```
    ///
    /// # Error handling
    /// If the read fails, it prints an error message and exits with status code 11.
    pub fn get_line<const CAP: usize>(buffer: &mut [u8; CAP]) -> &str {
        Linux::get_str(buffer, '\n')
    }

    /// Gets a string from *stdin* backed by a `buffer`,
    /// until the `stop` char is received.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut buf = [0_u8; 32];
    /// let name: &str = Linux::get_str::<32>(&mut buf, '\n');
    /// ```
    pub fn get_str<const CAP: usize>(buffer: &mut [u8; CAP], stop: char) -> &str {
        let mut index = 0;
        loop {
            if let Some(c) = Linux::get_char() {
                let mut c_buf = [0; 4];
                let c_str = c.encode_utf8(&mut c_buf);
                if c == stop {
                    break;
                } else if index + c_str.len() <= CAP {
                    Linux::print(c_str);
                    for &b in c_str.as_bytes() {
                        buffer[index] = b;
                        index += 1;
                    }
                }
            }
        }
        unsafe { from_utf8_unchecked(&buffer[..index]) }
    }
}

/// # Write-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Prints a string slice to standard output.
    ///
    /// This function makes use of [`Linux::sys_write`].
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    pub fn print(s: &str) {
        let mut s = s.as_bytes();
        while !s.is_empty() {
            let n = unsafe { Linux::sys_write(FILENO::STDOUT, s.as_ptr(), s.len()) };
            if n < 0 || n as usize > s.len() {
                Linux::print("write failed");
                unsafe { Linux::sys_exit(10) };
            }
            s = &s[n as usize..]; // update the byte slice to exclude the written bytes
        }
    }

    /// Prints a string slice to standard output, with a newline.
    ///
    /// This function makes use of [`Linux::sys_write`].
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    pub fn println(s: &str) { Linux::print(s); Linux::print("\n"); }

    /// Prints a string slice to standard error.
    ///
    /// This function makes use of [`Linux::sys_write`].
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    pub fn eprint(s: &str) {
        let mut s = s.as_bytes();
        while !s.is_empty() {
            let n = unsafe { Linux::sys_write(FILENO::STDERR, s.as_ptr(), s.len()) };
            if n < 0 || n as usize > s.len() {
                Linux::print("write failed");
                unsafe { Linux::sys_exit(10) };
            }
            s = &s[n as usize..]; // update the byte slice to exclude the written bytes
        }
    }

    /// Prints a string slice to standard error, with a newline.
    ///
    /// This function makes use of the [`Linux::sys_write`] syscall to print a string.
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    pub fn eprintln(s: &str) { Linux::eprint(s); Linux::eprint("\n"); }

    /// Prints a byte slice to *stdout*.
    ///
    /// This function makes use of the [`Linux::sys_write`] syscall to print bytes.
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    pub fn print_bytes(b: &[u8]) {
        let mut b = b;
        while !b.is_empty() {
            let n = unsafe { Linux::sys_write(FILENO::STDOUT, b.as_ptr(), b.len()) };
            if n < 0 || n as usize > b.len() {
                Linux::print("write failed");
                unsafe { Linux::sys_exit(10) };
            }
            b = &b[n as usize..]; // update the byte slice to exclude the written bytes
        }
    }
}

/// Thread-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
// IMPROVE: use a TAG
// #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
impl Linux {
    /// Suspends execution of calling thread for `duration`.
    ///
    /// This function makes use of the [`Linux::sys_nanosleep`] syscall.
    pub fn sleep(duration: Duration) {
        let mut req = LinuxTimespec::with(duration);
        let mut rem = LinuxTimespec::default();
        loop {
            let n = unsafe { Linux::sys_nanosleep(req.as_ptr(), rem.as_mut_ptr()) };
            match n.cmp(&0) {
                Ordering::Less => {
                    Linux::print("nanosleep failed");
                    unsafe { Linux::sys_exit(13) };
                }
                Ordering::Equal => break,
                Ordering::Greater => req = rem,
            }
        }
    }

    /// Returns the current process number.
    ///
    /// This function makes use of the [`Linux::sys_getpid`] syscall.
    pub fn getpid() -> i32 {
        unsafe { Linux::sys_getpid() }
    }
}

/// # Signaling-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Registers multiple signals using a handler function that never returns.
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// use devela::{Linux, LINUX_SIGNAL as LS, sleep4, ExtProcess, Process};
    /// fn handler(sig: i32) -> ! {
    ///    println!("\nsignal `{sig}` received! exiting. . .");
    ///    Process::exit(1);
    /// }
    /// fn main() {
    ///     // handle all the signals used to quit
    ///     Linux::sig_handler_no_return(handler, &[LS::SIGINT, LS::SIGQUIT, LS::SIGSEGV, LS::SIGABRT]);
    ///     // press Ctrl+C before the time expires to catch the SIGINT signal
    ///     sleep4!(2);
    ///     println!("bye");
    /// }
    /// # }
    /// ```
    ///
    /// # Rationale
    /// It would be very nice to be able to register a signal handler that can return,
    /// unfortunately I've been unable to make it work.
    ///
    /// Apparently the handler needs the [`SA_RESTORER`] flag to run, but doing so
    /// without providing a restorer function produces a segmentation fault. The only
    /// way to simply avoid that is to not return from the handler function.
    ///
    /// The `libc` library sets it up correctly but doing so manually seems a too
    /// complex too low level task.
    ///
    /// [`SA_RESTORER`]: SIGACTION::SA_RESTORER
    pub fn sig_handler_no_return(handler: fn(i32) -> !, signals: &[i32]) {
        // We store the given `handler` function in a static to be able to call it
        // from the new extern function which can't capture its environment.
        static HANDLER: AtomicPtr<fn(i32) -> !> = AtomicPtr::new(Ptr::null_mut());
        HANDLER.store(handler as *mut _, AtomicOrdering::SeqCst);

        extern "C" fn c_handler(sig: i32) {
            let handler = HANDLER.load(AtomicOrdering::SeqCst);
            if !handler.is_null() {
                #[allow(clippy::crosspointer_transmute)]
                // SAFETY: The non-null pointer is originally created from a `fn(i32) -> !` pointer.
                let handler = unsafe { transmute::<*mut fn(i32) -> !, fn(i32) -> !>(handler) };
                handler(sig);
            }
        }
        // Apparently Rust doesn't call the handler unless we set the SA_RESTORER flag.
        let flags = SIGACTION::SA_RESETHAND | SIGACTION::SA_RESTORER;
        let mask = LinuxSigset::default();
        let sigaction = LinuxSigaction::new(c_handler, flags, mask);
        for s in signals {
            if (1..=36).contains(s) { // make sure the signal is a valid number
                unsafe {
                    let _ = Linux::sys_rt_sigaction(*s, &sigaction,
                        Ptr::null_mut(), LinuxSigset::size());
                }
            }
        }
    }
}

/// Generates a `random_*` function for each given integer primitive
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
macro_rules! impl_random_fns {
    // $prim: the unsigned integer primitive
    // $len:  the length of the primitive in bytes
    ($($prim:ident : $len:literal),+) => { $crate::paste! { $(
        #[doc = "Generates a random `" $prim "` value that may not be criptographically secure."]
        pub fn [<random_ $prim>]() -> $prim {
            let mut r = [0; $len];
            let mut attempts = 0;
            loop {
                let n = unsafe { Linux::sys_getrandom(r.as_mut_ptr(), $len, Linux::RAND_FLAGS) };
                if n == $len {
                    // hot path!
                    break;
                } else if n == -ERRNO::EAGAIN {
                    iif![!Linux::getrandom_try_again_cold(&mut attempts); break];
                } else { // n < 0
                    Linux::getrandom_failed_cold();
                }
            }
            $prim::from_ne_bytes(r)
        }
    )+ }};
}

/// # Randomness-related methods.
///
/// They makes use of the `GRND_NONBLOCK` and `GRND_INSECURE` flags. So when the randomness
/// source is not ready, instead of blocking it may return less secure data in linux >= 5.6
/// or retry it a certain number of times, or even return 0 in some cases.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    impl_random_fns![u8:1, u16:2, u32:4, u64:8, u128:16];

    /// Fills the given `buffer` with random bytes that may not be cryptographically secure.
    ///
    /// # Panics
    /// Panics in debug if `buffer.len() > `[`isize::MAX`]
    pub fn random_bytes(buffer: &mut [u8]) {
        debug_assert![buffer.len() <= isize::MAX as usize];
        let (mut attempts, mut offset) = (0, 0);
        while offset < buffer.len() {
            let n = unsafe {
                Linux::sys_getrandom(buffer[offset..].as_mut_ptr(),
                    buffer.len() - offset, Linux::RAND_FLAGS)
            };
            if n == -ERRNO::EAGAIN {
                iif![!Linux::getrandom_try_again_cold(&mut attempts); break];
            } else if n < 0 {
                Linux::getrandom_failed_cold();
            } else {
                // hot path!
                offset += n as usize;
            }
        }
    }

    // from `sys/random.h`
    const GRND_NONBLOCK: c_uint = 0x0001;
    // const GRND_RANDOM: isize = 0x0002;
    const GRND_INSECURE: c_uint = 0x0004;
    const RAND_FLAGS: c_uint = Linux::GRND_NONBLOCK | Linux::GRND_INSECURE;
    ///
    const RAND_MAX_ATTEMPTS: usize = 15;

    /// the cold path for trying again
    #[cold] #[must_use]
    fn getrandom_try_again_cold(attempts: &mut usize) -> bool {
        // if *attempts >= Linux::RAND_MAX_ATTEMPTS { Linux::getrandom_failed_cold(); }
        *attempts += 1;
        *attempts <= Linux::RAND_MAX_ATTEMPTS
    }
    /// the cold path for some other error
    #[cold]
    fn getrandom_failed_cold() { Linux::print("getrandom failed"); unsafe { Linux::sys_exit(12); } }
}
