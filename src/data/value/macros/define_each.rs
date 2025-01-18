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
            $( $cvdoc:literal, $cvname:ident, $cvtype:ty, )* ;
        copy_variants_dep:
            $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal, )* ;
        copy_variants_psize:
            $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty, $cvpsize_psize:meta, )* ;
        copy_variants_psize_dep:
            $( $cvdoc_psize_dep:literal, $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal, )* ;
        noncopy_variants:
            $( $vdoc:literal, $vname:ident, $vtype:ty, )* ;
        noncopy_variants_dep:
            $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal, )* ;
        noncopy_variants_psize:
            $( $vdoc_psize:literal, $vname_psize:ident, $vtype_psize:ty,
            $vpsize_psize:meta, $vdep1_psize:literal, $vdep2_psize:literal, )* ;
        noncopy_variants_psize_dep:
            $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal, )* ;
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
                #[doc = $cvdoc]
                $cvname($cvtype),
            )*

            $( // pointer-size dependant
                #[cfg($cvpsize_psize)]
                #[doc = $cvdoc_psize]
                $cvname_psize($cvtype_psize),
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))))]
                #[doc = $cvdoc_dep]
                $cvname_dep($cvtype_dep),
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($cvpsize_psize_dep,
                        feature = $cvdep1_psize_dep,
                        feature = $cvdep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))))]
                #[doc = $cvdoc_psize_dep]
                $cvname_psize_dep($cvtype_psize_dep),
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
                $( $cvname, $cvtype ),* ;
            copy_variants_dep:
                $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize:
                $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep:
                $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
            noncopy_variants: ;
            noncopy_variants_dep: ;
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
                #[doc = $cvdoc]
                $cvname($cvtype),
            )*
            $(
                #[doc = $vdoc]
                $vname($vtype),
            )*

            $( // pointer-size dependant
                #[cfg($cvpsize_psize)]
                #[doc = $cvdoc_psize]
                $cvname_psize($cvtype_psize),
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))))]
                #[doc = $cvdoc_dep]
                $cvname_dep($cvtype_dep),
            )*
            $(
                #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))))]
                #[doc = $vdoc_dep]
                $vname_dep($vtype_dep),
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($cvpsize_psize_dep,
                        feature = $cvdep1_psize_dep,
                        feature = $cvdep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))))]
                #[doc = $cvdoc_psize_dep]
                $cvname_psize_dep($cvtype_psize_dep),
            )*
            $(
                #[cfg(all($vpsize_psize_dep,
                        feature = $vdep1_psize_dep,
                        feature = $vdep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))))]
                #[doc = $vdoc_psize_dep]
                $vname_psize_dep($vtype_psize_dep),
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
                $( $cvname, $cvtype ),* ;
            copy_variants_dep:
                $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize:
                $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep:
                $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
            noncopy_variants:
                $($vname, $vtype ),* ;
            noncopy_variants_dep:
                $( $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
            noncopy_variants_psize_dep:
                $( $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                $vdep1_psize_dep, $vdep2_psize_dep ),* ;
        ];

        // implement `TryFrom`<`DataValue`> for *contained-value*:

        $( // Copy
            impl<V: DataValueCopy> TryFrom<[<$Vname $b Copy With>]<V>> for $cvtype {
                type Error = ();
                fn try_from(v: [<$Vname $b Copy With>]<V>) -> Result<Self, Self::Error> {
                    match v {
                        [<$Vname $b Copy With>]::$cvname(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        $( // Copy feature-bound
            #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep ))]
            impl<V: DataValue> TryFrom<[<$Vname $b With>]<V>> for $cvtype_dep {
                type Error = ();
                fn try_from(v: [<$Vname $b With>]<V>) -> Result<Self, Self::Error> {
                    match v {
                        [<$Vname $b With>]::$cvname_dep(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        $( // non-Copy
            impl<V: DataValue> TryFrom<[<$Vname $b With>]<V>> for $vtype {
                type Error = ();
                fn try_from(v: [<$Vname $b With>]<V>) -> Result<Self, Self::Error> {
                    match v {
                        [<$Vname $b With>]::$vname(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        $( // non-Copy feature-bound
            #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep ))]
            impl<V: DataValue> TryFrom<[<$Vname $b With>]<V>> for $vtype_dep {
                type Error = ();
                fn try_from(v: [<$Vname $b With>]<V>) -> Result<Self, Self::Error> {
                    match c {
                        [<$Vname $b With>]::$vname_dep(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*

        // implement `From`<*contained-value*> for `DataValue`:

        $( // Copy
            impl<V: DataValueCopy> From<$cvtype> for [<$Vname $b Copy With>]<V> {
                fn from(v: $cvtype) -> Self {
                    [<$Vname $b Copy With>]::$cvname(v)
                }
            }
        )*
        $( // Copy feature-bound
            #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep ))]
            impl<V: DataValueCopy> From<$cvtype_dep> for [<$Vname $b Copy With>]<V> {
                fn from(v: $cvtype_dep) -> Self {
                    [<$Vname $b Copy With>]::$cvname_dep(v)
                }
            }
        )*
        $( // non-Copy
            impl<V: DataValue> From<$vtype> for [<$Vname $b With>]<V> {
                fn from(v: $vtype) -> Self {
                    [<$Vname $b With>]::$vname(v)
                }
            }
        )*
        $( // non-Copy feature-bound
            #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep ))]
            impl<V: DataValue> From<$vtype_dep> for [<$Vname $b With>]<V> {
                fn from(v: $vtype_dep) -> Self {
                    [<$Vname $b With>]::$vname_dep(v)
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
                        [<$Vname $b Copy With>]::$cvname(v) => Self { $cvname: v },
                    )*

                    $( // pointer-size dependant
                        #[cfg($cvpsize_psize)]
                        [<$Vname $b Copy With>]::$cvname_psize(v) => Self { $cvname_psize: v },
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                        #[cfg_attr(feature = "nightly_doc",
                            doc(cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))))]
                        [<$Vname $b Copy With>]::$cvname_dep(v) => Self { $cvname_dep: v },
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
            $( $cvdoc:literal, $cvname:ident, $cvtype:ty, )* ;
        copy_variants_dep:
            $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal, )* ;
        copy_variants_psize:
            $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
            $cvpsize_psize:meta, )* ;
        copy_variants_psize_dep:
            $( $cvdoc_psize_dep:literal, $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal, )* ;
        noncopy_variants:
            $( $vdoc:literal, $vname:ident, $vtype:ty, )* ;
        noncopy_variants_dep:
            $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal, )* ;
        noncopy_variants_psize:
            $( $vdoc_psize:literal, $vname_psize:ident, $vtype_psize:ty,
            $vpsize_psize:meta, $vdep1_psize:literal, $vdep2_psize:literal, )* ;
        noncopy_variants_psize_dep:
            $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal, )* ;
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
                #[doc = $cvdoc ]
                $cvname,
            )*

            $( // pointer-size dependant
                #[cfg($cvpsize_psize)]
                #[doc = $cvdoc_psize]
                $cvname_psize,
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))))]
                #[doc = $cvdoc_dep]
                $cvname_dep,
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))))]
                #[doc = $cvdoc_psize_dep]
                $cvname_psize_dep,
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
                $( $cvname, $cvtype ),* ;
            copy_variants_dep:
                $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize:
                $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep:
                $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
            noncopy_variants: ;
            noncopy_variants_dep: ;
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
                #[doc = $cvdoc ]
                $cvname,
            )*
            $(
                #[doc = $vdoc ]
                $vname,
            )*

            $( // pointer-size dependant
                #[cfg($cvpsize_psize)]
                #[doc = $cvdoc_psize]
                $cvname_psize,
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))))]
                #[doc = $cvdoc_dep]
                $cvname_dep,
            )*
            $(
                #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))))]
                #[doc = $vdoc_dep]
                $vname_dep,
            )*
            $( // pointer-size & feature-gated dependencies
                #[cfg(all($cvpsize_psize_dep,
                        feature = $cvdep1_psize_dep,
                        feature = $cvdep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))))]
                #[doc = $cvdoc_psize_dep]
                $cvname_psize_dep,
            )*
            $(
                #[cfg(all($vpsize_psize_dep,
                        feature = $vdep1_psize_dep,
                        feature = $vdep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))))]
                #[doc = $vdoc_psize_dep]
                $vname_psize_dep,
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
                $( $cvname, $cvtype ),* ;
            copy_variants_dep:
                $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize:
                $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep:
                $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
            noncopy_variants:
                $($vname, $vtype ),* ;
            noncopy_variants_dep:
                $( $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
            noncopy_variants_psize_dep:
                $( $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                $vdep1_psize_dep, $vdep2_psize_dep ),* ;
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
            $( $cvdoc:literal, $cvname:ident, $cvtype:ty, )* ;
        copy_variants_dep:
            $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal, )* ;
        copy_variants_psize:
            $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
                $cvpsize_psize:meta, )* ;
        copy_variants_psize_dep:
            $( $cvdoc_psize_dep:literal, $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal, )* ;
        noncopy_variants:
            $( $vdoc:literal, $vname:ident, $vtype:ty, )* ;
        noncopy_variants_dep:
            $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal, )* ;
        noncopy_variants_psize:
            $( $vdoc_psize:literal, $vname_psize:ident, $vtype_psize:ty,
            $vpsize_psize:meta, $vdep1_psize:literal, $vdep2_psize:literal, )* ;
        noncopy_variants_psize_dep:
            $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal, )* ;
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
                #[doc = $cvdoc]
                pub $cvname: $cvtype,
            )*

            $( // pointer-size dependant
                #[cfg($cvpsize_psize)]
                #[doc = $cvdoc_psize]
                $cvname_psize: $cvtype_psize,
            )*

            // feature-gated dependencies
            $(
                #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))))]
                #[doc = $cvdoc_dep]
                pub $cvname_dep: $cvtype_dep,
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($cvpsize_psize_dep,
                        feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))))]
                #[doc = $cvdoc_psize_dep]
                $cvname_psize_dep($cvtype_psize_dep),
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
