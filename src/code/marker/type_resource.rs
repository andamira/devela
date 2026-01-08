// devela::code::marker::type_resource
//
//! Type-safe resource IDs.
//
// TOC
// - macro type_resource!
// - trait TypeResourced
// - type TypeResource
// - core_impls
// - tests

#[doc = crate::_tags!(code uid construction)]
/// Defines zero-cost, zero-sized, type-safe *resource* IDs.
#[doc = crate::_doc_location!("code/marker")]
///
/// This macro generates zero-sized types associated with an inner ID type.
/// These types enable strong type safety at compile time
/// while adding no runtime overhead.
///
/// By associating resources with unique types, the system enforces correct
/// usage of identifiers and reduces errors from mixing unrelated IDs.
///
/// Each generated type provides a `new` constructor method returning a
/// [`TypeResource`] instance of the associated inner type.
///
/// Unlike [`type_marker!`][crate::type_marker], which generates purely marker
/// types with no data, `type_resource!` ties each type to an inner ID type
/// for handling type-safe resources.
///
/// # Example
/// ```
/// # use devela::type_resource;
/// type_resource![Id0:u8]; // single definition and resource
/// type_resource![Id1,Id2:u16]; // multiple definitions, same resource
/// type_resource![Id3,Id4:u32; Id5:u64; Id6,Id7:i8]; // diferent resources
/// ```
/// See also: [`TypeResource`] and [`TypeResourced`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! type_resource {
    // One or multiple resources share the same ID data type
    ($($name:ident),+ : $inner:ty) => {
        $(
            /// Type-safe resource ID with both phantom-based and newtype-based constructors.
            pub struct $name;

            impl $name {
                /// Creates a new `TypeResource`.
                #[allow(unused)]
                pub const fn new(data: $inner) -> $crate::TypeResource<$name> {
                    $crate::TypeResource::new(data)
                }
            }

            /// Associates the resource with its inner ID type for the newtype-based system.
            impl $crate::TypeResourced for $name {
                type TypeData = $inner;
            }
        )+
    };

    // Multiple groups of resources have different ID data types
    ($($($name:ident),+ : $inner:ty);+ $(;)?) => {
        $( type_resource!($($name),+ : $inner); )+
    };
}
#[doc(inline)]
pub use type_resource;

#[doc = crate::_tags!(code uid)]
/// Represents an association between a resource and its inner data type.
#[doc = crate::_doc_location!("code/marker")]
///
/// See also: [`TypeResource`] and [`type_resource!`].
pub trait TypeResourced {
    /// The inner type that serves as the unique identifier for the resource.
    type TypeData;
}

/// A newtype-based ID that associates a resource with its inner ID.
#[doc = crate::_doc_location!("code/marker")]
///
/// # Examples
/// ```
/// # use devela::type_resource;
/// type_resource![Id1, Id2: u32];
/// let (id1, id2) = (Id1::new(42), Id2::new(42));
///
/// assert_eq!(42, *id1.get());
/// assert_eq!(*id1.get(), *id2.get());
/// ```
/// ```compile_fail
/// # use devela::type_resource;
/// # type_resource![Id1, Id2: u32];
/// # let (id1, id2) = (Id1::new(42), Id2::new(42));
/// assert_eq!(id1, id2); // type mismatch compile error
/// ```
///
/// See also: [`type_resource!`], [`TypeResourced`].
#[repr(transparent)]
#[must_use]
pub struct TypeResource<T: TypeResourced> {
    data: T::TypeData,
}

impl<T: TypeResourced> TypeResource<T> {
    /// Creates a new `TypeResource` instance with the given inner ID.
    pub const fn new(data: T::TypeData) -> Self {
        TypeResource { data }
    }

    /// Gets a reference to the ID data.
    pub const fn get(&self) -> &T::TypeData {
        &self.data
    }

    /// Takes ownership of the ID data.
    pub fn take(self) -> T::TypeData {
        self.data
    }
}

#[rustfmt::skip]
mod impls {
    #[cfg(feature = "alloc")]
    use crate::String;
    use crate::{
        ConstInit, Debug, Display, FmtResult, Formatter, Hash, Hasher, Ordering, TypeResource,
        TypeResourced,
    };

    impl<T> Clone for TypeResource<T> where T: TypeResourced, T::TypeData: Clone {
        fn clone(&self) -> Self { TypeResource::new(self.data.clone()) }
    }
    impl<T> Copy for TypeResource<T> where T: TypeResourced, T::TypeData: Copy {}

    impl<T> Debug for TypeResource<T> where T: TypeResourced, T::TypeData: Debug {
        fn fmt(&self, f: &mut Formatter) -> FmtResult<()> { write!(f, "{:?}", self.data) }
    }
    impl<T> Display for TypeResource<T> where T: TypeResourced, T::TypeData: Display {
        fn fmt(&self, f: &mut Formatter) -> FmtResult<()> { write!(f, "{}", self.data) }
    }

    impl<T> PartialEq for TypeResource<T> where T: TypeResourced, T::TypeData: PartialEq {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }
    impl<T> Eq for TypeResource<T> where T: TypeResourced, T::TypeData: Eq {}

    impl<T> PartialOrd for TypeResource<T> where T: TypeResourced, T::TypeData: PartialOrd {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.data.partial_cmp(&other.data)
        }
    }
    impl<T> Ord for TypeResource<T> where T: TypeResourced, T::TypeData: Ord {
        fn cmp(&self, other: &Self) -> Ordering { self.data.cmp(&other.data) }
    }

    impl<T> Hash for TypeResource<T> where T: TypeResourced, T::TypeData: Hash {
        fn hash<H: Hasher>(&self, state: &mut H) { self.data.hash(state); }
    }
    impl<T> Default for TypeResource<T> where T: TypeResourced, T::TypeData: Default {
        fn default() -> Self { TypeResource::new(T::TypeData::default()) }
    }
    impl<T> ConstInit for TypeResource<T> where T: TypeResourced, T::TypeData: ConstInit {
        const INIT: Self = TypeResource::new(T::TypeData::INIT);
    }

    #[cfg(feature = "bit")]
    impl<T, const LEN: usize> crate::BitSized<LEN> for TypeResource<T>
        where
            T: TypeResourced,
            T::TypeData: crate::BitSized<LEN>
        {}

    macro_rules! impl_from {
        ($($t:ty),+) => { $( impl_from![@$t]; )+ };
        (@$t:ty) => {
            impl<T> From<$t> for TypeResource<T> where T: TypeResourced<TypeData = $t> {
                fn from(value: $t) -> Self { TypeResource::new(value) }
            }
        };
    }
    impl_from![
        bool, char,
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize
    ];

    impl<'a, T> From<&'a str> for TypeResource<T> where T: TypeResourced<TypeData = &'a str> {
        fn from(value: &'a str) -> Self { TypeResource::new(value) }
    }
    #[cfg(feature = "alloc")]
    impl<T> From<String> for TypeResource<T> where T: TypeResourced<TypeData = String> {
        fn from(value: String) -> Self { TypeResource::new(value) }
    }
}

#[cfg(test)]
mod tests {
    use crate::AnyExt;

    type_resource![ClosureId: u64];

    #[test]
    fn closures_with_type_resource() {
        let closure1 = || {};
        let closure2 = || {};

        let closure_id1 = ClosureId::new(closure1.type_hash());
        let closure_id2 = ClosureId::new(closure2.type_hash());

        assert_ne!(closure_id1, closure_id2);
    }
}
