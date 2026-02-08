// devela_base_core::code::util::type_count
//
//! Counting types.
//

#[doc = crate::_tags!(code)]
/// Returns the total number of types received.
#[doc = crate::_doc_location!("code/util")]
/// # Examples
/// ```
/// # use devela_base_core::{type_count, Duration};
/// assert_eq![0, type_count!()];
/// assert_eq![1, type_count!([f32; 4])];
/// assert_eq![3, type_count!(u8, (), Duration)];
/// assert_eq![4, type_count!(&str, Option<char>, bool, *mut usize)];
/// ```
// USEDBY: ...
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! type_count {
    ($($T:ty),*) => { <[()]>::len(&[ $( { let _ = $crate::PhantomData::<$T>; () } ),* ]) };
}
#[doc(inline)]
pub use type_count;
