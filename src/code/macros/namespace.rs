// devela::code::macros::namespace
//
//! namespacing functionality
//

/// Helps re-exporting standalone functions as methods implemented over a type.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! namespace_fns {
    (
        /* syntax */

        // $namespace                 : the type for namespacing, e.g a struct.
        // $( $fn_shared_attrs )+ +)? : optional list of shared attributes.
        // $( $fn_attrs )*            : optional list of $fn_name attributes.
        // $fn_vis                    : the visibility of the function. E.g. pub.
        // $fn_name                   : the name of the function.
        // $(                         : optional list of
        //   $arg                     : function argument name…
        //   $arg_ty                  : …and the argument's type.
        // ),*
        //
        $namespace:ty,
        $(#[$fn_shared_attrs:meta])*
        $(
            $(#[$fn_attrs:meta])*
            $fn_vis:vis fn $fn_name:ident($( $arg:ident : $arg_ty:ty ),*) -> $ret_ty:ty
        ),*
    ) => {
        impl $namespace {
            $(

                $( #[$common_fn_attrs] )*
                $( #[$fn_attrs] )*
                #[inline]
                $fn_vis fn $fn_name($( $arg: $arg_ty ),*) -> $ret_ty {
                    $fn_name($( $arg ),*)
                }
            )*
        }
    };
}
#[doc(inline)]
pub use namespace_fns;
