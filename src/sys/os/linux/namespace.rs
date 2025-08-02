// devela::sys::os::linux::namespace
//
//! Defines the [`Linux`] namespace.
//

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
crate::items! {
    use crate::{
        c_uint, c_void, is, transmute, AtomicOrdering, AtomicPtr, Duration, LinuxError,
        LinuxResult as Result, LinuxSigaction, LinuxSiginfo, LinuxSigset, LinuxTimespec,
        Ptr, _core::str::from_utf8_unchecked, LINUX_ERRNO as ERRNO,
        LINUX_FILENO as FILENO, LINUX_IOCTL as IOCTL, LINUX_SIGACTION as SIGACTION,
        MaybeUninit, c_int,
    };
    #[cfg(feature = "term")]
    use crate::{LinuxTermios, ScopeGuard, TermSize};
    #[cfg(feature = "alloc")]
    use crate::Vec;
}

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
        let mut buffer = crate::vec_![0u8; count];
        let n = unsafe { Linux::sys_read(FILENO::STDIN, buffer.as_mut_ptr(), count) };
        if n < 0 { Err(LinuxError::Sys(n)) } else { buffer.truncate(n as usize); Ok(buffer) }
    }

    /// Prompts the user for a string from *stdin*, backed by a `buffer`.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::Linux;
    /// let mut name_buffer = [0_u8; 32];
    /// let name: &str = Linux::prompt::<32>("Enter your name: ", &mut name_buffer).unwrap();
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
    /// let name: &str = Linux::get_line::<32>(&mut buf).unwrap();
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
    /// let name: &str = Linux::get_str::<32>(&mut buf, '\n').unwrap();
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
                    // Linux::print(c_str)?;
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
    /* core helpers */

    const STACK_BUFFER_LEN: usize = 512; // PIPE_BUF is typically 4096 but 512 is safe

    /// Writes all bytes to a file descriptor, handling partial writes.
    fn write_all(fd: c_int, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            let n = unsafe { Linux::sys_write(fd, buf.as_ptr(), buf.len()) };
            if n < 0 { return Err(LinuxError::Sys(n)); }
            buf = &buf[n as usize..];
        }
        Ok(())
    }
    /// Writes all bytes to stdout. Returns error on syscall failure.
    fn write_all_unchecked(fd: c_int, mut buf: &[u8]) {
        while !buf.is_empty() {
            let n = unsafe { Linux::sys_write(fd, buf.as_ptr(), buf.len()) };
            if n <= 0 { panic!("write failed with return value {n}"); }
            buf = &buf[n as usize..];
        }
    }

    /* public methods */

    /// Writes string + newline to stdout. Optimized for small strings.
    pub fn print(s: &str) -> Result<()> {
        Self::write_all(FILENO::STDOUT, s.as_bytes())
    }
    /// Like `print`, but panics on failure instead of returning errors.
    pub fn print_unchecked(s: &str) { Self::write_all_unchecked(FILENO::STDOUT, s.as_bytes()); }

    /// Writes string + newline to stdout. Optimized for small strings.
    pub fn println(s: &str) -> Result<()> {
        if s.len() <= Self::STACK_BUFFER_LEN { // small string optimization:
            let mut buf = [0u8; Self::STACK_BUFFER_LEN];
            let bytes = s.as_bytes();
            buf[..bytes.len()].copy_from_slice(bytes);
            buf[bytes.len()] = b'\n';
            Self::write_all(FILENO::STDOUT, &buf[..bytes.len() + 1])
        } else { // fallback for large strings:
            Self::write_all(FILENO::STDOUT, s.as_bytes())?;
            Self::write_all(FILENO::STDOUT, b"\n")
        }
    }
    /// Like `println`, but panics on failure instead of returning errors.
    pub fn println_unchecked(s: &str) {
        if s.len() <= Self::STACK_BUFFER_LEN {
            let mut buf = [0u8; Self::STACK_BUFFER_LEN];
            let bytes = s.as_bytes();
            buf[..bytes.len()].copy_from_slice(bytes);
            buf[bytes.len()] = b'\n';
            Self::write_all_unchecked(FILENO::STDOUT, &buf[..bytes.len() + 1]);
        } else {
            Self::write_all_unchecked(FILENO::STDOUT, s.as_bytes());
            Self::write_all_unchecked(FILENO::STDOUT, b"\n");
        }
    }
    /// Ultra-fast stdout write. Panics if not all bytes are written at once.
    /// Only use for small strings (<512 bytes).
    pub fn print_unchecked_fast(s: &str) {
        let bytes = s.as_bytes();
        let n = unsafe { Linux::sys_write(FILENO::STDOUT, bytes.as_ptr(), bytes.len()) };
        if n != bytes.len() as isize {
            panic!("write failed with return value {n}");
        }
    }
    /// Ultra-fast stdout write with newline. Panics if write isn't atomic.
    /// Only use for small strings (<511 bytes).
    pub fn println_unchecked_fast(s: &str) {
        let bytes = s.as_bytes();
        let mut buf = MaybeUninit::<[u8; Self::STACK_BUFFER_LEN]>::uninit();
        // SAFETY: we're about to initialize all accessed bytes
        unsafe {
            let buf_ptr = buf.as_mut_ptr() as *mut u8;
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr, bytes.len());
            *buf_ptr.add(bytes.len()) = b'\n'; // Add newline
            let n = Linux::sys_write(FILENO::STDOUT, buf_ptr, bytes.len() + 1); // single syscall
            if n != (bytes.len() + 1) as isize { panic!("write failed with return value {n}"); }
        }
    }
    /// Writes bytes to stdout. Returns error on syscall failure.
    pub fn print_bytes(b: &[u8]) -> Result<()> { Self::write_all(FILENO::STDOUT, b) }
    /// Like `print_bytes`, but panics on failure instead of returning errors.
    pub fn print_bytes_unchecked(b: &[u8]) { Self::write_all_unchecked(FILENO::STDOUT, b); }

    /// Writes all bytes to stderr. Returns error on syscall failure.
    pub fn eprint(s: &str) -> Result<()> { Self::write_all(FILENO::STDERR, s.as_bytes()) }
    /// Writes string + newline to stderr. Optimized for small strings.
    pub fn eprintln(s: &str) -> Result<()> {
        if s.len() <= Self::STACK_BUFFER_LEN { // small string optimization:
            let mut buf = [0u8; Self::STACK_BUFFER_LEN];
            let bytes = s.as_bytes();
            buf[..bytes.len()].copy_from_slice(bytes);
            buf[bytes.len()] = b'\n';
            Self::write_all(FILENO::STDERR, &buf[..bytes.len() + 1])
        } else { // fallback for large strings:
            Self::write_all(FILENO::STDERR, s.as_bytes())?;
            Self::write_all(FILENO::STDERR, b"\n")
        }
    }
    /// Writes bytes to stderr. Returns error on syscall failure.
    pub fn eprint_bytes(b: &[u8]) -> Result<()> {
        Self::write_all(FILENO::STDERR, b)
    }
}

/// # Terminal-related methods.
#[rustfmt::skip]
#[cfg(feature = "term")]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Returns `true` if we are in a terminal context.
    #[must_use]
    pub fn is_terminal() -> bool { LinuxTermios::is_terminal() }

    /// Returns the terminal dimensions.
    pub fn terminal_size() -> Result<TermSize> { LinuxTermios::get_winsize() }

    /// Enables raw mode.
    ///
    /// Raw mode is a way to configure the terminal so that it does not process or
    /// interpret any of the input but instead passes it directly to the program.
    pub fn enable_raw_mode() -> Result<()> { LinuxTermios::enable_raw_mode() }

    /// Disables raw mode.
    pub fn disable_raw_mode() -> Result<()> { LinuxTermios::disable_raw_mode() }

    /// Enables raw mode and returns a `ScopeGuard` that restores the original state on drop.
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

/// # Thread-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Suspends execution of calling thread for the given `duration`.
    pub fn sleep(duration: Duration) -> Result<()> {
        let mut req = LinuxTimespec::with(duration);
        let mut rem = LinuxTimespec::default();
        loop {
            let n = unsafe { Linux::sys_nanosleep(req.as_ptr(), rem.as_mut_ptr()) };
            if n == 0 {
                break Ok(()); // Success
            } else if n == -ERRNO::EINTR {
                req = rem; // Retry with remaining time
                continue;
            } else { // Actual error
                return Err(LinuxError::Sys(n));
            }
        }
    }
    /// Suspends execution of calling thread for the given `milliseconds`.
    pub fn sleep_ms(milliseconds: u64) -> Result<()> {
        Linux::sleep(Duration::from_millis(milliseconds))
    }

    /// Returns the current process number.
    pub fn getpid() -> i32 {
        unsafe { Linux::sys_getpid() }
    }
}

/* signals */

#[cfg(feature = "unsafe_syscall")]
crate::items! {
    /// Maximum number of signals.
    const LINUX_SIG_MAX: usize = 30;
    /// A static array to match signals with handlers.
    static LINUX_SIG_HANDLERS: [AtomicPtr<fn(i32)>; LINUX_SIG_MAX] = {
        const INIT: AtomicPtr<fn(i32)> = AtomicPtr::new(Ptr::null_mut());
        [INIT; LINUX_SIG_MAX]
    };
}

/// # Signaling-related methods.
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_syscall", not(miri)))]
impl Linux {
    /// Registers multiple signals using a handler function.
    ///
    /// # Notes
    /// - Unknown flags in the `flags` slice are ignored, and a warning is printed.
    ///
    /// # Arguments
    /// - `handler`: A function that will be called when one of the specified signals is received.
    ///   The function takes a single argument for the signal number.
    /// - `signals`: A slice of [`LINUX_SIGNAL`][super::LINUX_SIGNAL] values specifying the signals to handle.
    /// - `flags`: An optional slice of [`LINUX_SIGACTION`][super::LINUX_SIGACTION] flags.
    ///   If `None`, only the `SA_RESTORER` flag is used.
    ///
    /// # Notes
    /// - The `SA_RESTORER` flag is always included.
    /// - The `SA_SIGINFO` flag is ignored. Use [`sig_handler_info`][Self::sig_handler_info] instead.
    /// - Unknown flags in the `flags` slice are ignored, and a warning is printed.
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// # use devela::{Linux, LINUX_SIGNAL, sleep4};
    /// fn handler(sig: i32) {
    ///    println!("\nsignal `{sig}` received! continuing. . .");
    /// }
    /// fn main() {
    ///     println!("press Ctrl+C, or resize the terminal to catch the signals");
    ///     Linux::sig_handler(handler, &[LINUX_SIGNAL::SIGINT, LINUX_SIGNAL::SIGWINCH], None);
    ///     sleep4!(5);
    ///     println!("bye");
    /// }
    /// # }
    /// ```
    pub fn sig_handler(handler: fn(i32), signals: &[i32], flags: Option<&[usize]>) {
        extern "C" fn c_handler(sig: i32) {
            if sig >= 0 && (sig as usize) < LINUX_SIG_MAX {
                let handler = LINUX_SIG_HANDLERS[sig as usize].load(AtomicOrdering::SeqCst);
                if !handler.is_null() {
                    unsafe {
                        let handler: fn(i32) = transmute(handler);
                        handler(sig);
                    }
                }
            }
        }
        // Use the restorer function defined in assembly.
        unsafe extern "C" { safe fn __devela_linux_restore_rt(); }

        // Start with the SA_RESTORER flag, as it is always required.
        let mut combined_flags = SIGACTION::SA_RESTORER;
        // Add additional flags if provided.
        if let Some(provided_flags) = flags {
            for &flag in provided_flags {
                match flag { // Validate the flag to ensure it is one of the known values.
                    SIGACTION::SA_NOCLDSTOP
                    | SIGACTION::SA_NOCLDWAIT
                    | SIGACTION::SA_NODEFER
                    | SIGACTION::SA_ONSTACK
                    | SIGACTION::SA_RESETHAND
                    | SIGACTION::SA_RESTART => {
                        combined_flags |= flag;
                    }
                    _ => {
                        #[cfg(feature = "std")]
                        eprintln!("Warning: Ignoring unknown flag: {flag:#x}");
                    }
                }
            }
        }
        let mask = LinuxSigset::empty();
        for &sig in signals {
            if (1..=LINUX_SIG_MAX).contains(&(sig as usize)) { // make sure the signal is a valid number
                // Store the handler in the appropriate slot
                LINUX_SIG_HANDLERS[sig as usize].store(handler as *mut _, AtomicOrdering::SeqCst);
                // Register the handler with the OS
                let c_handler = c_handler as extern "C" fn(i32);
                let sigaction = LinuxSigaction::new(c_handler,
                    combined_flags, mask, Some(__devela_linux_restore_rt));
                unsafe {
                    let _ = Linux::sys_rt_sigaction(sig, &sigaction,
                        Ptr::null_mut(), LinuxSigset::size());
                }
            }
        }
    }

    /// Registers multiple signals using a handler function accepting additional signal information.
    ///
    /// # Arguments
    /// - `handler`: A function that will be called when one of the specified signals is received.
    ///   The function takes three arguments:
    ///   - `i32`: The signal number.
    ///   - `LinuxSiginfo`: Additional information about the signal.
    ///   - `*mut c_void`: A pointer to the signal context (ucontext_t).
    /// - `signals`: A slice of [`LINUX_SIGNAL`][super::LINUX_SIGNAL] values specifying the signals to handle.
    /// - `flags`: An optional slice of [`LINUX_SIGACTION`][super::LINUX_SIGACTION] flags.
    ///   If `None`, only the `SA_RESTORER` and `SA_SIGINFO` flags are used.
    ///
    /// # Notes
    /// - The `SA_RESTORER` and `SA_SIGINFO` flags are always included.
    /// - Unknown flags in the `flags` slice are ignored, and a warning is printed.
    ///
    /// # Examples
    /// ```no_run
    /// # use devela::{c_void, sleep4, Linux, LinuxSiginfo, LINUX_SIGNAL, LINUX_SIGACTION};
    /// fn handler(sig: i32, info: LinuxSiginfo, _context: *mut c_void) {
    ///     println!("Signal {} received from PID {}!", sig, info.pid());
    /// }
    /// fn main() {
    ///     Linux::sig_handler_info(handler, &[LINUX_SIGNAL::SIGINT],
    ///         Some(&[LINUX_SIGACTION::SA_RESETHAND, LINUX_SIGACTION::SA_RESTART]),
    ///     );
    ///     println!("press Ctrl+C, to trigger SIGINT");
    ///     loop {
    ///         println!("Waiting for signal...");
    ///         sleep4!(1);
    ///     }
    /// }
    /// ```
    pub fn sig_handler_info(
        handler: fn(i32, LinuxSiginfo, *mut c_void),
        signals: &[i32],
        flags: Option<&[usize]>,
    ) {
        // We store the given `handler` function in a static to be able to call it
        // from the new extern function which can't capture its environment.
        static HANDLER: AtomicPtr<fn(i32, LinuxSiginfo, *mut c_void)>
            = AtomicPtr::new(Ptr::null_mut());
        HANDLER.store(handler as *mut _, AtomicOrdering::SeqCst);

        extern "C" fn c_handler_siginfo(sig: i32, info: LinuxSiginfo, context: *mut c_void) {
            let handler = HANDLER.load(AtomicOrdering::SeqCst);
            if !handler.is_null() {
                #[expect(clippy::crosspointer_transmute)]
                let handler = unsafe {
                    transmute::<*mut fn(i32, LinuxSiginfo, *mut c_void),
                        fn(i32, LinuxSiginfo, *mut c_void)>(handler)
                };
                handler(sig, info, context);
            }
        }
        // Use the restorer function defined in assembly.
        unsafe extern "C" { safe fn __devela_linux_restore_rt(); }

        // Start with the SA_RESTORER and SA_SIGINFO flags, as they are always required.
        let mut combined_flags = SIGACTION::SA_RESTORER | SIGACTION::SA_SIGINFO;
        // Add additional flags if provided.
        if let Some(provided_flags) = flags {
            for &flag in provided_flags {
                match flag {
                    SIGACTION::SA_NOCLDSTOP
                    | SIGACTION::SA_NOCLDWAIT
                    | SIGACTION::SA_NODEFER
                    | SIGACTION::SA_ONSTACK
                    | SIGACTION::SA_RESETHAND
                    | SIGACTION::SA_RESTART => {
                        combined_flags |= flag;
                    }
                    _ => {
                        #[cfg(feature = "std")]
                        eprintln!("Warning: Ignoring unknown flag: {flag:#x}");
                    }
                }
            }
        }
        let mask = LinuxSigset::empty();
        let sigaction = LinuxSigaction::new_siginfo(
            c_handler_siginfo,
            combined_flags,
            mask,
            Some(__devela_linux_restore_rt),
        );
        for s in signals {
            if (1..=31).contains(s) { // make sure the signal is a valid number
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
                if n == $len { break; } // ← hot path
                else if n == -ERRNO::EAGAIN { // ←↓ cold paths
                    is![!Linux::getrandom_try_again_cold(&mut attempts); break];
                } else { return getrandom_failed_cold(n); } // n < 0
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
            let n = unsafe { Linux::sys_getrandom(buffer[offset..].as_mut_ptr(),
                buffer.len() - offset, Linux::RAND_FLAGS) };
            if n == -ERRNO::EAGAIN { is![!Linux::getrandom_try_again_cold(&mut attempts); break]; }
            else if n < 0 { return getrandom_failed_cold(n); } // ←↑ cold paths
            else { offset += n as usize; } // ← hot path
        }
        Ok(())
    }

    // from `sys/random.h`
    const GRND_NONBLOCK: c_uint = 0x0001;
    // const GRND_RANDOM: isize = 0x0002;
    const GRND_INSECURE: c_uint = 0x0004;
    const RAND_FLAGS: c_uint = Linux::GRND_NONBLOCK | Linux::GRND_INSECURE;
    const RAND_MAX_ATTEMPTS: usize = 15;
    /// the cold path for trying again
    #[cold] #[must_use]
    fn getrandom_try_again_cold(attempts: &mut usize) -> bool {
        // if *attempts >= Linux::RAND_MAX_ATTEMPTS { Linux::getrandom_failed_cold(); }
        *attempts += 1;
        *attempts <= Linux::RAND_MAX_ATTEMPTS
    }
}
