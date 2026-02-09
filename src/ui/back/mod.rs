// revela::ui::back
//
#![doc = crate::_DOC_UI_BACK!()] // private
#![doc = crate::_doc!(modules: crate::ui; back)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

mod definition; // UiService

crate::structural_mods! { // _mods
    _mods {
        #[cfg(ui··)]
        pub use super::definition::*;
    }
}
