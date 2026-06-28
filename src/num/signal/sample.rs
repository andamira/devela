// devela/src/num/signal/emit.rs
//
//! Signal emitters, generators, and produced value streams.
//

/* traits */

#[doc = crate::_tags!(num signal)]
/// A stateless or externally indexed signal.
#[doc = crate::_doc_meta!{location("num/signal")}]
pub trait SignalAt<X> {
    /// The sampled value type.
    type Sample;
    /// Samples the signal at `x`.
    fn at(&self, x: X) -> Self::Sample;
}
