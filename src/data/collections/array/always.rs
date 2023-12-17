// devela::data::collections::array::always
//
//! Array functionality always available for internal use.
//

#![allow(unused)]

/// Initializes a `[$T; $LEN]` array in multiple ways.
///
/// # Macro arms
/// - `safe_init`: safe array initialization in the stack with an `$init` expression.
/// - `safe_init_heap`: safe array initialization in the heap with an `$init` expression.
/// - `unsafe_init`: unsafe array initialization in the stack with an `$init` expression.
/// - `unsafe_init_heap`: unsafe array initialization in the heap with an `$init` expression.
/// ---
/// - `clone`: initialize an array in the stack by cloning `$element`.
/// - `clone_heap`: initialize an array in the heap by cloning `$element`.
/// - `default`: initialize an array in the stack with `$T::default()`.
/// - `default_heap`: initialize an array in the heap with `$T::default()`.
/// - `iter`: initialize an array in the stack with `$intoiter` and `$T::default()`
///    for any remaining elements.
/// - `iter_heap`: initialize an array in the heap with `$intoiter` and `$T::default()`
///    for any remaining elements.
/// ---
/// - `preop`: initialize an array in the stack by applying `$op` (in safe it
///   first clones `$pre`) propagating any errors from `$pre` or `$op`.
///
/// # Arguments
/// - `[$T; $LEN]`: the array's elements' type and length.
/// - `$init`: a function with an `usize` argument that returns `$T`.
/// - `$unsafe_feature`: the name of a feature that enables the use of `unsafe`.
/// - `$element`: a clonable element of type `$T`.
/// - `$intoiter`: an item that implements [`IntoIterator`].
/// - `$element`: a clonable element of type `$T`.
///
/// # Examples
/// ```
/// use devela::data::array_init;
///
/// assert_eq![[7,7,7], array_init![safe_init [i32; 3], |_| 7]];
/// #[cfg(feature = "unsafe_feat")]
/// assert_eq![[7,7,7], array_init![unsafe_init [i32; 3], |_| 7]];
///
/// assert_eq![[7,7,7], array_init![clone [i32; 3], "unsafe_feat",  7]];
/// assert_eq![[0,0,0], array_init![default [i32; 3], "unsafe_feat"]];
/// assert_eq![[4,5,6], array_init![iter [i32; 3], "unsafe_feat", vec![4,5,6,7,8]]];
/// assert_eq![[4,0,0], array_init![iter [i32; 3], "unsafe_feat", vec![4]]];
/// ```
///
/// # Features
/// The unsafe version uses [`MaybeUninit`][core::mem::MaybeUninit] in the case
/// of stack allocation or [`Box::from_raw`] in the case of heap allocation.
///
/// For `clone`, `default` and `iter` versions, uf the given `$unsafe_feature`
/// is enabled it will use unsafe initialization.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
#[macro_export]
macro_rules! array_init {
    // Safe array initialization in the stack
    (safe_init [$T:ty; $LEN:expr], $init:expr) => {{
        #[allow(clippy::redundant_closure_call)]
        core::array::from_fn(|i| $init(i))
    }};
    // safe array initialization in the heap
    (safe_init_heap [$T:ty; $LEN:expr], $init:expr) => {{
        let mut v = Vec::<$T>::with_capacity($LEN);
        for i in 0..$LEN {
            #[allow(clippy::redundant_closure_call)]
            v.push($init(i));
        }
        v.into_boxed_slice().try_into().unwrap_or_else(|_| {
            panic!("Can't turn the boxed slice into a boxed array")
        })
    }};
    // unsafe array initialization in the stack
    (unsafe_init [$T:ty; $LEN:expr], $init:expr) => {{
        // SAFETY: array will be fully initialized in the subsequent loop
        let mut arr: [core::mem::MaybeUninit<$T>; $LEN] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        for (i, e) in &mut arr[..].iter_mut().enumerate() {
            #[allow(clippy::redundant_closure_call)]
            let _ = e.write($init(i));
        }
        // WAIT: can't use transmute for now, have to use transmute_copy:
        // - https://github.com/rust-lang/rust/issues/62875
        // - https://github.com/rust-lang/rust/issues/61956
        // unsafe { core::mem::transmute::<_, [T; LEN]>(&arr) }
        // SAFETY: we've initialized all the elements
        unsafe { core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
    }};
    // unsafe array initialization in the heap
    (unsafe_init_heap [$T:ty; $LEN:expr], $init:expr) => {{
        let mut v = Vec::<$T>::with_capacity($LEN);
        #[allow(clippy::redundant_closure_call)]
        for i in 0..$LEN { v.push($init(i)); }
        let slice = v.into_boxed_slice();
        let raw_slice = Box::into_raw(slice);
        // SAFETY: pointer comes from using `into_raw`, and capacity is right.
        unsafe { Box::from_raw(raw_slice as *mut [$T; $LEN]) }
    }};

    // ---

    // initialize an array in the stack by cloning $element
    (clone [$T:ty; $LEN:expr], $unsafe_feature:literal, $element:expr) => {{
        #[cfg(not(feature = $unsafe_feature))]
        { array_init!(safe_init [$T; $LEN], |_| $element.clone()) }
        #[cfg(feature = $unsafe_feature)]
        { array_init!(unsafe_init [$T; $LEN], |_| $element.clone()) }
    }};
    // initialize an array in the heap, by cloning $element
    (clone_heap [$T:ty; $LEN:expr], $unsafe_feature:literal, $element:expr) => {{
        #[cfg(not(feature = $unsafe_feature))]
        { array_init!(safe_init_heap [$T; $LEN], |_| $element.clone()) }
        #[cfg(feature = $unsafe_feature)]
        { array_init!(unsafe_init_heap [$T; $LEN], |_| $element.clone()) }
    }};

    // initialize an array in the stack with $T::default()
    (default [$T:ty; $LEN:expr], $unsafe_feature:literal) => {{
        #[cfg(not(feature = $unsafe_feature))]
        { array_init!(safe_init [$T; $LEN], |_| <$T>::default()) }
        #[cfg(feature = $unsafe_feature)]
        { array_init!(unsafe_init [$T; $LEN], |_| <$T>::default()) }
    }};
    // initialize an array in the heap, with $T::default()
    (default_heap [$T:ty; $LEN:expr], $unsafe_feature:literal) => {{
        #[cfg(not(feature = $unsafe_feature))]
        { array_init!(safe_init_heap [$T; $LEN], |_| <$T>::default()) }
        #[cfg(feature = $unsafe_feature)]
        { array_init!(unsafe_init_heap [$T; $LEN], |_| <$T>::default()) }
    }};

    // initialize an array in the stack with an IntoIterator<Item = $T> and with
    // $T::default() in case the iterator length is < $LEN, for the remaining elements.
    (iter [$T:ty; $LEN:expr], $unsafe_feature:literal, $intoiter:expr) => {{
        let mut iterator = $intoiter.into_iter();
        let mut init_closure = |_| {
            if let Some(e) = iterator.next() { e } else { <$T>::default() }
        };
        #[cfg(not(feature = $unsafe_feature))]
        { array_init!(safe_init [$T; $LEN], init_closure) }
        #[cfg(feature = $unsafe_feature)]
        { array_init!(unsafe_init [$T; $LEN], init_closure) }
    }};

    // initialize an array in the heap with an IntoIterator<Item = $T> and with
    // $T::default() in case the iterator length is < $LEN, for the remaining elements.
    (iter_heap [$T:ty; $LEN:expr], $unsafe_feature:literal, $intoiter:expr) => {{
        let mut iterator = $intoiter.into_iter();
        let mut init_closure = |_| {
            if let Some(e) = iterator.next() { e } else { <$T>::default() }
        };
        #[cfg(not(feature = $unsafe_feature))]
        { array_init!(safe_init_heap [$T; $LEN], init_closure) }
        #[cfg(feature = $unsafe_feature)]
        { array_init!(unsafe_init_heap [$T; $LEN], init_closure) }
    }};

    // ---

    // initialize an array by applying $op (in safe mode it first clones $pre)
    // and propagates errors both from $pre and $op.
    (preop [$T:ty; $LEN:expr]?, $unsafe_feature:literal, $pre:expr, $op:expr) => {{
        #[cfg(not(feature = $unsafe_feature))]
        {
            let init_value = $pre?; // error prone initial value
            let mut arr: [$T; $LEN] = core::array::from_fn(|_| init_value.clone());
            for (i, data) in $op.enumerate() {
                arr[i] = data?;
            }
            arr
        }
        #[cfg(feature = $unsafe_feature)]
        {
            // SAFETY: array will be fully initialized in the subsequent loop
            let mut arr: [core::mem::MaybeUninit<$T>; $LEN] =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            for (i, data) in $op.enumerate() {
                arr[i].write(data?);
            }
            // SAFETY: we've initialized all the elements
            unsafe { core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
        }
    }};
}
pub use array_init;
