// devela::data::value::macros::define_each
//
//!
//
// TOC
// - define_data_value!
// - define_data_type!
// - define_data_raw!

/// for defining enum DataValue*
#[allow(unused_macros)]
macro_rules! define_data_value {
    (
        v: $Vname:ident, t: $Tname:ident, r: $Rname:ident,
        size: $B:literal, $b:literal,
        feature: $feature:literal,

        copy_variants:
            $( $C_doc:literal, $C_name:ident, $C_type:ty, )* ;
        copy_variants_dep:
            $( $C_doc_dep:literal, $C_name_dep:ident, $C_type_dep:ty,
            $C_dep1_dep:literal, $C_dep2_dep:literal, )* ;
        copy_variants_psize:
            $( $C_doc_psize:literal, $C_name_psize:ident, $C_type_psize:ty, $C_psize_psize:meta, )* ;
        copy_variants_psize_dep:
            $( $C_doc_psize_dep:literal, $C_name_psize_dep:ident, $C_type_psize_dep:ty,
            $C_psize_psize_dep:meta, $C_dep1_psize_dep:literal, $C_dep2_psize_dep:literal, )* ;
        noncopy_variants:
            $( $N_doc:literal, $N_name:ident, $N_type:ty, )* ;
        noncopy_variants_dep:
            $( $N_doc_dep:literal, $N_name_dep:ident, $N_type_dep:ty,
            $N_dep1_dep:literal, $N_dep2_dep:literal, )* ;
        noncopy_variants_psize:
            $( $N_doc_psize:literal, $N_name_psize:ident, $N_type_psize:ty,
            $N_psize_psize:meta, $N_dep1_psize:literal, $N_dep2_psize:literal, )* ;
        noncopy_variants_psize_dep:
            $( $N_doc_psize_dep:literal, $N_name_psize_dep:ident, $N_type_psize_dep:ty,
            $N_psize_psize_dep:meta, $N_dep1_psize_dep:literal, $N_dep2_psize_dep:literal, )* ;
    ) => { $crate::paste! {
        // ## copy version (DataValue)
        // -----------------------------------------------------------------
        #[doc = $b "-bit data *value*, restricted to `Copy` variants, with extra `V`." ]
        ///
        /// See also:
        #[doc = "- [" [<$Tname $b Copy With>] "]" ]
        #[doc = "- [" [<$Vname $b With>] "][" [<$Vname $b With>] "] -Copy" ]
        #[doc = "- [" [<$Vname $b Copy>] "][" [<$Vname $b Copy>] "] -With" ]
        #[doc = "- [" [<$Vname $b>] "][" [<$Vname $b>] "] -Copy -With" ]
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub enum [<$Vname $b Copy With>]<V: DataValue> {
            /// No data.
            NoData,
            /// Extra *data values*.
            Extra(V),

            $( // fundamental types
                #[doc = $C_doc]
                $C_name($C_type),
            )*

            $( // pointer-size dependant
                #[cfg($C_psize_psize)]
                #[doc = $C_doc_psize]
                $C_name_psize($C_type_psize),
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))))]
                #[doc = $C_doc_dep]
                $C_name_dep($C_type_dep),
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($C_psize_psize_dep,
                        feature = $C_dep1_psize_dep,
                        feature = $C_dep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_psize_dep, feature = $C_dep2_psize_dep))))]
                #[doc = $C_doc_psize_dep]
                $C_name_psize_dep($C_type_psize_dep),
            )*
        }

        // alias DataValue Copy
        #[doc = $b "-bit data *value*, restricted to `Copy` variants." ]
        ///
        /// See also:
        #[doc = "- [" [<$Tname $b Copy>] "]" ]
        #[doc = "- [" [<$Vname $b>] "][" [<$Vname $b>] "] -Copy" ]
        #[doc = "- [" [<$Vname $b Copy With>] "][" [<$Vname $b Copy With>] "] +With" ]
        #[doc = "- [" [<$Vname $b With>] "][" [<$Vname $b With>] "] -Copy +With" ]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub type [<$Vname $b Copy>] = [< $Vname $b With>]<$crate::NoData>;

        // implement the DataValue trait
        $crate::impl_data_value![
            v: [< $Vname $b Copy With >], DataValue,
            t: [< $Tname $b Copy With >], DataType,
            is_copy: true,
            copy_variants:
                $( $C_name, $C_type ),* ;
            copy_variants_dep:
                $( $C_name_dep, $C_type_dep, $C_dep1_dep, $C_dep2_dep ),* ;
            copy_variants_psize:
                $( $C_name_psize, $C_type_psize, $C_psize_psize ),* ;
            copy_variants_psize_dep:
                $( $C_name_psize_dep, $C_type_psize_dep, $C_psize_psize_dep,
                $C_dep1_psize_dep, $C_dep2_psize_dep ),* ;
            noncopy_variants: ;
            noncopy_variants_dep: ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep: ;
        ];
        impl<V: DataValueCopy> DataValueCopy for [< $Vname $b Copy With >]<V> { }

        // ## non-copy version (DataValue)
        // -----------------------------------------------------------------
        #[doc = $b "-bit data *value*, with extra `V`." ]
        ///
        /// See also:
        #[doc = "- [" [<$Tname $b With>] "]" ]
        #[doc = "- [" [<$Vname $b Copy With>] "][" [<$Vname $b Copy With>] "] +Copy" ]
        #[doc = "- [" [<$Vname $b>] "][" [<$Vname $b>] "] -Width" ]
        #[doc = "- [" [<$Vname $b Copy>] "][" [<$Vname $b Copy>] "] +Copy -Width" ]
        #[derive(Debug)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub enum [<$Vname $b With>]<V: DataValue> {
            /// No data.
            NoData,
            /// Extra *data values*.
            Extra(V),

            $( // fundamental types
                #[doc = $C_doc]
                $C_name($C_type),
            )*
            $(
                #[doc = $N_doc]
                $N_name($N_type),
            )*

            $( // pointer-size dependant
                #[cfg($C_psize_psize)]
                #[doc = $C_doc_psize]
                $C_name_psize($C_type_psize),
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))))]
                #[doc = $C_doc_dep]
                $C_name_dep($C_type_dep),
            )*
            $(
                #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))))]
                #[doc = $N_doc_dep]
                $N_name_dep($N_type_dep),
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($C_psize_psize_dep,
                        feature = $C_dep1_psize_dep,
                        feature = $C_dep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_psize_dep, feature = $C_dep2_psize_dep))))]
                #[doc = $C_doc_psize_dep]
                $C_name_psize_dep($C_type_psize_dep),
            )*
            $(
                #[cfg(all($N_psize_psize_dep,
                        feature = $N_dep1_psize_dep,
                        feature = $N_dep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $N_dep1_psize_dep, feature = $N_dep2_psize_dep))))]
                #[doc = $N_doc_psize_dep]
                $N_name_psize_dep($N_type_psize_dep),
            )*
        }

        // alias DataValue
        #[doc = $b "-bit data *value*." ]
        ///
        /// See also:
        #[doc = "- [" [<$Tname $b>] "]" ]
        #[doc = "- [" [<$Vname $b Copy>] "][" [<$Vname $b Copy>] "] +Copy" ]
        #[doc = "- [" [<$Vname $b With>] "][" [<$Vname $b With>] "] +With" ]
        #[doc = "- [" [<$Vname $b Copy With>] "][" [<$Vname $b Copy With>] "] +Copy +With" ]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub type [<$Vname $b>] = [< $Vname $b With>]<$crate::NoData>;

        // implement the DataValue trait
        $crate::impl_data_value![
            v: [< $Vname $b With >], DataValue,
            t: [< $Tname $b With >], DataType,
            is_copy: false,
            copy_variants:
                $( $C_name, $C_type ),* ;
            copy_variants_dep:
                $( $C_name_dep, $C_type_dep, $C_dep1_dep, $C_dep2_dep ),* ;
            copy_variants_psize:
                $( $C_name_psize, $C_type_psize, $C_psize_psize ),* ;
            copy_variants_psize_dep:
                $( $C_name_psize_dep, $C_type_psize_dep, $C_psize_psize_dep,
                $C_dep1_psize_dep, $C_dep2_psize_dep ),* ;
            noncopy_variants:
                $($N_name, $N_type ),* ;
            noncopy_variants_dep:
                $( $N_name_dep, $N_type_dep, $N_dep1_dep, $N_dep2_dep ),* ;
            noncopy_variants_psize:
                $( $N_name_psize, $N_type_psize, $N_psize_psize ),* ;
            noncopy_variants_psize_dep:
                $( $N_name_psize_dep, $N_type_psize_dep, $N_psize_psize_dep,
                $N_dep1_psize_dep, $N_dep2_psize_dep ),* ;
        ];

        // implement `TryFrom`<`DataValue`> for *contained-value*:

        $( // Copy
            impl<V: DataValueCopy> TryFrom<[<$Vname $b Copy With>]<V>> for $C_type {
                type Error = ();
                fn try_from(v: [<$Vname $b Copy With>]<V>) -> Result<Self, Self::Error> {
                    match v {
                        [<$Vname $b Copy With>]::$C_name(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        $( // Copy feature-bound
            #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep ))]
            impl<V: DataValue> TryFrom<[<$Vname $b With>]<V>> for $C_type_dep {
                type Error = ();
                fn try_from(v: [<$Vname $b With>]<V>) -> Result<Self, Self::Error> {
                    match v {
                        [<$Vname $b With>]::$C_name_dep(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        $( // non-Copy
            impl<V: DataValue> TryFrom<[<$Vname $b With>]<V>> for $N_type {
                type Error = ();
                fn try_from(v: [<$Vname $b With>]<V>) -> Result<Self, Self::Error> {
                    match v {
                        [<$Vname $b With>]::$N_name(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        $( // non-Copy feature-bound
            #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep ))]
            impl<V: DataValue> TryFrom<[<$Vname $b With>]<V>> for $N_type_dep {
                type Error = ();
                fn try_from(v: [<$Vname $b With>]<V>) -> Result<Self, Self::Error> {
                    match c {
                        [<$Vname $b With>]::$N_name_dep(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*

        // implement `From`<*contained-value*> for `DataValue`:

        $( // Copy
            impl<V: DataValueCopy> From<$C_type> for [<$Vname $b Copy With>]<V> {
                fn from(v: $C_type) -> Self {
                    [<$Vname $b Copy With>]::$C_name(v)
                }
            }
        )*
        $( // Copy feature-bound
            #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep ))]
            impl<V: DataValueCopy> From<$C_type_dep> for [<$Vname $b Copy With>]<V> {
                fn from(v: $C_type_dep) -> Self {
                    [<$Vname $b Copy With>]::$C_name_dep(v)
                }
            }
        )*
        $( // non-Copy
            impl<V: DataValue> From<$N_type> for [<$Vname $b With>]<V> {
                fn from(v: $N_type) -> Self {
                    [<$Vname $b With>]::$N_name(v)
                }
            }
        )*
        $( // non-Copy feature-bound
            #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep ))]
            impl<V: DataValue> From<$N_type_dep> for [<$Vname $b With>]<V> {
                fn from(v: $N_type_dep) -> Self {
                    [<$Vname $b With>]::$N_name_dep(v)
                }
            }
        )*

        // from DataValue to DataRaw
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
        impl<V: DataValueCopy> From<[<$Vname $b Copy With>]<V>> for $crate::[<$Rname $b Copy>] {
            fn from(cell: [<$Vname $b Copy With>]<V>) -> Self {
                match cell {
                    [<$Vname $b Copy With>]::NoData => Self { NoData: () },
                    [<$Vname $b Copy With>]::Extra(_) => Self { NoData: () },

                    $( // fundamental types
                        [<$Vname $b Copy With>]::$C_name(v) => Self { $C_name: v },
                    )*

                    $( // pointer-size dependant
                        #[cfg($C_psize_psize)]
                        [<$Vname $b Copy With>]::$C_name_psize(v) => Self { $C_name_psize: v },
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))))]
                        [<$Vname $b Copy With>]::$C_name_dep(v) => Self { $C_name_dep: v },
                    )*
                }
            }
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use define_data_value;
/// for defining enum DataType*
#[allow(unused_macros)]
macro_rules! define_data_type {
    (
        v: $Vname:ident, t: $Tname:ident, r: $Rname:ident,
        size: $B:literal, $b:literal,
        feature: $feature:literal,

        copy_variants:
            $( $C_doc:literal, $C_name:ident, $C_type:ty, )* ;
        copy_variants_dep:
            $( $C_doc_dep:literal, $C_name_dep:ident, $C_type_dep:ty,
            $C_dep1_dep:literal, $C_dep2_dep:literal, )* ;
        copy_variants_psize:
            $( $C_doc_psize:literal, $C_name_psize:ident, $C_type_psize:ty,
            $C_psize_psize:meta, )* ;
        copy_variants_psize_dep:
            $( $C_doc_psize_dep:literal, $C_name_psize_dep:ident, $C_type_psize_dep:ty,
            $C_psize_psize_dep:meta, $C_dep1_psize_dep:literal, $C_dep2_psize_dep:literal, )* ;
        noncopy_variants:
            $( $N_doc:literal, $N_name:ident, $N_type:ty, )* ;
        noncopy_variants_dep:
            $( $N_doc_dep:literal, $N_name_dep:ident, $N_type_dep:ty,
            $N_dep1_dep:literal, $N_dep2_dep:literal, )* ;
        noncopy_variants_psize:
            $( $N_doc_psize:literal, $N_name_psize:ident, $N_type_psize:ty,
            $N_psize_psize:meta, $N_dep1_psize:literal, $N_dep2_psize:literal, )* ;
        noncopy_variants_psize_dep:
            $( $N_doc_psize_dep:literal, $N_name_psize_dep:ident, $N_type_psize_dep:ty,
            $N_psize_psize_dep:meta, $N_dep1_psize_dep:literal, $N_dep2_psize_dep:literal, )* ;
    ) =>  { $crate::paste! {
        // ## copy version (DataType)
        // -----------------------------------------------------------------
        #[doc = $b "-bit data *type*, restricted to `Copy` variants, with extra `T`." ]
        ///
        /// See also:
        #[doc = "- [" [<$Vname $b Copy With>] "]" ]
        #[doc = "- [" [<$Tname $b With>]  "][" [<$Tname $b With>] "] -Copy" ]
        #[doc = "- [" [<$Tname $b Copy>]  "][" [<$Tname $b Copy>] "] -With" ]
        #[doc = "- [" [<$Tname $b>]  "][" [<$Tname $b>] "] -Copy -With" ]
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub enum [< $Tname $b Copy With >]<T: DataType> {
            /// No data.
            NoData,
            /// A custom *data type* extension.
            Extra(T),

            $( // fundamental types
                #[doc = $C_doc ]
                $C_name,
            )*

            $( // pointer-size dependant
                #[cfg($C_psize_psize)]
                #[doc = $C_doc_psize]
                $C_name_psize,
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))))]
                #[doc = $C_doc_dep]
                $C_name_dep,
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($C_psize_psize_dep, feature = $C_dep1_psize_dep, feature = $C_dep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_psize_dep, feature = $C_dep2_psize_dep))))]
                #[doc = $C_doc_psize_dep]
                $C_name_psize_dep,
            )*
        }

        // DataType Copy (-With) alias
        #[doc = $b "-bit data *type*, restricted to `Copy` variants." ]
        ///
        /// See also:
        #[doc = "- [" [<$Vname $b Copy>] "]" ]
        #[doc = "- [" [<$Tname $b>] "][" [<$Tname $b>] "] -Copy" ]
        #[doc = "- [" [<$Tname $b Copy With>] "][" [<$Tname $b Copy With>] "] +With" ]
        #[doc = "- [" [<$Tname $b With>] "][" [<$Tname $b With>] "] -Copy +With" ]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub type [<$Tname $b Copy>] = [< $Tname $b Copy With>]<$crate::NoData>;

        // implement the DataType trait
        $crate::impl_data_type![
            v: [< $Vname $b Copy With >], DataValue,
            t: [< $Tname $b Copy With >], DataType,
            is_copy: true,
            copy_variants:
                $( $C_name, $C_type ),* ;
            copy_variants_dep:
                $( $C_name_dep, $C_type_dep, $C_dep1_dep, $C_dep2_dep ),* ;
            copy_variants_psize:
                $( $C_name_psize, $C_type_psize, $C_psize_psize ),* ;
            copy_variants_psize_dep:
                $( $C_name_psize_dep, $C_type_psize_dep, $C_psize_psize_dep,
                $C_dep1_psize_dep, $C_dep2_psize_dep ),* ;
            noncopy_variants: ;
            noncopy_variants_dep: ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep: ;
        ];
        impl<T: DataTypeCopy> DataTypeCopy for [< $Tname $b Copy With >]<T>
            where T::Value: DataValueCopy {}

        // ## non-copy version (DataType)
        // -----------------------------------------------------------------
        #[doc = $b "-bit data *type*, with extra `T`." ]
        ///
        /// See also:
        #[doc = "- [" [<$Vname $b With>] "]" ]
        #[doc = "- [" [<$Tname $b Copy With>] "][" [<$Tname $b Copy With>] "] +Copy" ]
        #[doc = "- [" [<$Tname $b>] "][" [<$Tname $b>] "] -With" ]
        #[doc = "- [" [<$Tname $b Copy>] "][" [<$Tname $b Copy>] "] +Copy -With" ]
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub enum [< $Tname $b With >]<T: DataType> {
            /// No data.
            NoData,
            /// A custom *data type* extension.
            Extra(T),

            $( // fundamental types
                #[doc = $C_doc ]
                $C_name,
            )*
            $(
                #[doc = $N_doc ]
                $N_name,
            )*

            $( // pointer-size dependant
                #[cfg($C_psize_psize)]
                #[doc = $C_doc_psize]
                $C_name_psize,
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))))]
                #[doc = $C_doc_dep]
                $C_name_dep,
            )*
            $(
                #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))))]
                #[doc = $N_doc_dep]
                $N_name_dep,
            )*
            $( // pointer-size & feature-gated dependencies
                #[cfg(all($C_psize_psize_dep,
                        feature = $C_dep1_psize_dep,
                        feature = $C_dep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_psize_dep, feature = $C_dep2_psize_dep))))]
                #[doc = $C_doc_psize_dep]
                $C_name_psize_dep,
            )*
            $(
                #[cfg(all($N_psize_psize_dep,
                        feature = $N_dep1_psize_dep,
                        feature = $N_dep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $N_dep1_psize_dep, feature = $N_dep2_psize_dep))))]
                #[doc = $N_doc_psize_dep]
                $N_name_psize_dep,
            )*
        }

        // DataType (-With) alias
        #[doc = $b "-bit data *type*"]
        ///
        /// See also:
        #[doc = "- [" [<$Vname $b>] "]" ]
        #[doc = "- [" [<$Tname $b Copy>] "][" [<$Tname $b Copy>] "] +Copy" ]
        #[doc = "- [" [<$Tname $b With>] "][" [<$Tname $b With>] "] +With" ]
        #[doc = "- [" [<$Tname $b Copy With>] "][" [<$Tname $b Copy With>] "] +Copy +With" ]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub type [<$Tname $b>] = [< $Tname $b With>]<$crate::NoData>;

        // implement the DataType trait
        $crate::impl_data_type![
            v: [< $Vname $b With >], DataValue,
            t: [< $Tname $b With >], DataType,
            is_copy: false,
            copy_variants:
                $( $C_name, $C_type ),* ;
            copy_variants_dep:
                $( $C_name_dep, $C_type_dep, $C_dep1_dep, $C_dep2_dep ),* ;
            copy_variants_psize:
                $( $C_name_psize, $C_type_psize, $C_psize_psize ),* ;
            copy_variants_psize_dep:
                $( $C_name_psize_dep, $C_type_psize_dep, $C_psize_psize_dep,
                $C_dep1_psize_dep, $C_dep2_psize_dep ),* ;
            noncopy_variants:
                $($N_name, $N_type ),* ;
            noncopy_variants_dep:
                $( $N_name_dep, $N_type_dep, $N_dep1_dep, $N_dep2_dep ),* ;
            noncopy_variants_psize:
                $( $N_name_psize, $N_type_psize, $N_psize_psize ),* ;
            noncopy_variants_psize_dep:
                $( $N_name_psize_dep, $N_type_psize_dep, $N_psize_psize_dep,
                $N_dep1_psize_dep, $N_dep2_psize_dep ),* ;
        ];
    }};
}
#[allow(unused_imports)]
pub(crate) use define_data_type;

/// Defines the `DataRaw*` union.
///
/// It calls the macros: `impl_raw!`
// NOTES:
// - https://doc.rust-lang.org/stable/reference/items/unions.html
// - https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html
// - [non-Copy](https://github.com/rust-lang/rust/issues/55149)
// DONE:1.64: refs in unions https://github.com/rust-lang/rust/pull/97995 (closed 55149)
// WAIT? https://github.com/rust-lang/rust/issues/98102
// IMPROVE:
// - support With version (generics are supported I THINK)
// - support non-Copy types by wrapping with ManuallyDrop<T>.
// - add a not so unsafe api for first use cases, (space efficient rows).
#[allow(unused_macros)]
macro_rules! define_data_raw {
    (
        v: $Vname:ident, t: $Tname:ident, r: $Rname:ident,
        size: $B:literal, $b:literal,
        feature: $feature:literal,

        copy_variants:
            $( $C_doc:literal, $C_name:ident, $C_type:ty, )* ;
        copy_variants_dep:
            $( $C_doc_dep:literal, $C_name_dep:ident, $C_type_dep:ty,
            $C_dep1_dep:literal, $C_dep2_dep:literal, )* ;
        copy_variants_psize:
            $( $C_doc_psize:literal, $C_name_psize:ident, $C_type_psize:ty,
                $C_psize_psize:meta, )* ;
        copy_variants_psize_dep:
            $( $C_doc_psize_dep:literal, $C_name_psize_dep:ident, $C_type_psize_dep:ty,
            $C_psize_psize_dep:meta, $C_dep1_psize_dep:literal, $C_dep2_psize_dep:literal, )* ;
        noncopy_variants:
            $( $N_doc:literal, $N_name:ident, $N_type:ty, )* ;
        noncopy_variants_dep:
            $( $N_doc_dep:literal, $N_name_dep:ident, $N_type_dep:ty,
            $N_dep1_dep:literal, $N_dep2_dep:literal, )* ;
        noncopy_variants_psize:
            $( $N_doc_psize:literal, $N_name_psize:ident, $N_type_psize:ty,
            $N_psize_psize:meta, $N_dep1_psize:literal, $N_dep2_psize:literal, )* ;
        noncopy_variants_psize_dep:
            $( $N_doc_psize_dep:literal, $N_name_psize_dep:ident, $N_type_psize_dep:ty,
            $N_psize_psize_dep:meta, $N_dep1_psize_dep:literal, $N_dep2_psize_dep:literal, )* ;
    ) => { $crate::paste!{
        // ## copy version (DataRaw)
        // -----------------------------------------------------------------
        #[repr(C)]
        #[doc = $b "-bit *raw* data, restricted to `Copy` variants."]
        #[derive(Copy, Clone)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
        #[allow(non_snake_case, reason = "union fields named like enum variants")]
        pub union [< $Rname $b Copy >] {
            /// Represents the absence of *data*.
            pub NoData: (),

            $(
                #[doc = $C_doc]
                pub $C_name: $C_type,
            )*

            $( // pointer-size dependant
                #[cfg($C_psize_psize)]
                #[doc = $C_doc_psize]
                $C_name_psize: $C_type_psize,
            )*

            // feature-gated dependencies
            $(
                #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))))]
                #[doc = $C_doc_dep]
                pub $C_name_dep: $C_type_dep,
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($C_psize_psize_dep,
                        feature = $C_dep1_psize_dep, feature = $C_dep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_psize_dep, feature = $C_dep2_psize_dep))))]
                #[doc = $C_doc_psize_dep]
                $C_name_psize_dep($C_type_psize_dep),
            )*
        }
        // type aliases:
        // TODO
        // #[doc = $b "-bit *untyped (raw)* data"]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        // pub type [< $Rname $b Copy >] = [< $Rname $b >];

        // Debug
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
        impl core::fmt::Debug for [<$Rname $b Copy>] {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{} {{...}}", stringify!{[< $Rname $b Copy>]})
            }
        }

        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
        $crate::impl_data_raw![
            r: [< $Rname $b Copy>],
        ];
    }};
}
#[allow(unused_imports)]
pub(crate) use define_data_raw;
