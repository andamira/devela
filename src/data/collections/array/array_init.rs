// devela::data::collections::array::array_init
//
//!
//

/// Initializes a `[$T; $LEN]` array in multiple ways.
///
/// # Arguments
/// - `[$T; $LEN]`: the array's elements' type and length.
/// - `$init`: a function with an `usize` argument that returns `$T`.
/// - `$const_init`: a const fn with an `usize` argument that returns `$T: Copy`.
/// - `$copiable`: an expression that returns an element of type `$T: Copy`.
/// - `$clonable`: an expression that returns an element of type `$T: Clone`.
/// - `$fsafe`: a feature that forbids the use of `unsafe` when enabled.
/// - `$funsafe`: a feature that enables the use of `unsafe` when enabled.
/// - `$intoiter`: an item that implements [`IntoIterator`].
///
/// # Examples
/// ```
/// # use devela::data::array_init;
/// assert_eq![[2,4,6], array_init![safe_init [i32; 3], |n| (n as i32 + 1) * 2]];
/// #[cfg(feature = "unsafe_array")]
/// assert_eq![[3,6,9], array_init![unsafe_init [i32; 3], |n| (n as i32 + 1) * 3]];
///
/// assert_eq![[7,7,7], array_init![clone [i32; 3], "safe", "unsafe_array", 7]];
/// assert_eq![[0,0,0], array_init![default [i32; 3], "safe", "unsafe_array"]];
/// assert_eq![[4,5,6], array_init![iter [i32; 3], "safe", "unsafe_array", vec![4,5,6,7,8]]];
/// assert_eq![[4,0,0], array_init![iter [i32; 3], "safe", "unsafe_array", vec![4]]];
///
/// const fn init(n: usize) -> i32 { (n as i32 + 1) * 4 }
/// const ARRAY1: [i32; 3] = array_init![const_init [i32; 3], "safe", "unsafe_array", init, 0];
/// assert_eq![[4, 8, 12], ARRAY1];
///
/// const ARRAY2: [i32; 3] = array_init![const_default [i32; 3]];
/// assert_eq![[0, 0, 0], ARRAY2];
/// ```
///
/// # Features
/// The unsafe version uses [`MaybeUninit`][core::mem::MaybeUninit] in the case
/// of stack allocation or [`Box::from_raw`] in the case of heap allocation.
///
/// For the `const_init`, `clone`, `default` and `iter` versions, if the given
/// `$funsafe` is enabled and the given `$fsafe` is disabled, it will use unsafe
/// initialization.
#[macro_export]
macro_rules! array_init {
    (
    // safe array initialization in the stack
    safe_init [$T:ty; $LEN:expr], $init:expr) => {{

        #[allow(clippy::redundant_closure_call)]
        core::array::from_fn(|i| $init(i))
    }};
    (
    // safe array initialization in the heap
    safe_init_heap [$T:ty; $LEN:expr], $init:expr) => {{

        let mut v = $crate::_deps::alloc::vec::Vec::<$T>::with_capacity($LEN);
        for i in 0..$LEN {
            #[allow(clippy::redundant_closure_call)]
            v.push($init(i));
        }
        v.into_boxed_slice().try_into().unwrap_or_else(|_| {
            panic!("Can't turn the boxed slice into a boxed array")
        })
    }};

   (
    //
    // unsafe array initialization in the stack
    unsafe_init [$T:ty; $LEN:expr], $init:expr) => {{

        // SAFETY: array will be fully initialized in the subsequent loop
        let mut arr: [core::mem::MaybeUninit<$T>; $LEN] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        for (i, e) in &mut arr[..].iter_mut().enumerate() {
            #[allow(clippy::redundant_closure_call)]
            let _ = e.write($init(i));
        }
        // Can't use transmute for now, have to use transmute_copy:
        // - WAIT: [const generics transmute](https://github.com/rust-lang/rust/issues/61956)
        //   - https://github.com/rust-lang/rust/issues/62875 (duplicate)
        // unsafe { core::mem::transmute::<_, [T; LEN]>(&arr) }
        // SAFETY: we've initialized all the elements
        unsafe { core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
    }};
    (
    // unsafe array initialization in the heap
    unsafe_init_heap [$T:ty; $LEN:expr], $init:expr) => {{

        let mut v = $crate::_deps::alloc::vec::Vec::<$T>::with_capacity($LEN);
        #[allow(clippy::redundant_closure_call)]
        for i in 0..$LEN { v.push($init(i)); }
        let slice = v.into_boxed_slice();
        let raw_slice = Box::into_raw(slice);
        // SAFETY: pointer comes from using `into_raw`, and capacity is right.
        unsafe { Box::from_raw(raw_slice as *mut [$T; $LEN]) }
    }};

    (
    //
    // safe array initialization in the stack, compile-time friendly.
    safe_const_init [$T:ty; $LEN:expr], $const_init:expr, $copiable:expr) => {{

        let mut arr: [$T; $LEN] = [$copiable; $LEN];
        let mut i = 0;
        while i < $LEN {
            // WAIT: [const_closures](https://github.com/rust-lang/rust/issues/106003)
            arr[i] = $const_init(i);
            i += 1;
        }
        arr
    }};
    (
    // unsafe array initialization in the stack, compile-time friendly.
    unsafe_const_init [$T:ty; $LEN:expr], $const_init:expr) => {{

        // WAIT: [maybe_uninit_uninit_array](https://github.com/rust-lang/rust/issues/96097)
        // SAFETY: array will be fully initialized in the subsequent loop
        let mut arr: [core::mem::MaybeUninit<$T>; $LEN] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let mut i = 0;
        while i < $LEN {
            arr[i] = core::mem::MaybeUninit::new($const_init(i));
            i += 1;
        }
        // SAFETY: we've initialized all the elements
        unsafe { core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
    }};
    (
    // initialize an array the stack, compile-time friendly.
    // $copiable is only used by the safe version as temporary placeholder.
    const_init [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $const_init:expr, $copiable:expr) => {{

        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { array_init![safe_const_init [$T; $LEN], $const_init, $copiable] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { array_init![unsafe_const_init [$T; $LEN], $const_init ] }
    }};

    // ---

    (
    //
    // initialize an array in the stack by cloning $clonable
    clone [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $clonable:expr) => {{

        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { array_init![safe_init [$T; $LEN], |_| $clonable.clone()] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { array_init![unsafe_init [$T; $LEN], |_| $clonable.clone()] }
    }};
    (
    // initialize an array in the heap, by cloning $clonable
    clone_heap [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $clonable:expr) => {{

        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { array_init![safe_init_heap [$T; $LEN], |_| $clonable.clone()] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { array_init![unsafe_init_heap [$T; $LEN], |_| $clonable.clone()] }
    }};

    (
    //
    // initialize an array in the stack with $T: Default::default()
    default [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal) => {{

        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { array_init![safe_init [$T; $LEN], |_| <$T>::default()] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { array_init![unsafe_init [$T; $LEN], |_| <$T>::default()] }
    }};
    (
    // initialize an array in the heap, with $T: Default::default()
    default_heap [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal) => {{

        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { array_init![safe_init_heap [$T; $LEN], |_| <$T>::default()] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { array_init![unsafe_init_heap [$T; $LEN], |_| <$T>::default()] }
    }};

    (
    // initialize an array in the stack with $T: ConstDefault::DEFAULT
    const_default [$T:ty; $LEN:expr]) => {{
        [<$T as $crate::code::ConstDefault>::DEFAULT; $LEN]
    }};

    (
    //
    // initialize an array in the stack with an IntoIterator<Item = $T> and with
    // $T::default() in case the iterator length is < $LEN, for the remaining elements.
    iter [$T:ty; $LEN:expr], $fsafe:literal, $funsafe:literal, $intoiter:expr) => {{

        let mut iterator = $intoiter.into_iter();
        let mut init_closure = |_| {
            if let Some(e) = iterator.next() { e } else { <$T>::default() }
        };
        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        { array_init![safe_init [$T; $LEN], init_closure] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { array_init![unsafe_init [$T; $LEN], init_closure] }
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
        { array_init![safe_init_heap [$T; $LEN], init_closure] }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        { array_init![unsafe_init_heap [$T; $LEN], init_closure] }
    }};

    // ---

    (
    // initialize an array by applying $op (in safe mode it first clones $pre)
    // and propagates errors both from $pre and $op.
    preop [$T:ty; $LEN:expr]?, $fsafe:literal, $funsafe:literal, $pre:expr, $op:expr) => {{

        #[cfg(any(feature = $fsafe, not(feature = $funsafe)))]
        {
            let init_value = $pre?; // error prone initial value
            let mut arr: [$T; $LEN] = core::array::from_fn(|_| init_value.clone());
            for (i, data) in $op.enumerate() {
                arr[i] = data?;
            }
            arr
        }
        #[cfg(all(not(feature = $fsafe), feature = $funsafe))]
        {
            let mut arr: [core::mem::MaybeUninit<$T>; $LEN] =
                // SAFETY: array will be fully initialized in the subsequent loop
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
