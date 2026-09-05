// devela/src/sys/io/no_std_define/mod.rs
//
//! Reimplementations of I/O functionality for `no_std`.
//
// WAIT: [no_std io](https://github.com/rust-lang/rust/issues/48331)

crate::mods_in! {
    mod error; // IoError, IoErrorKind, IoResult
    mod read; // IoRead, IoBufRead, IoBytes, IoChain, IoTake
    mod write; // IoWrite

    #[cfg(feature = "io")]
    mod buffered; // IoBufReader, IoBufWriter, IoLineWriter, IntoInnerError
    #[cfg(feature = "io")]
    mod cursor; // IoSeek, IoSeekFrom, IoCursor
    #[cfg(feature = "io")]
    mod other; // io_copy, IoEmpty, IoRepeat
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            error::*,
            read::*,
            write::*,
        };
        #[cfg(feature = "io")]
        pub use super::{
            buffered::*,
            cursor::*,
            other::*,
        };
    }
}
