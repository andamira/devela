//
//! Defines [`Sign`].
//

#[doc = crate::_tags!(quant)]
/// The three-valued sign of a number: negative (−1), zero (0), or positive (+1).
#[doc = crate::_doc_meta!{
    location("num/quant", enum Sign),
    test_size_of(Sign = 1|8; niche Option),
}]
/// This enum models the mathematical signum function and is useful when
/// representing or manipulating a value's sign independently of its magnitude.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Sign {
    /// A negative sign (-).
    Negative = -1,

    #[doc = crate::_tags!(init)]
    /// The zero sign (0).
    #[default]
    Zero = 0,

    /// A positive sign (+).
    Positive = 1,
}
