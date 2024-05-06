// devela::lex::macros
//
//! Private macros.
//

#![allow(unused)]

// define type aliases for specific sizes.
macro_rules! impl_sized_alias {
    // $alias: the base name for the type alias.
    // $type: the base name of the original type.
    // $doc1: first doc text.
    // $doc2: second doc text.
    // $(
    //   $bits_det: determinant for the number of bits.
    //   $bits: number of bits.
    //   $bytes: number of bytes.
    //   $byte_plu: plural for the number of bytes.
    // )
    ($alias:ident, $type:ident, $doc1:literal, $doc2:literal:
     $($bits_det:literal $bits:literal, $bytes:literal $bytes_plu:literal);+ $(;)? ) => {
        $(
        $crate::lex::helpers::impl_sized_alias![
            @$alias, $type, $doc1, $doc2: $bits_det $bits, $bytes $bytes_plu
        ];
        )+
    };
    (@$alias:ident, $type:ident, $doc1:literal, $doc2:literal:
     $bits_det:literal $bits:literal, $bytes:literal $bytes_plu:literal) => { $crate::paste! {
        #[doc = "" $bits_det " " $bits "-bit sized " $doc1 $bytes " byte" $bytes_plu $doc2]
        pub type [<$alias $bits>] = $type<$bytes>;
    }};
}
pub(crate) use impl_sized_alias;
