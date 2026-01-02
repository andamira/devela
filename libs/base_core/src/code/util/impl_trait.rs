// devela_base_core::code::util::impl_trait
//
//! Defines [`impl_trait`].
//

#[doc = crate::_TAG_CODE!()]
#[doc = crate::_TAG_CONSTRUCTION!()]
/// A helper macro to concisely implement a few common utility traits.
#[doc = crate::_doc_location!("code/util")]
///
/// ## Traits supported
/// - FromStr
/// - Hash
/// - PartialEq (where other == &Self)
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
/// struct S3<T: Copy, const N: usize>{ v: [T; N] }
///
/// impl_trait![Hash for S0 |self, state| self.0.hash(state)];
///
/// // Note: assumes other == &Self
/// impl_trait![PartialEq for S0 |self, other| self.0 == other.0];
///
/// impl_trait![fmt::Binary for S1[T][T] where T |self, f| self.v.fmt(f)];
/// impl_trait!{fmt::Debug for
///     S1[T][T] where T;
///     S2['a, T]['a, T] where T;
///     S3[T: Copy, const N: usize][T, N] where T |self, f| {
///     write!(f, "S? {{ v: {:?} }}", self.v)
/// }}
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! impl_trait {
    (FromStr<$err:ty> for
    $( $type:ident $([$($decl:tt)+][$($args:tt)+ ])? $(where $($bounded:ident),+ )? );+
     | $s:ident | $expr:expr) => {
        $(
            impl< $($($decl)*)? > $crate::FromStr for $type $(<$($args)*>)?
            $(where $($bounded: $crate::FromStr,)+ )? {
                type Err = $err;
                fn from_str($s: &str) -> $crate::Result<Self, Self::Err> { $expr }
            }
        )+
    };
    (Hash for
     $( $type:ident $([$($decl:tt)+][$($args:tt)+ ])? $(where $($bounded:ident),+ )?);+
      | $self:ident, $state:ident | $expr:expr) => {
        $(
            impl< $($($decl)*)? > $crate::Hash for $type $(<$($args)*>)?
            $(where $($bounded: $crate::Hash,)+ )? {
                fn hash<__H: $crate::Hasher>(&$self, $state: &mut __H) { $expr }
            }
        )+
    };
    (PartialEq for
     $( $type:ident $([$($decl:tt)+][$($args:tt)+ ])? $(where $($bounded:ident),+ )?);+
      | $self:ident, $other:ident | $expr:expr) => {
        $(
            impl< $($($decl)*)? > $crate::PartialEq for $type $(<$($args)*>)?
            $(where $($bounded: $crate::PartialEq,)+ )? {
                fn eq(&$self, $other: &Self) -> bool { $expr }
            }
        )+
    };
    (fmt::$trait:ident for
        $( $type:ident $([$($decl:tt)+][$($args:tt)+ ])? $(where $($bounded:ident),+ )? );+
        | $self:ident, $f:ident | $expr:expr) => {
        $(
            impl< $($($decl)*)? > $crate::$trait for $type $(<$($args)*>)?
            $(where $($bounded: $crate::$trait,)+ )? {
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
    use crate::{format_buf, ParseIntError, PhantomData};

    struct S1 { v: i32 }
    struct S2 { v: i32 }
    struct GS1<T, U> { v1: T, v2: U }
    struct GS2<'a, T, U, V> { v1: T, v2: U, _v3: PhantomData<&'a V> }
    struct CGS3<T: Copy, const N: usize>([T; N]);

    // multiple types without generics
    impl_trait!(FromStr<ParseIntError> for S1; S2 |s| Ok(Self { v : i32::from_str(s)? }) );
    impl_trait!(PartialEq for S1; S2 |self, o| self.v == o.v);
    impl_trait!(Hash for S1; S2 |self, s| self.v.hash(s));
    impl_trait!(fmt::Debug for S1; S2 |self, f| {
        write!(f, "S? {{ v: {:?} }}", self.v)
    });
    // multiple types with different generics
    impl_trait!(fmt::Debug for
        GS1[T, U][T, U] where T, U;
        GS2['a, T, U, V]['a, T, U, V] where T, U |self, f| {
        // all sharing the same implementation
        write!(f, "GS? {{ v1: {:?}, v2: {:?} }}", self.v1, self.v2)
    });
    // types with const-generics
    impl_trait!(fmt::Debug for CGS3[T: Copy, const N: usize][T, N] where T |self, f| {
        write!(f, "CGS3({:?})", self.0)
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
        let g1 = GS1 { v1: "hi", v2: "ho" };
        let g2 = GS2 { v1: 3.14, v2: 159, _v3: PhantomData::<&'_ i32> };
        let cg3 = CGS3(['a'; 2]);
        let mut buf = [0; 64];
        assert_eq!(format_buf!(&mut buf, "{:?}", g1).unwrap(), "GS? { v1: \"hi\", v2: \"ho\" }");
        assert_eq!(format_buf!(&mut buf, "{:?}", g2).unwrap(), "GS? { v1: 3.14, v2: 159 }");
        assert_eq!(format_buf!(&mut buf, "{:?}", cg3).unwrap(), "CGS3(['a', 'a'])");
    }
}
