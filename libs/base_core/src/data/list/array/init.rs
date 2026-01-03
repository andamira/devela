// devela_base_core::data::list::array::init
//
//! Defines the [`array_init!`] macro.
//
// WAIT: [Stack overflow with Boxed array](https://github.com/rust-lang/rust/issues/53827)

#[doc = crate::_TAG_INIT!()]
#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// Initializes a [`[$T; $LEN]`][array] array in multiple ways.
#[doc = crate::_doc_location!("data/list/array")]
///
/// # Arguments
/// - `[$T; $LEN]`: the array's elements' type and length.
/// - `$init`: a function with an `usize` argument that returns `$T`.
/// - `$const_fn`: a const fn with an `usize` argument that returns `$T: Copy`.
/// - `$copiable`: an expression that returns an element of type `$T: Copy`.
/// - `$clonable`: an expression that returns an element of type `$T: Clone`.
/// - `$fsafe`: a feature that forbids the use of `unsafe` when enabled.
/// - `$funsafe`: a feature that enables the use of `unsafe` when enabled.
/// - `$intoiter`: an item that implements [`IntoIterator`].
/// - `$trait`: in the `INIT in` arm, the path to either `ConstInit` or `ConstInitCore`.
///
/// # Examples
/// ```
/// # use devela_base_core::array_init;
/// # #[cfg(feature = "alloc")]
/// # use devela::{Vec, ConstInit}; // IMPROVE: remove alloc/Vec, use ConstInitCore
/// assert_eq![[2,4,6], array_init![safe_init [i32; 3], |n| (n as i32 + 1) * 2]];
/// #[cfg(feature = "unsafe_array")]
/// assert_eq![[3,6,9], array_init![unsafe_init [i32; 3], |n| (n as i32 + 1) * 3]];
///
/// assert_eq![[2,4,6], array_init![init [i32; 3], "safe", "unsafe_array",
///     |n| (n as i32 + 1) * 2]];
/// # #[cfg(feature = "alloc")]
/// assert_eq![Box::new([2,4,6]), array_init![init_heap [i32; 3], "safe", "unsafe_array",
///     |n| (n as i32 + 1) * 2]];
///
/// assert_eq![[7,7,7], array_init![clone [i32; 3], "safe", "unsafe_array", 7]];
/// assert_eq![[0,0,0], array_init![default [i32; 3], "safe", "unsafe_array"]];
/// # #[cfg(feature = "alloc")] {
/// assert_eq![[4,5,6], array_init![iter [i32; 3], "safe", "unsafe_array", vec![4,5,6,7,8]]];
/// assert_eq![[4,0,0], array_init![iter [i32; 3], "safe", "unsafe_array", vec![4]]];
/// # }
///
/// const fn init(n: usize) -> i32 { (n as i32 + 1) * 4 }
/// const ARRAY1: [i32; 3] = array_init![const_fn [i32; 3], "safe", "unsafe_array", init, 0];
/// assert_eq![[4, 8, 12], ARRAY1];
///
/// # #[cfg(feature = "alloc")] {
/// const ARRAY2: [i32; 3] = array_init![INIT in ConstInit [i32; 3]];
/// assert_eq![[0, 0, 0], ARRAY2];
/// # }
/// ```
///
/// # Features
/// The unsafe version uses [`MaybeUninit`][crate::MaybeUninit] in the case
/// of stack allocation or [`Box::from_raw`] in the case of heap allocation.
#[doc = crate::doclink!(custom devela
    "[`Box::from_raw`]" "sys/mem/struct.Box.html#method.from_raw")]
///
/// For the `const`, `clone`, `default` and `iter` versions, if the given
/// `$funsafe` is enabled and the given `$fsafe` is disabled, it will use unsafe
/// initialization.
///
/// # Notes
/// For the heap-related arms needs to have `Vec` in scope.
// WAIT [array_repeat](https://github.com/rust-lang/rust/issues/126695)
// WAIT [array_try_from_fn](https://github.com/rust-lang/rust/issues/89379)
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! array_init {
    (
    /* safe initializations */

    // safe array initialization in the stack
    safe_init [$T:ty; $LEN:expr], $init:expr) => {{
        #[allow(clippy::redundant_closure_call, reason  = "macro arg isn't redundant")]
        ::core::array::from_fn(|i| $init(i))
    }};
    (
    // safe array initialization in the stack, compile-time friendly.
    safe_const_fn [$T:ty; $LEN:expr], $const_fn:expr, $copiable:expr) => {{
        let mut arr: [$T; $LEN] = [$copiable; $LEN];
        let mut i = 0;
        while i < $LEN {
            // WAIT: [const_closures](https://github.com/rust-lang/rust/issues/106003)
            arr[i] = $const_fn(i);
            i += 1;
        }
        arr
    }};
    (
    // safe array initialization in the heap
    safe_init_heap [$T:ty; $LEN:expr], $init:expr) => {{
        let mut v = Vec::<$T>::with_capacity($LEN);
        for i in 0..$LEN {
            #[allow(clippy::redundant_closure_call, reason  = "macro arg isn't redundant")]
            v.push($init(i));
        }
        v.into_boxed_slice().try_into().unwrap_or_else(|_| {
            panic!("Can't turn the boxed slice into a boxed array")
        })
    }};
    (

    /* unsafe initializations */

    // unsafe array initialization in the stack
    // TODO:CHECK:AGAIN
    unsafe_init [$T:ty; $LEN:expr], $init:expr) => {{
        let mut arr: [$crate::MaybeUninit<$T>; $LEN] =
            // SAFETY: array will be fully initialized in the subsequent loop
            unsafe { $crate::MaybeUninit::uninit().assume_init() };
        for (i, e) in &mut arr[..].iter_mut().enumerate() {
            #[allow(clippy::redundant_closure_call, reason  = "macro arg isn't redundant")]
            let _ = e.write($init(i)); // NOTE: const since 1.85
        }
        // Can't use transmute for now, have to use transmute_copy:
        // - WAIT:CLOSED:[const generics transmute](https://github.com/rust-lang/rust/issues/61956)
        //   - https://github.com/rust-lang/rust/issues/62875 (duplicate)
        // unsafe { $crate::transmute::<_, [T; LEN]>(&arr) }
        // SAFETY: we've initialized all the elements
        unsafe { ::core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
    }};
    (
    // unsafe array initialization in the stack, compile-time friendly.
    unsafe_const_fn [$T:ty; $LEN:expr], $const_fn:expr) => {{
        // WAIT: [maybe_uninit_uninit_array](https://github.com/rust-lang/rust/issues/96097)
        let mut arr: [$crate::MaybeUninit<$T>; $LEN] =
            // SAFETY: array will be fully initialized in the subsequent loop
            unsafe { $crate::MaybeUninit::uninit().assume_init() };
        let mut i = 0;
        while i < $LEN {
            arr[i] = $crate::MaybeUninit::new($const_fn(i));
            i += 1;
        }
        // SAFETY: we've initialized all the elements
        unsafe { ::core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
    }};
    (
    // unsafe array initialization in the heap
    unsafe_init_heap [$T:ty; $LEN:expr], $init:expr) => {{
        let mut v = Vec::<$T>::with_capacity($LEN);
        #[allow(clippy::redundant_closure_call, reason  = "macro arg isn't redundant")]
        for i in 0..$LEN { v.push($init(i)); }
        let slice = v.into_boxed_slice();
        let raw_slice = Box::into_raw(slice);
        // SAFETY: pointer comes from using `into_raw`, and capacity is right.
        unsafe { Box::from_raw(raw_slice as *mut [$T; $LEN]) }
    }};
    (

    /* safety-agnostic initializations */

    // initialize an array in the stack
    init [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $init:expr) => {{
        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { $crate::array_init![safe_init [$T; $LEN], $init] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { $crate::array_init![unsafe_init [$T; $LEN], $init] }
    }};
    (
    // initialize an array the stack, compile-time friendly.
    // $copiable is only used by the safe version as temporary placeholder.
    const_fn
    [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $const_fn:expr, $copiable:expr) => {{
        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { $crate::array_init![safe_const_fn [$T; $LEN], $const_fn, $copiable] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { $crate::array_init![unsafe_const_fn [$T; $LEN], $const_fn ] }
    }};
    (
    // initialize an array in the heap
    init_heap [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $init:expr) => {{
        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { $crate::array_init![safe_init_heap [$T; $LEN], $init] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { $crate::array_init![unsafe_init_heap [$T; $LEN], $init] }
    }};
    (

    // initialize an array in the stack by cloning $clonable
    clone [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $clonable:expr) => {{
        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { $crate::array_init![safe_init [$T; $LEN], |_| $clonable.clone()] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { $crate::array_init![unsafe_init [$T; $LEN], |_| $clonable.clone()] }
    }};
    (
    // initialize an array in the heap, by cloning $clonable
    clone_heap [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $clonable:expr) => {{
        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { $crate::array_init![safe_init_heap [$T; $LEN], |_| $clonable.clone()] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { $crate::array_init![unsafe_init_heap [$T; $LEN], |_| $clonable.clone()] }
    }};
    (
    // initialize an array in the stack with $T: Default::default()
    default [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal) => {{

        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { $crate::array_init![safe_init [$T; $LEN], |_| <$T>::default()] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { $crate::array_init![unsafe_init [$T; $LEN], |_| <$T>::default()] }
    }};
    (
    // initialize an array in the stack with $T: $trait::INIT
    INIT in $trait:path [$T:ty; $LEN:expr]) => {{
        [<$T as $trait>::INIT; $LEN]
    }};
    (
    // initialize an array in the heap, with $T: Default::default()
    default_heap [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal) => {{
        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { $crate::array_init![safe_init_heap [$T; $LEN], |_| <$T>::default()] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { $crate::array_init![unsafe_init_heap [$T; $LEN], |_| <$T>::default()] }
    }};
    (

    // initialize an array in the stack with an IntoIterator<Item = $T> and with
    // $T::default() in case the iterator length is < $LEN, for the remaining elements.
    iter [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $intoiter:expr) => {{
        let mut iterator = $intoiter.into_iter();
        let mut init_closure = |_| {
            if let Some(e) = iterator.next() { e } else { <$T>::default() }
        };
        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { $crate::array_init![safe_init [$T; $LEN], init_closure] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { $crate::array_init![unsafe_init [$T; $LEN], init_closure] }
    }};
    (
    // initialize an array in the heap with an IntoIterator<Item = $T> and with
    // $T::default() in case the iterator length is < $LEN, for the remaining elements.
    iter_heap [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $intoiter:expr) => {{

        let mut iterator = $intoiter.into_iter();
        let mut init_closure = |_| {
            if let Some(e) = iterator.next() { e } else { <$T>::default() }
        };
        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { $crate::array_init![safe_init_heap [$T; $LEN], init_closure] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { $crate::array_init![unsafe_init_heap [$T; $LEN], init_closure] }
    }};
    (

    // initialize an array by applying $op (in safe mode it first clones $pre)
    // and propagates errors both from $pre and $op.
    preop [$T:ty; $LEN:expr]?, $fsafe:literal, $funsafe:literal, $pre:expr, $op:expr) => {{

        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        {
            let init_value = $pre?; // error prone initial value
            let mut arr: [$T; $LEN] = ::core::array::from_fn(|_| init_value.clone());
            for (i, data) in $op.enumerate() {
                arr[i] = data?;
            }
            arr
        }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        {
            let mut arr: [$crate::MaybeUninit<$T>; $LEN] =
                // SAFETY: array will be fully initialized in the subsequent loop
                unsafe { $crate::MaybeUninit::uninit().assume_init() };
            for (i, data) in $op.enumerate() {
                arr[i].write(data?);
            }
            // SAFETY: we've initialized all the elements
            unsafe { ::core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
        }
    }};
}
#[doc(inline)]
pub use array_init;
