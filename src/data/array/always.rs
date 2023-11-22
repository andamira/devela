// devela::data::array::always
//
//! Array functionality always available for internal use.
//
// NOTE: can't use transmute for now, have to use transmute_copy:
// - https://github.com/rust-lang/rust/issues/62875
// - https://github.com/rust-lang/rust/issues/61956
// unsafe { core::mem::transmute::<_, [T; LEN]>(&arr) } // WAIT
// unsafe { core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }


#![allow(unused)]

/// Initializes a `[$T; $LEN]` array.
///
/// It expects the name of a feature that enables using unsafe functionality.
///
/// # Features
/// If the given `$unsafe_feature` is enabled it uses [`MaybeUninit`] in the
/// case of stack allocation or [`Box::from_raw`] in the case of heap allocation.
#[macro_export]
macro_rules! array_init {
    // initialize an array by cloning $element
    (clone [$T:ty; $LEN:expr], $unsafe_feature:literal, $element:expr) => {{
        #[cfg(not(feature = $unsafe_feature))]
        {
            core::array::from_fn(|_| $element.clone())
        }

        #[cfg(feature = $unsafe_feature)]
        {
            let mut arr: [core::mem::MaybeUninit<$T>; $LEN] =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            for i in &mut arr[..] {
                let _ = i.write($element.clone());
            }
            // SAFETY: we've initialized all the elements
            unsafe { core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
        }
    }};

    // initialize an array in the heap, by cloning $element
    (boxed_clone [$T:ty; $LEN:expr], $unsafe_feature:literal, $element:expr) => {{
        #[cfg(not(feature = $unsafe_feature))]
        {
            let mut v = Vec::<$T>::with_capacity($LEN);
            for _ in 0..$LEN {
                v.push($element.clone());
            }
            let array = v.into_boxed_slice().try_into().unwrap_or_else(|_| {
                panic!("Can't turn the boxed slice into a boxed array");
            });
            array
        }

        #[cfg(feature = $unsafe_feature)]
        {
            let mut v = Vec::<$T>::with_capacity($LEN);
            for _ in 0..$LEN {
                v.push($element.clone());
            }
            let slice = v.into_boxed_slice();
            let raw_slice = Box::into_raw(slice);
            // SAFETY: pointer comes from using `into_raw`, and capacity is right.
            unsafe { Box::from_raw(raw_slice as *mut [$T; $LEN]) }
        }
    }};

    // initialize an array with $T::default()
    (default [$T:ty; $LEN:expr], $unsafe_feature:literal) => {{
        #[cfg(not(feature = $unsafe_feature))]
        {
            core::array::from_fn(|_| <$T>::default())
        }

        #[cfg(feature = $unsafe_feature)]
        {
            let mut arr: [core::mem::MaybeUninit<$T>; $LEN] =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            for i in &mut arr[..] {
                let _ = i.write(<$T>::default());
            }
            // SAFETY: we've initialized all the elements
            unsafe { core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
        }
    }};

    // initialize an array in the heap, with $T::default()
    (boxed_default [$T:ty; $LEN:expr], $unsafe_feature:literal) => {{
        #[cfg(not(feature = $unsafe_feature))]
        {
            let mut v = Vec::<$T>::with_capacity($LEN);
            for _ in 0..$LEN {
                v.push(<$T>::default());
            }
            let array = v.into_boxed_slice().try_into().unwrap_or_else(|_| {
                panic!("Can't turn the boxed slice into a boxed array");
            });
            array
        }

        #[cfg(feature = $unsafe_feature)]
        {
            let mut v = Vec::<$T>::with_capacity($LEN);
            for _ in 0..$LEN {
                v.push(<$T>::default());
            }
            let slice = v.into_boxed_slice();
            let raw_slice = Box::into_raw(slice);
            // SAFETY: pointer comes from using `into_raw`, and capacity is right.
            unsafe { Box::from_raw(raw_slice as *mut [$T; $LEN]) }
        }
    }};

    // initialize an array with an IntoIterator<Item = $T> and with $T::default()
    // in case the iterator length is < $LEN, for the remaining elements.
    (iter [$T:ty; $LEN:expr], $unsafe_feature:literal, $iter:expr) => {{
        let mut iterator = $iter.into_iter();

        #[cfg(not(feature = $unsafe_feature))]
        {
            core::array::from_fn(|_| {
                if let Some(e) = iterator.next() {
                    e
                } else {
                    <$T>::default()
                }
            })
        }

        #[cfg(feature = $unsafe_feature)]
        {
            let mut arr: [core::mem::MaybeUninit<$T>; $LEN] =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            for i in &mut arr[..] {
                if let Some(e) = iterator.next() {
                    let _ = i.write(e);
                } else {
                    let _ = i.write(T::default());
                }
            }
            // SAFETY: we've initialized all the elements
            unsafe { core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
        }
    }};

    // initialize an array in the heap with an IntoIterator<Item = $T> and with
    // $T::default() in case the iterator length is < $LEN, for the remaining elements.
    (boxed_iter [$T:ty; $LEN:expr], $unsafe_feature:literal, $iter:expr) => {{
        let mut iterator = $iter.into_iter();

        #[cfg(not(feature = $unsafe_feature))]
        {
            let mut v = Vec::<$T>::with_capacity($LEN);
            for _ in 0..$LEN {
                if let Some(e) = iterator.next() {
                    v.push(e);
                } else {
                    v.push(<$T>::default());
                }
            }
            let array = v.into_boxed_slice().try_into().unwrap_or_else(|_| {
                panic!("Can't turn the boxed slice into a boxed array");
            });
            array
        }

        #[cfg(feature = $unsafe_feature)]
        {
            let mut v = Vec::<$T>::with_capacity($LEN);
            for _ in 0..$LEN {
                if let Some(e) = iterator.next() {
                    v.push(e);
                } else {
                    v.push(<$T>::default());
                }

            }
            let slice = v.into_boxed_slice();
            let raw_slice = Box::into_raw(slice);
            // SAFETY: pointer comes from using `into_raw`, and capacity is right.
            unsafe { Box::from_raw(raw_slice as *mut [$T; $LEN]) }
        }
    }};

    // initialize an array by applying $init (in safe mode first clones $pre_init)
    // and propagates errors both from $pre_init and $init.
    ([$T:ty; $LEN:expr]?, $unsafe_feature:literal, $pre_init:expr, $init:expr) => {{
        #[cfg(not(feature = $unsafe_feature))]
        {
            let init_value = $pre_init?; // error prone initial value
            let mut arr: [$T; $LEN] = core::array::from_fn(|_| init_value.clone());
            for (i, data) in $init.enumerate() {
                arr[i] = data?;
            }
            arr
        }

        #[cfg(feature = $unsafe_feature)]
        {
            let mut arr: [core::mem::MaybeUninit<$T>; $LEN] =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            for (i, data) in $init.enumerate() {
                arr[i].write(data?);
            }
            // SAFETY: we've initialized all the elements
            unsafe { core::mem::transmute_copy::<_, [$T; $LEN]>(&arr) }
        }
    }};
}
pub use array_init;
