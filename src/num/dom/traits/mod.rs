// devela::num::dom::traits
//
//!
//

mod constants; // NumConst

#[cfg(feature = "num")]
crate::items! {
    mod num; // Num
    mod r#ref; // NumRef

    mod impls;

    impl<T: Num> NumRef<'_> for &T { type Own = T; }
    impl<T: Num> NumRef<'_> for &mut T { type Own = T; }
}

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            constants::*,
        };
        #[cfg(feature = "num")]
        pub use super::{
            num::*,
            r#ref::*,
        };
    }
}
