// devela::sys::os::linux::namespace
//
//! Defines the [`Linux`] namespace.
//

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
use crate::{
    c_uint, iif, transmute, AtomicOrdering, AtomicPtr, Duration, LinuxError, LinuxResult as Result,
    LinuxSigaction, LinuxSigset, LinuxTerminalSize, LinuxTermios, LinuxTimespec, Ordering, Ptr,
    ScopeGuard, _core::str::from_utf8_unchecked, LINUX_ERRNO as ERRNO, LINUX_FILENO as FILENO,
    LINUX_IOCTL as IOCTL, LINUX_SIGACTION as SIGACTION,
};
#[cfg(feature = "alloc")]
use crate::{vec_ as vec, Vec};

#[doc = crate::TAG_NAMESPACE!()]
/// Linux-related operations.
///
/// # Features
/// All the methods depend on the features `linux` and `unsafe_syscall`.
///
/// # Methods
/// - [read](#read-related-methods)
/// - [write](#write-related-methods)
/// - [term](#terminal-related-methods)
/// - [thread](#thread-related-methods)
/// - [signal](#signaling-related-methods)
/// - [random](#randomness-related-methods)
/// - [syscalls](#system-calls)
pub struct Linux;

/// # Read-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Checks if there is input available to read from stdin.
    pub fn has_input() -> bool { Self::available_bytes().unwrap_or(0) > 0 }

    /// Returns the number of bytes available to be read from *stdin*.
    pub fn available_bytes() -> Result<usize> {
        let mut n = 0;
        let result = unsafe {
            Linux::sys_ioctl(FILENO::STDIN, IOCTL::FIONREAD, &mut n as *mut i32 as *mut u8)
        };
        if result < 0 { Err(LinuxError::Sys(result)) } else { Ok(n as usize) }
    }

    /// Gets a single byte from *stdin*.
    pub fn get_byte() -> Result<u8> {
        let mut c = 0;
        loop {
            let n = unsafe { Linux::sys_read(FILENO::STDIN, &mut c as *mut u8, 1) };
            if n < 0 { return Err(LinuxError::Sys(n)); }
            if n == 1 { break; }
        }
        Ok(c)
    }

    /// Attempts to get a single byte from stdin without blocking.
    ///
    /// Checks if there is input available using `has_input`,
    /// and if so, reads a byte using `get_byte`.
    ///
    /// # Returns
    /// - `Some(u8)` if a byte is successfully read.
    /// - `None` if no data is available.
    pub fn try_get_byte() -> Option<u8> {
        if Linux::has_input() { Linux::get_byte().ok() } else { None }
    }

    /// Pauses execution until receiving from *stdin* any `char` in the `list`.
    pub fn pause_until_char(list: &[char]) {
        loop { if list.contains(&Linux::get_dirty_char()) { break; } }
    }

    /// Gets a single `char` from *stdin*.
    ///
    /// # Returns
    /// - `Ok(char)` if a valid UTF-8 character is read.
    /// - `Err(LinuxError::InvalidUtf8)` if the bytes are not valid UTF-8.
    /// - `Err(LinuxError::NoInput)` if the UTF-8 sequence is valid but empty.
    pub fn get_char() -> Result<char> {
        let bytes = Linux::get_utf8_bytes()?;
        let s = unsafe { from_utf8_unchecked(&bytes) };
        s.chars().next().ok_or(LinuxError::NoInput)
    }

    /// Gets either a single `char` from *stdin*, or the replacement character.
    ///
    /// If the bytes received doesn't form a valid unicode scalar then the
    /// [replacement character (�)][char::REPLACEMENT_CHARACTER] will be returned.
    pub fn get_dirty_char() -> char {
        match Linux::get_utf8_bytes() {
            Ok(bytes) => {
                let s = unsafe { from_utf8_unchecked(&bytes) };
                s.chars().next().unwrap()
            }
            Err(_) => char::REPLACEMENT_CHARACTER,
        }
    }

    /// Gets a UTF-8 encoded byte sequence from *stdin* representing a `char`.
    pub fn get_utf8_bytes() -> Result<[u8; 4]> {
        let mut bytes = [0u8; 4];
        let len;
        // read the first byte to determine the length of the character
        bytes[0] = Linux::get_byte()?;
        if bytes[0] & 0x80 == 0 { return Ok([bytes[0], 0, 0, 0]); } // ASCII char
        // IMPROVE: use text module functionality
        else if bytes[0] & 0xE0 == 0xC0 { len = 2; }
        else if bytes[0] & 0xF0 == 0xE0 { len = 3; }
        else if bytes[0] & 0xF8 == 0xF0 { len = 4; }
        else { return Err(LinuxError::InvalidUtf8) }
        // read the remaining bytes of the character
        for i in 1..len {
            bytes[i as usize] = Linux::get_byte()?;
            if bytes[i as usize] & 0xC0 != 0x80 { return Err(LinuxError::InvalidUtf8); }
        }
        Ok(bytes)
    }

    /// Reads all available bytes from stdin.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)` containing the bytes read.
    /// - `Err(isize)` if the read fails, returning the error code.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn read_available_bytes() -> Result<Vec<u8>> {
        let count = Linux::available_bytes()?;
        if count == 0 { return Ok(Vec::new()); }
        let mut buffer = vec![0u8; count];
        let n = unsafe { Linux::sys_read(FILENO::STDIN, buffer.as_mut_ptr(), count) };
        if n < 0 { Err(LinuxError::Sys(n)) } else { buffer.truncate(n as usize); Ok(buffer) }
    }

    /// Prompts the user for a string from *stdin*, backed by a `buffer`.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut name_buffer = [0_u8; 32];
    /// let name: &str = Linux::prompt::<32>("Enter your name: ", &mut name_buffer);
    /// ```
    pub fn prompt<'i, const CAP: usize>(text: &str, buffer: &'i mut [u8; CAP]) -> Result<&'i str> {
        Linux::print(text)?; Linux::get_line(buffer)
    }

    /// Gets a string from *stdin* backed by a `buffer`, until a newline.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut buf = [0_u8; 32];
    /// let name: &str = Linux::get_line::<32>(&mut buf);
    /// ```
    pub fn get_line<const CAP: usize>(buffer: &mut [u8; CAP]) -> Result<&str> {
        Linux::get_str(buffer, '\n')
    }

    /// Gets a string from *stdin* backed by a `buffer`, until the `stop` char is received.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut buf = [0_u8; 32];
    /// let name: &str = Linux::get_str::<32>(&mut buf, '\n');
    /// ```
    pub fn get_str<const CAP: usize>(buffer: &mut [u8; CAP], stop: char) -> Result<&str> {
        let mut index = 0;
        loop {
            if let Ok(c) = Linux::get_char() {
                let mut c_buf = [0; 4];
                let c_str = c.encode_utf8(&mut c_buf);
                if c == stop {
                    break;
                } else if index + c_str.len() <= CAP {
                    Linux::print(c_str)?;
                    for &b in c_str.as_bytes() {
                        buffer[index] = b;
                        index += 1;
                    }
                }
            }
        }
        Ok(unsafe { from_utf8_unchecked(&buffer[..index]) })
    }
}

/// # Write-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Prints a string slice to standard output.
    pub fn print(s: &str) -> Result <()> {
        let mut s = s.as_bytes();
        while !s.is_empty() {
            let n = unsafe { Linux::sys_write(FILENO::STDOUT, s.as_ptr(), s.len()) };
            if n < 0 || n as usize > s.len() { return Err(LinuxError::Sys(n)); }
            s = &s[n as usize..]; // update the byte slice to exclude the written bytes
        }
        Ok(())
    }

    /// Prints a string slice to standard output, with a newline.
    pub fn println(s: &str) -> Result<()> { Linux::print(s)?; Linux::print("\n") }

    /// Prints a string slice to standard error.
    pub fn eprint(s: &str) -> Result<()> {
        let mut s = s.as_bytes();
        while !s.is_empty() {
            let n = unsafe { Linux::sys_write(FILENO::STDERR, s.as_ptr(), s.len()) };
            if n < 0 || n as usize > s.len() { return Err(LinuxError::Sys(n)); }
            s = &s[n as usize..]; // update the byte slice to exclude the written bytes
        }
        Ok(())
    }

    /// Prints a string slice to standard error, with a newline.
    pub fn eprintln(s: &str) -> Result<()> { Linux::eprint(s)?; Linux::eprint("\n") }

    /// Prints a byte slice to *stdout*.
    pub fn print_bytes(b: &[u8]) -> Result<()> {
        let mut b = b;
        while !b.is_empty() {
            let n = unsafe { Linux::sys_write(FILENO::STDOUT, b.as_ptr(), b.len()) };
            if n < 0 || n as usize > b.len() { return Err(LinuxError::Sys(n)); }
            b = &b[n as usize..]; // update the byte slice to exclude the written bytes
        }
        Ok(())
    }
}

/// Terminal-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Returns `true` if we are in a terminal context.
    #[must_use]
    pub fn is_terminal() -> bool { LinuxTermios::is_terminal() }

    /// Returns the terminal dimensions.
    ///
    /// # Errors
    /// Returns the [`LINUX_ERRNO`] value from [`Linux::sys_ioctl`].
    pub fn terminal_size() -> Result<LinuxTerminalSize> { LinuxTermios::get_winsize() }

    /// Enables raw mode.
    ///
    /// Raw mode is a way to configure the terminal so that it does not process or
    /// interpret any of the input but instead passes it directly to the program.
    ///
    /// # Errors
    /// Returns the [`LINUX_ERRNO`] value from [`Linux::sys_ioctl`].
    pub fn enable_raw_mode() -> Result<()> { LinuxTermios::enable_raw_mode() }

    /// Disables raw mode.
    ///
    /// # Errors
    /// Returns the [`LINUX_ERRNO`] value from [`Linux::sys_ioctl`].
    pub fn disable_raw_mode() -> Result<()> { LinuxTermios::disable_raw_mode() }

    /// Enables raw mode and returns a `ScopeGuard` that restores the original state on drop.
    #[allow(clippy::type_complexity)]
    pub fn scoped_raw_mode()
        -> Result<ScopeGuard<LinuxTermios, impl FnOnce(LinuxTermios, &()), ()>> {
        let initial_state = LinuxTermios::get_state()?;
        LinuxTermios::enable_raw_mode()?;

        Ok(ScopeGuard::with(initial_state, (), |state, ()| {
            LinuxTermios::set_state(state).unwrap();
        }))
    }

    // pub fn scoped_raw_mode() -> Result<ScopeGuard<LinuxTermios, impl FnOnce(LinuxTermios), &()>> {
    //     let initial_state = LinuxTermios::get_state()?;
    //     LinuxTermios::enable_raw_mode()?;
    //     Ok(ScopeGuard::new(initial_state, |state| {
    //         LinuxTermios::set_state(state).unwrap();
    //     }))
    //
    //     // Ok(devela::ScopeGuard::new(move || {
    //     //     LinuxTermios::set_state(initial_state).unwrap();
    //     // }))
    // }
}

/// Thread-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Suspends execution of calling thread for `duration`.
    pub fn sleep(duration: Duration) -> Result<()> {
        let mut req = LinuxTimespec::with(duration);
        let mut rem = LinuxTimespec::default();
        loop {
            let n = unsafe { Linux::sys_nanosleep(req.as_ptr(), rem.as_mut_ptr()) };
            match n.cmp(&0) {
                Ordering::Less => return Err(LinuxError::Sys(n)),
                Ordering::Equal => break Ok(()),
                Ordering::Greater => req = rem,
            }
        }
    }

    /// Returns the current process number.
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
        pub fn [<random_ $prim>]() -> Result<$prim> {
            #[cold] fn getrandom_failed_cold(n: isize) -> Result<$prim> { Err(LinuxError::Sys(n)) }

            let (mut r, mut attempts) = ([0; $len], 0);
            loop {
                let n = unsafe { Linux::sys_getrandom(r.as_mut_ptr(), $len, Linux::RAND_FLAGS) };
                if n == $len {
                    // hot path!
                    break;
                } else if n == -ERRNO::EAGAIN {
                    iif![!Linux::getrandom_try_again_cold(&mut attempts); break];
                } else { // n < 0
                    return getrandom_failed_cold(n);
                }
            }
            Ok($prim::from_ne_bytes(r))
        }
    )+ }};
}

/// # Randomness-related methods.
///
/// They make use of the `GRND_NONBLOCK` and `GRND_INSECURE` flags. So when the randomness
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
    pub fn random_bytes(buffer: &mut [u8]) -> Result<()> {
        #[cold] fn getrandom_failed_cold(n: isize) -> Result<()> { Err(LinuxError::Sys(n)) }

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
                return getrandom_failed_cold(n);
            } else {
                // hot path!
                offset += n as usize;
            }
        }
        Ok(())
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
}
