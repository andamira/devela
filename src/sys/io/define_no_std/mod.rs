// devela::sys::io::define_no_std
//
//! Reimplementations of I/O functionality for `no_std`.
//
// WAIT: [no_std io](https://github.com/rust-lang/rust/issues/48331)

mod error; // IoError, IoErrorKind, IoResult
mod write; // IoWrite
pub use {error::*, write::*};

#[cfg(feature = "io")]
crate::items! {
    mod buffered; // IoBufReader, IoBufWriter, IoLineWriter, IntoInnerError
    mod cursor; // IoCursor
    mod fns; // io_copy
    mod impls;
    mod other; // IoSeekFrom, (IoBufRead, IoRead, IoSeek), (IoBytes, IoChain, IoTake)
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "io")))]
    pub use {buffered::*, cursor::*, fns::*, other::*};
}
