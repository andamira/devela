// devela::code::type::marker
//
//! Zero-cost generic marker IDs.
//
// TOC
// - macro type_marker!
// - tests

/// Defines zero-cost, generic marker IDs.
///
/// Supports attributes and generics without bounds, in which case also
/// defines a `new` constructor method.
///
/// # Example
/// ```
/// # use devela::type_marker;
/// type_marker![Id0];
/// type_marker![Id1<A>];
/// type_marker![Id2<A, B>; Id3<A>; Id4];
///
/// type_marker![
///     /// supports attributes
///     TypeA;
///
///     /// on each type
///     #[repr(transparent)]
///     TypeB<A, B, C>;
/// ];
///
/// impl Id0 { fn hi() {} }
/// impl<A, B> Id2<A, B> { fn hi() {} }
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! type_marker {
    (
        // no generics
        //
        // $meta: an optional list of attributes
        // $name: the name of the marker type
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name;

        impl $name {
            #[doc = concat!("Creates a new `", stringify!($name), "`.")]
            #[inline] #[allow(dead_code)]
            pub fn new() -> Self { Self }
        }

    };
    (
        // one or more generics
        //
        // ...
        // $gen:  a list of generics (>= 1)
        $(#[$meta:meta])*
        $name:ident< $($gen:ident),+ >
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[allow(unused_parens, reason = "can have >1 generics")]
        pub struct $name< $($gen),+ > {
            _marker: core::marker::PhantomData<($($gen),+)>
        }

        impl<$($gen),+> $name<$($gen),+> {
            #[doc = concat!("Creates a new `", stringify!($name), "`.")]
            #[inline] #[allow(dead_code)]
            pub fn new() -> Self {
                Self { _marker: core::marker::PhantomData }
            }
        }
    };
    (
        // multiple types separated by a semicolon (;)
        //
        // ...
        $(
            $(#[$meta:meta])*
            $name:ident $(< $($gen:ident),* >)?
        );+ $(;)?
    ) => {
        $(
            type_marker! {
                $(#[$meta])*
                $name $(< $($gen),* >)?
            }
        )+
    };
}
#[doc(inline)]
pub use type_marker;

#[cfg(test)]
mod tests {
    use crate::{type_marker, TypeId};

    #[test]
    fn type_marker_multiple() {
        type_marker![ZeroG; OneG<A>];

        let zero = ZeroG;
        let one = OneG::<char>::new();

        assert_eq![0, size_of_val(&zero)];
        assert_eq![0, size_of_val(&one)];
    }

    #[test]
    fn type_marker_no_generics() {
        type_marker![ZeroG];

        let zero = ZeroG;

        assert_eq![0, size_of_val(&zero)];
    }

    #[test]
    fn type_marker_generics() {
        type_marker![Two<A, B>];

        let two = Two::<i32, i64>::new();

        assert_eq![0, size_of_val(&two)];
        assert_eq![0, size_of::<Two<char, bool>>()];

        assert_eq![TypeId::of::<Two<i32, i64>>(), TypeId::of::<Two<i32, i64>>()];
        assert_ne![TypeId::of::<Two<i32, i64>>(), TypeId::of::<Two<char, bool>>()];
    }
}
