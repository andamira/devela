// devela_base_core::num::grain::cast::macros
//
//! Defines [`cast!`].
//

#[doc = crate::_tags!(num)]
/// Provides *const* primitive casting, joining and splitting operations.
#[doc = crate::_doc_location!("num/grain")]
///
/// A thin macro wrapper over [`Cast`][crate::Cast].
/// Expands directly to the corresponding `Cast` method.
///
/// ```rust
/// # use devela_base_core::{Cast, cast};
/// // numeric cast
/// let c: u64 = 2000;
/// assert_eq![
///     cast!(checked c => i16),
///     Cast(c).checked_cast_to_i16()
/// ];
///
/// // join (aggregate smaller primitives into a wider one)
/// let j: [u8; 4] = [12, 34, 56, 78];
/// assert_eq![
///     cast!(join [u8] j => u32),
///     Cast::<u32>::from_u8_ne(j)
/// ];
///
/// // split (decompose into smaller primitives)
/// let s: u32 = 12345678;
/// assert_eq![
///     cast!(split s => [u16]),
///     Cast(s).into_u16_ne()
/// ];
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _cast {
    (
    /* cast */

    checked $from:expr => $P:ident) => { $crate::paste! {
        $crate::Cast($from).[<checked_cast_to_$P>]()
    }};
    (checked? $from:expr => $P:ident) => {
        $crate::unwrap![ok? $crate::cast![checked $from => $P]]
    };
    (checked_unwrap $from:expr => $P:ident) => {
        $crate::unwrap![ok $crate::cast![checked $from => $P]]
    };
    (saturating $from:expr => $P:ident) => { $crate::paste! {
        $crate::Cast($from).[<saturating_cast_to_$P>]()
    }};
    (wrapping $from:expr => $P:ident) => { $crate::paste! {
        $crate::Cast($from).[<wrapping_cast_to_$P>]()
    }};
    (
    /* join */

    join [$F:ident] $from:expr => $P:ident) => { $crate::paste! {
        $crate::Cast::<$P>::[<from_$F _ne>]($from)
    }};
    (join be [$F:ident] $from:expr => $P:ident) => { $crate::paste! {
        $crate::Cast::<$P>::[<from_$F _be>]($from)
    }};
    (join le [$F:ident] $from:expr => $P:ident) => { $crate::paste! {
        $crate::Cast::<$P>::[<from_$F _le>]($from)
    }};
    (join ne [$F:ident] $from:expr => $P:ident) => { $crate::paste! {
        $crate::Cast::<$P>::[<from_$F _ne>]($from)
    }};
    (
    /* split */

    split $from:expr => [$P:ident]) => { $crate::paste! {
        $crate::Cast($from).[<into_$P _ne>]()
    }};
    (split be $from:expr => [$P:ident]) => { $crate::paste! {
        $crate::Cast($from).[<into_$P _be>]()
    }};
    (split le $from:expr => [$P:ident]) => { $crate::paste! {
        $crate::Cast($from).[<into_$P _le>]()
    }};
    (split ne $from:expr => [$P:ident]) => { $crate::paste! {
        $crate::Cast($from).[<into_$P _ne>]()
    }};
}
#[doc(inline)]
pub use _cast as cast;

#[cfg(test)]
crate::items! {
    use crate::Cast;

    #[test]
    fn test_cast() {
        assert![cast![checked 340_usize => i8].is_err()];
        assert_eq![cast![checked 340_usize => i16], Ok(340_i16)];
        assert_eq![cast![checked_unwrap 340_usize => i16], 340_i16];
    }
    #[test]
    fn test_join() {
        let j: [u8; 4] = [12, 34, 56, 78];
        assert_eq![
            cast!(join [u8] j => u32),
            Cast::<u32>::from_u8_ne(j)
        ];
    }
    #[test]
    fn test_split() {
        let s: u32 = 12345678;
        assert_eq![
            cast!(split s => [u16]),
            Cast(s).into_u16_ne()
        ];
        assert_eq![
            cast!(split be s => [u8]),
            Cast(s).into_u8_be()
        ];
    }
}
