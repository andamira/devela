// devela/src/num/dom/traits/_.rs
//
//!
//

crate::mods_in! {
    mod constants; // NumConst

    #[cfg(feature = "num")]
    mod num; // Num
    #[cfg(feature = "num")]
    mod r#ref; // NumRef

    #[cfg(feature = "num")]
    mod impls;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            constants::NumConst,
        };
        #[cfg(feature = "num")]
        pub use super::{
            num::*,
            r#ref::*,
        };
    }
}

// IMPROVE
#[cfg(feature = "num")]
impl<T: Num> NumRef<'_> for &T {
    type Own = T;
}
#[cfg(feature = "num")]
impl<T: Num> NumRef<'_> for &mut T {
    type Own = T;
}
