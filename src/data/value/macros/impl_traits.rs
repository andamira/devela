// devela::data::value::macros::impls
//
//!
//
// TOC
// - impl_data_type!
// - impl_data_value!
// - impl_data_raw!

/// Implements the `DataType` trait.
#[allow(unused_macros)]
macro_rules! impl_data_type {
    (
        v: $Vname:ident, $cbound:ident,
        t: $Tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,
        copy_variants:
            $( $C_name:ident, $C_type:ty ),* ;
        copy_variants_dep:
            $( $C_name_dep:ident, $C_type_dep:ty,
            $C_dep1_dep:literal, $C_dep2_dep:literal ),* ;
        copy_variants_psize:
            $( $C_name_psize:ident, $C_type_psize:ty, $C_psize_psize:meta ),* ;
        copy_variants_psize_dep:
            $( $C_name_psize_dep:ident, $C_type_psize_dep:ty,
            $C_psize_psize_dep:meta, $C_dep1_psize_dep:literal, $C_dep2_psize_dep:literal ),* ;

        noncopy_variants:
            $( $N_name:ident, $N_type:ty ),* ;
        noncopy_variants_dep:
            $( $N_name_dep:ident, $N_type_dep:ty,
            $N_dep1_dep:literal, $N_dep2_dep:literal ),* ;
        noncopy_variants_psize:
            $( $N_name_psize:ident, $N_type_psize:ty, $N_psize_psize:meta ),* ;
        noncopy_variants_psize_dep:
            $( $N_name_psize_dep:ident, $N_type_psize_dep:ty,
            $N_psize_psize_dep:meta, $N_dep1_psize_dep:literal, $N_dep2_psize_dep:literal ),* ;
    ) => { $crate::paste! {
        impl<T: $tbound> $crate::DataType for $Tname<T> {
            type Value = $Vname<T::Value>;

            fn data_value_is_copy(&self) -> bool { $is_copy }

            fn data_value_default(&self) -> Option<Self::Value> {
                match self {
                    $Tname::NoData => Some(Self::Value::NoData),
                    $Tname::Extra(t) => Some(Self::Value::Extra(t.data_value_default()?)),
                    _ => todo![], // TEMP TODO
                }
            }

            fn data_value_align(&self) -> usize {
                match self {
                    $Tname::NoData => align_of::<()>(),
                    $Tname::Extra(t) => t.data_value_align(),

                    $( $Tname::$C_name => align_of::<$C_type>(), )*
                    $( $Tname::$N_name => align_of::<$N_type>(), )*

                    $( // pointer-size dependant
                        #[cfg($C_psize_psize)]
                        $Tname::$C_name_psize => align_of::<$C_type_psize>(),
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_dep,
                                        feature = $C_dep2_dep))))]
                        $Tname::$C_name_dep => align_of::<$C_type_dep>(),
                    )*
                    $(
                        #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_dep,
                                        feature = $N_dep2_dep))))]
                        $Tname::$N_name_dep => align_of::<$N_type_dep>(),
                    )*

                    $( // pointer-size & feature-gated dependencies
                        #[cfg(all($C_psize_psize_dep,
                                feature = $C_dep1_psize_dep,
                                feature = $C_dep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_psize_dep,
                                        feature = $C_dep2_psize_dep))))]
                        $Tname::$C_name_psize_dep => align_of::<$C_type_psize_dep>(),
                    )*
                    $(
                        #[cfg(all($N_psize_psize_dep,
                                feature = $N_dep1_psize_dep,
                                feature = $N_dep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_psize_dep,
                                        feature = $N_dep2_psize_dep))))]
                        $Tname::$N_name_psize_dep => align_of::<$N_type_psize_dep>(),
                    )*
                }
            }
            fn data_value_size(&self) -> usize {
                match self {
                    $Tname::NoData => size_of::<()>(),
                    $Tname::Extra(t) => t.data_value_size(),

                    $( $Tname::$C_name => size_of::<$C_type>(), )*
                    $( $Tname::$N_name => size_of::<$N_type>(), )*

                    $( // pointer-size dependant
                        #[cfg($C_psize_psize)]
                        $Tname::$C_name_psize => size_of::<$C_type_psize>(),
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_dep,
                                        feature = $C_dep2_dep))))]
                        $Tname::$C_name_dep => size_of::<$C_type_dep>(),
                    )*
                    $(
                        #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_dep,
                                        feature = $N_dep2_dep))))]
                        $Tname::$N_name_dep => size_of::<$N_type_dep>(),
                    )*

                    $( // pointer-size & feature-gated dependencies
                        #[cfg(all($C_psize_psize_dep,
                                feature = $C_dep1_psize_dep, feature = $C_dep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_psize_dep,
                                        feature = $C_dep2_psize_dep))))]
                        $Tname::$C_name_psize_dep => size_of::<$C_type_psize_dep>(),
                    )*
                    $(
                        #[cfg(all($N_psize_psize_dep,
                                feature = $N_dep1_psize_dep, feature = $N_dep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_psize_dep,
                                        feature = $N_dep2_psize_dep))))]
                        $Tname::$N_name_psize_dep => size_of::<$N_type_psize_dep>(),
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
        v: $Vname:ident, $cbound:ident,
        t: $Tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,
        copy_variants:
            $( $C_name:ident, $C_type:ty ),* ;
        copy_variants_dep:
            $( $C_name_dep:ident, $C_type_dep:ty,
            $C_dep1_dep:literal, $C_dep2_dep:literal ),* ;
        copy_variants_psize:
            $( $C_name_psize:ident, $C_type_psize:ty, $C_psize_psize:meta ),* ;
        copy_variants_psize_dep:
            $( $C_name_psize_dep:ident, $C_type_psize_dep:ty,
            $C_psize_psize_dep:meta, $C_dep1_psize_dep:literal, $C_dep2_psize_dep:literal ),* ;

        noncopy_variants:
            $( $N_name:ident, $N_type:ty ),* ;
        noncopy_variants_dep:
            $( $N_name_dep:ident, $N_type_dep:ty,
            $N_dep1_dep:literal, $N_dep2_dep:literal ),* ;
        noncopy_variants_psize:
            $( $N_name_psize:ident, $N_type_psize:ty, $N_psize_psize:meta ),* ;
        noncopy_variants_psize_dep:
            $( $N_name_psize_dep:ident, $N_type_psize_dep:ty,
            $N_psize_psize_dep:meta, $N_dep1_psize_dep:literal, $N_dep2_psize_dep:literal ),* ;
    ) => { $crate::paste! {
        impl<V: $cbound> $crate::DataValue for $Vname<V> {
            type Type = $Tname<V::Type>;

            fn data_value_is_copy(&self) -> bool { $is_copy }
            fn data_type(&self) -> Self::Type {
                match self {
                    $Vname::NoData => Self::Type::NoData,
                    $Vname::Extra(t) => Self::Type::Extra(t.data_type()),

                    $( $Vname::$C_name(_) => Self::Type::$C_name, )*
                    $( $Vname::$N_name(_) => Self::Type::$N_name, )*

                    $( // pointer-size dependant
                        #[cfg($C_psize_psize)]
                        $Vname::$C_name_psize(_) => Self::Type::$C_name_psize,
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_dep,
                                        feature = $C_dep2_dep))))]
                        $Vname::$C_name_dep(_) => Self::Type::$C_name_dep,
                    )*
                    $(
                        #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_dep,
                                        feature = $N_dep2_dep))))]
                        $Vname::$N_name_dep(_) => Self::Type::$N_type_dep,
                    )*

                    $( // pointer-size & feature-gated dependencies
                        #[cfg(all($C_psize_psize_dep,
                            feature = $C_dep1_psize_dep,
                            feature = $C_dep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_psize_dep,
                                        feature = $C_dep2_psize_dep))))]
                        $Vname::$C_name_psize_dep(_) => Self::Type::$C_name_psize_dep,
                    )*
                    $(
                        #[cfg(all($N_psize_psize_dep,
                                feature = $N_dep1_psize_dep,
                                feature = $N_dep2_psize_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_psize_dep,
                                        feature = $N_dep2_psize_dep))))]
                        $Vname::$N_name_psize_dep(_) => Self::Type::$N_name_psize_dep,
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
      r: $Rname:ident,
    ) => {
        unsafe impl DataRaw for $Rname {}
    };
}
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
#[allow(unused_imports)]
pub(crate) use impl_data_raw;
