// devela::data::value::macros::impls
//
//! Implements the traits: `DataValue`*, `DataType`*, `DataRaw`*.
//
// TOC
// - impl_data_value!
// - impl_data_type!
// - impl_data_raw!

/// Implements the [`DataValue`] trait.
#[allow(unused_macros)]
macro_rules! impl_data_value {
    (
        v: $Vname:ident, $cbound:ident,
        t: $Tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,

        copy:
            $( $C_name:ident, $C_type:ty
            ),* ;
        copy@dep:
            $( $C_name_dep:ident, $C_type_dep:ty,
               $C_dep1_dep:literal, $C_dep2_dep:literal
            ),* ;
        copy@ptr:
            $( $C_name_ptr:ident, $C_type_ptr:ty,
               $C_ptr_ptr:meta
            ),* ;
        copy@ptrdep:
            $( $C_name_ptrdep:ident, $C_type_ptrdep:ty,
               $C_ptr_ptrdep:meta,
               $C_dep1_ptrdep:literal, $C_dep2_ptrdep:literal
            ),* ;

        noncopy:
            $( $N_name:ident, $N_type:ty
            ),* ;
        noncopy@dep:
            $( $N_name_dep:ident, $N_type_dep:ty,
               $N_dep1_dep:literal, $N_dep2_dep:literal
            ),* ;
        noncopy@ptr:
            $( $N_name_ptr:ident, $N_type_ptr:ty,
               $N_ptr_ptr:meta
            ),* ;
        noncopy@ptrdep:
            $( $N_name_ptrdep:ident, $N_type_ptrdep:ty,
               $N_ptr_ptrdep:meta,
               $N_dep1_ptrdep:literal, $N_dep2_ptrdep:literal
            ),* ;
    ) => { $crate::paste! {
        impl<V: $cbound> $crate::DataValue for $Vname<V> {
            type Type = $Tname<V::Type>;

            fn data_value_is_copy(&self) -> bool { $is_copy }
            fn data_type(&self) -> Self::Type {
                match self {
                    $Vname::None => Self::Type::None,
                    $Vname::Extra(t) => Self::Type::Extra(t.data_type()),

                    $( $Vname::$C_name(_) => Self::Type::$C_name, )*
                    $( $Vname::$N_name(_) => Self::Type::$N_name, )*

                    $( // pointer-size dependant
                        #[cfg($C_ptr_ptr)]
                        $Vname::$C_name_ptr(_) => Self::Type::$C_name_ptr,
                    )* $(
                        #[cfg($N_ptr_ptr)]
                        $Vname::$N_name_ptr(_) => Self::Type::$N_name_ptr,
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_dep,
                                        feature = $C_dep2_dep))))]
                        $Vname::$C_name_dep(_) => Self::Type::$C_name_dep,
                    )* $(
                        #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_dep,
                                        feature = $N_dep2_dep))))]
                        $Vname::$N_name_dep(_) => Self::Type::$N_type_dep,
                    )*

                    $( // pointer-size & feature-gated dependencies
                        #[cfg(all($C_ptr_ptrdep,
                            feature = $C_dep1_ptrdep,
                            feature = $C_dep2_ptrdep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_ptrdep,
                                        feature = $C_dep2_ptrdep))))]
                        $Vname::$C_name_ptrdep(_) => Self::Type::$C_name_ptrdep,
                    )* $(
                        #[cfg(all($N_ptr_ptrdep,
                                feature = $N_dep1_ptrdep,
                                feature = $N_dep2_ptrdep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_ptrdep,
                                        feature = $N_dep2_ptrdep))))]
                        $Vname::$N_name_ptrdep(_) => Self::Type::$N_name_ptrdep,
                    )*
                }
            }
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use impl_data_value;

/// Implements the [`DataType`] trait.
#[allow(unused_macros)]
macro_rules! impl_data_type {
    (
        v: $Vname:ident, $cbound:ident,
        t: $Tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,

        copy:
            $( $C_name:ident, $C_type:ty,
               [def:$C_def:stmt]
            )* ;
        copy@dep:
            $( $C_name_dep:ident, $C_type_dep:ty,
               $C_dep1_dep:literal, $C_dep2_dep:literal,
               [def:$C_def_dep:stmt]
            )* ;
        copy@ptr:
            $( $C_name_ptr:ident, $C_type_ptr:ty,
               $C_ptr_ptr:meta,
               [def:$C_def_ptr:stmt]
            )* ;
        copy@ptrdep:
            $( $C_name_ptrdep:ident, $C_type_ptrdep:ty,
               $C_ptr_ptrdep:meta,
               $C_dep1_ptrdep:literal, $C_dep2_ptrdep:literal,
               [def:$C_def_ptrdep:stmt]
            )* ;

        noncopy:
            $( $N_name:ident, $N_type:ty,
               [def:$N_def:stmt]
            )* ;
        noncopy@dep:
            $( $N_name_dep:ident, $N_type_dep:ty,
               $N_dep1_dep:literal, $N_dep2_dep:literal,
               [def:$N_def_dep:stmt]
            )* ;
        noncopy@ptr:
            $( $N_name_ptr:ident, $N_type_ptr:ty,
               $N_ptr_ptr:meta,
               [def:$N_def_ptr:stmt]
            )* ;
        noncopy@ptrdep:
            $( $N_name_ptrdep:ident, $N_type_ptrdep:ty,
               $N_ptr_ptrdep:meta,
               $N_dep1_ptrdep:literal, $N_dep2_ptrdep:literal,
               [def:$N_def_ptrdep:stmt]
            )* ;
    ) => { $crate::paste! {
        impl<T: $tbound> $crate::DataType for $Tname<T> {
            type Value = $Vname<T::Value>;

            fn data_value_is_copy(&self) -> bool { $is_copy }

            fn data_value_default(&self) -> Option<Self::Value> {
                match self {
                    $Tname::None => Some(Self::Value::None),
                    $Tname::Extra(t) => Some(Self::Value::Extra(t.data_value_default()?)),

                    $(
                        $Tname::$C_name =>
                            $crate::maybe![default: $C_def, $C_type]
                                .map(Self::Value::$C_name),
                    )* $(
                        $Tname::$N_name =>
                            $crate::maybe![default: $N_def, $N_type]
                                .map(Self::Value::$N_name),
                    )*

                    $(  // pointer-size dependant
                        #[cfg($C_ptr_ptr)]
                        $Tname::$C_name_ptr =>
                            $crate::maybe![default: $C_def_ptr, $C_type_ptr]
                                .map(Self::Value::$C_name_ptr),
                    )* $(
                        #[cfg($N_ptr_ptr)]
                        $Tname::$N_name_ptr =>
                           $crate::maybe![default: $N_def_ptr, $N_type_ptr]
                           .map(Self::Value::$N_name_ptr),
                    )*

                    $(  // feature-gated dependencies
                        #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_dep,
                                        feature = $C_dep2_dep))))]
                        $Tname::$C_name_dep =>
                            $crate::maybe![default: $C_def_dep, $C_type_dep]
                                .map(Self::Value::$C_name_dep),
                    )* $(
                        #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_dep,
                                        feature = $N_dep2_dep))))]
                        $Tname::$N_name_dep =>
                            $crate::maybe![default: $N_def_dep, $N_type_dep]
                                .map(Self::Value::$N_name_dep),
                    )*

                    $(  // pointer-size & feature-gated dependencies
                        #[cfg(all($C_ptr_ptrdep,
                                feature = $C_dep1_ptrdep,
                                feature = $C_dep2_ptrdep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_ptrdep,
                                        feature = $C_dep2_ptrdep))))]
                        $Tname::$C_name_ptrdep =>
                            $crate::maybe![default: $C_def_ptrdep, $C_type_ptrdep]
                                .map(Self::Value::$C_name_ptrdep),
                    )* $(
                        #[cfg(all($N_ptr_ptrdep,
                                feature = $N_dep1_ptrdep,
                                feature = $N_dep2_ptrdep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_ptrdep,
                                        feature = $N_dep2_ptrdep))))]
                        $Tname::$N_name_ptrdep =>
                            $crate::maybe![default: $N_def_ptrdep, $N_type_ptrdep]
                                .map(Self::Value::$N_name_ptrdep),
                    )*
                }
            }

            fn data_value_align(&self) -> usize {
                match self {
                    $Tname::None => align_of::<()>(),
                    $Tname::Extra(t) => t.data_value_align(),

                    $( $Tname::$C_name => align_of::<$C_type>(), )*
                    $( $Tname::$N_name => align_of::<$N_type>(), )*

                    $(  // pointer-size dependant
                        #[cfg($C_ptr_ptr)]
                        $Tname::$C_name_ptr => align_of::<$C_type_ptr>(),
                    )* $(
                        #[cfg($N_ptr_ptr)]
                        $Tname::$N_name_ptr => align_of::<$N_type_ptr>(),
                    )*

                    $(  // feature-gated dependencies
                        #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_dep,
                                        feature = $C_dep2_dep))))]
                        $Tname::$C_name_dep => align_of::<$C_type_dep>(),
                    )* $(
                        #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_dep,
                                        feature = $N_dep2_dep))))]
                        $Tname::$N_name_dep => align_of::<$N_type_dep>(),
                    )*

                    $(  // pointer-size & feature-gated dependencies
                        #[cfg(all($C_ptr_ptrdep,
                                feature = $C_dep1_ptrdep,
                                feature = $C_dep2_ptrdep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_ptrdep,
                                        feature = $C_dep2_ptrdep))))]
                        $Tname::$C_name_ptrdep => align_of::<$C_type_ptrdep>(),
                    )* $(
                        #[cfg(all($N_ptr_ptrdep,
                                feature = $N_dep1_ptrdep,
                                feature = $N_dep2_ptrdep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_ptrdep,
                                        feature = $N_dep2_ptrdep))))]
                        $Tname::$N_name_ptrdep => align_of::<$N_type_ptrdep>(),
                    )*
                }
            }
            fn data_value_size(&self) -> usize {
                match self {
                    $Tname::None => size_of::<()>(),
                    $Tname::Extra(t) => t.data_value_size(),

                    $( $Tname::$C_name => size_of::<$C_type>(), )*
                    $( $Tname::$N_name => size_of::<$N_type>(), )*

                    $(  // pointer-size dependant
                        #[cfg($C_ptr_ptr)]
                        $Tname::$C_name_ptr => size_of::<$C_type_ptr>(),
                    )* $(
                        #[cfg($N_ptr_ptr)]
                        $Tname::$N_name_ptr => size_of::<$N_type_ptr>(),
                    )*

                    $(  // feature-gated dependencies
                        #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_dep,
                                        feature = $C_dep2_dep))))]
                        $Tname::$C_name_dep => size_of::<$C_type_dep>(),
                    )* $(
                        #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_dep,
                                        feature = $N_dep2_dep))))]
                        $Tname::$N_name_dep => size_of::<$N_type_dep>(),
                    )*

                    $(  // pointer-size & feature-gated dependencies
                        #[cfg(all($C_ptr_ptrdep,
                                feature = $C_dep1_ptrdep, feature = $C_dep2_ptrdep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_ptrdep,
                                        feature = $C_dep2_ptrdep))))]
                        $Tname::$C_name_ptrdep => size_of::<$C_type_ptrdep>(),
                    )* $(
                        #[cfg(all($N_ptr_ptrdep,
                                feature = $N_dep1_ptrdep, feature = $N_dep2_ptrdep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $N_dep1_ptrdep,
                                        feature = $N_dep2_ptrdep))))]
                        $Tname::$N_name_ptrdep => size_of::<$N_type_ptrdep>(),
                    )*
                }
            }
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use impl_data_type;

/// Implements the [`DataRaw`] trait.
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
