// devela/src/data/codec/symbol/ean/define.rs
//
//! Defines [`Ean`].
//

#[doc = crate::_tags!(codec namespace)]
/// EAN barcode codecs selected by digit count.
#[doc = crate::_doc_meta!{
    location("data/codec/symbol", struct Ean),
    test_size_of(__: Ean::<13> = 0),
}]
/// `N` identifies a supported standardized EAN form.
///
/// Currently implemented:
/// - `Ean<8>`: EAN-8, with 8 digits encoded into 67 logical modules.
/// - `Ean<13>`: EAN-13, with 13 digits encoded into 95 logical modules.
///
/// Digits are represented numerically as values in `0..=9`, rather than as
/// ASCII bytes.
///
/// Encoded symbols are packed into the low bits of a [`u128`], with the
/// leftmost barcode module stored in the most significant used bit:
///
/// ```text
///  most significant used bit             bit 0
///      │                                    │
///      ▼                                    ▼
///      leftmost module  ...  rightmost module
/// ```
/// A set bit represents a dark module (bar),
/// and a cleared bit a light module (space).
///
/// The packed representation contains only the logical barcode modules.
/// Quiet zones, physical dimensions, bar height, guard-bar extension and
/// human-readable text are rendering concerns and are not included.
///
/// Form-specific operations are implemented only for supported values of `N`.
///
/// # Examples
///
/// EAN-8:
/// ```
/// # use devela::Ean;
/// let digits = [9, 6, 3, 8, 5, 0, 7, 4];
///
/// assert!(Ean::<8>::is_valid(digits));
/// let modules = Ean::<8>::encode(digits).unwrap();
/// assert_eq!(Ean::<8>::decode(modules), Some(digits));
/// ```
///
/// EAN-13:
/// ```
/// # use devela::Ean;
/// let digits = [4, 0, 0, 6, 3, 8, 1, 3, 3, 3, 9, 3, 1];
///
/// assert!(Ean::<13>::is_valid(digits));
/// let modules = Ean::<13>::encode(digits).unwrap();
/// assert_eq!(Ean::<13>::decode(modules), Some(digits));
/// ```
#[derive(Debug)]
pub struct Ean<const N: usize>;

impl<const N: usize> Ean<N> {
    /// Number of digits in this EAN form.
    pub const DIGITS: usize = N;
}
