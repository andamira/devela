//
//! Application-specific filesystem paths across host platforms.
//

crate::mods_in! {
    #[cfg(feature = "alloc")]
    mod info;
    #[cfg(feature = "std")]
    mod paths;
    #[cfg(feature = "std")]
    mod xdg;
    #[cfg(feature = "std")]
    mod unix;
    #[cfg(feature = "std")]
    mod apple;
    #[cfg(feature = "std")]
    mod windows;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "alloc")]
        pub use super::{
            info::AppInfo,
        };
        #[cfg(feature = "std")]
        pub use super::{
            paths::AppPaths,
            xdg::AppPathsXdg,
            unix::AppPathsUnix,
            apple::AppPathsApple,
            windows::AppPathsWindows,
        };
    }
}
