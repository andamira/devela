// devela::sys::os::linux::consts::syscall::aarch64
//
//!
//
// - https://arm64.syscall.sh/

/// Linux `sys/syscall.h` constants for `aarch64`.
#[allow(non_camel_case_types)]
pub struct LINUX_SYS_AARCH64;

#[allow(missing_docs)]
impl LINUX_SYS_AARCH64 {
    pub const ACCEPT: isize = 202;
    pub const ACCEPT4: isize = 242;
    pub const ACCT: isize = 89;
    pub const ADD_KEY: isize = 217;
    pub const ADJTIMEX: isize = 171;
    pub const ARCH_SPECIFIC_SYSCALL: isize = 244;
    pub const BIND: isize = 200;
    pub const BPF: isize = 280;
    pub const BRK: isize = 214;
    pub const CAPGET: isize = 90;
    pub const CAPSET: isize = 91;
    pub const CHDIR: isize = 49;
    pub const CHROOT: isize = 51;
    pub const CLOCK_ADJTIME: isize = 266;
    pub const CLOCK_GETRES: isize = 114;
    pub const CLOCK_GETTIME: isize = 113;
    pub const CLOCK_NANOSLEEP: isize = 115;
    pub const CLOCK_SETTIME: isize = 112;
    pub const CLONE: isize = 220;
    pub const CLONE3: isize = 435;
    pub const CLOSE: isize = 57;
    pub const CLOSE_RANGE: isize = 436;
    pub const CONNECT: isize = 203;
    pub const COPY_FILE_RANGE: isize = 285;
    pub const DELETE_MODULE: isize = 106;
    pub const DUP: isize = 23;
    pub const DUP3: isize = 24;
    pub const EPOLL_CREATE1: isize = 20;
    pub const EPOLL_CTL: isize = 21;
    pub const EPOLL_PWAIT: isize = 22;
    pub const EPOLL_PWAIT2: isize = 441;
    pub const EVENTFD2: isize = 19;
    pub const EXECVE: isize = 221;
    pub const EXECVEAT: isize = 281;
    pub const EXIT: isize = 93;
    pub const EXIT_GROUP: isize = 94;
    pub const FACCESSAT2: isize = 439;
    pub const FACCESSAT: isize = 48;
    pub const FALLOCATE: isize = 47;
    pub const FANOTIFY_INIT: isize = 262;
    pub const FANOTIFY_MARK: isize = 263;
    pub const FCHDIR: isize = 50;
    pub const FCHMOD: isize = 52;
    pub const FCHMODAT: isize = 53;
    pub const FCHOWN: isize = 55;
    pub const FCHOWNAT: isize = 54;
    pub const FDATASYNC: isize = 83;
    pub const FGETXATTR: isize = 10;
    pub const FINIT_MODULE: isize = 273;
    pub const FLISTXATTR: isize = 13;
    pub const FLOCK: isize = 32;
    pub const FREMOVEXATTR: isize = 16;
    pub const FSCONFIG: isize = 431;
    pub const FSETXATTR: isize = 7;
    pub const FSMOUNT: isize = 432;
    pub const FSOPEN: isize = 430;
    pub const FSPICK: isize = 433;
    pub const FSYNC: isize = 82;
    pub const FUTEX: isize = 98;
    pub const GETCPU: isize = 168;
    pub const GETCWD: isize = 17;
    pub const GETDENTS64: isize = 61;
    pub const GETEGID: isize = 177;
    pub const GETEUID: isize = 175;
    pub const GETGID: isize = 176;
    pub const GETGROUPS: isize = 158;
    pub const GETITIMER: isize = 102;
    pub const GET_MEMPOLICY: isize = 236;
    pub const GETPEERNAME: isize = 205;
    pub const GETPGID: isize = 155;
    pub const GETPID: isize = 172;
    pub const GETPPID: isize = 173;
    pub const GETPRIORITY: isize = 141;
    pub const GETRANDOM: isize = 278;
    pub const GETRESGID: isize = 150;
    pub const GETRESUID: isize = 148;
    pub const GETRLIMIT: isize = 163;
    pub const GET_ROBUST_LIST: isize = 100;
    pub const GETRUSAGE: isize = 165;
    pub const GETSID: isize = 156;
    pub const GETSOCKNAME: isize = 204;
    pub const GETSOCKOPT: isize = 209;
    pub const GETTID: isize = 178;
    pub const GETTIMEOFDAY: isize = 169;
    pub const GETUID: isize = 174;
    pub const GETXATTR: isize = 8;
    pub const INIT_MODULE: isize = 105;
    pub const INOTIFY_ADD_WATCH: isize = 27;
    pub const INOTIFY_INIT1: isize = 26;
    pub const INOTIFY_RM_WATCH: isize = 28;
    pub const IO_CANCEL: isize = 3;
    pub const IOCTL: isize = 29;
    pub const IO_DESTROY: isize = 1;
    pub const IO_GETEVENTS: isize = 4;
    pub const IO_PGETEVENTS: isize = 292;
    pub const IOPRIO_GET: isize = 31;
    pub const IOPRIO_SET: isize = 30;
    pub const IO_SETUP: isize = 0;
    pub const IO_SUBMIT: isize = 2;
    pub const IO_URING_ENTER: isize = 426;
    pub const IO_URING_REGISTER: isize = 427;
    pub const IO_URING_SETUP: isize = 425;
    pub const KCMP: isize = 272;
    pub const KEXEC_FILE_LOAD: isize = 294;
    pub const KEXEC_LOAD: isize = 104;
    pub const KEYCTL: isize = 219;
    pub const KILL: isize = 129;
    pub const LANDLOCK_ADD_RULE: isize = 445;
    pub const LANDLOCK_CREATE_RULESET: isize = 444;
    pub const LANDLOCK_RESTRICT_SELF: isize = 446;
    pub const LGETXATTR: isize = 9;
    pub const LINKAT: isize = 37;
    pub const LISTEN: isize = 201;
    pub const LISTXATTR: isize = 11;
    pub const LLISTXATTR: isize = 12;
    pub const LOOKUP_DCOOKIE: isize = 18;
    pub const LREMOVEXATTR: isize = 15;
    pub const LSETXATTR: isize = 6;
    pub const MADVISE: isize = 233;
    pub const MBIND: isize = 235;
    pub const MEMBARRIER: isize = 283;
    pub const MEMFD_CREATE: isize = 279;
    pub const MEMFD_SECRET: isize = 447;
    pub const MIGRATE_PAGES: isize = 238;
    pub const MINCORE: isize = 232;
    pub const MKDIRAT: isize = 34;
    pub const MKNODAT: isize = 33;
    pub const MLOCK: isize = 228;
    pub const MLOCK2: isize = 284;
    pub const MLOCKALL: isize = 230;
    pub const MOUNT: isize = 40;
    pub const MOUNT_SETATTR: isize = 442;
    pub const MOVE_MOUNT: isize = 429;
    pub const MOVE_PAGES: isize = 239;
    pub const MPROTECT: isize = 226;
    pub const MQ_GETSETATTR: isize = 185;
    pub const MQ_NOTIFY: isize = 184;
    pub const MQ_OPEN: isize = 180;
    pub const MQ_TIMEDRECEIVE: isize = 183;
    pub const MQ_TIMEDSEND: isize = 182;
    pub const MQ_UNLINK: isize = 181;
    pub const MREMAP: isize = 216;
    pub const MSGCTL: isize = 187;
    pub const MSGGET: isize = 186;
    pub const MSGRCV: isize = 188;
    pub const MSGSND: isize = 189;
    pub const MSYNC: isize = 227;
    pub const MUNLOCK: isize = 229;
    pub const MUNLOCKALL: isize = 231;
    pub const MUNMAP: isize = 215;
    pub const NAME_TO_HANDLE_AT: isize = 264;
    pub const NANOSLEEP: isize = 101;
    pub const NFSSERVCTL: isize = 42;
    pub const OPENAT2: isize = 437;
    pub const OPENAT: isize = 56;
    pub const OPEN_BY_HANDLE_AT: isize = 265;
    pub const OPEN_TREE: isize = 428;
    pub const PERF_EVENT_OPEN: isize = 241;
    pub const PERSONALITY: isize = 92;
    pub const PIDFD_GETFD: isize = 438;
    pub const PIDFD_OPEN: isize = 434;
    pub const PIDFD_SEND_SIGNAL: isize = 424;
    pub const PIPE2: isize = 59;
    pub const PIVOT_ROOT: isize = 41;
    pub const PKEY_ALLOC: isize = 289;
    pub const PKEY_FREE: isize = 290;
    pub const PKEY_MPROTECT: isize = 288;
    pub const PPOLL: isize = 73;
    pub const PRCTL: isize = 167;
    pub const PREAD64: isize = 67;
    pub const PREADV2: isize = 286;
    pub const PREADV: isize = 69;
    pub const PRLIMIT64: isize = 261;
    pub const PROCESS_MADVISE: isize = 440;
    pub const PROCESS_MRELEASE: isize = 448;
    pub const PROCESS_VM_READV: isize = 270;
    pub const PROCESS_VM_WRITEV: isize = 271;
    pub const PSELECT6: isize = 72;
    pub const PTRACE: isize = 117;
    pub const PWRITE64: isize = 68;
    pub const PWRITEV2: isize = 287;
    pub const PWRITEV: isize = 70;
    pub const QUOTACTL: isize = 60;
    pub const QUOTACTL_FD: isize = 443;
    pub const READ: isize = 63;
    pub const READAHEAD: isize = 213;
    pub const READLINKAT: isize = 78;
    pub const READV: isize = 65;
    pub const REBOOT: isize = 142;
    pub const RECVFROM: isize = 207;
    pub const RECVMMSG: isize = 243;
    pub const RECVMSG: isize = 212;
    pub const REMAP_FILE_PAGES: isize = 234;
    pub const REMOVEXATTR: isize = 14;
    pub const RENAMEAT2: isize = 276;
    pub const RENAMEAT: isize = 38;
    pub const REQUEST_KEY: isize = 218;
    pub const RESTART_SYSCALL: isize = 128;
    pub const RSEQ: isize = 293;
    pub const RT_SIGACTION: isize = 134;
    pub const RT_SIGPENDING: isize = 136;
    pub const RT_SIGPROCMASK: isize = 135;
    pub const RT_SIGQUEUEINFO: isize = 138;
    pub const RT_SIGRETURN: isize = 139;
    pub const RT_SIGSUSPEND: isize = 133;
    pub const RT_SIGTIMEDWAIT: isize = 137;
    pub const RT_TGSIGQUEUEINFO: isize = 240;
    pub const SCHED_GETAFFINITY: isize = 123;
    pub const SCHED_GETATTR: isize = 275;
    pub const SCHED_GETPARAM: isize = 121;
    pub const SCHED_GET_PRIORITY_MAX: isize = 125;
    pub const SCHED_GET_PRIORITY_MIN: isize = 126;
    pub const SCHED_GETSCHEDULER: isize = 120;
    pub const SCHED_RR_GET_INTERVAL: isize = 127;
    pub const SCHED_SETAFFINITY: isize = 122;
    pub const SCHED_SETATTR: isize = 274;
    pub const SCHED_SETPARAM: isize = 118;
    pub const SCHED_SETSCHEDULER: isize = 119;
    pub const SCHED_YIELD: isize = 124;
    pub const SECCOMP: isize = 277;
    pub const SEMCTL: isize = 191;
    pub const SEMGET: isize = 190;
    pub const SEMOP: isize = 193;
    pub const SEMTIMEDOP: isize = 192;
    pub const SENDMMSG: isize = 269;
    pub const SENDMSG: isize = 211;
    pub const SENDTO: isize = 206;
    pub const SETDOMAINNAME: isize = 162;
    pub const SETFSGID: isize = 152;
    pub const SETFSUID: isize = 151;
    pub const SETGID: isize = 144;
    pub const SETGROUPS: isize = 159;
    pub const SETHOSTNAME: isize = 161;
    pub const SETITIMER: isize = 103;
    pub const SET_MEMPOLICY: isize = 237;
    pub const SETNS: isize = 268;
    pub const SETPGID: isize = 154;
    pub const SETPRIORITY: isize = 140;
    pub const SETREGID: isize = 143;
    pub const SETRESGID: isize = 149;
    pub const SETRESUID: isize = 147;
    pub const SETREUID: isize = 145;
    pub const SETRLIMIT: isize = 164;
    pub const SET_ROBUST_LIST: isize = 99;
    pub const SETSID: isize = 157;
    pub const SETSOCKOPT: isize = 208;
    pub const SET_TID_ADDRESS: isize = 96;
    pub const SETTIMEOFDAY: isize = 170;
    pub const SETUID: isize = 146;
    pub const SETXATTR: isize = 5;
    pub const SHMAT: isize = 196;
    pub const SHMCTL: isize = 195;
    pub const SHMDT: isize = 197;
    pub const SHMGET: isize = 194;
    pub const SHUTDOWN: isize = 210;
    pub const SIGALTSTACK: isize = 132;
    pub const SIGNALFD4: isize = 74;
    pub const SOCKET: isize = 198;
    pub const SOCKETPAIR: isize = 199;
    pub const SPLICE: isize = 76;
    pub const STATX: isize = 291;
    pub const SWAPOFF: isize = 225;
    pub const SWAPON: isize = 224;
    pub const SYMLINKAT: isize = 36;
    pub const SYNC: isize = 81;
    pub const SYNC_FILE_RANGE: isize = 84;
    pub const SYNCFS: isize = 267;
    pub const SYSCALLS: isize = 449;
    pub const SYSINFO: isize = 179;
    pub const SYSLOG: isize = 116;
    pub const TEE: isize = 77;
    pub const TGKILL: isize = 131;
    pub const TIMER_CREATE: isize = 107;
    pub const TIMER_DELETE: isize = 111;
    pub const TIMERFD_CREATE: isize = 85;
    pub const TIMERFD_GETTIME: isize = 87;
    pub const TIMERFD_SETTIME: isize = 86;
    pub const TIMER_GETOVERRUN: isize = 109;
    pub const TIMER_GETTIME: isize = 108;
    pub const TIMER_SETTIME: isize = 110;
    pub const TIMES: isize = 153;
    pub const TKILL: isize = 130;
    pub const UMASK: isize = 166;
    pub const UMOUNT2: isize = 39;
    pub const UNAME: isize = 160;
    pub const UNLINKAT: isize = 35;
    pub const UNSHARE: isize = 97;
    pub const USERFAULTFD: isize = 282;
    pub const UTIMENSAT: isize = 88;
    pub const VHANGUP: isize = 58;
    pub const VMSPLICE: isize = 75;
    pub const WAIT4: isize = 260;
    pub const WAITID: isize = 95;
    pub const WRITE: isize = 64;
    pub const WRITEV: isize = 66;
}
