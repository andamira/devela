// devela::data::id::res
//
//! Type-safe resource IDs.
//
// TOC
// - macro id_resource!
// - trait IdResourced
// - type IdResource
// - core_impls
// - tests

/// Defines zero-sized, type-safe resource IDs.
///
/// Defines a zero-sized struct per each `$name`, implements [`IdResourced`]
/// for it, and a `new` constructor that returns [`IdResource`].
///
///
/// # Example
/// ```
/// # use devela::id_resource;
/// id_resource![Id0:u8]; // single definition and resource
/// id_resource![Id1,Id2:u16]; // multiple definitions, same resource
/// id_resource![Id3,Id4:u32; Id5:u64; Id6,Id7:i8]; // diferent resources
/// ```
#[macro_export]
macro_rules! id_resource {
    // One or multiple resources share the same ID data type
    ($($name:ident),+ : $inner:ty) => {
        $(
            /// Type-safe resource ID with both phantom-based and newtype-based constructors.
            pub struct $name;

            impl $name {
                /// Creates a new `IdResource`.
                #[inline]
                #[allow(unused)]
                pub fn new(data: $inner) -> $crate::IdResource<$name> {
                    $crate::IdResource::new(data)
                }
            }

            /// Associates the resource with its inner ID type for the newtype-based system.
            impl $crate::IdResourced for $name {
                type IdData = $inner;
            }
        )+
    };

    // Multiple groups of resources have different ID data types
    ($($($name:ident),+ : $inner:ty);+ $(;)?) => {
        $( id_resource!($($name),+ : $inner); )+
    };
}
pub use id_resource;

/// Represents an association between a resource and its inner data type.
///
/// See also: [`IdResource`] and [`id_resource!`].
pub trait IdResourced {
    /// The inner type that serves as the unique identifier for the resource.
    type IdData;
}

/// A newtype-based ID that associates a resource with its inner ID.
///
/// # Example
/// ```
/// # use devela::id_resource;
/// id_resource![Id1, Id2: u32];
/// let (id1, id2) = (Id1::new(42), Id2::new(42));
///
/// assert_eq!(42, *id1.get());
/// assert_eq!(*id1.get(), *id2.get());
/// ```
/// ```compile_fail
/// # use devela::id_resource;
/// # id_resource![Id1, Id2: u32];
/// # let (id1, id2) = (Id1::new(42), Id2::new(42));
/// assert_eq!(id1, id2); // type mismatch compile error
/// ```
///
/// See also: [`id_resource!`].
#[repr(transparent)]
#[must_use]
pub struct IdResource<T: IdResourced> {
    data: T::IdData,
}

impl<T: IdResourced> IdResource<T> {
    /// Creates a new `IdResource` instance with the given inner ID.
    #[inline]
    pub const fn new(data: T::IdData) -> Self {
        IdResource { data }
    }

    /// Gets a reference to the ID data.
    #[inline]
    pub const fn get(&self) -> &T::IdData {
        &self.data
    }

    /// Takes ownership of the ID data.
    #[inline]
    pub fn take(self) -> T::IdData {
        self.data
    }
}

#[rustfmt::skip]
mod impls {
    use core::{cmp::Ordering, fmt, hash};
    use super::{IdResource, IdResourced};
    use crate::ConstDefault;
    #[cfg(feature = "alloc")]
    use crate::String;

    impl<T> Clone for IdResource<T> where T: IdResourced, T::IdData: Clone {
        fn clone(&self) -> Self { IdResource::new(self.data.clone()) }
    }
    impl<T> Copy for IdResource<T> where T: IdResourced, T::IdData: Copy {}

    impl<T> fmt::Debug for IdResource<T> where T: IdResourced, T::IdData: fmt::Debug {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self.data) }
    }
    impl<T> fmt::Display for IdResource<T> where T: IdResourced, T::IdData: fmt::Display {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.data) }
    }

    impl<T> PartialEq for IdResource<T> where T: IdResourced, T::IdData: PartialEq {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }
    impl<T> Eq for IdResource<T> where T: IdResourced, T::IdData: Eq {}

    impl<T> PartialOrd for IdResource<T> where T: IdResourced, T::IdData: PartialOrd {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.data.partial_cmp(&other.data) }
    }
    impl<T> Ord for IdResource<T> where T: IdResourced, T::IdData: Ord {
        fn cmp(&self, other: &Self) -> Ordering { self.data.cmp(&other.data) }
    }

    impl<T> hash::Hash for IdResource<T> where T: IdResourced, T::IdData: hash::Hash {
        fn hash<H: hash::Hasher>(&self, state: &mut H) { self.data.hash(state); }
    }
    impl<T> Default for IdResource<T> where T: IdResourced, T::IdData: Default {
        fn default() -> Self { IdResource::new(T::IdData::default()) }
    }
    impl<T> ConstDefault for IdResource<T> where T: IdResourced, T::IdData: ConstDefault {
        const DEFAULT: Self = IdResource::new(T::IdData::DEFAULT);
    }

    #[cfg(feature = "mem_bit")]
    impl<T, const LEN: usize> crate::BitSized<LEN> for IdResource<T>
        where
            T: IdResourced,
            T::IdData: crate::BitSized<LEN>
        {}

    macro_rules! impl_from {
        ($($t:ty),+) => { $( impl_from![@$t]; )+ };
        (@$t:ty) => {
            impl<T> From<$t> for IdResource<T> where T: IdResourced<IdData = $t> {
                fn from(value: $t) -> Self { IdResource::new(value) }
            }
        };
    }
    impl_from![
        bool, char,
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize
    ];

    impl<'a, T> From<&'a str> for IdResource<T> where T: IdResourced<IdData = &'a str> {
        fn from(value: &'a str) -> Self { IdResource::new(value) }
    }
    #[cfg(feature = "alloc")]
    impl<T> From<String> for IdResource<T> where T: IdResourced<IdData = String> {
        fn from(value: String) -> Self { IdResource::new(value) }
    }
}
