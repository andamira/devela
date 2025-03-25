// devela::sys::os::linux::consts::errno
//
//! Defines [`LINUX_ERRNO`].
//

#![allow(non_camel_case_types)]

/// [`Linux`][crate::Linux] `sys/errno.h` constants.
//
// - /usr/include/asm-generic/errno.h
pub struct LINUX_ERRNO;
#[allow(missing_docs)]
impl LINUX_ERRNO {
    /// “Operation not permitted.”
    ///
    /// Only the owner of the file (or other resource) or processes with special
    /// privileges can perform the operation.
    pub const EPERM: isize = 1;

    /// “No such file or directory.”
    ///
    /// This is a “file doesn’t exist” error for ordinary files that are referenced
    /// in contexts where they are expected to already exist.
    pub const ENOENT: isize = 2;

    /// “No such process.”
    ///
    /// No process matches the specified process ID.
    pub const ESRCH: isize = 3;

    /// “Interrupted system call.”
    ///
    /// An asynchronous signal occurred and prevented completion of the call.
    /// When this happens, you should try the call again.
    ///
    /// You can choose to have functions resume after a signal that is handled,
    /// rather than failing with `EINTR`.
    pub const EINTR: isize = 4;

    /// “Input/output error.”
    ///
    /// Usually used for physical read or write errors.
    pub const EIO: isize = 5;

    /// “No such device or address.”
    ///
    /// The system tried to use the device represented by a file you specified, and
    /// it couldn’t find the device. This can mean that the device file was
    /// installed incorrectly, or that the physical device is missing or not
    /// correctly attached to the computer.
    pub const ENXIO: isize = 6;

    /// “Argument list too long.”
    ///
    /// Used when the arguments passed to a new program being executed with one of
    /// the exec functions (see Executing a File) occupy too much memory space. This
    /// condition never arises on GNU/Hurd systems.
    pub const E2BIG: isize = 7;

    /// “Exec format error.”
    ///
    /// Invalid executable file format. This condition is detected by the exec functions.
    pub const ENOEXEC: isize = 8;

    /// “Bad file descriptor.”
    ///
    /// For example, I/O on a descriptor that has been closed or reading from a
    /// descriptor open only for writing (or vice versa).
    pub const EBADF: isize = 9;

    /// “No child processes.”
    ///
    /// This error happens on operations that are supposed to manipulate child
    /// processes, when there aren’t any processes to manipulate.
    pub const ECHILD: isize = 10;

    /// “Resource temporarily unavailable.”
    ///
    /// The call might work if you try again later. The constant
    /// [`EWOULDBLOCK`][Self::EWOULDBLOCK] is another name for `EAGAIN`.
    pub const EAGAIN: isize = 11;

    /// “Cannot allocate memory.”
    ///
    /// The system cannot allocate more virtual memory because its capacity is full.
    pub const ENOMEM: isize = 12;

    /// “Permission denied.”
    ///
    /// The file permissions do not allow the attempted operation.
    pub const EACCES: isize = 13;

    /// “Bad address.”
    ///
    /// An invalid pointer was detected. On GNU/Hurd systems, this error never
    /// happens; you get a signal instead.
    pub const EFAULT: isize = 14;

    /// “Block device required.”
    ///
    /// A file that isn’t a block special file was given in a situation that
    /// requires one. For example, trying to mount an ordinary file as a file system
    /// in Unix gives this error.
    pub const ENOTBLK: isize = 15;

    /// “Device or resource busy.”
    ///
    /// A system resource that can’t be shared is already in use. For example, if
    /// you try to delete a file that is the root of a currently mounted filesystem,
    /// you get this error.
    pub const EBUSY: isize = 16;

    /// “File exists.”
    ///
    /// An existing file was specified in a context where it only makes sense to
    /// specify a new file.
    pub const EEXIST: isize = 17;

    /// “Invalid cross-device link.”
    ///
    /// An attempt to make an improper link across file systems was detected. This
    /// happens not only when you use link (see Hard Links) but also when you rename
    /// a file with rename (see Renaming Files).
    pub const EXDEV: isize = 18;

    /// “No such device.”
    ///
    /// The wrong type of device was given to a function that expects a particular
    /// sort of device.
    pub const ENODEV: isize = 19;

    /// “Not a directory.”
    ///
    /// A file that isn’t a directory was specified when a directory is required.
    pub const ENOTDIR: isize = 20;

    /// “Is a directory.”
    ///
    /// You cannot open a directory for writing, or create or remove hard links to it.
    pub const EISDIR: isize = 21;

    /// “Invalid argument.”
    ///
    /// This is used to indicate various kinds of problems with passing the wrong
    /// argument to a library function.
    pub const EINVAL: isize = 22;

    /// “Too many open files in system.”
    ///
    /// There are too many distinct file openings in the entire system. Note that
    /// any number of linked channels count as just one file opening; see Linked
    /// Channels. This error never occurs on GNU/Hurd systems.
    pub const ENFILE: isize = 23;

    /// “Too many open files.”
    ///
    /// The current process has too many files open and can’t open any more.
    /// Duplicate descriptors do count toward this limit.
    pub const EMFILE: isize = 24;

    /// “Inappropriate ioctl for device.”
    ///
    /// Inappropriate I/O control operation, such as trying to set terminal modes on
    /// an ordinary file.
    pub const ENOTTY: isize = 25;

    /// “Text file busy.”
    ///
    /// An attempt to execute a file that is currently open for writing, or write to
    /// a file that is currently being executed. Often using a debugger to run a
    /// program is considered having it open for writing and will cause this error.
    /// (The name stands for “text file busy”.) This is not an error on GNU/Hurd
    /// systems; the text is copied as necessary.
    pub const ETXTBSY: isize = 26;

    /// “File too large.”
    ///
    /// The size of a file would be larger than allowed by the system.
    pub const EFBIG: isize = 27;

    /// “No space left on device.”
    ///
    /// Write operation on a file failed because the disk is full.
    pub const ENOSPC: isize = 28;

    /// “Illegal seek.”
    ///
    /// Invalid seek operation (such as on a pipe).
    pub const ESPIPE: isize = 29;

    /// “Read-only file system.”
    ///
    /// An attempt was made to modify something on a read-only file system.
    pub const EROFS: isize = 30;

    /// “Too many links.”
    ///
    /// The link count of a single file would become too large. rename can cause
    /// this error if the file being renamed already has as many links as it can take.
    pub const EMLINK: isize = 31;

    /// “Broken pipe.”
    ///
    /// There is no process reading from the other end of a pipe. Every library
    /// function that returns this error code also generates a `SIGPIPE` signal; this
    /// signal terminates the program if not handled or blocked. Thus, your program
    /// will never actually see `EPIPE` unless it has handled or blocked `SIGPIPE`.
    pub const EPIPE: isize = 32;

    /// “Numerical argument out of domain.”
    ///
    /// Used by mathematical functions when an argument value does not fall into the
    /// domain over which the function is defined.
    pub const EDOM: isize = 33;

    /// “Numerical result out of range.”
    ///
    /// Used by mathematical functions when the result value is not representable
    /// because of overflow or underflow.
    pub const ERANGE: isize = 34;

    /// “Operation would block.”
    ///
    /// In the GNU C Library, this is another name for [`EAGAIN`][Self::EAGAIN].
    /// The values are always the same, on every operating system.
    ///
    /// C libraries in many older Unix systems have EWOULDBLOCK as a separate error code.
    pub const EWOULDBLOCK: isize = Self::EAGAIN;

    pub const EDEADLK: isize = 35;
    pub const ENAMETOOLONG: isize = 36;
    pub const ENOLCK: isize = 37;
    pub const ENOSYS: isize = 38;
    pub const ENOTEMPTY: isize = 39;
    pub const ELOOP: isize = 40;
    pub const ENOMSG: isize = 42;
    pub const EIDRM: isize = 43;
    pub const ECHRNG: isize = 44;
    pub const EL2NSYNC: isize = 45;
    pub const EL3HLT: isize = 46;
    pub const EL3RST: isize = 47;
    pub const ELNRNG: isize = 48;
    pub const EUNATCH: isize = 49;
    pub const ENOCSI: isize = 50;
    pub const EL2HLT: isize = 51;
    pub const EBADE: isize = 52;
    pub const EBADR: isize = 53;
    pub const EXFULL: isize = 54;
    pub const ENOANO: isize = 55;
    pub const EBADRQC: isize = 56;
    pub const EBADSLT: isize = 57;
    pub const EMULTIHOP: isize = 72;
    pub const EOVERFLOW: isize = 75;
    pub const ENOTUNIQ: isize = 76;
    pub const EBADFD: isize = 77;
    pub const EBADMSG: isize = 74;
    pub const EREMCHG: isize = 78;
    pub const ELIBACC: isize = 79;
    pub const ELIBBAD: isize = 80;
    pub const ELIBSCN: isize = 81;
    pub const ELIBMAX: isize = 82;
    pub const ELIBEXEC: isize = 83;
    pub const EILSEQ: isize = 84;
    pub const ERESTART: isize = 85;
    pub const ESTRPIPE: isize = 86;
    pub const EUSERS: isize = 87;
    pub const ENOTSOCK: isize = 88;
    pub const EDESTADDRREQ: isize = 89;
    pub const EMSGSIZE: isize = 90;
    pub const EPROTOTYPE: isize = 91;
    pub const ENOPROTOOPT: isize = 92;
    pub const EPROTONOSUPPORT: isize = 93;
    pub const ESOCKTNOSUPPORT: isize = 94;
    pub const EOPNOTSUPP: isize = 95;
    pub const EPFNOSUPPORT: isize = 96;
    pub const EAFNOSUPPORT: isize = 97;
    pub const EADDRINUSE: isize = 98;
    pub const EADDRNOTAVAIL: isize = 99;
    pub const ENETDOWN: isize = 100;
    pub const ENETUNREACH: isize = 101;
    pub const ENETRESET: isize = 102;
    pub const ECONNABORTED: isize = 103;
    pub const ECONNRESET: isize = 104;
    pub const ENOBUFS: isize = 105;
    pub const EISCONN: isize = 106;
    pub const ENOTCONN: isize = 107;
    pub const ESHUTDOWN: isize = 108;
    pub const ETOOMANYREFS: isize = 109;
    pub const ETIMEDOUT: isize = 110;
    pub const ECONNREFUSED: isize = 111;
    pub const EHOSTDOWN: isize = 112;
    pub const EHOSTUNREACH: isize = 113;
    pub const EALREADY: isize = 114;
    pub const EINPROGRESS: isize = 115;
    pub const ESTALE: isize = 116;
    pub const EDQUOT: isize = 122;
    pub const ENOMEDIUM: isize = 123;
    pub const EMEDIUMTYPE: isize = 124;
    pub const ECANCELED: isize = 125;
    pub const ENOKEY: isize = 126;
    pub const EKEYEXPIRED: isize = 127;
    pub const EKEYREVOKED: isize = 128;
    pub const EKEYREJECTED: isize = 129;
    pub const EOWNERDEAD: isize = 130;
    pub const ENOTRECOVERABLE: isize = 131;
    pub const EHWPOISON: isize = 133;
    pub const ERFKILL: isize = 132;
}
