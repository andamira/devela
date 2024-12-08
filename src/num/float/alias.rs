// devela::num::float::alias

#![allow(non_camel_case_types)]

macro_rules! impl_fsize {
    () => {
        impl_fsize![f32, "32"];
        impl_fsize![f64, "64"];
        #[cfg(feature = "nightly_float")]
        impl_fsize![::core::f16, "16"];
        #[cfg(feature = "nightly_float")]
        impl_fsize![::core::f128, "128"];
    };
    ($float:ty , $pointer_width:literal) => {
        #[doc = crate::doc_primitive!()]
        /// A pointer-sized floating-point primitive.
        ///
        /// # Features
        /// Makes use of `nightly_float` in 16-bit architectures.
        // #[cfg_attr(
        //     feature = "nightly_doc",
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
