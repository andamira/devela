// devela_base_core::code::util::ident
//
//! identifier related macro functionality
//

#[doc = crate::_tags!(code)]
/// Defines a constant for every given identifier with a value of its index in the list.
#[doc = crate::_doc_location!("code/util")]
/// # Examples
/// ```
/// # use devela_base_core::ident_const_index;
/// ident_const_index![pub, 3; first, second, third]; // with commas
/// assert![0 == first && 1 == second && 2 == third];
///
/// ident_const_index![pub(crate), 2; fourth fifth]; // without commas
/// assert![0 == fourth && 1 == fifth];
/// ```
// USEDBY: enumset macro
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! ident_const_index {
    ( // without commas:
        // $vis: the visibility of the constants (pub, pub(super), …).
        // $total: the total number of identifiers.
        // $ident, $($rest),*: the list of identifiers.
        $vis:vis, $total:expr; $ident:ident $($rest:ident)*
    ) => { $crate::paste! {
        #[allow(non_upper_case_globals)]
        $vis const $ident: usize = $total - $crate::ident_total!($($rest)*) - 1;
        $( $crate::ident_const_index!($vis, $total; $rest); )*
    }};
    ( // with commas:
        // $vis: the visibility of the constants (pub, pub(super), …).
        // $total: the total number of identifiers.
        // $ident, $($rest),*: the list of identifiers.
        $vis:vis, $total:expr; $ident:ident $(, $($rest:ident),* $(,)? )?
    ) => { $crate::paste! {
        #[allow(non_upper_case_globals)]
        $vis const $ident: usize = $total - $crate::ident_total!( $($($rest)*)? ) - 1;
        $( $crate::ident_const_index!($vis, $total; $($rest),*); )?
    }};
}
#[doc(inline)]
pub use ident_const_index;
