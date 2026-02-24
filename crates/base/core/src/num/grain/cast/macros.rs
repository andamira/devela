// devela_base_core::num::grain::cast::macros
//
//! Defines [`cast!`].
//

#[rustfmt::skip]
#[doc = crate::_tags!(num)]
/// Provides *const* Casting operations.
#[doc = crate::_doc_location!("num/grain")]
///
/// ```rust
/// # use devela_base_core::{Cast, cast};
/// let from: u64 = 2000;
/// assert_eq![
///     cast!(checked from => i16),
///     Cast(from).checked_cast_to_i16()
/// ];
/// ```
///
/// It introduces no new comparison semantics and forwards directly to [Cast][crate::Cast].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _cast {
    (checked  $from:expr => $P:ty) => { $crate::paste! {
        $crate::Cast($from).[<checked_cast_to_$P>]()
    }};
    (checked? $from:expr => $P:ty) => {
        $crate::unwrap![ok? $crate::cast![checked $from => $P]]
    };
    (checked_unwrap $from:expr => $P:ty) => {
        $crate::unwrap![ok $crate::cast![checked $from => $P]]
    };
    (saturating  $from:expr => $P:ty) => { $crate::paste! {
        $crate::Cast($from).[<saturating_cast_to_$P>]()
    }};
    (wrapping  $from:expr => $P:ty) => { $crate::paste! {
        $crate::Cast($from).[<wrapping_cast_to_$P>]()
    }};
}
#[doc(inline)]
pub use _cast as cast;

#[cfg(test)]
crate::items! {
    #[test]
    fn test_cast() {
        assert![cast![checked 340_usize => i8].is_err()];
        assert_eq![cast![checked 340_usize => i16], Ok(340_i16)];
        assert_eq![cast![checked_unwrap 340_usize => i16], 340_i16];
    }
}
