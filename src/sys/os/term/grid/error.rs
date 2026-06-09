// devela::sys::os::term::grid::error
//
//! Defines [`TermGridError`].
//

#[doc = crate::_tags!(term data_structure error)]
/// Terminal grid construction error.
#[doc = crate::_doc_meta!{location("sys/os/term/grid")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TermGridError {
    /// The grid extent overflows the index representation.
    ExtentOverflow,

    /// The storage has fewer elements than the grid requires.
    NotEnoughElements {
        /// The required number of elements.
        required: usize,
        /// The available number of elements.
        available: usize,
    },
}
crate::impl_trait! {
    fmt::Display+Error for TermGridError |self, f| match self {
        Self::ExtentOverflow =>
            f.write_str("terminal grid extent overflows the index representation"),
        Self::NotEnoughElements { required, available } =>
            write!(
                f,
                "terminal grid requires {required} elements, but only {available} are available"
            ),
    }
}
