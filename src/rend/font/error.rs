// devela::rend::font::error
//
//!
//

/// A font rendering result.
pub type FontResult<T> = crate::Result<T, FontError>;

/// A font rendering error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontError {
    /// TEMP
    FontError,
}

mod core_impls {
    use crate::{Display, FmtResult, FontError, Formatter};

    impl crate::Error for FontError {}
    impl crate::ExtError for FontError {
        type Kind = ();
        fn error_eq(&self, other: &Self) -> bool {
            self == other
        }
        fn error_kind(&self) -> Self::Kind {}
    }

    impl Display for FontError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write![f, "FontError"]
        }
    }
}
