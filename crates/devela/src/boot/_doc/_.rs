//
//! Essential documentation machinery and root-independent fallbacks.
//

mod doclink;

#[doc(hidden)]
#[macro_export]
macro_rules! _DOC_ALL· {
    () => {
        concat!(
            "All crate items re-exported in a single flat namespace.\n\n",
            "Every item in this namespace is forwarded upward, as every other public module does."
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _DOC_ALL_· {
    () => {
        "All crate items re-exported, grouped by their root modules."
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _DOC_ALL_PLUS_· {
    () => {
        concat!(
            "\n\n", // prevents the following line to appear in the Modules list
            "Each root module appears here and provides its own flat view ",
            "of all its public children.\n\n",
            "Click on the `▽` symbol after the list of submodules ",
            "to switch to hierarchical view, and `◉` to switch back to flat view."
        )
    };
}
#[doc(hidden)]
pub use {_DOC_ALL_PLUS_· as _DOC_ALL_PLUS_, _DOC_ALL_· as _DOC_ALL_, _DOC_ALL· as _DOC_ALL};

crate::boot::meta::CONST! { hidden macro_export,
    _DOC_BOOT = "Bootstrap vocabulary.";
    _DOC_BOOT_MODULES = "\
        <br/><i style='margin-left:0em;'></i>\
        <span style='font-size:90%;word-spacing:0px'>\
        [boot][mod@crate::boot]::{\
        [base](mod@crate::boot::base), \
        [meta](mod@crate::boot::meta), \
        [rust](mod@crate::boot::rust)}</span>";
}

/* shims */

#[cfg(not(root_yard))]
macro_rules! _doc_meta {
    ($($tt:tt)*) => {
        ""
    };
}
#[cfg(not(root_yard))]
macro_rules! _tags {
    ($($tt:tt)*) => {
        ""
    };
}
#[cfg(not(root_yard))]
pub(crate) use {_doc_meta, _tags};

pub(crate) mod _all {
    pub use super::doclink::{__DOCLINK_CUSTOM_CHANNEL, __DOCLINK_CUSTOM_DOMAIN, doclink};
    #[doc(hidden)]
    pub use super::{_DOC_ALL, _DOC_ALL_, _DOC_ALL_PLUS_, _DOC_BOOT, _DOC_BOOT_MODULES};
}
pub(crate) mod _crate_internals {
    #[cfg(not(root_yard))]
    pub(crate) use super::{_doc_meta, _tags};
}
