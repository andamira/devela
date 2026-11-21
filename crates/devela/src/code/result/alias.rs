#[doc = crate::_tags!(result no error)]
/// A result type whose error case can never occur.
#[doc = crate::_doc_meta!{
    location("code/result", type InfallibleResult),
}]
pub type InfallibleResult<T> = Result<T, !>;
