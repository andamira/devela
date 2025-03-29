// devela::sys::os::linux::consts::syscall::x86_64
//
//!
//
// - https://x64.syscall.sh/
// - https://syscalls.mebeim.net/?table=x86/64/x64/latest
// - https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/

/// Linux `sys/syscall.h` constants for `x86_64`.
pub struct LINUX_SYS_X86_64;

impl LINUX_SYS_X86_64 {
    /// Accept a connection on a socket.
    pub const ACCEPT: isize = 43;
    /// Accept a connection on a socket with flags.
    pub const ACCEPT4: isize = 288;
    /// Check user's permissions for a file.
    pub const ACCESS: isize = 21;
    /// Switch process accounting on/off.
    pub const ACCT: isize = 163;
    /// Add a key to the kernel's key management facility.
    pub const ADD_KEY: isize = 248;
    /// Tune kernel clock.
    pub const ADJTIMEX: isize = 159;
    /// Reserved for AFS.
    pub const AFS_SYSCALL: isize = 183;
    /// Schedule an alarm clock signal.
    pub const ALARM: isize = 37;
    /// Set architecture-specific thread state.
    pub const ARCH_PRCTL: isize = 158;
    /// Bind a name to a socket.
    pub const BIND: isize = 49;
    /// Perform a command on an extended BPF map/program.
    pub const BPF: isize = 321;
    /// Change data segment size.
    pub const BRK: isize = 12;
    /// Get capabilities of a process.
    pub const CAPGET: isize = 125;
    /// Set capabilities of a process.
    pub const CAPSET: isize = 126;
    /// Change working directory.
    pub const CHDIR: isize = 80;
    /// Change permissions of a file.
    pub const CHMOD: isize = 90;
    /// Change ownership of a file.
    pub const CHOWN: isize = 92;
    /// Change root directory.
    pub const CHROOT: isize = 161;
    /// Tune a clock.
    pub const CLOCK_ADJTIME: isize = 305;
    /// Get clock resolution.
    pub const CLOCK_GETRES: isize = 229;
    /// Get time from a clock.
    pub const CLOCK_GETTIME: isize = 228;
    /// Sleep with high precision.
    pub const CLOCK_NANOSLEEP: isize = 230;
    /// Set time for a clock.
    pub const CLOCK_SETTIME: isize = 227;
    /// Create a child process.
    pub const CLONE: isize = 56;
    /// Create a child process with more control.
    pub const CLONE3: isize = 435;
    /// Close a file descriptor.
    pub const CLOSE: isize = 3;
    /// Close a range of file descriptors.
    pub const CLOSE_RANGE: isize = 436;
    /// Connect to a socket.
    pub const CONNECT: isize = 42;
    /// Copy data between file descriptors.
    pub const COPY_FILE_RANGE: isize = 326;
    /// Create a file.
    pub const CREAT: isize = 85;
    /// Create a loadable module entry.
    pub const CREATE_MODULE: isize = 174;
    /// Delete a kernel module.
    pub const DELETE_MODULE: isize = 176;
    /// Duplicate file descriptor.
    pub const DUP: isize = 32;
    /// Duplicate file descriptor to specific number.
    pub const DUP2: isize = 33;
    /// Duplicate file descriptor with flags.
    pub const DUP3: isize = 292;
    /// Create an epoll file descriptor.
    pub const EPOLL_CREATE: isize = 213;
    /// Create an epoll file descriptor with flags.
    pub const EPOLL_CREATE1: isize = 291;
    /// Control interface for epoll.
    pub const EPOLL_CTL: isize = 233;
    /// Old version of epoll_ctl.
    pub const EPOLL_CTL_OLD: isize = 214;
    /// Wait for an epoll file descriptor with timeout.
    pub const EPOLL_PWAIT: isize = 281;
    /// Extended version of epoll_wait.
    pub const EPOLL_PWAIT2: isize = 441;
    /// Wait for an epoll file descriptor.
    pub const EPOLL_WAIT: isize = 232;
    /// Old version of epoll_wait.
    pub const EPOLL_WAIT_OLD: isize = 215;
    /// Create a file descriptor for event notification.
    pub const EVENTFD: isize = 284;
    /// Create an eventfd file descriptor with flags.
    pub const EVENTFD2: isize = 290;
    /// Execute program.
    pub const EXECVE: isize = 59;
    /// Execute program relative to directory file descriptor.
    pub const EXECVEAT: isize = 322;
    /// Terminate calling process.
    pub const EXIT: isize = 60;
    /// Terminate all threads in process.
    pub const EXIT_GROUP: isize = 231;
    /// Check permissions of a file relative to directory fd.
    pub const FACCESSAT: isize = 269;
    /// Check permissions with additional flags.
    pub const FACCESSAT2: isize = 439;
    /// Predeclare file access pattern.
    pub const FADVISE64: isize = 221;
    /// Manipulate file space.
    pub const FALLOCATE: isize = 285;
    /// Create fanotify group.
    pub const FANOTIFY_INIT: isize = 300;
    /// Add/remove fanotify marks.
    pub const FANOTIFY_MARK: isize = 301;
    /// Change working directory by fd.
    pub const FCHDIR: isize = 81;
    /// Change permissions of a file by fd.
    pub const FCHMOD: isize = 91;
    /// Change permissions of file relative to directory fd.
    pub const FCHMODAT: isize = 268;
    /// Change ownership of a file by fd.
    pub const FCHOWN: isize = 93;
    /// Change ownership of file relative to directory fd.
    pub const FCHOWNAT: isize = 260;
    /// Manipulate file descriptor.
    pub const FCNTL: isize = 72;
    /// Synchronize file data to storage.
    pub const FDATASYNC: isize = 75;
    /// Get extended attribute value by fd.
    pub const FGETXATTR: isize = 193;
    /// Load kernel module from fd.
    pub const FINIT_MODULE: isize = 313;
    /// List extended attribute names by fd.
    pub const FLISTXATTR: isize = 196;
    /// Apply/advisory lock on open file.
    pub const FLOCK: isize = 73;
    /// Create a child process.
    pub const FORK: isize = 57;
    /// Remove extended attribute by fd.
    pub const FREMOVEXATTR: isize = 199;
    /// Configure filesystem context.
    pub const FSCONFIG: isize = 431;
    /// Set extended attribute value by fd.
    pub const FSETXATTR: isize = 190;
    /// Mount filesystem context.
    pub const FSMOUNT: isize = 432;
    /// Open filesystem context.
    pub const FSOPEN: isize = 430;
    /// Pick filesystem context.
    pub const FSPICK: isize = 433;
    /// Get file status by fd.
    pub const FSTAT: isize = 5;
    /// Get filesystem statistics by fd.
    pub const FSTATFS: isize = 138;
    /// Synchronize file's complete state to storage.
    pub const FSYNC: isize = 74;
    /// Truncate file to specified length by fd.
    pub const FTRUNCATE: isize = 77;
    /// Fast user-space locking.
    pub const FUTEX: isize = 202;
    /// Change file timestamps relative to directory fd.
    pub const FUTIMESAT: isize = 261;
    /// Determine CPU and NUMA node.
    pub const GETCPU: isize = 309;
    /// Get current working directory.
    pub const GETCWD: isize = 79;
    /// Get directory entries (64-bit version).
    pub const GETDENTS64: isize = 217;
    /// Get directory entries.
    pub const GETDENTS: isize = 78;
    /// Get effective group ID.
    pub const GETEGID: isize = 108;
    /// Get effective user ID.
    pub const GETEUID: isize = 107;
    /// Get group identity.
    pub const GETGID: isize = 104;
    /// Get list of supplementary group IDs.
    pub const GETGROUPS: isize = 115;
    /// Get value of interval timer.
    pub const GETITIMER: isize = 36;
    /// Get exported kernel and module symbols.
    pub const GET_KERNEL_SYMS: isize = 177;
    /// Get NUMA memory policy for thread.
    pub const GET_MEMPOLICY: isize = 239;
    /// Get name of connected peer socket.
    pub const GETPEERNAME: isize = 52;
    /// Get process group ID.
    pub const GETPGID: isize = 121;
    /// Get process group.
    pub const GETPGRP: isize = 111;
    /// Get process ID.
    pub const GETPID: isize = 39;
    /// Get message from STREAMS buffer.
    pub const GETPMSG: isize = 181;
    /// Get parent process ID.
    pub const GETPPID: isize = 110;
    /// Get program scheduling priority.
    pub const GETPRIORITY: isize = 140;
    /// Get random bytes.
    pub const GETRANDOM: isize = 318;
    /// Get real, effective and saved group IDs.
    pub const GETRESGID: isize = 120;
    /// Get real, effective and saved user IDs.
    pub const GETRESUID: isize = 118;
    /// Get resource limits.
    pub const GETRLIMIT: isize = 97;
    /// Get list of robust futexes.
    pub const GET_ROBUST_LIST: isize = 274;
    /// Get resource usage.
    pub const GETRUSAGE: isize = 98;
    /// Get session ID.
    pub const GETSID: isize = 124;
    /// Get socket name.
    pub const GETSOCKNAME: isize = 51;
    /// Get options on sockets.
    pub const GETSOCKOPT: isize = 55;
    /// Get thread area.
    pub const GET_THREAD_AREA: isize = 211;
    /// Get thread ID.
    pub const GETTID: isize = 186;
    /// Get time (microsecond precision).
    pub const GETTIMEOFDAY: isize = 96;
    /// Get user identity.
    pub const GETUID: isize = 102;
    /// Get extended attribute value.
    pub const GETXATTR: isize = 191;
    /// Initialize a kernel module.
    pub const INIT_MODULE: isize = 175;
    /// Add watch to inotify instance.
    pub const INOTIFY_ADD_WATCH: isize = 254;
    /// Initialize inotify instance.
    pub const INOTIFY_INIT: isize = 253;
    /// Initialize inotify instance with flags.
    pub const INOTIFY_INIT1: isize = 294;
    /// Remove watch from inotify instance.
    pub const INOTIFY_RM_WATCH: isize = 255;
    /// Cancel asynchronous I/O operation.
    pub const IO_CANCEL: isize = 210;
    /// Destroy asynchronous I/O context.
    pub const IO_DESTROY: isize = 207;
    /// Read asynchronous I/O events.
    pub const IO_GETEVENTS: isize = 208;
    /// Read asynchronous I/O events (extended).
    pub const IO_PGETEVENTS: isize = 333;
    /// Set up asynchronous I/O context.
    pub const IO_SETUP: isize = 206;
    /// Submit asynchronous I/O blocks.
    pub const IO_SUBMIT: isize = 209;
    /// Control device.
    pub const IOCTL: isize = 16;
    /// Set I/O privilege level.
    pub const IOPL: isize = 172;
    /// Set port input/output permissions.
    pub const IOPERM: isize = 173;
    /// Get I/O scheduling class/priority.
    pub const IOPRIO_GET: isize = 252;
    /// Set I/O scheduling class/priority.
    pub const IOPRIO_SET: isize = 251;
    /// Set up io_uring interface.
    pub const IO_URING_SETUP: isize = 425;
    /// Enter io_uring for completion.
    pub const IO_URING_ENTER: isize = 426;
    /// Register files or buffers for io_uring.
    pub const IO_URING_REGISTER: isize = 427;
    /// Compare two processes.
    pub const KCMP: isize = 312;
    /// Load new kernel for execution.
    pub const KEXEC_LOAD: isize = 246;
    /// Load new kernel for execution with file descriptor.
    pub const KEXEC_FILE_LOAD: isize = 320;
    /// Manipulate kernel keyring.
    pub const KEYCTL: isize = 250;
    /// Send signal to process.
    pub const KILL: isize = 62;
    /// Create Landlock ruleset.
    pub const LANDLOCK_CREATE_RULESET: isize = 444;
    /// Add rule to Landlock ruleset.
    pub const LANDLOCK_ADD_RULE: isize = 445;
    /// Enforce Landlock ruleset on current thread.
    pub const LANDLOCK_RESTRICT_SELF: isize = 446;
    /// Change ownership of symbolic link.
    pub const LCHOWN: isize = 94;
    /// Get extended attribute value (symlink).
    pub const LGETXATTR: isize = 192;
    /// Create hard link.
    pub const LINK: isize = 86;
    /// Create hard link relative to directory fd.
    pub const LINKAT: isize = 265;
    /// Listen for connections on socket.
    pub const LISTEN: isize = 50;
    /// List extended attribute names.
    pub const LISTXATTR: isize = 194;
    /// List extended attribute names (symlink).
    pub const LLISTXATTR: isize = 195;
    /// Return cookie for DCOP.
    pub const LOOKUP_DCOOKIE: isize = 212;
    /// Remove extended attribute (symlink).
    pub const LREMOVEXATTR: isize = 198;
    /// Reposition read/write file offset.
    pub const LSEEK: isize = 8;
    /// Set extended attribute value (symlink).
    pub const LSETXATTR: isize = 189;
    /// Get file status (symlink).
    pub const LSTAT: isize = 6;
    /// Give advice about memory usage.
    pub const MADVISE: isize = 28;
    /// Set NUMA memory policy for range.
    pub const MBIND: isize = 237;
    /// Issue memory barriers.
    pub const MEMBARRIER: isize = 324;
    /// Create anonymous file.
    pub const MEMFD_CREATE: isize = 319;
    /// Create secret memory area.
    pub const MEMFD_SECRET: isize = 447;
    /// Move pages between nodes.
    pub const MIGRATE_PAGES: isize = 256;
    /// Determine whether pages are resident in memory.
    pub const MINCORE: isize = 27;
    /// Create directory.
    pub const MKDIR: isize = 83;
    /// Create directory relative to directory fd.
    pub const MKDIRAT: isize = 258;
    /// Create special or ordinary file.
    pub const MKNOD: isize = 133;
    /// Create special or ordinary file relative to directory fd.
    pub const MKNODAT: isize = 259;
    /// Lock memory.
    pub const MLOCK: isize = 149;
    /// Lock memory (extended).
    pub const MLOCK2: isize = 325;
    /// Lock all current/future memory.
    pub const MLOCKALL: isize = 151;
    /// Map files/devices into memory.
    pub const MMAP: isize = 9;
    /// Modify LDT.
    pub const MODIFY_LDT: isize = 154;
    /// Mount filesystem.
    pub const MOUNT: isize = 165;
    /// Change mount attributes.
    pub const MOUNT_SETATTR: isize = 442;
    /// Move mount from source to destination.
    pub const MOVE_MOUNT: isize = 429;
    /// Move individual pages between processes.
    pub const MOVE_PAGES: isize = 279;
    /// Control memory protection.
    pub const MPROTECT: isize = 10;
    /// Get/set message queue attributes.
    pub const MQ_GETSETATTR: isize = 245;
    /// Register for notification of message arrival.
    pub const MQ_NOTIFY: isize = 244;
    /// Create message queue.
    pub const MQ_OPEN: isize = 240;
    /// Receive message from queue with timeout.
    pub const MQ_TIMEDRECEIVE: isize = 243;
    /// Send message to queue with timeout.
    pub const MQ_TIMEDSEND: isize = 242;
    /// Remove message queue.
    pub const MQ_UNLINK: isize = 241;
    /// Remap virtual memory address.
    pub const MREMAP: isize = 25;
    /// System V message control.
    pub const MSGCTL: isize = 71;
    /// Get System V message queue.
    pub const MSGGET: isize = 68;
    /// Receive System V message.
    pub const MSGRCV: isize = 70;
    /// Send System V message.
    pub const MSGSND: isize = 69;
    /// Synchronize file with memory map.
    pub const MSYNC: isize = 26;
    /// Unlock memory.
    pub const MUNLOCK: isize = 150;
    /// Unlock all current/future memory.
    pub const MUNLOCKALL: isize = 152;
    /// Unmap files/devices from memory.
    pub const MUNMAP: isize = 11;
    /// Translate name to handle and mount ID.
    pub const NAME_TO_HANDLE_AT: isize = 303;
    /// High-resolution sleep.
    pub const NANOSLEEP: isize = 35;
    /// Get file status relative to directory fd.
    pub const NEWFSTATAT: isize = 262;
    /// NFS server control.
    pub const NFSSERVCTL: isize = 180;
    /// Open file.
    pub const OPEN: isize = 2;
    /// Open file relative to directory fd.
    pub const OPENAT: isize = 257;
    /// Open file with extended attributes.
    pub const OPENAT2: isize = 437;
    /// Open file via handle.
    pub const OPEN_BY_HANDLE_AT: isize = 304;
    /// Open mount tree.
    pub const OPEN_TREE: isize = 428;
    /// Suspend process until signal.
    pub const PAUSE: isize = 34;
    /// Set up performance monitoring.
    pub const PERF_EVENT_OPEN: isize = 298;
    /// Set process execution domain.
    pub const PERSONALITY: isize = 135;
    /// Get file descriptor from pidfd.
    pub const PIDFD_GETFD: isize = 438;
    /// Obtain file descriptor for process.
    pub const PIDFD_OPEN: isize = 434;
    /// Send signal through pidfd.
    pub const PIDFD_SEND_SIGNAL: isize = 424;
    /// Create pipe.
    pub const PIPE: isize = 22;
    /// Create pipe with flags.
    pub const PIPE2: isize = 293;
    /// Change root filesystem.
    pub const PIVOT_ROOT: isize = 155;
    /// Allocate protection key.
    pub const PKEY_ALLOC: isize = 330;
    /// Free protection key.
    pub const PKEY_FREE: isize = 331;
    /// Set protection on memory pages.
    pub const PKEY_MPROTECT: isize = 329;
    /// Wait for some event on file descriptors.
    pub const POLL: isize = 7;
    /// Wait for some event with timeout (extended).
    pub const PPOLL: isize = 271;
    /// Process control.
    pub const PRCTL: isize = 157;
    /// Read from file descriptor at offset.
    pub const PREAD64: isize = 17;
    /// Scatter/gather read with offset.
    pub const PREADV: isize = 295;
    /// Scatter/gather read with offset (extended).
    pub const PREADV2: isize = 327;
    /// Get/set resource limits.
    pub const PRLIMIT64: isize = 302;
    /// Provide memory advice for another process.
    pub const PROCESS_MADVISE: isize = 440;
    /// Release memory of another process.
    pub const PROCESS_MRELEASE: isize = 448;
    /// Read memory from another process.
    pub const PROCESS_VM_READV: isize = 310;
    /// Write memory to another process.
    pub const PROCESS_VM_WRITEV: isize = 311;
    /// Synchronous I/O multiplexing with timeout.
    pub const PSELECT6: isize = 270;
    /// Process trace.
    pub const PTRACE: isize = 101;
    /// Put message to STREAMS buffer.
    pub const PUTPMSG: isize = 182;
    /// Query module information.
    pub const QUERY_MODULE: isize = 178;
    /// Manipulate disk quotas.
    pub const QUOTACTL: isize = 179;
    /// Manipulate disk quotas by fd.
    pub const QUOTACTL_FD: isize = 443;
    /// Write to file descriptor at offset.
    pub const PWRITE64: isize = 18;
    /// Scatter/gather write with offset.
    pub const PWRITEV: isize = 296;
    /// Scatter/gather write with offset (extended).
    pub const PWRITEV2: isize = 328;
    /// Read from file descriptor.
    pub const READ: isize = 0;
    /// Preload file data into page cache.
    pub const READAHEAD: isize = 187;
    /// Read value of symbolic link.
    pub const READLINK: isize = 89;
    /// Read value of symbolic link relative to directory fd.
    pub const READLINKAT: isize = 267;
    /// Scatter/gather read.
    pub const READV: isize = 19;
    /// Reboot system or enable/disable Ctrl-Alt-Del.
    pub const REBOOT: isize = 169;
    /// Receive message from socket.
    pub const RECVFROM: isize = 45;
    /// Receive multiple messages on socket.
    pub const RECVMMSG: isize = 299;
    /// Receive message from socket.
    pub const RECVMSG: isize = 47;
    /// Create nonlinear file mapping.
    pub const REMAP_FILE_PAGES: isize = 216;
    /// Remove extended attribute.
    pub const REMOVEXATTR: isize = 197;
    /// Rename file.
    pub const RENAME: isize = 82;
    /// Rename file relative to directory fds.
    pub const RENAMEAT: isize = 264;
    /// Rename file with flags.
    pub const RENAMEAT2: isize = 316;
    /// Request key from kernel's key management facility.
    pub const REQUEST_KEY: isize = 249;
    /// Restart system call after interruption.
    pub const RESTART_SYSCALL: isize = 219;
    /// Remove directory.
    pub const RMDIR: isize = 84;
    /// Restartable sequences.
    pub const RSEQ: isize = 334;
    /// Examine and change signal action.
    pub const RT_SIGACTION: isize = 13;
    /// Examine pending signals.
    pub const RT_SIGPENDING: isize = 127;
    /// Examine and change blocked signals.
    pub const RT_SIGPROCMASK: isize = 14;
    /// Queue signal and data.
    pub const RT_SIGQUEUEINFO: isize = 129;
    /// Return from signal handler.
    pub const RT_SIGRETURN: isize = 15;
    /// Wait for signal.
    pub const RT_SIGSUSPEND: isize = 130;
    /// Wait for signal with timeout.
    pub const RT_SIGTIMEDWAIT: isize = 128;
    /// Queue signal and data to thread.
    pub const RT_TGSIGQUEUEINFO: isize = 297;
    /// Get CPU affinity.
    pub const SCHED_GETAFFINITY: isize = 204;
    /// Get scheduling attributes.
    pub const SCHED_GETATTR: isize = 315;
    /// Get scheduling parameters.
    pub const SCHED_GETPARAM: isize = 143;
    /// Get static priority max.
    pub const SCHED_GET_PRIORITY_MAX: isize = 146;
    /// Get static priority min.
    pub const SCHED_GET_PRIORITY_MIN: isize = 147;
    /// Get scheduling policy.
    pub const SCHED_GETSCHEDULER: isize = 145;
    /// Get SCHED_RR interval.
    pub const SCHED_RR_GET_INTERVAL: isize = 148;
    /// Set CPU affinity.
    pub const SCHED_SETAFFINITY: isize = 203;
    /// Set scheduling attributes.
    pub const SCHED_SETATTR: isize = 314;
    /// Set scheduling parameters.
    pub const SCHED_SETPARAM: isize = 142;
    /// Set scheduling policy/parameters.
    pub const SCHED_SETSCHEDULER: isize = 144;
    /// Yield processor.
    pub const SCHED_YIELD: isize = 24;
    /// Secure computing.
    pub const SECCOMP: isize = 317;
    /// Reserved for security.
    pub const SECURITY: isize = 185;
    /// Synchronous I/O multiplexing.
    pub const SELECT: isize = 23;
    /// System V semaphore control.
    pub const SEMCTL: isize = 66;
    /// Get System V semaphore set.
    pub const SEMGET: isize = 64;
    /// System V semaphore operations.
    pub const SEMOP: isize = 65;
    /// System V semaphore operations with timeout.
    pub const SEMTIMEDOP: isize = 220;
    /// Transfer data between file descriptors.
    pub const SENDFILE: isize = 40;
    /// Send multiple messages on socket.
    pub const SENDMMSG: isize = 307;
    /// Send message on socket.
    pub const SENDMSG: isize = 46;
    /// Send message on socket.
    pub const SENDTO: isize = 44;
    /// Set NIS domain name.
    pub const SETDOMAINNAME: isize = 171;
    /// Set filesystem group ID.
    pub const SETFSGID: isize = 123;
    /// Set filesystem user ID.
    pub const SETFSUID: isize = 122;
    /// Set group identity.
    pub const SETGID: isize = 106;
    /// Set list of supplementary group IDs.
    pub const SETGROUPS: isize = 116;
    /// Set hostname.
    pub const SETHOSTNAME: isize = 170;
    /// Set interval timer.
    pub const SETITIMER: isize = 38;
    /// Set NUMA memory policy.
    pub const SET_MEMPOLICY: isize = 238;
    /// Reassociate thread with namespace.
    pub const SETNS: isize = 308;
    /// Set process group ID.
    pub const SETPGID: isize = 109;
    /// Set program scheduling priority.
    pub const SETPRIORITY: isize = 141;
    /// Set real and effective group IDs.
    pub const SETREGID: isize = 114;
    /// Set real, effective and saved group IDs.
    pub const SETRESGID: isize = 119;
    /// Set real, effective and saved user IDs.
    pub const SETRESUID: isize = 117;
    /// Set real and effective user IDs.
    pub const SETREUID: isize = 113;
    /// Set resource limits.
    pub const SETRLIMIT: isize = 160;
    /// Set list of robust futexes.
    pub const SET_ROBUST_LIST: isize = 273;
    /// Create session and set process group ID.
    pub const SETSID: isize = 112;
    /// Set options on sockets.
    pub const SETSOCKOPT: isize = 54;
    /// Set thread area.
    pub const SET_THREAD_AREA: isize = 205;
    /// Set thread ID for child clear-on-return.
    pub const SET_TID_ADDRESS: isize = 218;
    /// Set time.
    pub const SETTIMEOFDAY: isize = 164;
    /// Set user identity.
    pub const SETUID: isize = 105;
    /// Set extended attribute value.
    pub const SETXATTR: isize = 188;
    /// System V shared memory attach.
    pub const SHMAT: isize = 30;
    /// System V shared memory control.
    pub const SHMCTL: isize = 31;
    /// System V shared memory detach.
    pub const SHMDT: isize = 67;
    /// Allocate System V shared memory segment.
    pub const SHMGET: isize = 29;
    /// Shut down socket send/receive operations.
    pub const SHUTDOWN: isize = 48;
    /// Set and/or get alternate signal stack.
    pub const SIGALTSTACK: isize = 131;
    /// Create file descriptor for signal acceptance.
    pub const SIGNALFD: isize = 282;
    /// Create file descriptor for signal acceptance with flags.
    pub const SIGNALFD4: isize = 289;
    /// Create socket.
    pub const SOCKET: isize = 41;
    /// Create pair of connected sockets.
    pub const SOCKETPAIR: isize = 53;
    /// Splice data to/from pipe.
    pub const SPLICE: isize = 275;
    /// Get file status.
    pub const STAT: isize = 4;
    /// Get filesystem statistics.
    pub const STATFS: isize = 137;
    /// Get file attributes (extended).
    pub const STATX: isize = 332;
    /// Stop swapping to file/device.
    pub const SWAPOFF: isize = 168;
    /// Start swapping to file/device.
    pub const SWAPON: isize = 167;
    /// Create symbolic link.
    pub const SYMLINK: isize = 88;
    /// Create symbolic link relative to directory fd.
    pub const SYMLINKAT: isize = 266;
    /// Schedule file system sync.
    pub const SYNC: isize = 162;
    /// Sync a file segment with disk.
    pub const SYNC_FILE_RANGE: isize = 277;
    /// Sync filesystem containing fd.
    pub const SYNCFS: isize = 306;
    /// Read/write system parameters.
    pub const _SYSCTL: isize = 156;
    /// Get filesystem type information.
    pub const SYSFS: isize = 139;
    /// Return system information.
    pub const SYSINFO: isize = 99;
    /// Read and/or clear kernel message ring buffer.
    pub const SYSLOG: isize = 103;
    /// Duplicate pipe content.
    pub const TEE: isize = 276;
    /// Get time in seconds.
    pub const TIME: isize = 201;
    /// Send signal to specific thread.
    pub const TGKILL: isize = 234;
    /// Create POSIX per-process timer.
    pub const TIMER_CREATE: isize = 222;
    /// Delete POSIX per-process timer.
    pub const TIMER_DELETE: isize = 226;
    /// Fetch state of POSIX timer.
    pub const TIMER_GETOVERRUN: isize = 225;
    /// Fetch state of POSIX timer.
    pub const TIMER_GETTIME: isize = 224;
    /// Arm/disarm POSIX timer.
    pub const TIMER_SETTIME: isize = 223;
    /// Create timerfd timer.
    pub const TIMERFD_CREATE: isize = 283;
    /// Get timerfd timer expiration.
    pub const TIMERFD_GETTIME: isize = 287;
    /// Set timerfd timer expiration.
    pub const TIMERFD_SETTIME: isize = 286;
    /// Get process times.
    pub const TIMES: isize = 100;
    /// Send signal to specific thread.
    pub const TKILL: isize = 200;
    /// Truncate file to specified length.
    pub const TRUNCATE: isize = 76;
    /// Reserved for tux.
    pub const TUXCALL: isize = 184;
    /// Set file mode creation mask.
    pub const UMASK: isize = 95;
    /// Unmount filesystem.
    pub const UMOUNT2: isize = 166;
    /// Get name and information about current kernel.
    pub const UNAME: isize = 63;
    /// Delete name and possibly the file it refers to.
    pub const UNLINK: isize = 87;
    /// Delete name relative to directory fd.
    pub const UNLINKAT: isize = 263;
    /// Disassociate parts of process execution context.
    pub const UNSHARE: isize = 272;
    /// Load shared library.
    pub const USELIB: isize = 134;
    /// Get filesystem statistics (legacy).
    pub const USTAT: isize = 136;
    /// Create userfaultfd handle.
    pub const USERFAULTFD: isize = 323;
    /// Change file last access/modification times.
    pub const UTIME: isize = 132;
    /// Change file timestamps with nanosecond precision.
    pub const UTIMENSAT: isize = 280;
    /// Change file last access/modification times.
    pub const UTIMES: isize = 235;
    /// Create child process and block parent.
    pub const VFORK: isize = 58;
    /// Hang up current terminal.
    pub const VHANGUP: isize = 153;
    /// Splice user pages into pipe.
    pub const VMSPLICE: isize = 278;
    /// Reserved for vsyscall.
    pub const VSERVER: isize = 236;
    /// Wait for process to change state.
    pub const WAIT4: isize = 61;
    /// Wait for process to change state (extended).
    pub const WAITID: isize = 247;
    /// Write to file descriptor.
    pub const WRITE: isize = 1;
    /// Scatter/gather write.
    pub const WRITEV: isize = 20;
}
