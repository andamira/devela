// devela::code::ident
//
//! identifier related macro functionality
//

/// Defines a constant for every given identifier with a value of its index in the list.
/// # Examples
/// ```
/// # use devela::code::ident_const_index;
/// ident_const_index![pub, 3; first, second, third]; // with commas
/// assert![0 == first && 1 == second && 2 == third];
///
/// ident_const_index![pub(crate), 2; fourth fifth]; // without commas
/// assert![0 == fourth && 1 == fifth];
/// ```
// USEDBY: enumset macro
#[macro_export]
macro_rules! ident_const_index {
    ( // without commas:
        // $vis: the visibility of the constants (pub, pub(super), …).
        // $total: the total number of identifiers.
        // $ident, $($rest),*: the list of identifiers.
        $vis:vis, $total:expr; $ident:ident $($rest:ident)*
    ) => { $crate::code::paste! {
        #[allow(non_upper_case_globals)]
        $vis const $ident: usize = $total - $crate::code::ident_total!($($rest)*) - 1;
        $( $crate::code::ident_const_index!($vis, $total; $rest); )*
    }};
    ( // with commas:
        // $vis: the visibility of the constants (pub, pub(super), …).
        // $total: the total number of identifiers.
        // $ident, $($rest),*: the list of identifiers.
        $vis:vis, $total:expr; $ident:ident $(, $($rest:ident),* $(,)? )?
    ) => { $crate::code::paste! {
        #[allow(non_upper_case_globals)]
        $vis const $ident: usize = $total - $crate::code::ident_total!( $($($rest)*)? ) - 1;
        $( $crate::code::ident_const_index!($vis, $total; $($rest),*); )?
    }};
}
pub use ident_const_index;
