// devela::data::value::macros::impls
//
//!
//
// TOC
// - impl_data_type
// - impl_data_value
// - impl_data_raw/

/// Implements the `DataType` trait.
#[allow(unused_macros)]
macro_rules! impl_data_type {
    (
        v: $cname:ident, $cbound:ident,
        t: $tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,
        copy_variants:
            $( $cvname:ident, $cvtype:ty ),* ;
        copy_variants_dep:
            $( $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize:
            $( $cvname_psize:ident, $cvtype_psize:ty, $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep:
            $( $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
        noncopy_variants:
            $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep:
            $( $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),* ;
        noncopy_variants_psize_dep:
            $( $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),* ;
    ) => { $crate::paste! {
        impl<T: $tbound> $crate::DataType for $tname<T> {
            type Value = $cname<T::Value>;

            fn data_value_is_copy(&self) -> bool { $is_copy }

            fn data_value_default(&self) -> Option<Self::Value> {
                match self {
                    $tname::NoData => Some(Self::Value::NoData),
                    $tname::Extra(t) => Some(Self::Value::Extra(t.data_value_default()?)),

                    // TODO: have to pass it as an argument for each type
                    // $( $tname::$cvname =>
                    //     if $def { Some(Self::Type::$cvname($cvtype::default())) } else { None },
                    // )*

                    _ => todo![], // TEMP
                }
            }

            fn data_value_align(&self) -> usize {
                match self {
                    $tname::NoData => align_of::<()>(),
                    $tname::Extra(t) => t.data_value_align(),

                    $( $tname::$cvname => align_of::<$cvtype>(), )*
                    $( $tname::$vname => align_of::<$vtype>(), )*

                    $( // pointer-size dependant
                        #[cfg($cvpsize_psize)]
                        $tname::$cvname_psize => align_of::<$cvtype_psize>(),
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))))]
                        $tname::$cvname_dep => align_of::<$cvtype_dep>(),
                    )*
                    $(
                        #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))))]
                        $tname::$vname_dep => align_of::<$vtype_dep>(),
                    )*

                    $( // pointer-size & feature-gated dependencies
                        #[cfg(all($cvpsize_psize_dep,
                                feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))))]
                        $tname::$cvname_psize_dep => align_of::<$cvtype_psize_dep>(),
                    )*
                    $(
                        #[cfg(all($vpsize_psize_dep,
                                feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))))]
                        $tname::$vname_psize_dep => align_of::<$vtype_psize_dep>(),
                    )*
                }
            }
            fn data_value_size(&self) -> usize {
                match self {
                    $tname::NoData => size_of::<()>(),
                    $tname::Extra(t) => t.data_value_size(),

                    $( $tname::$cvname => size_of::<$cvtype>(), )*
                    $( $tname::$vname => size_of::<$vtype>(), )*

                    $( // pointer-size dependant
                        #[cfg($cvpsize_psize)]
                        $tname::$cvname_psize => size_of::<$cvtype_psize>(),
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))))]
                        $tname::$cvname_dep => size_of::<$cvtype_dep>(),
                    )*
                    $(
                        #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))))]
                        $tname::$vname_dep => size_of::<$vtype_dep>(),
                    )*

                    $( // pointer-size & feature-gated dependencies
                        #[cfg(all($cvpsize_psize_dep,
                                feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))))]
                        $tname::$cvname_psize_dep => size_of::<$cvtype_psize_dep>(),
                    )*
                    $(
                        #[cfg(all($vpsize_psize_dep,
                                feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))))]
                        $tname::$vname_psize_dep => size_of::<$vtype_psize_dep>(),
                    )*
                }
            }
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use impl_data_type;

/// Implements the [`DataValue`] trait.
#[allow(unused_macros)]
macro_rules! impl_data_value {
    (
        v: $cname:ident, $cbound:ident,
        t: $tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,
        copy_variants:
            $( $cvname:ident, $cvtype:ty ),* ;
        copy_variants_dep:
            $( $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize:
            $( $cvname_psize:ident, $cvtype_psize:ty, $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep:
            $( $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
        noncopy_variants:
            $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep:
            $( $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),* ;
        noncopy_variants_psize_dep:
            $( $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),* ;
    ) => { $crate::paste! {
        impl<V: $cbound> $crate::DataValue for $cname<V> {
            type Type = $tname<V::Type>;

            fn data_value_is_copy(&self) -> bool { $is_copy }
            fn data_type(&self) -> Self::Type {
                match self {
                    $cname::NoData => Self::Type::NoData,
                    $cname::Extra(t) => Self::Type::Extra(t.data_type()),

                    $( $cname::$cvname(_) => Self::Type::$cvname, )*
                    $( $cname::$vname(_) => Self::Type::$vname, )*

                    $( // pointer-size dependant
                        #[cfg($cvpsize_psize)]
                        $cname::$cvname_psize(_) => Self::Type::$cvname_psize,
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))))]
                        $cname::$cvname_dep(_) => Self::Type::$cvname_dep,
                    )*
                    $(
                        #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))))]
                        $cname::$vname_dep(_) => Self::Type::$vtype_dep,
                    )*

                    $( // pointer-size & feature-gated dependencies
                        #[cfg(all($cvpsize_psize_dep,
                            feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))))]
                        $cname::$cvname_psize_dep(_) => Self::Type::$cvname_psize_dep,
                    )*
                    $(
                        #[cfg(all($vpsize_psize_dep,
                                feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))))]
                        $cname::$vname_psize_dep(_) => Self::Type::$vname_psize_dep,
                    )*
                }
            }
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use impl_data_value;

/// Implements the `DataRaw` trait.
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
#[allow(unused_macros)]
macro_rules! impl_data_raw {
    (
      b: $bname:ident,
    ) => {
        unsafe impl DataRaw for $bname {}
    };
}
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
#[allow(unused_imports)]
pub(crate) use impl_data_raw;
