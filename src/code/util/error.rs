// devela::code::util::error
//
//! Defines [`impl_error!`]
//

/// Helper to define individual and composite error types.
///
/// It also helps implementing `From` and `TryFrom` between them.
macro_rules! impl_error {
    /* individual errors */

    (
    // Defines a standalone error tuple-struct with elements.
    individual: $struct_vis:vis struct $struct_name:ident
        $(( $($elem_vis:vis $elem_ty:ty),+ $(,)? ))?;
        $DOC_NAME:ident = $doc_str:literal,
        $self:ident + $fmt:ident => $display_expr:expr
        $(,)?
    ) => {
        $crate::CONST! { pub(crate) $DOC_NAME = $doc_str; }

        #[doc = crate::TAG_ERROR!()]
        #[doc = $DOC_NAME!()]
        #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
        $struct_vis struct $struct_name $(( $($elem_vis $elem_ty),+ ) )?;

        $crate::impl_error![@individual $struct_name, $self+$fmt=>$display_expr];
    };
    (
    // Defines a standalone error struct with fields.
    individual: $struct_vis:vis struct $struct_name:ident {
            $($(#[$field_attr:meta])* $field_vis:vis $field_name:ident : $field_ty:ty ),+ $(,)?
        }
        $DOC_NAME:ident = $doc_str:literal,
        $self:ident + $fmt:ident => $display_expr:expr
        $(,)?
    ) => {
        $crate::CONST! { pub(crate) $DOC_NAME = $doc_str; }

        #[doc = crate::TAG_ERROR!()]
        #[doc = $DOC_NAME!()]
        #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
        $struct_vis struct $struct_name {
            $( $(#[$field_attr])*
            $field_vis $field_name: $field_ty),+
        }

        $crate::impl_error![@individual $struct_name, $self+$fmt=>$display_expr];
    };
    (
    // shared code between both previous *individual* definitions
    @individual $struct_name:ident, $self:ident+$fmt:ident => $display_expr:expr) => {
        impl $crate::Display for $struct_name {
            fn fmt(&$self, $fmt: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                $display_expr
            }
        }
        impl $crate::Error for $struct_name {}
        impl $crate::ExtError for $struct_name {
            type Kind = ();
            fn error_eq(&self, other: &Self) -> bool { self == other }
            fn error_kind(&self) -> Self::Kind {}
        }
    };

    /* composite errors */

    (
    // Defines a composite Error enum, plus:
    // - impl Error, ExtError and Display.
    // - impl From and TryFrom in reverse.
    composite: fmt($fmt:ident)
        $(#[$enum_attr:meta])*
        $vis:vis enum $composite_error_name:ident { $(
            $DOC_VAR:ident:
            $variant_name:ident
                $(( $($elem_name:ident | $elem_num:literal: $elem_ty:ty),+ ))? // tuple-struct, or…
                $({ $($(#[$field_attr:meta])* $field_name:ident: $field_ty:ty),+ })? // field-struct
                => $individual_error_name:ident
                    $(( $($elem_display_expr:expr),+ ))? // tuple-struct, or…
                    $({ $($field_display_name:ident: $field_display_expr:expr),+ })? // field-struct
        ),+ $(,)? }
    ) => {
        #[doc = crate::TAG_ERROR_COMPOSITE!()]
        $(#[$enum_attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis enum $composite_error_name { $(
            #[doc = $DOC_VAR!()]
            $variant_name
                $(( $($elem_ty),+ ))? // tuple-struct, or…
                $({ $($(#[$field_attr])* $field_name: $field_ty),+ })? // field-struct
        ),+ }

        // impl Error, ExtError & Display:
        impl $crate::Error for $composite_error_name {}
        impl $crate::ExtError for $composite_error_name {
            type Kind = ();
            fn error_eq(&self, other: &Self) -> bool { self == other }
            fn error_kind(&self) -> Self::Kind {}
        }
        impl $crate::Display for $composite_error_name  {
            fn fmt(&self, $fmt: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                match self { $(
                    $composite_error_name::$variant_name
                    $(( $($elem_name),+ ))? // tuple-struct, or…
                    $({ $($field_name),+ })? // field-struct
                    =>
                        $crate::Display::fmt(&$individual_error_name
                            $(( $($elem_display_expr),+ ))? // tuple-struct, or…
                            $({ $($field_display_name: $field_display_expr),+})? // field-struct
                        , $fmt),
                )+ }
            }
        }
        // impl From multiple individual errors for a composite error, and TryFrom in reverse:
        $(
            $crate::impl_error! { from(_f): $individual_error_name, for: $composite_error_name
                => $variant_name
                $(( $($elem_name, $crate::field_of![_f, $elem_num] ),+ ))? // tuple-struct, or…
                $({ $($field_name, $crate::field_of![_f, $field_name] ),+ })? // field-struct
            }
        )+
    };
    (
    // Implements `From` an individual error type to a composite error containing it,
    // as well as implementing `TryFrom` for the inverse direction.
    from($fn_arg:ident): $from_individual:ident, for: $for_composite:ident
    => $variant_name:ident
        $(( $($elem_name:ident, $elem_expr:expr),+ ))? // tuple-struct, or…
        $({ $($field_name:ident,  $field_expr:expr),+ })? // field-struct

    $(,)? ) => {
        /* (example)
        impl From<NotEnoughElements> for DataNotEnough {
            fn from (_f: NotEnoughElements) -> DataNotEnough {
                DataNotEnough::$Elements(_f.0)
            }
        } */
        impl From<$from_individual> for $for_composite {
            fn from($fn_arg: $from_individual) -> $for_composite {
                $for_composite::$variant_name
                    $(( $($elem_expr),+ ))? // tuple-struct, or…
                    $({ $($field_name: $field_expr),+ })? // …field-struct
            }
        }
        /* (example)
        impl TryFrom<DataNotEnough> for NotEnoughElements {
            type Error = FailedErrorConversion;
            fn from (_f: DataNotEnough) -> Result<NotEnoughElements, FailedErrorConversion> {
                match _f {
                    DataNotEnough::Elements(i) => NotEnoughElements(i),
                    _ => Err(FailedErrorConversion)
                }
            }
        } */
        impl TryFrom<$for_composite> for $from_individual {
            type Error = crate::FailedErrorConversion;
            fn try_from($fn_arg: $for_composite) -> Result<$from_individual, Self::Error> {
                match $fn_arg {
                    $for_composite::$variant_name
                        $(( $($elem_name),+ ))? // tuple-struct, or…
                        $({ $($field_name),+ })? // field-struct
                        => Ok($from_individual
                            $(( $($elem_name),+ ))? // tuple-struct, or…
                            $({ $($field_name),+ })? // field-struct
                        ),
                        #[allow(unreachable_patterns)]
                        _ => Err(crate::FailedErrorConversion)
                }
            }
        }
    };
    (
    // Implements `From` a composite error type and a composite error superset of it,
    // as well as implementing `TryFrom` in the inverse direction.
    // E.g. from(f): DataNotEnough for: DataError
    composite: from($fn_arg:ident): $from_subset:ident, for: $for_superset:ident { $(
        $from_variant:ident
            $(( $($from_elem:ident),+ ))? // tuple-struct, or…
            $({ $($from_field:ident),+ })? // field-struct
                => $for_variant:ident
                    $(( $($for_elem:ident),+ ))? // tuple-struct, or…
                    $({ $($for_field:ident),+ })? // field-struct
    ),+ $(,)? }
    ) => {
        impl From<$from_subset> for $for_superset {
            fn from($fn_arg: $from_subset) -> $for_superset { match $fn_arg { $(
                $from_subset::$from_variant
                    $(( $($from_elem),+ ))? // tuple-struct, or…
                    $({ $($from_field),+ })? // field-struct
                    => $for_superset::$for_variant
                        $(( $($for_elem),+ ))? // tuple-struct, or…
                        $({ $($for_field),+ })? // field-struct
            ),+ } }
        }
        impl TryFrom<$for_superset> for $from_subset {
            type Error = crate::FailedErrorConversion;
            fn try_from($fn_arg: $for_superset)
                -> Result<$from_subset, crate::FailedErrorConversion> { match $fn_arg { $(
                    $for_superset::$for_variant
                        $(( $($for_elem),+ ))? // tuple-struct, or…
                        $({ $($for_field),+ })? // field-struct
                            => Ok($from_subset::$from_variant
                                $(( $($from_elem),+ ))? // tuple-struct, or…
                                $({ $($from_field),+ })? // field-struct
                    ),)+
                    _ => Err(crate::FailedErrorConversion)
                }
            }
        }
    };
}
pub(crate) use impl_error;

#[cfg(test)]
mod tests {
    use super::impl_error;

    #[test]
    fn impl_error() {
        /* define individual errors */

        impl_error! { individual: pub struct UnitStruct;
            DOC_UNIT_STRUCT = "docstring", self+f => write!(f, "display"),
        }
        impl_error! { individual: pub struct SingleElement(pub(crate) Option<u8>);
            DOC_SINGLE_ELEMENT = "docstring", self+f => write!(f, "display"),
        }
        impl_error! { individual: pub struct MultipleElements(pub i32, u32,);
            DOC_MULTI_ELEMENT = "docstring", self+f => write!(f, "display")
        }
        impl_error! { individual: pub struct StructFields {
                #[doc = "field1"] pub f1: bool,
                #[doc = "field2"] f2: Option<char>
            }
            DOC_STRUCT_FIELDS = "docstring", self+f => write!(f, "display")
        }

        /* define composite errors
         * (includes conversions between variants and indidual errors) */

        impl_error! { composite: fmt(f)
            /// A composite error superset.
            pub enum CompositeSuperset {
                DOC_UNIT_STRUCT: SuperUnit => UnitStruct,
                DOC_SINGLE_ELEMENT: SuperSingle(i|0: Option<u8>) => SingleElement(*i),
                DOC_MULTI_ELEMENT: SuperMultiple(i|0: i32, j|1: u32) => MultipleElements(*i, *j),
                DOC_STRUCT_FIELDS: SuperStruct { f1: bool, f2: Option<char> }
                    => StructFields { f1: *f1, f2: *f2 },
            }
        }
        impl_error! { composite: fmt(f)
            /// A composite error subset.
            // removed the unit struct variant (the most trivial case)
            pub enum CompositeSubset {
                DOC_SINGLE_ELEMENT: SubSingle(i|0: Option<u8>) => SingleElement(*i),
                DOC_MULTI_ELEMENT: SubMultiple(i|0: i32, j|1: u32) => MultipleElements(*i, *j),
                DOC_STRUCT_FIELDS: SubStruct { f1: bool, f2: Option<char> }
                    => StructFields { f1: *f1, f2: *f2 },
            }
        }

        /* implement conversions between composite errors */

        impl_error! { composite: from(f): CompositeSubset, for: CompositeSuperset {
            SubSingle(i) => SuperSingle(i),
            SubMultiple(i, j) => SuperMultiple(i, j),
            SubStruct { f1, f2 } => SuperStruct { f1, f2 },
        }}

        /* check the conversions between individual and composite errors */

        assert_eq![CompositeSuperset::SuperUnit, UnitStruct.into()];
        assert![UnitStruct::try_from(CompositeSuperset::SuperUnit).is_ok()];
        assert![UnitStruct::try_from(CompositeSuperset::SuperSingle(None)).is_err()];

        assert_eq![CompositeSuperset::SuperSingle(Some(1)), SingleElement(Some(1)).into()];
        assert![SingleElement::try_from(CompositeSuperset::SuperSingle(Some(2))).is_ok()];

        assert_eq![CompositeSuperset::SuperMultiple(3, 5), MultipleElements(3, 5).into()];
        assert![MultipleElements::try_from(CompositeSuperset::SuperMultiple(7, 13)).is_ok()];

        assert_eq![
            CompositeSuperset::SuperStruct { f1: true, f2: Some('a') },
            StructFields { f1: true, f2: Some('a') }.into()
        ];
        assert![
            StructFields::try_from(CompositeSuperset::SuperStruct { f1: false, f2: None }).is_ok()
        ];

        /* check the conversions between composite errors */

        assert_eq![
            CompositeSuperset::SuperSingle(Some(1)),
            CompositeSubset::SubSingle(Some(1)).into()
        ];
        assert_eq![
            CompositeSubset::try_from(CompositeSuperset::SuperSingle(Some(2))),
            Ok(CompositeSubset::SubSingle(Some(2)))
        ];

        assert_eq![
            CompositeSuperset::SuperMultiple(4, 6),
            CompositeSubset::SubMultiple(4, 6).into()
        ];
        assert_eq![
            CompositeSubset::try_from(CompositeSuperset::SuperMultiple(5, 7)),
            Ok(CompositeSubset::SubMultiple(5, 7))
        ];

        assert_eq![
            CompositeSuperset::SuperStruct { f1: true, f2: Some('z') },
            CompositeSubset::SubStruct { f1: true, f2: Some('z') }.into()
        ];
        assert_eq![
            CompositeSubset::try_from(CompositeSuperset::SuperStruct { f1: true, f2: None }),
            Ok(CompositeSubset::SubStruct { f1: true, f2: None })
        ];
    }
}
