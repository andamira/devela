// devela_base_core::code::util::impl_trait

/// A helper macro to concisely implement a few common utility traits.
///
/// ## Traits supported
/// - Hash
/// - fmt:: Debug, Display…
///
/// ## Features:
/// - Allows multiple types in a single invocation, separated by semicolon
/// - Supports types with or without lifetimes and generics.
/// - Comma between lifetimes and generics is optional but recommended for clarity.
/// - Requires the same formatting trait (`fmt::<trait>`) to be implemented for all generics.
///
/// ## Example
/// ```
/// # use devela_base_core::impl_trait;
/// struct S0(usize);
/// struct S1<T> { v: T }
/// struct S2<'a, T> { v: &'a T }
///
/// impl_trait![Hash for S0 |self, state| self.0.hash(state)];
///
/// impl_trait![fmt::Binary for S1<T> where T |self, f| self.v.fmt(f)];
///
/// impl_trait!{fmt::Debug for S1<T> where T; S2<'a, T> where T |self, f| {
///     write!(f, "S? {{ v: {:?} }}", self.v)
/// }}
/// ```
// IMPROVE: support const-generic arguments, like e.g. for NonValue*
// IMPROVE? support `$generic` as `ty` instead of `ident` (e.g. for `Divisor`)
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! impl_trait {
    (Hash for
     $(
        $type:ident<$($lt:lifetime),* $(,)? $($generic:ident),*>
        $(where $($bounded:ident),+ )?
     );+ |$self:ident, $state:ident| $expr:expr) => {
        $(
            impl<$($lt,)* $($generic,)*> $crate::Hash for $type<$($lt,)* $($generic,)*>
                $(where $($bounded: $crate::Hash,)+ )? {
                fn hash<__H: $crate::Hasher>(&$self, $state: &mut __H) { $expr }
            }
        )+
    };
    (Hash for $($type:ident),+ |$self:ident, $state:ident| $expr:expr) => {
        $(
            impl $crate::Hash for $type {
                fn hash<__H: $crate::Hasher>(&$self, $state: &mut __H) { $expr }
            }
        )+
    };

    (fmt::$trait:ident for
     $(
        $type:ident<$($lt:lifetime),* $(,)? $($generic:ident),*>
        $(where $($bounded:ident),+ )?
     );+ |$self:ident, $f:ident| $expr:expr) => {
        $(
            impl<$($lt,)* $($generic,)*> $crate::$trait for $type<$($lt,)* $($generic,)*>
                $(where $($bounded: $crate::$trait,)+ )? {
                fn fmt(&$self, $f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> { $expr }
            }
        )+
    };
    (fmt::$trait:ident for $($type:ident),+ |$self:ident, $f:ident| $expr:expr) => {
        $(
            impl $crate::$trait for $type {
                fn fmt(&$self, $f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> { $expr }
            }
        )+
    };
}

#[doc(inline)]
pub use impl_trait;

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use crate::{format_buf, PhantomData};

    struct S1 { v: i32 }
    struct S2 { v: i32 }
    struct GS1<T, U> { v1: T, v2: U }
    struct GS2<'a, T, U, V> { v1: T, v2: U, _v3: PhantomData<&'a V> }

    // multiple types without generics
    impl_trait!(fmt::Debug for S1, S2 |self, f| {
        write!(f, "S? {{ v: {:?} }}", self.v)
    });
    // multiple types with different generics
    impl_trait!(fmt::Debug for GS1<T, U> where T, U; GS2<'a, T, U, V> where T, U |self, f| {
        // all sharing the same implementation
        write!(f, "GS? {{ v1: {:?}, v2: {:?} }}", self.v1, self.v2)
    });

    #[test]
    fn impl_non_generic_debug() {
        let s1 = S1 { v: 42 };
        let s2 = S2 { v: 84 };
        let mut buf = [0; 64];
        assert_eq!(format_buf!(&mut buf, "{:?}", s1).unwrap(), "S? { v: 42 }");
        assert_eq!(format_buf!(&mut buf, "{:?}", s2).unwrap(), "S? { v: 84 }");
    }

    #[test]
    fn impl_generic_debug() {
        let g1 = GS1 { v1: "hello", v2: "world" };
        let g2 = GS2 { v1: 3.14, v2: 159, _v3: PhantomData::<&'_ i32> };
        let mut buf = [0; 64];
        assert_eq!(format_buf!(&mut buf, "{:?}", g1).unwrap(), "GS? { v1: \"hello\", v2: \"world\" }");
        assert_eq!(format_buf!(&mut buf, "{:?}", g2).unwrap(), "GS? { v1: 3.14, v2: 159 }");
    }
}
