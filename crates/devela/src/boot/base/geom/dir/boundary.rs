//
//! Defines [`Boundary1d`].
//

#[doc = crate::_tags!(geom dir)]
/// Orientation relative to a one-dimensional boundary.
#[doc = crate::_doc_meta!{
    location("geom/dir", enum Boundary1d),
    test_size_of(Boundary1d = 1|8; niche Option),
}]
/// Identifies the lower or upper side of a bounded one-dimensional domain,
/// corresponding to the extrema of a totally ordered extent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Boundary1d {
    /// The lower boundary side.
    Lower,
    /// The upper boundary side.
    Upper,
}
