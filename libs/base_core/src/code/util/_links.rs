// devela_base_core::code::util::_links
//
//! Private common intra-doc links definitions built using the [`_doclink`] macro
//

#![allow(clippy::crate_in_macro_def, reason = "can't work with $crate")]

use crate::CONST;

// intradoc links

CONST! { hidden macro_export,
    _DOCLINK_CONST_DEFAULT =
        crate::doclink!(custom devela "[`ConstDefault`]" "code/trait.ConstDefault.html");
}
