// devela/src/run/regime/info.rs
//
//! Identity and profile data.
//

#[cfg(feature = "alloc")]
crate::items! {
    use crate::String;

    #[doc = crate::_tags!(runtime)]
    /// Runtime system identity metadata.
    #[doc = crate::_doc_meta!{
        location("run/regime"),
        #[cfg(target_pointer_width = "32")]
        test_size_of(RunSystemInfo = 36|288),
        #[cfg(target_pointer_width = "64")]
        test_size_of(RunSystemInfo = 72|576),
    }]
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    #[crate::macro_apply(crate::__doc_show(feature = "alloc"))]
    pub struct RunSystemInfo {
        /// Detected operating system or platform version.
        pub os_version: Option<String>,

        /// Current user name, if known.
        pub user_name: Option<String>,

        /// Current host name, if known.
        pub host_name: Option<String>,
    }
}

// FUTURE
/*
pub struct RunSystemInfoRef<'a> {
    pub os_version: Option<&'a str>,
    pub user_name: Option<&'a str>,
    pub host_name: Option<&'a str>,
}
pub struct RunSystemInfoId {
    pub os_version: Option<StringId>,
    pub user_name: Option<StringId>,
    pub host_name: Option<StringId>,
}
*/
