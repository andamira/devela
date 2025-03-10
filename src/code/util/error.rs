// devela::code::util::error
//
//! Defines [`define_error!`]
//

/// Helper to define individual and composite error types.
///
/// It also helps implementing `From` and `TryFrom` between them.
//
// NOTES:
// - alternative sections for tuple-struct and field-struct variants are indicated in the margin.
// - we are employing the trick `$(;$($_a:lifetime)?` for the optional semicolon terminator,
//   where the never expected lifetime allows to refer to the non-identifier `;` later on.
macro_rules! define_error {
    (
    // Defines a standalone error tuple-struct with elements.
    individual:
        $(#[$attributes:meta])*
        $struct_vis:vis struct $struct_name:ident
        $(( $($e_vis:vis $e_ty:ty),+ $(,)? ))? $(;$($_a:lifetime)?)?              // tuple-struct↓
        $({ $($(#[$f_attr:meta])* $f_vis:vis $f_name:ident: $f_ty:ty),+ $(,)? })? // field-struct↑
        $DOC_NAME:ident = $doc_str:literal,
        $self:ident + $fmt:ident => $display_expr:expr
        $(,)?
    ) => {
        $(#[$attributes])*
        $crate::CONST! { pub(crate) $DOC_NAME = $doc_str; }

        $(#[$attributes])*
        #[doc = crate::TAG_ERROR!()]
        #[doc = $DOC_NAME!()]
        #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
        $struct_vis struct $struct_name
            $(( $($e_vis $e_ty),+ ) )? $(; $($_a)?)?                              // tuple-struct↓
        $({ $( $(#[$f_attr])* $f_vis $f_name: $f_ty),+ })?                        // field-struct↑

        $(#[$attributes])*
        impl $crate::Display for $struct_name {
            fn fmt(&$self, $fmt: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                $display_expr
            }
        }
        $(#[$attributes])*
        impl $crate::Error for $struct_name {}
        $(#[$attributes])*
        impl $crate::ExtError for $struct_name {
            type Kind = ();
            fn error_eq(&self, other: &Self) -> bool { self == other }
            fn error_kind(&self) -> Self::Kind {}
        }
    };
    (
    // Defines a composite Error enum, as well as:
    // - implements Error, ExtError and Display.
    // - implements From and TryFrom in reverse.
    composite: fmt($fmt:ident)
        $(#[$enum_attr:meta])*
        $vis:vis enum $composite_error_name:ident { $(
            $(#[$variant_attr:meta])*
            $DOC_VARIANT:ident:
            $variant_name:ident
                $(( $($e_name:ident| $e_numb:literal: $e_ty:ty),+ ))?             // tuple-struct↓
                $({ $($(#[$f_attr:meta])* $f_name:ident: $f_ty:ty),+ })?          // field-struct↑
                => $individual_error_name:ident
                    $(( $($e_display_expr:expr),+ ))?                             // tuple-struct↓
                    $({ $($f_display_name:ident: $f_display_exp:expr),+ })?       // field-struct↑
        ),+ $(,)? }
    ) => {
        #[doc = crate::TAG_ERROR_COMPOSITE!()]
        $(#[$enum_attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis enum $composite_error_name { $(
            $(#[$variant_attr])*
            #[doc = $DOC_VARIANT!()]
            $variant_name
                $(( $($e_ty),+ ))?                                                // tuple-struct↓
                $({ $($(#[$f_attr])* $f_name: $f_ty),+ })?                        // field-struct↑
        ),+ }

        // implements Error, ExtError & Display:
        impl $crate::Error for $composite_error_name {}
        impl $crate::ExtError for $composite_error_name {
            type Kind = ();
            fn error_eq(&self, other: &Self) -> bool { self == other }
            fn error_kind(&self) -> Self::Kind {}
        }
        impl $crate::Display for $composite_error_name  {
            fn fmt(&self, $fmt: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                match self { $(
                    $(#[$variant_attr])*
                    $composite_error_name::$variant_name
                    $(( $($e_name),+ ))?                                          // tuple-struct↓
                    $({ $($f_name),+ })?                                          // field-struct↑
                    =>
                        $crate::Display::fmt(&$individual_error_name
                            $(( $($e_display_expr),+ ))?                          // tuple-struct↓
                            $({ $($f_display_name: $f_display_exp),+})?           // field-struct↑
                        , $fmt),
                )+ }
            }
        }
        // implements From multiple individual errors for a composite error,
        // and implements TryFrom in reverse:
        $(
            $(#[$variant_attr])*
            $crate::define_error! { from(_f): $individual_error_name, for: $composite_error_name
                => $variant_name
                $(( $($e_name, $crate::field_of![_f, $e_numb] ),+ ))?             // tuple-struct↓
                $({ $($f_name, $crate::field_of![_f, $f_name] ),+ })?             // field-struct↑
            }
        )+
    };
    (
    // Implements `From` an individual error type to a composite error containing it,
    // as well as implementing `TryFrom` in reverse.
    from($fn_arg:ident): $from_individual:ident, for: $for_composite:ident
    => $variant_name:ident
        $(( $($e_name:ident, $e_expr:expr),+ ))?                                  // tuple-struct↓
        $({ $($f_name:ident,  $f_expr:expr),+ })?                                 // field-struct↑
    $(,)? ) => {
        $crate::paste! {
            impl $for_composite {
                #[doc = "*const* version of `From<" $from_individual "> for " $for_composite "`."]
                #[allow(dead_code, reason = "seldomly used")]
                // EXPERIMENTAL:IMPROVE make sure to only make it const if the type allows it
                pub const fn [<from_ $from_individual:snake:lower>]($fn_arg: $from_individual)
                -> $for_composite {
                    $for_composite::$variant_name
                        $(( $($e_expr),+ ))?                                      // tuple-struct↓
                        $({ $($f_name: $f_expr),+ })?                             // field-struct↑
                }
            }
        }

        /* (example)
        impl From<NotEnoughElements> for DataNotEnough {
            fn from (_f: NotEnoughElements) -> DataNotEnough {
                DataNotEnough::$Elements(_f.0)
            }
        } */
        impl From<$from_individual> for $for_composite {
            fn from($fn_arg: $from_individual) -> $for_composite {
                $for_composite::$variant_name
                    $(( $($e_expr),+ ))?                                          // tuple-struct↓
                    $({ $($f_name: $f_expr),+ })?                                 // field-struct↑
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
                        $(( $($e_name),+ ))?                                      // tuple-struct↓
                        $({ $($f_name),+ })?                                      // field-struct↑
                        => Ok($from_individual
                            $(( $($e_name),+ ))?                                  // tuple-struct↓
                            $({ $($f_name),+ })?                                  // field-struct↑
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
            $(( $($from_elem:ident),+ ))?                                         // tuple-struct↓
            $({ $($from_field:ident),+ })?                                        // field-struct↑
                => $for_variant:ident
                    $(( $($for_elem:ident),+ ))?                                  // tuple-struct↓
                    $({ $($for_field:ident),+ })?                                 // field-struct↑
    ),+ $(,)? }
    ) => {
        impl From<$from_subset> for $for_superset {
            fn from($fn_arg: $from_subset) -> $for_superset { match $fn_arg { $(
                $from_subset::$from_variant
                    $(( $($from_elem),+ ))?                                       // tuple-struct↓
                    $({ $($from_field),+ })?                                      // field-struct↑
                    => $for_superset::$for_variant
                        $(( $($for_elem),+ ))?                                    // tuple-struct↓
                        $({ $($for_field),+ })?                                   // field-struct↑
            ),+ } }
        }
        impl TryFrom<$for_superset> for $from_subset {
            type Error = crate::FailedErrorConversion;
            fn try_from($fn_arg: $for_superset)
                -> Result<$from_subset, crate::FailedErrorConversion> { match $fn_arg { $(
                    $for_superset::$for_variant
                        $(( $($for_elem),+ ))?                                    // tuple-struct↓
                        $({ $($for_field),+ })?                                   // field-struct↑
                            => Ok($from_subset::$from_variant
                                $(( $($from_elem),+ ))?                           // tuple-struct↓
                                $({ $($from_field),+ })?                          // field-struct↑
                    ),)+
                    _ => Err(crate::FailedErrorConversion)
                }
            }
        }
    };
}
pub(crate) use define_error;

#[cfg(test)]
mod tests {
    use super::define_error;

    #[test]
    fn define_error() {
        /* define individual errors */

        define_error! { individual: pub struct UnitStruct;
            DOC_UNIT_STRUCT = "docstring", self+f => write!(f, "display"),
        }
        define_error! { individual: pub struct SingleElement(pub(crate) Option<u8>);
            DOC_SINGLE_ELEMENT = "docstring", self+f => write!(f, "display"),
        }
        define_error! { individual: pub struct MultipleElements(pub i32, u32,);
            DOC_MULTI_ELEMENT = "docstring", self+f => write!(f, "display")
        }
        define_error! { individual: pub struct StructFields {
                #[doc = "field1"] pub f1: bool,
                #[doc = "field2"] f2: Option<char>
            }
            DOC_STRUCT_FIELDS = "docstring", self+f => write!(f, "display")
        }

        /* define composite errors
         * (includes conversions between variants and indidual errors) */

        define_error! { composite: fmt(f)
            /// A composite error superset.
            pub enum CompositeSuperset {
                DOC_UNIT_STRUCT: SuperUnit => UnitStruct,
                DOC_SINGLE_ELEMENT: SuperSingle(i|0: Option<u8>) => SingleElement(*i),
                DOC_MULTI_ELEMENT: SuperMultiple(i|0: i32, j|1: u32) => MultipleElements(*i, *j),
                DOC_STRUCT_FIELDS: SuperStruct { f1: bool, f2: Option<char> }
                    => StructFields { f1: *f1, f2: *f2 },
            }
        }
        define_error! { composite: fmt(f)
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

        define_error! { composite: from(f): CompositeSubset, for: CompositeSuperset {
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
