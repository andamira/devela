// devela::sys::os::linux::consts::syscall::x86_64
//
//!
//
// - https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/

/// Linux `sys/syscall.h` constants for `x86_64`.
#[allow(non_camel_case_types)]
pub struct LINUX_SYS_X86_64;

#[allow(missing_docs)]
impl LINUX_SYS_X86_64 {
    pub const ACCEPT4: isize = 288;
    pub const ACCEPT: isize = 43;
    pub const ACCESS: isize = 21;
    pub const ACCT: isize = 163;
    pub const ADD_KEY: isize = 248;
    pub const ADJTIMEX: isize = 159;
    pub const AFS_SYSCALL: isize = 183;
    pub const ALARM: isize = 37;
    pub const ARCH_PRCTL: isize = 158;
    pub const BIND: isize = 49;
    pub const BPF: isize = 321;
    pub const BRK: isize = 12;
    pub const CAPGET: isize = 125;
    pub const CAPSET: isize = 126;
    pub const CHDIR: isize = 80;
    pub const CHMOD: isize = 90;
    pub const CHOWN: isize = 92;
    pub const CHROOT: isize = 161;
    pub const CLOCK_ADJTIME: isize = 305;
    pub const CLOCK_GETRES: isize = 229;
    pub const CLOCK_GETTIME: isize = 228;
    pub const CLOCK_NANOSLEEP: isize = 230;
    pub const CLOCK_SETTIME: isize = 227;
    pub const CLONE3: isize = 435;
    pub const CLONE: isize = 56;
    pub const CLOSE: isize = 3;
    pub const CLOSE_RANGE: isize = 436;
    pub const CONNECT: isize = 42;
    pub const COPY_FILE_RANGE: isize = 326;
    pub const CREAT: isize = 85;
    pub const CREATE_MODULE: isize = 174;
    pub const DELETE_MODULE: isize = 176;
    pub const DUP2: isize = 33;
    pub const DUP: isize = 32;
    pub const DUP3: isize = 292;
    pub const EPOLL_CREATE1: isize = 291;
    pub const EPOLL_CREATE: isize = 213;
    pub const EPOLL_CTL: isize = 233;
    pub const EPOLL_CTL_OLD: isize = 214;
    pub const EPOLL_PWAIT2: isize = 441;
    pub const EPOLL_PWAIT: isize = 281;
    pub const EPOLL_WAIT: isize = 232;
    pub const EPOLL_WAIT_OLD: isize = 215;
    pub const EVENTFD2: isize = 290;
    pub const EVENTFD: isize = 284;
    pub const EXECVE: isize = 59;
    pub const EXECVEAT: isize = 322;
    pub const EXIT: isize = 60;
    pub const EXIT_GROUP: isize = 231;
    pub const FACCESSAT2: isize = 439;
    pub const FACCESSAT: isize = 269;
    pub const FADVISE64: isize = 221;
    pub const FALLOCATE: isize = 285;
    pub const FANOTIFY_INIT: isize = 300;
    pub const FANOTIFY_MARK: isize = 301;
    pub const FCHDIR: isize = 81;
    pub const FCHMOD: isize = 91;
    pub const FCHMODAT: isize = 268;
    pub const FCHOWN: isize = 93;
    pub const FCHOWNAT: isize = 260;
    pub const FCNTL: isize = 72;
    pub const FDATASYNC: isize = 75;
    pub const FGETXATTR: isize = 193;
    pub const FINIT_MODULE: isize = 313;
    pub const FLISTXATTR: isize = 196;
    pub const FLOCK: isize = 73;
    pub const FORK: isize = 57;
    pub const FREMOVEXATTR: isize = 199;
    pub const FSCONFIG: isize = 431;
    pub const FSETXATTR: isize = 190;
    pub const FSMOUNT: isize = 432;
    pub const FSOPEN: isize = 430;
    pub const FSPICK: isize = 433;
    pub const FSTAT: isize = 5;
    pub const FSTATFS: isize = 138;
    pub const FSYNC: isize = 74;
    pub const FTRUNCATE: isize = 77;
    pub const FUTEX: isize = 202;
    pub const FUTIMESAT: isize = 261;
    pub const GETCPU: isize = 309;
    pub const GETCWD: isize = 79;
    pub const GETDENTS64: isize = 217;
    pub const GETDENTS: isize = 78;
    pub const GETEGID: isize = 108;
    pub const GETEUID: isize = 107;
    pub const GETGID: isize = 104;
    pub const GETGROUPS: isize = 115;
    pub const GETITIMER: isize = 36;
    pub const GET_KERNEL_SYMS: isize = 177;
    pub const GET_MEMPOLICY: isize = 239;
    pub const GETPEERNAME: isize = 52;
    pub const GETPGID: isize = 121;
    pub const GETPGRP: isize = 111;
    pub const GETPID: isize = 39;
    pub const GETPMSG: isize = 181;
    pub const GETPPID: isize = 110;
    pub const GETPRIORITY: isize = 140;
    pub const GETRANDOM: isize = 318;
    pub const GETRESGID: isize = 120;
    pub const GETRESUID: isize = 118;
    pub const GETRLIMIT: isize = 97;
    pub const GET_ROBUST_LIST: isize = 274;
    pub const GETRUSAGE: isize = 98;
    pub const GETSID: isize = 124;
    pub const GETSOCKNAME: isize = 51;
    pub const GETSOCKOPT: isize = 55;
    pub const GET_THREAD_AREA: isize = 211;
    pub const GETTID: isize = 186;
    pub const GETTIMEOFDAY: isize = 96;
    pub const GETUID: isize = 102;
    pub const GETXATTR: isize = 191;
    pub const INIT_MODULE: isize = 175;
    pub const INOTIFY_ADD_WATCH: isize = 254;
    pub const INOTIFY_INIT1: isize = 294;
    pub const INOTIFY_INIT: isize = 253;
    pub const INOTIFY_RM_WATCH: isize = 255;
    pub const IO_CANCEL: isize = 210;
    pub const IOCTL: isize = 16;
    pub const IO_DESTROY: isize = 207;
    pub const IO_GETEVENTS: isize = 208;
    pub const IOPERM: isize = 173;
    pub const IO_PGETEVENTS: isize = 333;
    pub const IOPL: isize = 172;
    pub const IOPRIO_GET: isize = 252;
    pub const IOPRIO_SET: isize = 251;
    pub const IO_SETUP: isize = 206;
    pub const IO_SUBMIT: isize = 209;
    pub const IO_URING_ENTER: isize = 426;
    pub const IO_URING_REGISTER: isize = 427;
    pub const IO_URING_SETUP: isize = 425;
    pub const KCMP: isize = 312;
    pub const KEXEC_FILE_LOAD: isize = 320;
    pub const KEXEC_LOAD: isize = 246;
    pub const KEYCTL: isize = 250;
    pub const KILL: isize = 62;
    pub const LANDLOCK_ADD_RULE: isize = 445;
    pub const LANDLOCK_CREATE_RULESET: isize = 444;
    pub const LANDLOCK_RESTRICT_SELF: isize = 446;
    pub const LCHOWN: isize = 94;
    pub const LGETXATTR: isize = 192;
    pub const LINK: isize = 86;
    pub const LINKAT: isize = 265;
    pub const LISTEN: isize = 50;
    pub const LISTXATTR: isize = 194;
    pub const LLISTXATTR: isize = 195;
    pub const LOOKUP_DCOOKIE: isize = 212;
    pub const LREMOVEXATTR: isize = 198;
    pub const LSEEK: isize = 8;
    pub const LSETXATTR: isize = 189;
    pub const LSTAT: isize = 6;
    pub const MADVISE: isize = 28;
    pub const MBIND: isize = 237;
    pub const MEMBARRIER: isize = 324;
    pub const MEMFD_CREATE: isize = 319;
    pub const MEMFD_SECRET: isize = 447;
    pub const MIGRATE_PAGES: isize = 256;
    pub const MINCORE: isize = 27;
    pub const MKDIR: isize = 83;
    pub const MKDIRAT: isize = 258;
    pub const MKNOD: isize = 133;
    pub const MKNODAT: isize = 259;
    pub const MLOCK: isize = 149;
    pub const MLOCK2: isize = 325;
    pub const MLOCKALL: isize = 151;
    pub const MMAP: isize = 9;
    pub const MODIFY_LDT: isize = 154;
    pub const MOUNT: isize = 165;
    pub const MOUNT_SETATTR: isize = 442;
    pub const MOVE_MOUNT: isize = 429;
    pub const MOVE_PAGES: isize = 279;
    pub const MPROTECT: isize = 10;
    pub const MQ_GETSETATTR: isize = 245;
    pub const MQ_NOTIFY: isize = 244;
    pub const MQ_OPEN: isize = 240;
    pub const MQ_TIMEDRECEIVE: isize = 243;
    pub const MQ_TIMEDSEND: isize = 242;
    pub const MQ_UNLINK: isize = 241;
    pub const MREMAP: isize = 25;
    pub const MSGCTL: isize = 71;
    pub const MSGGET: isize = 68;
    pub const MSGRCV: isize = 70;
    pub const MSGSND: isize = 69;
    pub const MSYNC: isize = 26;
    pub const MUNLOCK: isize = 150;
    pub const MUNLOCKALL: isize = 152;
    pub const MUNMAP: isize = 11;
    pub const NAME_TO_HANDLE_AT: isize = 303;
    pub const NANOSLEEP: isize = 35;
    pub const NEWFSTATAT: isize = 262;
    pub const NFSSERVCTL: isize = 180;
    pub const OPEN: isize = 2;
    pub const OPENAT2: isize = 437;
    pub const OPENAT: isize = 257;
    pub const OPEN_BY_HANDLE_AT: isize = 304;
    pub const OPEN_TREE: isize = 428;
    pub const PAUSE: isize = 34;
    pub const PERF_EVENT_OPEN: isize = 298;
    pub const PERSONALITY: isize = 135;
    pub const PIDFD_GETFD: isize = 438;
    pub const PIDFD_OPEN: isize = 434;
    pub const PIDFD_SEND_SIGNAL: isize = 424;
    pub const PIPE: isize = 22;
    pub const PIPE2: isize = 293;
    pub const PIVOT_ROOT: isize = 155;
    pub const PKEY_ALLOC: isize = 330;
    pub const PKEY_FREE: isize = 331;
    pub const PKEY_MPROTECT: isize = 329;
    pub const POLL: isize = 7;
    pub const PPOLL: isize = 271;
    pub const PRCTL: isize = 157;
    pub const PREAD64: isize = 17;
    pub const PREADV2: isize = 327;
    pub const PREADV: isize = 295;
    pub const PRLIMIT64: isize = 302;
    pub const PROCESS_MADVISE: isize = 440;
    pub const PROCESS_MRELEASE: isize = 448;
    pub const PROCESS_VM_READV: isize = 310;
    pub const PROCESS_VM_WRITEV: isize = 311;
    pub const PSELECT6: isize = 270;
    pub const PTRACE: isize = 101;
    pub const PUTPMSG: isize = 182;
    pub const PWRITE64: isize = 18;
    pub const PWRITEV2: isize = 328;
    pub const PWRITEV: isize = 296;
    pub const QUERY_MODULE: isize = 178;
    pub const QUOTACTL: isize = 179;
    pub const QUOTACTL_FD: isize = 443;
    pub const READ: isize = 0;
    pub const READAHEAD: isize = 187;
    pub const READLINK: isize = 89;
    pub const READLINKAT: isize = 267;
    pub const READV: isize = 19;
    pub const REBOOT: isize = 169;
    pub const RECVFROM: isize = 45;
    pub const RECVMMSG: isize = 299;
    pub const RECVMSG: isize = 47;
    pub const REMAP_FILE_PAGES: isize = 216;
    pub const REMOVEXATTR: isize = 197;
    pub const RENAME: isize = 82;
    pub const RENAMEAT2: isize = 316;
    pub const RENAMEAT: isize = 264;
    pub const REQUEST_KEY: isize = 249;
    pub const RESTART_SYSCALL: isize = 219;
    pub const RMDIR: isize = 84;
    pub const RSEQ: isize = 334;
    pub const RT_SIGACTION: isize = 13;
    pub const RT_SIGPENDING: isize = 127;
    pub const RT_SIGPROCMASK: isize = 14;
    pub const RT_SIGQUEUEINFO: isize = 129;
    pub const RT_SIGRETURN: isize = 15;
    pub const RT_SIGSUSPEND: isize = 130;
    pub const RT_SIGTIMEDWAIT: isize = 128;
    pub const RT_TGSIGQUEUEINFO: isize = 297;
    pub const SCHED_GETAFFINITY: isize = 204;
    pub const SCHED_GETATTR: isize = 315;
    pub const SCHED_GETPARAM: isize = 143;
    pub const SCHED_GET_PRIORITY_MAX: isize = 146;
    pub const SCHED_GET_PRIORITY_MIN: isize = 147;
    pub const SCHED_GETSCHEDULER: isize = 145;
    pub const SCHED_RR_GET_INTERVAL: isize = 148;
    pub const SCHED_SETAFFINITY: isize = 203;
    pub const SCHED_SETATTR: isize = 314;
    pub const SCHED_SETPARAM: isize = 142;
    pub const SCHED_SETSCHEDULER: isize = 144;
    pub const SCHED_YIELD: isize = 24;
    pub const SECCOMP: isize = 317;
    pub const SECURITY: isize = 185;
    pub const SELECT: isize = 23;
    pub const SEMCTL: isize = 66;
    pub const SEMGET: isize = 64;
    pub const SEMOP: isize = 65;
    pub const SEMTIMEDOP: isize = 220;
    pub const SENDFILE: isize = 40;
    pub const SENDMMSG: isize = 307;
    pub const SENDMSG: isize = 46;
    pub const SENDTO: isize = 44;
    pub const SETDOMAINNAME: isize = 171;
    pub const SETFSGID: isize = 123;
    pub const SETFSUID: isize = 122;
    pub const SETGID: isize = 106;
    pub const SETGROUPS: isize = 116;
    pub const SETHOSTNAME: isize = 170;
    pub const SETITIMER: isize = 38;
    pub const SET_MEMPOLICY: isize = 238;
    pub const SETNS: isize = 308;
    pub const SETPGID: isize = 109;
    pub const SETPRIORITY: isize = 141;
    pub const SETREGID: isize = 114;
    pub const SETRESGID: isize = 119;
    pub const SETRESUID: isize = 117;
    pub const SETREUID: isize = 113;
    pub const SETRLIMIT: isize = 160;
    pub const SET_ROBUST_LIST: isize = 273;
    pub const SETSID: isize = 112;
    pub const SETSOCKOPT: isize = 54;
    pub const SET_THREAD_AREA: isize = 205;
    pub const SET_TID_ADDRESS: isize = 218;
    pub const SETTIMEOFDAY: isize = 164;
    pub const SETUID: isize = 105;
    pub const SETXATTR: isize = 188;
    pub const SHMAT: isize = 30;
    pub const SHMCTL: isize = 31;
    pub const SHMDT: isize = 67;
    pub const SHMGET: isize = 29;
    pub const SHUTDOWN: isize = 48;
    pub const SIGALTSTACK: isize = 131;
    pub const SIGNALFD: isize = 282;
    pub const SIGNALFD4: isize = 289;
    pub const SOCKET: isize = 41;
    pub const SOCKETPAIR: isize = 53;
    pub const SPLICE: isize = 275;
    pub const STAT: isize = 4;
    pub const STATFS: isize = 137;
    pub const STATX: isize = 332;
    pub const SWAPOFF: isize = 168;
    pub const SWAPON: isize = 167;
    pub const SYMLINK: isize = 88;
    pub const SYMLINKAT: isize = 266;
    pub const SYNC: isize = 162;
    pub const SYNC_FILE_RANGE: isize = 277;
    pub const SYNCFS: isize = 306;
    pub const _SYSCTL: isize = 156;
    pub const SYSFS: isize = 139;
    pub const SYSINFO: isize = 99;
    pub const SYSLOG: isize = 103;
    pub const TEE: isize = 276;
    pub const TGKILL: isize = 234;
    pub const TIME: isize = 201;
    pub const TIMER_CREATE: isize = 222;
    pub const TIMER_DELETE: isize = 226;
    pub const TIMERFD_CREATE: isize = 283;
    pub const TIMERFD_GETTIME: isize = 287;
    pub const TIMERFD_SETTIME: isize = 286;
    pub const TIMER_GETOVERRUN: isize = 225;
    pub const TIMER_GETTIME: isize = 224;
    pub const TIMER_SETTIME: isize = 223;
    pub const TIMES: isize = 100;
    pub const TKILL: isize = 200;
    pub const TRUNCATE: isize = 76;
    pub const TUXCALL: isize = 184;
    pub const UMASK: isize = 95;
    pub const UMOUNT2: isize = 166;
    pub const UNAME: isize = 63;
    pub const UNLINK: isize = 87;
    pub const UNLINKAT: isize = 263;
    pub const UNSHARE: isize = 272;
    pub const USELIB: isize = 134;
    pub const USERFAULTFD: isize = 323;
    pub const USTAT: isize = 136;
    pub const UTIME: isize = 132;
    pub const UTIMENSAT: isize = 280;
    pub const UTIMES: isize = 235;
    pub const VFORK: isize = 58;
    pub const VHANGUP: isize = 153;
    pub const VMSPLICE: isize = 278;
    pub const VSERVER: isize = 236;
    pub const WAIT4: isize = 61;
    pub const WAITID: isize = 247;
    pub const WRITE: isize = 1;
    pub const WRITEV: isize = 20;
}