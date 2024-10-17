// devela::rend::font::error
//
//!
//

/// A font rendering result.
pub type FontResult<T> = core::result::Result<T, FontError>;

/// A font rendering error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontError {
    /// TEMP
    FontError,
}

impl crate::error::Error for FontError {}

mod core_impls {
    use super::FontError;
    use core::fmt;

    impl fmt::Display for FontError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write![f, "FontError"]
        }
    }
}
