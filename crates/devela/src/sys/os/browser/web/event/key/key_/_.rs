crate::mods_in! {
    mod define; // WebEventKey

    mod compact;
    mod impls;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::WebEventKey,
        };
    }
}
