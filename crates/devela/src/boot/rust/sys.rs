use crate::{_reexport, _tags};

pub(crate) use {
    arch::*, arch_wasm::*, env::*, fs::*, fs_path::*, io::*, mem::*, mem_alloc::*,
    mem_bound_pin::*, mem_bound_ptr::*, mem_cell::*, mem_view_borrow::*, net::*, net_inet_ip::*,
    net_transport::*, net_transport_tcp::*, net_transport_udp::*, os_fd::*,
};

pub(crate) mod arch {
    use super::*;

    /* macros */

    _reexport! { rust: core::arch,
        location: "sys/arch" => macro asm, tag: _tags!(code platform),
        doc: "Inline assembly.",
        asm
    }
    _reexport! { rust: std::arch,
        location: "sys/arch" => macro detect_aarch64, tag: _tags!(platform),
        doc: "Tests at *runtime* whether an `aarch64` feature is enabled.",
        @is_aarch64_feature_detected as detect_aarch64
    }
    _reexport! { rust: std::arch,
        location: "sys/arch" => macro detect_x86, tag: _tags!(platform),
        doc: "Tests at *runtime* whether an `x86/x86-64` feature is enabled.",
        @is_x86_feature_detected as detect_x86
    }
    _reexport! { rust: core::arch,
        location: "sys/arch" => macro global_asm, tag: _tags!(code platform),
        doc: "Module-level inline assembly.",
        global_asm
    }
}
pub(crate) mod arch_wasm {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    // #[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))] // FIXME?
    crate::_reexport! { rust: core::arch::wasm32,
        location: "sys/arch" => struct w_v128,
        tag: _tags!(platform mem num),
        doc: "WASM-specific 128-bit wide SIMD vector type.",
        @v128 as w_v128
    }

    // doc-shim
    #[cfg(all(doc, not(target_arch = "wasm32")))]
    #[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
    #[doc = crate::_tags!(platform mem num)]
    /// WASM-specific 128-bit wide SIMD vector type.
    #[doc = crate::_doc_meta! {
        location("sys/arch", struct w_v128),
        origin(rust core::arch::wasm32; renamed(v128 as w_v128)),
    }]
    #[allow(non_camel_case_types)]
    pub struct w_v128([u8; 16]);
}

pub(crate) mod env {
    use super::*;

    /* core macros */

    _reexport! { rust: core, location: "sys/env" => macro option_env, tag: _tags!(code),
        doc: "Optionally inspects an environment variable at compile time.",
        option_env
    }

    #[doc = _tags!(code)]
    // NOTE: the following macro and the corresponding module have the same name:
    /// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
    /// Inspects an environment variable at compile time.
    #[doc = crate::_doc_meta!{
        location("sys/env", macro env_),
    }]
    #[doc = "*Re-exported from [`core::env`][macro@::core::env]*."]
    #[doc = "\n\n---"]
    ///
    /// The reason of the `_` suffix is to avoid conflicting with Rust's prelude
    /// when glob importing from this crate. Since this macro has the same name
    /// as its sibling module `core::env`, to be able to re-export
    /// just the macro we have to wrap it with our own.
    #[macro_export]
    #[cfg_attr(cargo_primary_package, doc(hidden))]
    macro_rules! env_ { ($($tt:tt)*) => { core::env![$($tt)*] } }
    #[doc(inline)]
    pub use env_;

    /* structs */

    _reexport! { rust: std::env,
        location: "sys/env" => struct JoinPathsError, tag: _tags!(error platform),
        doc: "Returned from [`Env::join_paths`][crate::Env::join_paths].",
        JoinPathsError,
    }
    _reexport! { rust: std::env,
        location: "sys/env" => struct VarError, tag: _tags!(error platform),
        doc: "Returned from [`Env::var`][crate::Env::var].",
        VarError
    }
}

pub(crate) mod fs {
    use super::*;

    /* structs */

    _reexport! { rust: std::fs,
        location: "sys/fs" => struct DirBuilder, tag: _tags!(fs),
        doc: "A builder used to create directories in various manners.",
        DirBuilder
    }
    _reexport! { rust: std::fs,
        location: "sys/fs" => struct DirEntry, tag: _tags!(fs),
        doc: "Entries returned by the ReadDir iterator.",
        DirEntry
    }
    _reexport! { rust: std::fs,
        location: "sys/fs" => struct File, tag: _tags!(fs),
        doc: "An object providing access to an open file on the filesystem.",
        File
    }
    _reexport! { rust: std::fs,
        location: "sys/fs" => struct FileTimes, tag: _tags!(fs),
        doc: "Representation of the various timestamps on a file.",
        FileTimes
    }
    _reexport! { rust: std::fs,
        location: "sys/fs" => struct FileType, tag: _tags!(fs),
        doc: "Represents a type of file with accessors for each file type.",
        FileType
    }
    _reexport! { rust: std::fs,
        location: "sys/fs" => struct FileMetadata, tag: _tags!(fs),
        doc: "Metadata information about a file.",
        @Metadata as FileMetadata
    }
    _reexport! { rust: std::fs,
        location: "sys/fs" => struct FileOpenOptions, tag: _tags!(fs),
        doc: "Options and flags which can be used to configure how a file is opened.",
        @OpenOptions as FileOpenOptions
    }
    _reexport! { rust: std::fs,
        location: "sys/fs" => struct FilePermissions, tag: _tags!(fs),
        doc: "Representation of the various permissions on a file.",
        @Permissions as FilePermissions
    }
    _reexport! { rust: std::fs,
        location: "sys/fs" => struct IterDirRead, tag: _tags!(fs iterator),
        doc: "Iterator over the entries in a directory.",
        @ReadDir as IterDirRead
    }
}

pub(crate) mod fs_path {
    use super::*;

    /* structs */

    _reexport! { rust: std::path,
        location: "sys/fs" => struct Path, tag: _tags!(fs lifetime),
        doc: "A slice of a path (akin to [`str`]).",
        Path
    }
    _reexport! { rust: std::path,
        location: "sys/fs" => struct PathBuf, tag: _tags!(fs),
        doc: "An owned, mutable path (akin to [`String`]).",
        PathBuf
    }
    _reexport! { rust: std::path,
        location: "sys/fs" => struct PathWindowsPrefixComponent, tag: _tags!(fs windows),
        doc: "Wraps a Windows path prefix as well as its unparsed string representation.",
        @PrefixComponent as PathWindowsPrefixComponent
    }
    _reexport! { rust: std::path,
        location: "sys/fs" => struct PathStripPrefixError, tag: _tags!(fs error),
        doc: "An error returned from [`Path::strip_prefix`] if the prefix was not found.",
        @StripPrefixError as PathStripPrefixError
    }

    /* enums */

    _reexport! { rust: std::path,
        location: "sys/fs" => enum PathComponent, tag: _tags!(fs),
        doc: "A single component of a path.",
        @Component as PathComponent
    }
    _reexport! { rust: std::path,
        location: "sys/fs" => enum PathWindowsPrefix, tag: _tags!(fs windows),
        doc: r"Windows path prefixes, e.g., `C:` or `\\server\share.`",
        @Prefix as PathWindowsPrefix
    }
}

pub(crate) mod io {
    use super::*;

    /* traits */

    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => trait IoBufRead, tag: _tags!(io),
        doc: "A type of `Read`er which has an internal buffer.",
        @BufRead as IoBufRead
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => trait IoRead, tag: _tags!(io),
        doc: "Allows for reading bytes from a source.",
        @Read as IoRead
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => trait IoSeek, tag: _tags!(io),
        doc: "Provides a cursor which can be moved within a stream of bytes.",
        @Seek as IoSeek
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => trait IoWrite, tag: _tags!(io),
        doc: "A trait for objects which are byte-oriented sinks.",
        @Write as IoWrite
    }

    /* structs */

    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoBufReader, tag: _tags!(io),
        doc: "Adds buffering to any reader.",
        @BufReader as IoBufReader
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoBufWriter, tag: _tags!(io),
        doc: "Wraps a writer and buffers its output.",
        @BufWriter as IoBufWriter
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoBytes, tag: _tags!(io),
        doc: "An iterator over `u8` values of a reader.",
        @Bytes as IoBytes
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoChain, tag: _tags!(io),
        doc: "Adapter to chain together two readers.",
        @Chain as IoChain
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoCursor, tag: _tags!(io),
        doc: "Wraps an in-memory buffer and provides it with an [`IoSeek`] implementation.",
        @Cursor as IoCursor
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoEmpty, tag: _tags!(io),
        doc: "Ignores any data written via [`IoWrite`], and read via [`IoRead`].",
        @Empty as IoEmpty
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoError, tag: _tags!(io error_composite),
        doc: "Error type for [`IoRead`], [`IoWrite`], [`IoSeek`] operations.",
        @Error as IoError
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct IoIntoInnerError, tag: _tags!(io result),
        doc: "An error returned by [`IoBufWriter::into_inner`]",
        @IntoInnerError as IoIntoInnerError
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct IoPipeReader, tag: _tags!(io),
        doc: "Read end of an anonymous pipe.",
        @PipeReader as IoPipeReader
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct IoPipeWriter, tag: _tags!(io),
        doc: "Write end of an anonymous pipe.",
        @PipeWriter as IoPipeWriter
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct IoSlice, tag: _tags!(io),
        doc: "A buffer type used with `IoWrite::write_vectored`.",
        IoSlice
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct IoSliceMut, tag: _tags!(io),
        doc: "A buffer type used with `IoRead::read_vectored`.",
        IoSliceMut
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoLineWriter, tag: _tags!(io),
        doc: "Like `BufWriter`, but flushing whenever a newline (`0x0a`, `'\n'`) is detected.",
        @LineWriter as IoLineWriter
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct IoLines, tag: _tags!(io),
        doc: "An iterator over the lines of an instance of [`IoBufRead`].",
        @Lines as IoLines
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoRepeat, tag: _tags!(io),
        doc: "A reader which yields one byte over and over and over and over and over and…",
        @Repeat as IoRepeat
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoResult, tag: _tags!(io result),
        doc: "A specialized [`Result`] type for I/O operations.",
        @Result as IoResult
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct IoSink, tag: _tags!(io),
        doc: "A writer which will move data into the void.",
        @Sink as IoSink
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct IoSplit, tag: _tags!(io),
        doc: "An iterator over the contents of an instance of BufRead split on a particular byte.",
        @Split as IoSplit
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct Stderr, tag: _tags!(io),
        doc: "A handle to the standard error stream of a process.",
        Stderr
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct Stdin, tag: _tags!(io),
        doc: "A handle to the standard input stream of a process.",
        Stdin
    }
    _reexport! { rust: std::io,
        location: "sys/io" => struct Stdout, tag: _tags!(io),
        doc: "A handle to the global standard output stream of the current process.",
        Stdout
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => struct IoTake, tag: _tags!(io),
        doc: "Reader adapter which limits the bytes read from an underlying reader.",
        @Take as IoTake
    }
    // @WriterPanicked as IoWriterPanicked

    /* enums */

    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => enum IoErrorKind, tag: _tags!(io error_composite),
        doc: "A list specifying general categories of I/O error.",
        @ErrorKind as IoErrorKind
    }
    _reexport! { rust: not(std)|std::io,
        location: "sys/io" => enum IoSeekFrom, tag: _tags!(io),
        doc: "A cursor position used by [`IoSeek`].",
        @SeekFrom as IoSeekFrom
    }
}

pub(crate) mod mem {
    use super::*;
    #[cfg(all(doc, feature = "alloc"))]
    use crate::Boxed;
    #[cfg(doc)]
    use crate::{Bare, BareBox, Storage};

    _reexport! { rust: alloc::boxed,
        location: "sys/mem" => struct Box, tag: _tags!(mem),
        doc: "A pointer type that uniquely owns a heap allocation of type `T`.",
        +doc: "It is used as the underlying [`Storage`] for the [`Boxed`] marker struct,
        just as a [`BareBox`] is used as the storage for [`Bare`].

A special magic property of `Box` is that it allows moving with `*boxed`, unlike
other `Deref` types. It is hoped that an eventual `DerefMove` trait will make it
possible for other types to opt in to move-from-deref.
    ",
        Box
    }

    /* structs */

    _reexport! { rust: core::mem,
        location: "sys/mem" => struct Discriminant, tag: _tags!(mem),
        doc: "Opaque type representing the discriminant of an enum.",
        Discriminant
    }
    _reexport! { rust: core::mem,
        location: "sys/mem" => struct DropGuard, tag: _tags!(mem),
        doc: "Wrap a value and run a closure when dropped.",
        DropGuard
    }
    _reexport! { rust: core::mem,
        location: "sys/mem" => struct ManuallyDrop, tag: _tags!(mem),
        doc: "A wrapper to inhibit compiler from automatically calling `T`'s destructor.",
        ManuallyDrop
    }
    _reexport! { rust: alloc::rc,
        location: "sys/mem" => struct Rc, tag: _tags!(mem lifetime),
        doc: "A single-threaded reference-counting pointer.",
        Rc
    }
    _reexport! { rust: alloc::rc,
        location: "sys/mem" => struct RcWeak, tag: _tags!(mem lifetime),
        doc: "A version of `Rc` that holds a non-owning ref to the managed allocation.",
        @Weak as RcWeak
    }

    /* unions */

    _reexport! { rust: core::mem,
        location: "sys/mem" => union MaybeUninit, tag: _tags!(maybe mem),
        doc: "A wrapper type to construct uninitialized instances of `T`.",
        MaybeUninit
    }

    /* macros */

    _reexport! { rust: core::mem,
        location: "sys/mem" => macro offset_of, tag: _tags!(mem),
        doc: "Expands to the offset in bytes of a field from the beginning of the given type.",
        offset_of
    }
}
pub(crate) mod mem_alloc {
    use super::*;

    /* structs */

    _reexport! { rust: core::alloc,
        location: "sys/mem/alloc" => struct AllocError, tag: _tags!(allocation error),
        doc: "An allocation failure.",
        AllocError
    }
    _reexport! { rust: alloc::alloc,
        location: "sys/mem/alloc" => struct AllocGlobal, tag: _tags!(allocation),
        doc: "An allocator that forwards to the configured global allocator.",
        @Global as AllocGlobal
    }
    _reexport! { rust: std::alloc,
        location: "sys/mem/alloc" => struct AllocSystem, tag: _tags!(allocation),
        doc: "The default memory allocator provided by the operating system.",
        @System as AllocSystem
    }
    _reexport! { rust: core::alloc,
        location: "sys/mem/alloc" => struct MemLayout, tag: _tags!(allocation),
        doc: "Layout of a block of memory.",
        @Layout as MemLayout
    }
    _reexport! { rust: core::alloc,
        location: "sys/mem/alloc" => struct MemLayoutError, tag: _tags!(allocation error),
        doc: "The [`MemLayout`] parameters violated constraints.",
        @LayoutError as MemLayoutError
    }

    /* traits */

    _reexport! { rust: core::alloc,
        location: "sys/mem/alloc" => trait Allocator, tag: _tags!(allocation),
        doc: "An allocator for arbitrary memory blocks.",
        Allocator
    }
    _reexport! { rust: core::alloc,
        location: "sys/mem/alloc" => trait GlobalAlloc, tag: _tags!(allocation),
        doc: "A memory allocator that can be registered as the standard library's default.",
        GlobalAlloc
    }
}
pub(crate) mod mem_bound_pin {
    use super::*;

    /* macros */

    _reexport! { rust: core::pin,
        location: "sys/mem/bound" => macro pin, tag: _tags!(code mem lifetime),
        doc: "Constructs a <code>[Pin]<[&mut] T></code>, by pinning a `value: T` locally.",
        pin
    }

    /* structs */

    _reexport! { rust: core::pin,
        location: "sys/mem/bound" => struct Pin, tag: _tags!(lifetime mem guard),
        doc: "A pointer which pins its pointee in place.",
        Pin
    }

    // WAIT: [derive_coerce_pointee](https://github.com/rust-lang/rust/issues/123430)
    // _reexport! { rust: core::pin,
    //     doc: "A pointer or a wrapper for one, where unsizing can be performed on the pointee when it is pinned.",
    //     PinCoerceUnsized
    // }
}
pub(crate) mod mem_bound_ptr {
    use super::*;

    /* structs */

    _reexport! { rust: core::ptr,
        location: "sys/mem/bound" => struct PtrNonNull, tag: _tags!(mem niche),
        doc: "`*mut T` but non-zero and *covariant*.",
        @NonNull as PtrNonNull
    }

    /* functions */

    _reexport! { rust: core::ptr,
        location: "sys/mem/bound" => fn fn_addr_eq, tag: _tags!(mem),
        doc: "Compares the addresses of the two function pointers for equality.",
        fn_addr_eq // NOTE: Can't be namespaced in Ptr::fn_addr_eq because: WAIT:4EVER: fn_ptr_trait
    }
}
pub(crate) mod mem_cell {
    use super::*;

    /* structs */

    _reexport! { rust: core::cell,
        location: "sys/mem/cell" => struct RefCellBorrowError, tag: _tags!(lifetime error),
        doc: "An error returned by [`RefCell::try_borrow`].",
        @BorrowError as RefCellBorrowError
    }
    _reexport! { rust: core::cell,
        location: "sys/mem/cell" => struct RefCellBorrowMutError, tag: _tags!(lifetime error),
        doc: "An error returned by [`RefCell::try_borrow_mut`].",
        @BorrowMutError as RefCellBorrowMutError
    }
    _reexport! { rust: core::cell,
        location: "sys/mem/cell" => struct Cell, tag: _tags!(mem),
        doc: "A mutable memory location.",
        Cell
    }
    _reexport! { rust: core::cell,
        location: "sys/mem/cell" => struct LazyCell, tag: _tags!(mem init) ,
        doc: "A value which is initialized on the first access.",
        LazyCell
    }
    _reexport! { rust: core::cell,
        location: "sys/mem/cell" => struct OnceCell, tag: _tags!(mem init) ,
        doc: "A cell which can nominally be written to only once.",
        OnceCell
    }
    _reexport! { rust: core::cell,
        location: "sys/mem/cell" => struct RefCell, tag: _tags!(mem lifetime),
        doc: "A mutable memory location with dynamically checked borrow rules.",
        RefCell
    }
    _reexport! { rust: core::cell,
        location: "sys/mem/cell" => struct Ref, tag: _tags!(lifetime),
        doc: "A wrapper type for an inmutably borrowed value from a `RefCell<T>`",
        Ref
    }
    _reexport! { rust: core::cell,
        location: "sys/mem/cell" => struct RefMut, tag: _tags!(lifetime),
        doc: "A wrapper type for a mutably borrowed value from a `RefCell<T>`",
        RefMut
    }
    _reexport! { rust: core::cell,
        location: "sys/mem/cell" => struct UnsafeCell, tag: _tags!(mem),
        doc: "The core primitive for interior mutability in Rust.",
        UnsafeCell
    }
}
pub(crate) mod mem_view_borrow {
    use super::*;

    _reexport! { rust: core::borrow,
        location: "sys/mem/view" => trait Borrow, tag: _tags!(lifetime),
        doc: "A trait for borrowing data.",
        Borrow
    }
    _reexport! { rust: core::borrow,
        location: "sys/mem/view" => trait BorrowMut, tag: _tags!(lifetime),
        doc: "A trait for mutably borrowing data.",
        BorrowMut
    }
    _reexport! { rust: alloc::borrow,
        location: "sys/mem/view" => struct Cow, tag: _tags!(mem lifetime value),
        doc: "A clone-on-write smart pointer.",
        Cow
    }
    _reexport! { rust: alloc::borrow,
        location: "sys/mem/view" => trait ToOwned, tag: _tags!(mem value),
        doc: "A generalization of Clone to borrowed data.",
        ToOwned
    }
}

pub(crate) mod net {
    use super::*;

    /* structs */

    _reexport! { rust: core::net,
        location: "sys/net" => struct AddrParseError, tag: _tags!(network error),
        doc: "An error which can be returned when parsing an IP or socket address.",
        @AddrParseError as NetAddrParseError
    }
}
pub(crate) mod net_inet_ip {
    use super::*;

    /* structs */

    _reexport! { rust: core::net,
        location: "sys/net/inet" => struct Ipv4Addr, tag: _tags!(network),
        doc: "An IPv4 address.",
        Ipv4Addr
    }
    _reexport! { rust: core::net,
        location: "sys/net/inet" => struct Ipv6Addr, tag: _tags!(network),
        doc: "An IPv6 address.",
        Ipv6Addr
    }
    _reexport! { rust: core::net,
        location: "sys/net/inet" => struct SocketAddrV4, tag: _tags!(network),
        doc: "An IPv4 socket address.",
        SocketAddrV4
    }
    _reexport! { rust: core::net,
        location: "sys/net/inet" => struct SocketAddrV6, tag: _tags!(network),
        doc: "An IPv6 socket address.",
        SocketAddrV6
    }

    /* enums */

    _reexport! { rust: core::net,
        location: "sys/net/inet" => enum IpAddr, tag: _tags!(network),
        doc: "An IP address, either IPv4 or IPv6.",
        IpAddr
    }
    _reexport! { rust: core::net,
        location: "sys/net/inet" => enum SocketAddr, tag: _tags!(network id),
        doc: "An internet socket address, either IPv4 or IPv6.",
        SocketAddr
    }
}
pub(crate) mod net_transport {
    use super::*;
    #[cfg(doc)]
    use crate::SocketAddr;

    /* traits */

    _reexport! { rust: std::net,
        location: "sys/net" => trait ToSocketAddrs, tag: _tags!(network),
        doc: "Objects which can be converted or resolved to one or more [`SocketAddr`] values.",
        ToSocketAddrs
    }
}
pub(crate) mod net_transport_tcp {
    use super::*;

    /* structs */

    _reexport! { rust: std::net,
        location: "sys/net" => struct TcpListener, tag: _tags!(network),
        doc: "A TCP socket server, listening for connections.",
        TcpListener
    }
    _reexport! { rust: std::net,
        location: "sys/net" => struct TcpStream, tag: _tags!(network),
        doc: "A TCP stream between a local and a remote socket.",
        TcpStream
    }

    /* enums */

    _reexport! { rust: std::net,
        location: "sys/net" => enum TcpShutdown, tag: _tags!(network),
        doc: "Possible values which can be passed to the [`TcpStream::shutdown`] method.",
        @Shutdown as TcpShutdown
    }
}
pub(crate) mod net_transport_udp {
    use super::*;

    /* structs */

    _reexport! { rust: std::net,
        location: "sys/net" => struct UdpSocket, tag: _tags!(network),
        doc: "A UDP socket.",
        UdpSocket
    }
}

pub(crate) mod os_fd {
    use super::*;

    #[cfg(all(not(miri), any(unix, target_os = "wasi")))]
    pub(crate) use unix_wasi::*;
    #[cfg(all(not(miri), any(unix, target_os = "wasi")))]
    mod unix_wasi {
        use super::*;

        /* structs */

        _reexport! { rust: std::os::fd,
            location: "sys/os/fd" => struct FdOwned, tag: _tags!(fs guard),
            doc: "An owned file descriptor.",
            @OwnedFd as FdOwned
        }
        _reexport! { rust: std::os::fd,
            location: "sys/os/fd" => struct FdBorrowed, tag: _tags!(fs lifetime),
            doc: "A borrowed file descriptor.",
            @BorrowedFd as FdBorrowed
        }

        /* traits */

        _reexport! { rust: std::os::fd,
            location: "sys/os/fd" => trait AsFd, tag: _tags!(fs),
            doc: "A trait to borrow the file descriptor from an underlying object.",
            AsFd
        }
        _reexport! { rust: std::os::fd,
            location: "sys/os/fd" => trait AsFdRaw, tag: _tags!(fs id),
            doc: "A trait to extract the raw file descriptor from an underlying object.",
            @AsRawFd as AsFdRaw
        }
        _reexport! { rust: std::os::fd,
            location: "sys/os/fd" => trait FromFdRaw, tag: _tags!(fs id value),
            doc: "Expresses the ability to construct an object from a raw file descriptor.",
            @FromRawFd as FromFdRaw
        }
        _reexport! { rust: std::os::fd,
            location: "sys/os/fd" => trait IntoFdRaw, tag: _tags!(fs id value),
            doc: "Expresses the ability to consume an object and own its raw file descriptor.",
            @IntoRawFd as IntoFdRaw
        }
    }
}
