// devela_base_core::code::error::define_error
//
//! Defines [`define_error!`].
//
// TOC
// - define_error!
// - tests
//
// IMPROVE: make the automatic error tag optional.
// IMPROVE: do not depend on included types being Copy.
// IMPROVE: make it possibe to share publicly (conditional compilation, macro_export arms).

#[doc = crate::_TAG_CONSTRUCTION!()]
/// Defines individual and composite error types.
#[doc = crate::_doc!(location: "code/error")]
///
/// It can also implement `From` and `TryFrom` traits between them.
///
/** # Example
```
# use devela_base_core::define_error;
// Define simple individual error types
define_error! {
    individual:
    /// A basic error with a code
    pub struct ErrorCode(pub u32);
    DOC_ERROR_CODE = "An error with a numeric code",
    self + f => write!(f, "Error code: {}", self.0)
}
define_error! {
    individual:
    /// An error with message details
    pub struct ErrorDetails {
        pub code: u32,
        pub message: &'static str,
    }
    DOC_ERROR_DETAILS = "An error with code and message details",
    self + f => write!(f, "Error {}: {}", self.code, self.message)
}

// Define a composite error
define_error! {
    composite: fmt(f)
    /// Container for multiple error types
    pub enum CompositeError {
        DOC_ERROR_CODE: +const CodeVariant(code|0: u32) => ErrorCode(*code),
        DOC_ERROR_DETAILS: // ← this variant won't have const conversion
            DetailsVariant { code: u32, message: &'static str } =>
                ErrorDetails { code: *code, message: *message }
    }
}

// Create individual errors
let code_error = ErrorCode(42);
let details_error = ErrorDetails {
    code: 100,
    message: "Example error",
};

// Automatic conversion to composite error
let composite_from_code: CompositeError = code_error.into();
let composite_from_details: CompositeError = details_error.into();

// Try to convert back (only works for matching variants)
match ErrorCode::try_from(composite_from_code) {
    Ok(original) => println!("Successfully converted back: {}", original),
    Err(_) => println!("Conversion failed - wrong variant"),
}

// Const conversion available for +const variants
const CONST_ERROR: CompositeError = CompositeError::from_error_code(ErrorCode(99));
```
**/
// NOTES:
// - alternative sections for tuple-struct and field-struct variants are indicated in the margin.
// - we are employing the trick `$(;$($_a:lifetime)?)?` for the optional semicolon terminator,
//   where the never expected lifetime allows to refer to the non-identifier `;` later on;
//   the same with `$(+const$($_c:lifetime)?)?` for the optional const fn implementation.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _define_error {
    (
    // Defines a standalone error struct with optional tuple or field variants.
    //
    // # Arguments
    // - `attributes`  :   Optional attributes applied to struct and implementations
    // - `struct_vis`  :   Struct visibility (e.g., `pub`, `pub(crate)`)
    // - `struct_name` :   Name of the error struct
    //
    // ## Struct Body (choose one):
    // - Unit variant  :  No body (just struct name)
    // - Tuple variant : `(vis1 Type1, vis2 Type2, ...)`
    // - Field variant : `{ #[attr] vis1 field1: Type1, #[attr] vis2 field2: Type2, ... }`
    //
    // ## Documentation:
    // - `+tag: "tag text",`              : Optional additional doc tag
    // - `DOC_CONST_NAME = "doc string",` : Constant name and documentation string
    //
    // ## Display Implementation:
    // - `self_var + fmt_var => expression`: Display implementation where:
    //   - `self_var`   : Name for self reference in display method
    //   - `fmt_var`    : Name for formatter in display method
    //   - `expression` : Expression writing to formatter
    individual:
        $(#[$attributes:meta])*
        $struct_vis:vis struct $struct_name:ident
        $(( $($e_vis:vis $e_ty:ty),+ $(,)? ))? $(;$($_a:lifetime)?)?              // tuple-struct↓
        $({ $($(#[$f_attr:meta])* $f_vis:vis $f_name:ident: $f_ty:ty),+ $(,)? })? // field-struct↑
        $(+location: $($location:literal)+ ,)?
        $(+tag: $($tag:expr)+ ,)?
        $DOC_NAME:ident = $doc_str:literal,
        $self:ident + $fmt:ident => $display_expr:expr
        $(,)?
    ) => {
        $(#[$attributes])*
        // IMPROVE: make it possibe to share publicly (conditional compilation, macro_export arms)
        $crate::CONST! { pub(crate) $DOC_NAME = $doc_str; }

        $( $(#[doc = $tag])+ )?
        #[doc = $crate::_TAG_ERROR!()] // IMPROVE: make optional
        $(#[$attributes])*
        #[doc = $DOC_NAME!()]
        $(#[doc = $crate::_doc![location_inline: $($location)?]])? // canonical location
        #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
        $struct_vis struct $struct_name
            $(( $($e_vis $e_ty),+ ) )? $(; $($_a)?)?                              // tuple-struct↓
        $({ $( $(#[$f_attr])* $f_vis $f_name: $f_ty),+ })?                        // field-struct↑

        $(#[$attributes])*
        impl $crate::Error for $struct_name {}
        $(#[$attributes])*
        impl $crate::Display for $struct_name {
            fn fmt(&$self, $fmt: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                $display_expr
            }
        }
    };
    (
    // Defines a composite error enum that aggregates multiple individual error types.
    //
    // # Arguments
    // - `fmt($fmt:ident)`      : Display formatter variable name for the Display implementation
    // - `enum_attr`            : Optional attributes applied to the enum declaration
    // - `vis`                  : Enum visibility (e.g., `pub`, `pub(crate)`)
    // - `composite_error_name` : Name of the composite error enum
    //
    // ## Variant Definitions (one or more):
    // For each variant:
    // - `+tag: "tag text",` : Optional additional doc tag for the variant
    // - `variant_attr`      : Optional attributes for the variant
    // - `DOC_VARIANT`       : Constant name for variant documentation
    // - `+const`            : Optional flag to generate const conversion methods
    // - `variant_name`      : Name of the variant
    //
    // ## Variant Data (choose one per variant):
    // - Unit variant  : No data (just the variant name)
    // - Tuple variant : `(field_name|index: field_type, ...)`
    // - Field variant : `{ #[attr] field_name: field_type, ... }`
    //
    // ## Conversion Mapping:
    // - `=> individual_error_name`: Individual error type this variant maps to
    // - Display expression mapping (matches variant data structure):
    //   - Unit  : No mapping needed
    //   - Tuple : `(expression_for_field1, expression_for_field2, ...)`
    //   - Field : `{ field_name: expression_for_field, ... }`
    //
    // # Generates:
    // - Composite enum with specified variants (unit, tuple or field)
    // - Error and Display implementations
    // - From/TryFrom implementations for each individual error type
    // - Optional const conversion methods for variants marked with +const
    composite: fmt($fmt:ident)
        $(+location: $($location:literal)+ ,)?
        $(+tag: $tag:expr ,)?
        $(#[$enum_attr:meta])*
        $vis:vis enum $composite_error_name:ident { $(
            $(+tag: $tag_variant:expr ,)?
            $(#[$variant_attr:meta])*
            $DOC_VARIANT:ident:
            $(+const$($_c:lifetime)?)?
            $variant_name:ident
                $(( $($e_name:ident| $e_numb:literal: $e_ty:ty),+ ))?             // tuple-struct↓
                $({ $($(#[$f_attr:meta])* $f_name:ident: $f_ty:ty),+ })?          // field-struct↑
                => $individual_error_name:ident
                    $(( $($e_display_expr:expr),+ ))?                             // tuple-struct↓
                    $({ $($f_display_name:ident: $f_display_exp:expr),+ })?       // field-struct↑
        ),+ $(,)? }
    ) => {
        $(#[doc = $tag])?
        #[doc = $crate::_TAG_ERROR_COMPOSITE!()] // IMPROVE: make optional
        $(#[$enum_attr])*
        $(#[doc = $crate::_doc![location_inline: $($location)?]])? // canonical location
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis enum $composite_error_name { $(
            $(#[doc = $tag_variant])?
            #[doc = $crate::_TAG_ERROR!()] // IMPROVE: make optional
            $(#[$variant_attr])*
            #[doc = $DOC_VARIANT!()]
            $variant_name
                $(( $($e_ty),+ ))?                                                // tuple-struct↓
                $({ $($(#[$f_attr])* $f_name: $f_ty),+ })?                        // field-struct↑
        ),+ }

        // implements Error, & Display:
        impl $crate::Error for $composite_error_name {}
        impl $crate::Display for $composite_error_name  {
            fn fmt(&self, $fmt: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                match self { $(
                    $(#[$variant_attr])*
                    #[allow(unused_doc_comments, reason = "repeated here for feature-gating")]
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
            $crate::define_error! { $(+const$($_c)?)?
                from(_f): $individual_error_name, for: $composite_error_name => $variant_name
                $(( $($e_name, $crate::field_of![_f, $e_numb] ),+ ))?             // tuple-struct↓
                $({ $($f_name, $crate::field_of![_f, $f_name] ),+ })?             // field-struct↑
            }
        )+
    };
    (
    // Implements `From` an individual error type to a composite error containing it, and
    // implements `TryFrom` in reverse. Optionally implements a `const fn from_individual` method.
    $(+const$($_c:lifetime)?)?
    from($fn_arg:ident): $from_individual:ident, for: $for_composite:ident
    => $variant_name:ident
        $(( $($e_name:ident, $e_expr:expr),+ ))?                                  // tuple-struct↓
        $({ $($f_name:ident,  $f_expr:expr),+ })?                                 // field-struct↑
    $(,)? ) => {
        $crate::paste! {
            impl $for_composite {
                $( #[doc = "*const*"] $($_c)?)?
                #[doc = "method equivalent to `From<" $from_individual "> for " $for_composite "`."]
                #[allow(dead_code, reason = "seldomly used")]
                pub $(const$($_c)?)? fn
                    [<from_ $from_individual:snake:lower>]($fn_arg: $from_individual)
                    -> $for_composite {
                        $for_composite::$variant_name
                            $(( $($e_expr),+ ))?                                  // tuple-struct↓
                            $({ $($f_name: $f_expr),+ })?                         // field-struct↑
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
            type Error = $crate::FailedErrorConversion;
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
                        _ => Err($crate::FailedErrorConversion)
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
            type Error = $crate::FailedErrorConversion;
            fn try_from($fn_arg: $for_superset)
                -> Result<$from_subset, $crate::FailedErrorConversion> { match $fn_arg { $(
                    $for_superset::$for_variant
                        $(( $($for_elem),+ ))?                                    // tuple-struct↓
                        $({ $($for_field),+ })?                                   // field-struct↑
                            => Ok($from_subset::$from_variant
                                $(( $($from_elem),+ ))?                           // tuple-struct↓
                                $({ $($from_field),+ })?                          // field-struct↑
                    ),)+
                    _ => Err($crate::FailedErrorConversion)
                }
            }
        }
    };
}
#[doc(inline)]
pub use _define_error as define_error;

#[cfg(test)]
mod tests {
    use super::define_error;

    #[test]
    fn define_error() {
        /* define individual errors */

        define_error! { individual: pub struct UnitStruct;
            +tag: "tag",
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
         * (includes conversions between variants and indidual errors)
         * (also enables some const conversion method implementations) */

        define_error! { composite: fmt(f)
            /// A composite error superset.
            pub enum CompositeSuperset {
                +tag: "tag",
                DOC_UNIT_STRUCT: +const SuperUnit => UnitStruct,
                DOC_SINGLE_ELEMENT: SuperSingle(i|0: Option<u8>) => SingleElement(*i),
                DOC_MULTI_ELEMENT: SuperMultiple(i|0: i32, j|1: u32) => MultipleElements(*i, *j),
                DOC_STRUCT_FIELDS: +const SuperStruct { f1: bool, f2: Option<char> }
                    => StructFields { f1: *f1, f2: *f2 },
            }
        }
        define_error! { composite: fmt(f)
            /// A composite error subset.
            // NOTE: removed the unit struct variant (the most trivial case)
            pub enum CompositeSubset {
                DOC_SINGLE_ELEMENT: SubSingle(i|0: Option<u8>) => SingleElement(*i),
                DOC_MULTI_ELEMENT: SubMultiple(i|0: i32, j|1: u32) => MultipleElements(*i, *j),
                DOC_STRUCT_FIELDS: +const SubStruct { f1: bool, f2: Option<char> }
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

        assert![CompositeSubset::try_from(CompositeSuperset::SuperUnit).is_err()];
    }
}
