//
//! Minimal documentation fragments for crate-root re-export views.
//

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
