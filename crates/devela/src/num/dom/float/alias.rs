//
//! Defines [`fsize`].
//

#![allow(non_camel_case_types)]

macro_rules! _num_dom_float_define_fsize {
    () => {
        _num_dom_float_define_fsize![f32, "32"];
        _num_dom_float_define_fsize![f64, "64"];
        #[cfg(nightly_float)]
        _num_dom_float_define_fsize![::core::f16, "16"];
    };
    ($float:ty , $pointer_width:literal) => {
        #[doc = crate::_tags!(primitive num)]
        /// A pointer-sized floating-point primitive.
        #[doc = crate::_doc_meta!{location("num/dom/float", type fsize)}]
        /// # Features
        /// `nightly_float` enables 16-bit pointer targets.
        #[cfg(target_pointer_width = $pointer_width)]
        #[cfg_attr(nightly_doc, doc(cfg(all())))]
        pub type fsize = $float;
    };
}
_num_dom_float_define_fsize![];
