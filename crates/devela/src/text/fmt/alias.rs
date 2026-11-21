use crate::FmtError;

#[doc = crate::_tags!(fmt result)]
/// The type returned by formatter methods.
#[doc = crate::_doc_meta!{
    location("text/fmt", type FmtResult),
}]
/// Note that this is not the same as [`core::fmt::Result`],
/// since this one doesn't hardcode the returned type to `()`.
pub type FmtResult<T> = Result<T, FmtError>;
