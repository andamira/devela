// devela_base_core::num::float::alias
//
//! Defines [`fsize`].
//

#![allow(non_camel_case_types)]

macro_rules! impl_fsize {
    () => {
        impl_fsize![f32, "32"];
        impl_fsize![f64, "64"];
        #[cfg(nightly_float)]
        impl_fsize![::core::f16, "16"];
        #[cfg(nightly_float)]
        impl_fsize![::core::f128, "128"];
    };
    ($float:ty , $pointer_width:literal) => {
        #[doc = crate::_TAG_PRIMITIVE!()]
        #[doc = crate::_TAG_NUM!()]
        /// A pointer-sized floating-point primitive.
        #[doc = crate::_doc_location!("num")]
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
impl_fsize![];
