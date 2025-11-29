// devela::num::traits
//
//!
//

mod num; // Num
mod r#ref; // NumRef

mod impls;

crate::sf! {
    impl<T: Num> NumRef<'_> for &T { type Own = T; }
    impl<T: Num> NumRef<'_> for &mut T { type Own = T; }
}

crate::structural_mods! { //_mods
    _mods {
        pub use super::{
            num::*,
            r#ref::*,
        };

        // re-exports
        pub use devela_base_core::num::{
            NumConst,
        };
    }
}
