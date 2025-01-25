// devela::data::table::value::macros::define_each
//
//!
//
// TOC
// - define_data_value!
// - define_data_type!
// - define_data_raw!

/// for defining enum DataValue*
//
// macro args for variants
// - $C_+   :    copy variants
// - $N_+   : noncopy variants
// - _doc*  :
// - _name* :
// - _type* :
// - _dep*  :
// - _ptr*  :
#[allow(unused_macros)]
macro_rules! define_data_value {
    (
        v: $Value:ident, t: $Type:ident, r: $Raw:ident,
        size: $b:literal, feature: $feature:literal,

        copy:
            $( $C_doc:literal, $C_name:ident, $C_type:ty,
               [def:$C_def:stmt]
            )* ;
        copy@dep:
            $( $C_doc_dep:literal, $C_name_dep:ident, $C_type_dep:ty,
               $C_dep1_dep:literal, $C_dep2_dep:literal,
               [def:$C_def_dep:stmt]
            )* ;
        copy@ptr:
            $( $C_doc_ptr:literal, $C_name_ptr:ident, $C_type_ptr:ty,
               $C_ptr_ptr:meta,
               [def:$C_def_ptr:stmt]
            )* ;
        copy@ptrdep:
            $( $C_doc_ptrdep:literal, $C_name_ptrdep:ident, $C_type_ptrdep:ty,
               $C_ptr_ptrdep:meta, $C_dep1_ptrdep:literal, $C_dep2_ptrdep:literal,
               [def:$C_def_ptrdep:stmt]
             )* ;

        noncopy:
            $( $N_doc:literal, $N_name:ident, $N_type:ty,
               [def:$N_def:stmt]
            )* ;
        noncopy@dep:
            $( $N_doc_dep:literal, $N_name_dep:ident, $N_type_dep:ty,
               $N_dep1_dep:literal, $N_dep2_dep:literal,
               [def:$N_def_dep:stmt]
            )* ;
        noncopy@ptr:
            $( $N_doc_ptr:literal, $N_name_ptr:ident, $N_type_ptr:ty,
               $N_ptr_ptr:meta, $N_dep1_ptr:literal, $N_dep2_ptr:literal,
               [def:$N_def_ptr:stmt]
            )* ;
        noncopy@ptrdep:
            $( $N_doc_ptrdep:literal, $N_name_ptrdep:ident, $N_type_ptrdep:ty,
               $N_ptr_ptrdep:meta, $N_dep1_ptrdep:literal, $N_dep2_ptrdep:literal,
               [def:$N_def_ptrdep:stmt]
            )* ;
    ) => { $crate::paste! {
        // ## copy version (DataValue)
        // -----------------------------------------------------------------
        #[doc = $b "-bit data *value*, restricted to `Copy` variants, with extra `V`." ]
        ///
        /// See also:
        #[doc = "- [" [<$Type $b Copy With>] "]" ]
        #[doc = "- [" [<$Value $b With>] "][" [<$Value $b With>] "] -Copy" ]
        #[doc = "- [" [<$Value $b Copy>] "][" [<$Value $b Copy>] "] -With" ]
        #[doc = "- [" [<$Value $b>] "][" [<$Value $b>] "] -Copy -With" ]
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub enum [<$Value $b Copy With>]<V: DataValue> {
            /// Represents the absence of *data*.
            None,
            /// Extra *data values*.
            Extra(V),

            $( // fundamental types
                #[doc = $C_doc]
                $C_name($C_type),
            )*

            $( // pointer-size dependant
                #[cfg($C_ptr_ptr)]
                #[doc = $C_doc_ptr]
                $C_name_ptr($C_type_ptr),
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_dep,
                                feature = $C_dep2_dep))))]
                #[doc = $C_doc_dep]
                $C_name_dep($C_type_dep),
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($C_ptr_ptrdep,
                        feature = $C_dep1_ptrdep,
                        feature = $C_dep2_ptrdep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_ptrdep,
                                feature = $C_dep2_ptrdep))))]
                #[doc = $C_doc_ptrdep]
                $C_name_ptrdep($C_type_ptrdep),
            )*
        }

        // alias DataValue Copy
        #[doc = $b "-bit data *value*, restricted to `Copy` variants." ]
        ///
        /// See also:
        #[doc = "- [" [<$Type $b Copy>] "]" ]
        #[doc = "- [" [<$Value $b>] "][" [<$Value $b>] "] -Copy" ]
        #[doc = "- [" [<$Value $b Copy With>] "][" [<$Value $b Copy With>] "] +With" ]
        #[doc = "- [" [<$Value $b With>] "][" [<$Value $b With>] "] -Copy +With" ]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub type [<$Value $b Copy>] = [< $Value $b With>]<$crate::NoData>;

        // implement the DataValue trait
        $crate::impl_data_value![
            v: [< $Value $b Copy With >], DataValue,
            t: [< $Type $b Copy With >], DataType,
            is_copy: true,

            copy:
                $( $C_name, $C_type
                ),* ;
            copy@dep:
                $( $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep
                ),* ;
            copy@ptr:
                $( $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr
                ),* ;
            copy@ptrdep:
                $( $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep,
                   $C_dep1_ptrdep, $C_dep2_ptrdep
                ),* ;

            noncopy: ;
            noncopy@dep: ;
            noncopy@ptr: ;
            noncopy@ptrdep: ;
        ];
        impl<V: DataValueCopy> DataValueCopy for [< $Value $b Copy With >]<V> { }

        // ## non-copy version (DataValue)
        // -----------------------------------------------------------------
        #[doc = $b "-bit data *value*, with extra `V`." ]
        ///
        /// See also:
        #[doc = "- [" [<$Type $b With>] "]" ]
        #[doc = "- [" [<$Value $b Copy With>] "][" [<$Value $b Copy With>] "] +Copy" ]
        #[doc = "- [" [<$Value $b>] "][" [<$Value $b>] "] -Width" ]
        #[doc = "- [" [<$Value $b Copy>] "][" [<$Value $b Copy>] "] +Copy -Width" ]
        #[derive(Debug)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub enum [<$Value $b With>]<V: DataValue> {
            /// Represents the absence of *data*.
            None,
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
                #[cfg($C_ptr_ptr)]
                #[doc = $C_doc_ptr]
                $C_name_ptr($C_type_ptr),
            )*
            $(
                #[cfg($N_ptr_ptr)]
                #[doc = $N_doc_ptr]
                $N_name_ptr($N_type_ptr),
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
                #[cfg(all($C_ptr_ptrdep,
                        feature = $C_dep1_ptrdep,
                        feature = $C_dep2_ptrdep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_ptrdep,
                                feature = $C_dep2_ptrdep))))]
                #[doc = $C_doc_ptrdep]
                $C_name_ptrdep($C_type_ptrdep),
            )*
            $(
                #[cfg(all($N_ptr_ptrdep,
                        feature = $N_dep1_ptrdep,
                        feature = $N_dep2_ptrdep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $N_dep1_ptrdep,
                                feature = $N_dep2_ptrdep))))]
                #[doc = $N_doc_ptrdep]
                $N_name_ptrdep($N_type_ptrdep),
            )*
        }

        // alias DataValue
        #[doc = $b "-bit data *value*." ]
        ///
        /// See also:
        #[doc = "- [" [<$Type $b>] "]" ]
        #[doc = "- [" [<$Value $b Copy>] "][" [<$Value $b Copy>] "] +Copy" ]
        #[doc = "- [" [<$Value $b With>] "][" [<$Value $b With>] "] +With" ]
        #[doc = "- [" [<$Value $b Copy With>] "][" [<$Value $b Copy With>] "] +Copy +With" ]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub type [<$Value $b>] = [< $Value $b With>]<$crate::NoData>;

        // implement the DataValue trait
        $crate::impl_data_value![
            v: [< $Value $b With >], DataValue,
            t: [< $Type $b With >], DataType,
            is_copy: false,

            copy:
                $( $C_name, $C_type
                ),* ;
            copy@dep:
                $( $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep
                ),* ;
            copy@ptr:
                $( $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr
                ),* ;
            copy@ptrdep:
                $( $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep,
                   $C_dep1_ptrdep, $C_dep2_ptrdep
                ),* ;

            noncopy:
                $( $N_name, $N_type
                ),* ;
            noncopy@dep:
                $( $N_name_dep, $N_type_dep,
                   $N_dep1_dep, $N_dep2_dep
                ),* ;
            noncopy@ptr:
                $( $N_name_ptr, $N_type_ptr,
                   $N_ptr_ptr
                ),* ;
            noncopy@ptrdep:
                $( $N_name_ptrdep, $N_type_ptrdep,
                   $N_ptr_ptrdep,
                   $N_dep1_ptrdep, $N_dep2_ptrdep
                ),* ;
        ];

        // implement `TryFrom`<`DataValue`> for *contained-value*:

        $( // Copy
            impl<V: DataValueCopy> TryFrom<[<$Value $b Copy With>]<V>> for $C_type {
                type Error = ();
                fn try_from(v: [<$Value $b Copy With>]<V>) -> Result<Self, Self::Error> {
                    match v {
                        [<$Value $b Copy With>]::$C_name(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        $( // Copy feature-bound
            #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep ))]
            impl<V: DataValue> TryFrom<[<$Value $b With>]<V>> for $C_type_dep {
                type Error = ();
                fn try_from(v: [<$Value $b With>]<V>) -> Result<Self, Self::Error> {
                    match v {
                        [<$Value $b With>]::$C_name_dep(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        $( // non-Copy
            impl<V: DataValue> TryFrom<[<$Value $b With>]<V>> for $N_type {
                type Error = ();
                fn try_from(v: [<$Value $b With>]<V>) -> Result<Self, Self::Error> {
                    match v {
                        [<$Value $b With>]::$N_name(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        $( // non-Copy feature-bound
            #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep ))]
            impl<V: DataValue> TryFrom<[<$Value $b With>]<V>> for $N_type_dep {
                type Error = ();
                fn try_from(v: [<$Value $b With>]<V>) -> Result<Self, Self::Error> {
                    match c {
                        [<$Value $b With>]::$N_name_dep(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*

        // implement `From`<*contained-value*> for `DataValue`:

        $( // Copy
            impl<V: DataValueCopy> From<$C_type> for [<$Value $b Copy With>]<V> {
                fn from(v: $C_type) -> Self {
                    [<$Value $b Copy With>]::$C_name(v)
                }
            }
        )*
        $( // Copy feature-bound
            #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep ))]
            impl<V: DataValueCopy> From<$C_type_dep> for [<$Value $b Copy With>]<V> {
                fn from(v: $C_type_dep) -> Self {
                    [<$Value $b Copy With>]::$C_name_dep(v)
                }
            }
        )*
        $( // non-Copy
            impl<V: DataValue> From<$N_type> for [<$Value $b With>]<V> {
                fn from(v: $N_type) -> Self {
                    [<$Value $b With>]::$N_name(v)
                }
            }
        )*
        $( // non-Copy feature-bound
            #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep ))]
            impl<V: DataValue> From<$N_type_dep> for [<$Value $b With>]<V> {
                fn from(v: $N_type_dep) -> Self {
                    [<$Value $b With>]::$N_name_dep(v)
                }
            }
        )*

        // from DataValue to DataRawCopy
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
        impl<V: DataValueCopy> From<[<$Value $b Copy With>]<V>> for [<$Raw $b Copy>] {
            fn from(cell: [<$Value $b Copy With>]<V>) -> Self {
                match cell {
                    [<$Value $b Copy With>]::None => Self { None: () },
                    [<$Value $b Copy With>]::Extra(_) => Self { None: () }, // FIX:IMPROVE

                    $( // fundamental types
                        [<$Value $b Copy With>]::$C_name(v) => Self { $C_name: v },
                    )*

                    $( // pointer-size dependant
                        #[cfg($C_ptr_ptr)]
                        [<$Value $b Copy With>]::$C_name_ptr(v) => Self { $C_name_ptr: v },
                    )*

                    $( // feature-gated dependencies
                        #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                        [<$Value $b Copy With>]::$C_name_dep(v) => Self { $C_name_dep: v },
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
        v: $Value:ident, t: $Type:ident, r: $Raw:ident,
        size: $b:literal, feature: $feature:literal,

        copy:
            $( $C_doc:literal, $C_name:ident, $C_type:ty,
               [def:$C_def:stmt]
            )* ;
        copy@dep:
            $( $C_doc_dep:literal, $C_name_dep:ident, $C_type_dep:ty,
               $C_dep1_dep:literal, $C_dep2_dep:literal,
               [def:$C_def_dep:stmt]
            )* ;
        copy@ptr:
            $( $C_doc_ptr:literal, $C_name_ptr:ident, $C_type_ptr:ty,
               $C_ptr_ptr:meta,
               [def:$C_def_ptr:stmt]
            )* ;
        copy@ptrdep:
            $( $C_doc_ptrdep:literal, $C_name_ptrdep:ident, $C_type_ptrdep:ty,
               $C_ptr_ptrdep:meta, $C_dep1_ptrdep:literal, $C_dep2_ptrdep:literal,
               [def:$C_def_ptrdep:stmt]
            )* ;

        noncopy:
            $( $N_doc:literal, $N_name:ident, $N_type:ty,
               [def:$N_def:stmt]
            )* ;
        noncopy@dep:
            $( $N_doc_dep:literal, $N_name_dep:ident, $N_type_dep:ty,
               $N_dep1_dep:literal, $N_dep2_dep:literal,
               [def:$N_def_dep:stmt]
            )* ;
        noncopy@ptr:
            $( $N_doc_ptr:literal, $N_name_ptr:ident, $N_type_ptr:ty,
               $N_ptr_ptr:meta, $N_dep1_ptr:literal, $N_dep2_ptr:literal,
               [def:$N_def_ptr:stmt]
            )* ;
        noncopy@ptrdep:
            $( $N_doc_ptrdep:literal, $N_name_ptrdep:ident, $N_type_ptrdep:ty,
               $N_ptr_ptrdep:meta, $N_dep1_ptrdep:literal, $N_dep2_ptrdep:literal,
               [def:$N_def_ptrdep:stmt]
            )* ;
    ) =>  { $crate::paste! {
        // ## copy version (DataType)
        // -----------------------------------------------------------------
        #[doc = $b "-bit data *type*, restricted to `Copy` variants, with extra `T`." ]
        ///
        /// See also:
        #[doc = "- [" [<$Value $b Copy With>] "]" ]
        #[doc = "- [" [<$Type $b With>]  "][" [<$Type $b With>] "] -Copy" ]
        #[doc = "- [" [<$Type $b Copy>]  "][" [<$Type $b Copy>] "] -With" ]
        #[doc = "- [" [<$Type $b>]  "][" [<$Type $b>] "] -Copy -With" ]
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub enum [< $Type $b Copy With >]<T: DataType> {
            /// Represents the absence of *data*.
            None,
            /// A custom *data type* extension.
            Extra(T),

            $( // fundamental types
                #[doc = $C_doc ]
                $C_name,
            )*

            $( // pointer-size dependant
                #[cfg($C_ptr_ptr)]
                #[doc = $C_doc_ptr]
                $C_name_ptr,
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_dep,
                                feature = $C_dep2_dep))))]
                #[doc = $C_doc_dep]
                $C_name_dep,
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($C_ptr_ptrdep, feature = $C_dep1_ptrdep,
                        feature = $C_dep2_ptrdep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_ptrdep, feature = $C_dep2_ptrdep))))]
                #[doc = $C_doc_ptrdep]
                $C_name_ptrdep,
            )*
        }

        // DataType Copy (-With) alias
        #[doc = $b "-bit data *type*, restricted to `Copy` variants." ]
        ///
        /// See also:
        #[doc = "- [" [<$Value $b Copy>] "]" ]
        #[doc = "- [" [<$Type $b>] "][" [<$Type $b>] "] -Copy" ]
        #[doc = "- [" [<$Type $b Copy With>] "][" [<$Type $b Copy With>] "] +With" ]
        #[doc = "- [" [<$Type $b With>] "][" [<$Type $b With>] "] -Copy +With" ]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub type [<$Type $b Copy>] = [< $Type $b Copy With>]<$crate::NoData>;

        // implement the DataType trait
        $crate::impl_data_type![
            v: [< $Value $b Copy With >], DataValue,
            t: [< $Type $b Copy With >], DataType,
            is_copy: true,

            copy:
                $( $C_name, $C_type,
                   [def:$C_def]
                )* ;
            copy@dep:
                $( $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep,
                   [def:$C_def_dep]
                )* ;
            copy@ptr:
                $( $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr,
                   [def:$C_def_ptr]
                )* ;
            copy@ptrdep:
                $( $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep,
                   $C_dep1_ptrdep, $C_dep2_ptrdep,
                   [def:$C_def_ptrdep]
                )* ;

            noncopy: ;
            noncopy@dep: ;
            noncopy@ptr: ;
            noncopy@ptrdep: ;
        ];
        impl<T: DataTypeCopy> DataTypeCopy for [< $Type $b Copy With >]<T>
            where T::Value: DataValueCopy {}

        // ## non-copy version (DataType)
        // -----------------------------------------------------------------
        #[doc = $b "-bit data *type*, with extra `T`." ]
        ///
        /// See also:
        #[doc = "- [" [<$Value $b With>] "]" ]
        #[doc = "- [" [<$Type $b Copy With>] "][" [<$Type $b Copy With>] "] +Copy" ]
        #[doc = "- [" [<$Type $b>] "][" [<$Type $b>] "] -With" ]
        #[doc = "- [" [<$Type $b Copy>] "][" [<$Type $b Copy>] "] +Copy -With" ]
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub enum [< $Type $b With >]<T: DataType> {
            /// Represents the absence of *data*.
            None,
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
                #[cfg($C_ptr_ptr)]
                #[doc = $C_doc_ptr]
                $C_name_ptr,
            )*
            $(
                #[cfg($N_ptr_ptr)]
                #[doc = $N_doc_ptr]
                $N_name_ptr,
            )*

            $( // feature-gated dependencies
                #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_dep,
                                feature = $C_dep2_dep))))]
                #[doc = $C_doc_dep]
                $C_name_dep,
            )*
            $(
                #[cfg(all(feature = $N_dep1_dep, feature = $N_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $N_dep1_dep,
                                feature = $N_dep2_dep))))]
                #[doc = $N_doc_dep]
                $N_name_dep,
            )*
            $( // pointer-size & feature-gated dependencies
                #[cfg(all($C_ptr_ptrdep,
                        feature = $C_dep1_ptrdep,
                        feature = $C_dep2_ptrdep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_ptrdep,
                                feature = $C_dep2_ptrdep))))]
                #[doc = $C_doc_ptrdep]
                $C_name_ptrdep,
            )*
            $(
                #[cfg(all($N_ptr_ptrdep,
                        feature = $N_dep1_ptrdep,
                        feature = $N_dep2_ptrdep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $N_dep1_ptrdep,
                                feature = $N_dep2_ptrdep))))]
                #[doc = $N_doc_ptrdep]
                $N_name_ptrdep,
            )*
        }

        // DataType (-With) alias
        #[doc = $b "-bit data *type*"]
        ///
        /// See also:
        #[doc = "- [" [<$Value $b>] "]" ]
        #[doc = "- [" [<$Type $b Copy>] "][" [<$Type $b Copy>] "] +Copy" ]
        #[doc = "- [" [<$Type $b With>] "][" [<$Type $b With>] "] +With" ]
        #[doc = "- [" [<$Type $b Copy With>] "][" [<$Type $b Copy With>] "] +Copy +With" ]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        pub type [<$Type $b>] = [< $Type $b With>]<$crate::NoData>;

        // implement the DataType trait
        $crate::impl_data_type![
            v: [< $Value $b With >], DataValue,
            t: [< $Type $b With >], DataType,
            is_copy: false,

            copy:
                $( $C_name, $C_type,
                   [def:$C_def]
                )* ;
            copy@dep:
                $( $C_name_dep, $C_type_dep,
                   $C_dep1_dep, $C_dep2_dep,
                   [def:$C_def_dep]
                )* ;
            copy@ptr:
                $( $C_name_ptr, $C_type_ptr,
                   $C_ptr_ptr,
                   [def:$C_def_ptr]
                )* ;
            copy@ptrdep:
                $( $C_name_ptrdep, $C_type_ptrdep,
                   $C_ptr_ptrdep,
                   $C_dep1_ptrdep, $C_dep2_ptrdep,
                   [def:$C_def_ptrdep]
                )* ;

            noncopy:
                $($N_name, $N_type,
                   [def:$N_def]
                )* ;
            noncopy@dep:
                $( $N_name_dep, $N_type_dep,
                   $N_dep1_dep, $N_dep2_dep,
                   [def:$N_def_dep]
                )* ;
            noncopy@ptr:
                $( $N_name_ptr, $N_type_ptr,
                   $N_ptr_ptr,
                   [def:$N_def_ptr]
                )* ;
            noncopy@ptrdep:
                $( $N_name_ptrdep, $N_type_ptrdep,
                   $N_ptr_ptrdep,
                   $N_dep1_ptrdep, $N_dep2_ptrdep,
                   [def:$N_def_ptrdep]
                )* ;
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
        v: $Value:ident, t: $Type:ident, r: $Raw:ident,
        size: $b:literal, feature: $feature:literal,

        copy:
            $( $C_doc:literal, $C_name:ident, $C_type:ty,
               [def:$C_def:stmt]
            )* ;
        copy@dep:
            $( $C_doc_dep:literal, $C_name_dep:ident, $C_type_dep:ty,
               $C_dep1_dep:literal, $C_dep2_dep:literal,
               [def:$C_def_dep:stmt]
            )* ;
        copy@ptr:
            $( $C_doc_ptr:literal, $C_name_ptr:ident, $C_type_ptr:ty,
               $C_ptr_ptr:meta,
               [def:$C_def_ptr:stmt]
            )* ;
        copy@ptrdep:
            $( $C_doc_ptrdep:literal, $C_name_ptrdep:ident, $C_type_ptrdep:ty,
               $C_ptr_ptrdep:meta, $C_dep1_ptrdep:literal, $C_dep2_ptrdep:literal,
               [def:$C_def_ptrdep:stmt]
            )* ;

        noncopy:
            $( $N_doc:literal, $N_name:ident, $N_type:ty,
               [def:$N_def:stmt]
            )* ;
        noncopy@dep:
            $( $N_doc_dep:literal, $N_name_dep:ident, $N_type_dep:ty,
               $N_dep1_dep:literal, $N_dep2_dep:literal,
               [def:$N_def_dep:stmt]
            )* ;
        noncopy@ptr:
            $( $N_doc_ptr:literal, $N_name_ptr:ident, $N_type_ptr:ty,
               $N_ptr_ptr:meta, $N_dep1_ptr:literal, $N_dep2_ptr:literal,
               [def:$N_def_ptr:stmt]
            )* ;
        noncopy@ptrdep:
            $( $N_doc_ptrdep:literal, $N_name_ptrdep:ident, $N_type_ptrdep:ty,
               $N_ptr_ptrdep:meta,
               $N_dep1_ptrdep:literal, $N_dep2_ptrdep:literal,
               [def:$N_def_ptrdep:stmt]
            )* ;
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
        pub union [< $Raw $b Copy >] {
            /// Represents the absence of *data*.
            pub None: (),

            $(
                #[doc = $C_doc]
                pub $C_name: $C_type,
            )*

            $( // pointer-size dependant
                #[cfg($C_ptr_ptr)]
                #[doc = $C_doc_ptr]
                $C_name_ptr: $C_type_ptr,
            )*

            // feature-gated dependencies
            $(
                #[cfg(all(feature = $C_dep1_dep, feature = $C_dep2_dep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_dep,
                                feature = $C_dep2_dep))))]
                #[doc = $C_doc_dep]
                pub $C_name_dep: $C_type_dep,
            )*

            $( // pointer-size & feature-gated dependencies
                #[cfg(all($C_ptr_ptrdep,
                        feature = $C_dep1_ptrdep,
                        feature = $C_dep2_ptrdep))]
                #[cfg_attr(feature = "nightly_doc",
                    doc(cfg(all(feature = $C_dep1_ptrdep,
                                feature = $C_dep2_ptrdep))))]
                #[doc = $C_doc_ptrdep]
                $C_name_ptrdep($C_type_ptrdep),
            )*
        }
        // type aliases:
        // TODO
        // #[doc = $b "-bit *untyped (raw)* data"]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $feature)))]
        // pub type [< $Raw $b Copy >] = [< $Raw $b >];

        // Debug
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
        impl core::fmt::Debug for [<$Raw $b Copy>] {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{} {{...}}", stringify!{[< $Raw $b Copy>]})
            }
        }

        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
        $crate::impl_data_raw![
            r: [< $Raw $b Copy>],
        ];
    }};
}
#[allow(unused_imports)]
pub(crate) use define_data_raw;
