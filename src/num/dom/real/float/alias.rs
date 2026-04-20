// devela::num::dom::real::float::alias
//
//! Defines [`fsize`].
//

#![allow(non_camel_case_types)]

macro_rules! _num_dom_real_float_define_fsize {
    () => {
        _num_dom_real_float_define_fsize![f32, "32"];
        _num_dom_real_float_define_fsize![f64, "64"];
        #[cfg(nightly_float)]
        _num_dom_real_float_define_fsize![::core::f16, "16"];
        #[cfg(nightly_float)]
        _num_dom_real_float_define_fsize![::core::f128, "128"];
    };
    ($float:ty , $pointer_width:literal) => {
        #[doc = crate::_tags!(primitive num)]
        /// A pointer-sized floating-point primitive.
        #[doc = crate::_doc_location!("num/dom/real")]
        ///
        /// # Features
        /// Makes use of `nightly_float` in 16-bit architectures.
        // #[cfg_attr(
        //     nightly_doc,
        //     doc(cfg(any(
        //         target_pointer_width = "16",
        //         target_pointer_width = "32",
        //         target_pointer_width = "64"
        //     )))
        // )]
        #[cfg(target_pointer_width = $pointer_width)]
        pub type fsize = $float;
    };
}
_num_dom_real_float_define_fsize![];
