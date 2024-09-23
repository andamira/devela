// devela::data::id::marker
//
//! Zero-cost generic marker IDs.
//
// TOC
// - macro id_marker!
// - tests

/// Defines zero-cost, generic marker IDs.
///
/// Supports attributes and generics without bounds, in which case also
/// defines a `new` constructor method.
///
/// # Example
/// ```
/// # use devela::id_marker;
/// id_marker![Id0];
/// id_marker![Id1<A>];
/// id_marker![Id2<A, B>; Id3<A>; Id4];
///
/// id_marker![
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
macro_rules! id_marker {
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
            id_marker! {
                $(#[$meta])*
                $name $(< $($gen),* >)?
            }
        )+
    };
}
#[doc(inline)]
pub use id_marker;

#[cfg(test)]
mod tests {
    use super::id_marker;
    use core::any::TypeId;

    #[test]
    fn id_marker_multiple() {
        id_marker![ZeroG; OneG<A>];

        let zero = ZeroG;
        let one = OneG::<char>::new();

        assert_eq![0, size_of_val(&zero)];
        assert_eq![0, size_of_val(&one)];
    }

    #[test]
    fn id_marker_no_generics() {
        id_marker![ZeroG];

        let zero = ZeroG;

        assert_eq![0, size_of_val(&zero)];
    }

    #[test]
    fn id_marker_generics() {
        id_marker![Two<A, B>];

        let two = Two::<i32, i64>::new();

        assert_eq![0, size_of_val(&two)];
        assert_eq![0, size_of::<Two<char, bool>>()];

        assert_eq![TypeId::of::<Two<i32, i64>>(), TypeId::of::<Two<i32, i64>>()];
        assert_ne![TypeId::of::<Two<i32, i64>>(), TypeId::of::<Two<char, bool>>()];
    }
}
