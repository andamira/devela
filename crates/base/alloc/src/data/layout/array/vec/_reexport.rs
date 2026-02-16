// devela_base_alloc::data::layout::array::vec::_reexport

use crate::{_reexport, _tags};

_reexport! { rust: alloc::vec,
    location: "data/layout/array",
    tag: _tags!(data_structure),
    doc: "A contiguous growable array.",
    Vec
}

// NOTE: the macro and the module have the same name
//
#[doc = _tags!(construction data_structure)]
/// <span class='stab portability' title='re-exported from rust&#39;s `alloc`'>`alloc`</span>
/// Creates a [`Vec`] containing the arguments.
#[doc = crate::_doc_location!("data/layout/array")]
///
#[doc = "*Re-exported from [`alloc::vec`][macro@alloc::vec]*."]
#[doc = "\n\n---"]
///
/// The reason of the `_` suffix is to avoid conflicting with Rust's prelude
/// when glob importing from this crate. Since this macro has the same name
/// as its sibling module `std::vec`, in order to be able to re-export
/// only the macro we have to wrap it with our own.
#[macro_export]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _vec_ { ($($tt:tt)*) => { ::alloc::vec![$($tt)*] } }

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use _vec_ as vec_;
