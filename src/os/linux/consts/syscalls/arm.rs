// devela::os::linux::consts::syscalls::arm
//
//! Defines the syscall numbers for `armv7`.
//

/// Syscall numbers for `armv7`.
#[allow(non_camel_case_types)]
pub struct SYS_ARM;

impl SYS_ARM {
    pub const ACCEPT: isize = 285;
    pub const ACCEPT4: isize = 366;
    pub const ACCESS: isize = 33;
    pub const ACCT: isize = 51;
    pub const ADD_KEY: isize = 309;
    pub const ADJTIMEX: isize = 124;
    pub const ARM_FADVISE64_64: isize = 270;
    pub const ARM_SYNC_FILE_RANGE: isize = 341;
    pub const BDFLUSH: isize = 134;
    pub const BIND: isize = 282;
    pub const BPF: isize = 386;
    pub const BRK: isize = 45;
    pub const CAPGET: isize = 184;
    pub const CAPSET: isize = 185;
    pub const CHDIR: isize = 12;
    pub const CHMOD: isize = 15;
    pub const CHOWN: isize = 182;
    pub const CHOWN32: isize = 212;
    pub const CHROOT: isize = 61;
    pub const CLOCK_ADJTIME: isize = 372;
    pub const CLOCK_ADJTIME64: isize = 405;
    pub const CLOCK_GETRES: isize = 264;
    pub const CLOCK_GETRES_TIME64: isize = 406;
    pub const CLOCK_GETTIME: isize = 263;
    pub const CLOCK_GETTIME64: isize = 403;
    pub const CLOCK_NANOSLEEP: isize = 265;
    pub const CLOCK_NANOSLEEP_TIME64: isize = 407;
    pub const CLOCK_SETTIME: isize = 262;
    pub const CLOCK_SETTIME64: isize = 404;
    pub const CLONE: isize = 120;
    pub const CLONE3: isize = 435;
    pub const CLOSE: isize = 6;
    pub const CLOSE_RANGE: isize = 436;
    pub const CONNECT: isize = 283;
    pub const COPY_FILE_RANGE: isize = 391;
    pub const CREAT: isize = 8;
    pub const DELETE_MODULE: isize = 129;
    pub const DUP2: isize = 63;
    pub const DUP3: isize = 358;
    pub const DUP: isize = 41;
    pub const EPOLL_CREATE1: isize = 357;
    pub const EPOLL_CREATE: isize = 250;
    pub const EPOLL_CTL: isize = 251;
    pub const EPOLL_PWAIT2: isize = 441;
    pub const EPOLL_PWAIT: isize = 346;
    pub const EPOLL_WAIT: isize = 252;
    pub const EVENTFD2: isize = 356;
    pub const EVENTFD: isize = 351;
    pub const EXECVE: isize = 11;
    pub const EXECVEAT: isize = 387;
    pub const EXIT: isize = 1;
    pub const EXIT_GROUP: isize = 248;
    pub const FACCESSAT2: isize = 439;
    pub const FACCESSAT: isize = 334;
    pub const FALLOCATE: isize = 352;
    pub const FANOTIFY_INIT: isize = 367;
    pub const FANOTIFY_MARK: isize = 368;
    pub const FCHDIR: isize = 133;
    pub const FCHMOD: isize = 94;
    pub const FCHMODAT: isize = 333;
    pub const FCHOWN32: isize = 207;
    pub const FCHOWN: isize = 95;
    pub const FCHOWNAT: isize = 325;
    pub const FCNTL: isize = 55;
    pub const FCNTL64: isize = 221;
    pub const FDATASYNC: isize = 148;
    pub const FGETXATTR: isize = 231;
    pub const FINIT_MODULE: isize = 379;
    pub const FLISTXATTR: isize = 234;
    pub const FLOCK: isize = 143;
    pub const FORK: isize = 2;
    pub const FREMOVEXATTR: isize = 237;
    pub const FSCONFIG: isize = 431;
    pub const FSETXATTR: isize = 228;
    pub const FSMOUNT: isize = 432;
    pub const FSOPEN: isize = 430;
    pub const FSPICK: isize = 433;
    pub const FSTAT: isize = 108;
    pub const FSTAT64: isize = 197;
    pub const FSTATAT64: isize = 327;
    pub const FSTATFS: isize = 100;
    pub const FSTATFS64: isize = 267;
    pub const FSYNC: isize = 118;
    pub const FTRUNCATE64: isize = 194;
    pub const FTRUNCATE: isize = 93;
    pub const FUTEX: isize = 240;
    pub const FUTEX_TIME64: isize = 422;
    pub const FUTIMESAT: isize = 326;
    pub const GETCPU: isize = 345;
    pub const GETCWD: isize = 183;
    pub const GETDENTS: isize = 141;
    pub const GETDENTS64: isize = 217;
    pub const GETEGID32: isize = 202;
    pub const GETEGID: isize = 50;
    pub const GETEUID32: isize = 201;
    pub const GETEUID: isize = 49;
    pub const GETGID32: isize = 200;
    pub const GETGID: isize = 47;
    pub const GETGROUPS32: isize = 205;
    pub const GETGROUPS: isize = 80;
    pub const GETITIMER: isize = 105;
    pub const GET_MEMPOLICY: isize = 320;
    pub const GETPEERNAME: isize = 287;
    pub const GETPGID: isize = 132;
    pub const GETPGRP: isize = 65;
    pub const GETPID: isize = 20;
    pub const GETPPID: isize = 64;
    pub const GETPRIORITY: isize = 96;
    pub const GETRANDOM: isize = 384;
    pub const GETRESGID: isize = 171;
    pub const GETRESGID32: isize = 211;
    pub const GETRESUID: isize = 165;
    pub const GETRESUID32: isize = 209;
    pub const GET_ROBUST_LIST: isize = 339;
    pub const GETRUSAGE: isize = 77;
    pub const GETSID: isize = 147;
    pub const GETSOCKNAME: isize = 286;
    pub const GETSOCKOPT: isize = 295;
    pub const GETTID: isize = 224;
    pub const GETTIMEOFDAY: isize = 78;
    pub const GETUID: isize = 24;
    pub const GETUID32: isize = 199;
    pub const GETXATTR: isize = 229;
    pub const INIT_MODULE: isize = 128;
    pub const INOTIFY_ADD_WATCH: isize = 317;
    pub const INOTIFY_INIT1: isize = 360;
    pub const INOTIFY_INIT: isize = 316;
    pub const INOTIFY_RM_WATCH: isize = 318;
    pub const IO_CANCEL: isize = 247;
    pub const IOCTL: isize = 54;
    pub const IO_DESTROY: isize = 244;
    pub const IO_GETEVENTS: isize = 245;
    pub const IO_PGETEVENTS: isize = 399;
    pub const IO_PGETEVENTS_TIME64: isize = 416;
    pub const IOPRIO_GET: isize = 315;
    pub const IOPRIO_SET: isize = 314;
    pub const IO_SETUP: isize = 243;
    pub const IO_SUBMIT: isize = 246;
    pub const IO_URING_ENTER: isize = 426;
    pub const IO_URING_REGISTER: isize = 427;
    pub const IO_URING_SETUP: isize = 425;
    pub const KCMP: isize = 378;
    pub const KEXEC_FILE_LOAD: isize = 401;
    pub const KEXEC_LOAD: isize = 347;
    pub const KEYCTL: isize = 311;
    pub const KILL: isize = 37;
    pub const LANDLOCK_ADD_RULE: isize = 445;
    pub const LANDLOCK_CREATE_RULESET: isize = 444;
    pub const LANDLOCK_RESTRICT_SELF: isize = 446;
    pub const LCHOWN: isize = 16;
    pub const LCHOWN32: isize = 198;
    pub const LGETXATTR: isize = 230;
    pub const LINK: isize = 9;
    pub const LINKAT: isize = 330;
    pub const LISTEN: isize = 284;
    pub const LISTXATTR: isize = 232;
    pub const LLISTXATTR: isize = 233;
    pub const _LLSEEK: isize = 140;
    pub const LOOKUP_DCOOKIE: isize = 249;
    pub const LREMOVEXATTR: isize = 236;
    pub const LSEEK: isize = 19;
    pub const LSETXATTR: isize = 227;
    pub const LSTAT: isize = 107;
    pub const LSTAT64: isize = 196;
    pub const MADVISE: isize = 220;
    pub const MBIND: isize = 319;
    pub const MEMBARRIER: isize = 389;
    pub const MEMFD_CREATE: isize = 385;
    pub const MIGRATE_PAGES: isize = 400;
    pub const MINCORE: isize = 219;
    pub const MKDIR: isize = 39;
    pub const MKDIRAT: isize = 323;
    pub const MKNOD: isize = 14;
    pub const MKNODAT: isize = 324;
    pub const MLOCK: isize = 150;
    pub const MLOCK2: isize = 390;
    pub const MLOCKALL: isize = 152;
    pub const MMAP2: isize = 192;
    pub const MOUNT: isize = 21;
    pub const MOUNT_SETATTR: isize = 442;
    pub const MOVE_MOUNT: isize = 429;
    pub const MOVE_PAGES: isize = 344;
    pub const MPROTECT: isize = 125;
    pub const MQ_GETSETATTR: isize = 279;
    pub const MQ_NOTIFY: isize = 278;
    pub const MQ_OPEN: isize = 274;
    pub const MQ_TIMEDRECEIVE: isize = 277;
    pub const MQ_TIMEDRECEIVE_TIME64: isize = 419;
    pub const MQ_TIMEDSEND: isize = 276;
    pub const MQ_TIMEDSEND_TIME64: isize = 418;
    pub const MQ_UNLINK: isize = 275;
    pub const MREMAP: isize = 163;
    pub const MSGCTL: isize = 304;
    pub const MSGGET: isize = 303;
    pub const MSGRCV: isize = 302;
    pub const MSGSND: isize = 301;
    pub const MSYNC: isize = 144;
    pub const MUNLOCK: isize = 151;
    pub const MUNLOCKALL: isize = 153;
    pub const MUNMAP: isize = 91;
    pub const NAME_TO_HANDLE_AT: isize = 370;
    pub const NANOSLEEP: isize = 162;
    pub const _NEWSELECT: isize = 142;
    pub const NFSSERVCTL: isize = 169;
    pub const NICE: isize = 34;
    pub const OPEN: isize = 5;
    pub const OPENAT2: isize = 437;
    pub const OPENAT: isize = 322;
    pub const OPEN_BY_HANDLE_AT: isize = 371;
    pub const OPEN_TREE: isize = 428;
    pub const PAUSE: isize = 29;
    pub const PCICONFIG_IOBASE: isize = 271;
    pub const PCICONFIG_READ: isize = 272;
    pub const PCICONFIG_WRITE: isize = 273;
    pub const PERF_EVENT_OPEN: isize = 364;
    pub const PERSONALITY: isize = 136;
    pub const PIDFD_GETFD: isize = 438;
    pub const PIDFD_OPEN: isize = 434;
    pub const PIDFD_SEND_SIGNAL: isize = 424;
    pub const PIPE2: isize = 359;
    pub const PIPE: isize = 42;
    pub const PIVOT_ROOT: isize = 218;
    pub const PKEY_ALLOC: isize = 395;
    pub const PKEY_FREE: isize = 396;
    pub const PKEY_MPROTECT: isize = 394;
    pub const POLL: isize = 168;
    pub const PPOLL: isize = 336;
    pub const PPOLL_TIME64: isize = 414;
    pub const PRCTL: isize = 172;
    pub const PREAD64: isize = 180;
    pub const PREADV2: isize = 392;
    pub const PREADV: isize = 361;
    pub const PRLIMIT64: isize = 369;
    pub const PROCESS_MADVISE: isize = 440;
    pub const PROCESS_MRELEASE: isize = 448;
    pub const PROCESS_VM_READV: isize = 376;
    pub const PROCESS_VM_WRITEV: isize = 377;
    pub const PSELECT6: isize = 335;
    pub const PSELECT6_TIME64: isize = 413;
    pub const PTRACE: isize = 26;
    pub const PWRITE64: isize = 181;
    pub const PWRITEV2: isize = 393;
    pub const PWRITEV: isize = 362;
    pub const QUOTACTL: isize = 131;
    pub const QUOTACTL_FD: isize = 443;
    pub const READ: isize = 3;
    pub const READAHEAD: isize = 225;
    pub const READLINK: isize = 85;
    pub const READLINKAT: isize = 332;
    pub const READV: isize = 145;
    pub const REBOOT: isize = 88;
    pub const RECV: isize = 291;
    pub const RECVFROM: isize = 292;
    pub const RECVMMSG: isize = 365;
    pub const RECVMMSG_TIME64: isize = 417;
    pub const RECVMSG: isize = 297;
    pub const REMAP_FILE_PAGES: isize = 253;
    pub const REMOVEXATTR: isize = 235;
    pub const RENAME: isize = 38;
    pub const RENAMEAT2: isize = 382;
    pub const RENAMEAT: isize = 329;
    pub const REQUEST_KEY: isize = 310;
    pub const RESTART_SYSCALL: isize = 0;
    pub const RMDIR: isize = 40;
    pub const RSEQ: isize = 398;
    pub const RT_SIGACTION: isize = 174;
    pub const RT_SIGPENDING: isize = 176;
    pub const RT_SIGPROCMASK: isize = 175;
    pub const RT_SIGQUEUEINFO: isize = 178;
    pub const RT_SIGRETURN: isize = 173;
    pub const RT_SIGSUSPEND: isize = 179;
    pub const RT_SIGTIMEDWAIT: isize = 177;
    pub const RT_SIGTIMEDWAIT_TIME64: isize = 421;
    pub const RT_TGSIGQUEUEINFO: isize = 363;
    pub const SCHED_GETAFFINITY: isize = 242;
    pub const SCHED_GETATTR: isize = 381;
    pub const SCHED_GETPARAM: isize = 155;
    pub const SCHED_GET_PRIORITY_MAX: isize = 159;
    pub const SCHED_GET_PRIORITY_MIN: isize = 160;
    pub const SCHED_GETSCHEDULER: isize = 157;
    pub const SCHED_RR_GET_INTERVAL: isize = 161;
    pub const SCHED_RR_GET_INTERVAL_TIME64: isize = 423;
    pub const SCHED_SETAFFINITY: isize = 241;
    pub const SCHED_SETATTR: isize = 380;
    pub const SCHED_SETPARAM: isize = 154;
    pub const SCHED_SETSCHEDULER: isize = 156;
    pub const SCHED_YIELD: isize = 158;
    pub const SECCOMP: isize = 383;
    pub const SEMCTL: isize = 300;
    pub const SEMGET: isize = 299;
    pub const SEMOP: isize = 298;
    pub const SEMTIMEDOP: isize = 312;
    pub const SEMTIMEDOP_TIME64: isize = 420;
    pub const SEND: isize = 289;
    pub const SENDFILE: isize = 187;
    pub const SENDFILE64: isize = 239;
    pub const SENDMMSG: isize = 374;
    pub const SENDMSG: isize = 296;
    pub const SENDTO: isize = 290;
    pub const SETDOMAINNAME: isize = 121;
    pub const SETFSGID: isize = 139;
    pub const SETFSGID32: isize = 216;
    pub const SETFSUID: isize = 138;
    pub const SETFSUID32: isize = 215;
    pub const SETGID32: isize = 214;
    pub const SETGID: isize = 46;
    pub const SETGROUPS32: isize = 206;
    pub const SETGROUPS: isize = 81;
    pub const SETHOSTNAME: isize = 74;
    pub const SETITIMER: isize = 104;
    pub const SET_MEMPOLICY: isize = 321;
    pub const SETNS: isize = 375;
    pub const SETPGID: isize = 57;
    pub const SETPRIORITY: isize = 97;
    pub const SETREGID32: isize = 204;
    pub const SETREGID: isize = 71;
    pub const SETRESGID: isize = 170;
    pub const SETRESGID32: isize = 210;
    pub const SETRESUID: isize = 164;
    pub const SETRESUID32: isize = 208;
    pub const SETREUID32: isize = 203;
    pub const SETREUID: isize = 70;
    pub const SETRLIMIT: isize = 75;
    pub const SET_ROBUST_LIST: isize = 338;
    pub const SETSID: isize = 66;
    pub const SETSOCKOPT: isize = 294;
    pub const SET_TID_ADDRESS: isize = 256;
    pub const SETTIMEOFDAY: isize = 79;
    pub const SETUID: isize = 23;
    pub const SETUID32: isize = 213;
    pub const SETXATTR: isize = 226;
    pub const SHMAT: isize = 305;
    pub const SHMCTL: isize = 308;
    pub const SHMDT: isize = 306;
    pub const SHMGET: isize = 307;
    pub const SHUTDOWN: isize = 293;
    pub const SIGACTION: isize = 67;
    pub const SIGALTSTACK: isize = 186;
    pub const SIGNALFD: isize = 349;
    pub const SIGNALFD4: isize = 355;
    pub const SIGPENDING: isize = 73;
    pub const SIGPROCMASK: isize = 126;
    pub const SIGRETURN: isize = 119;
    pub const SIGSUSPEND: isize = 72;
    pub const SOCKET: isize = 281;
    pub const SOCKETPAIR: isize = 288;
    pub const SPLICE: isize = 340;
    pub const STAT: isize = 106;
    pub const STAT64: isize = 195;
    pub const STATFS64: isize = 266;
    pub const STATFS: isize = 99;
    pub const STATX: isize = 397;
    pub const SWAPOFF: isize = 115;
    pub const SWAPON: isize = 87;
    pub const SYMLINK: isize = 83;
    pub const SYMLINKAT: isize = 331;
    pub const SYNC: isize = 36;
    pub const SYNCFS: isize = 373;
    pub const SYSCALL_BASE: isize = 0;
    pub const _SYSCTL: isize = 149;
    pub const SYSFS: isize = 135;
    pub const SYSINFO: isize = 116;
    pub const SYSLOG: isize = 103;
    pub const TEE: isize = 342;
    pub const TGKILL: isize = 268;
    pub const TIMER_CREATE: isize = 257;
    pub const TIMER_DELETE: isize = 261;
    pub const TIMERFD_CREATE: isize = 350;
    pub const TIMERFD_GETTIME: isize = 354;
    pub const TIMERFD_GETTIME64: isize = 410;
    pub const TIMERFD_SETTIME: isize = 353;
    pub const TIMERFD_SETTIME64: isize = 411;
    pub const TIMER_GETOVERRUN: isize = 260;
    pub const TIMER_GETTIME: isize = 259;
    pub const TIMER_GETTIME64: isize = 408;
    pub const TIMER_SETTIME: isize = 258;
    pub const TIMER_SETTIME64: isize = 409;
    pub const TIMES: isize = 43;
    pub const TKILL: isize = 238;
    pub const TRUNCATE64: isize = 193;
    pub const TRUNCATE: isize = 92;
    pub const UGETRLIMIT: isize = 191;
    pub const UMASK: isize = 60;
    pub const UMOUNT2: isize = 52;
    pub const UNAME: isize = 122;
    pub const UNLINK: isize = 10;
    pub const UNLINKAT: isize = 328;
    pub const UNSHARE: isize = 337;
    pub const USELIB: isize = 86;
    pub const USERFAULTFD: isize = 388;
    pub const USTAT: isize = 62;
    pub const UTIMENSAT: isize = 348;
    pub const UTIMENSAT_TIME64: isize = 412;
    pub const UTIMES: isize = 269;
    pub const VFORK: isize = 190;
    pub const VHANGUP: isize = 111;
    pub const VMSPLICE: isize = 343;
    pub const VSERVER: isize = 313;
    pub const WAIT4: isize = 114;
    pub const WAITID: isize = 280;
    pub const WRITE: isize = 4;
    pub const WRITEV: isize = 146;
}
