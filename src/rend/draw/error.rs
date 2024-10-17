// devela::rend::draw::error
//
//!
//

/// A drawing result.
pub type DrawResult<T> = core::result::Result<T, DrawError>;

/// A drawing error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DrawError {
    /// TEMP
    DrawError,
}

impl crate::error::Error for DrawError {}

mod core_impls {
    use super::DrawError;
    use core::fmt;

    impl fmt::Display for DrawError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write![f, "DrawError"]
        }
    }
}
