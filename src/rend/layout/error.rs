// devela::rend::layout::error
//
//!
//

/// A layout rendering result.
pub type LayoutResult<T> = core::result::Result<T, LayoutError>;

/// A layout rendering error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutError {
    /// TEMP
    LayoutError,
}

impl crate::error::Error for LayoutError {}

mod core_impls {
    use super::LayoutError;
    use core::fmt;

    impl fmt::Display for LayoutError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write![f, "LayoutError"]
        }
    }
}
