// devela::run::regime::info
//
//! Identity and profile data.
//

#[cfg(feature = "alloc")]
crate::items! {
    use crate::{test_size_of, String};

    #[doc = crate::_tags!(runtime)]
    /// Runtime system identity metadata.
    #[doc = crate::_doc_location!("run/regime")]
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct RunSystemInfo {
        /// Detected operating system or platform version.
        pub os_version: Option<String>,

        /// Current user name, if known.
        pub user_name: Option<String>,

        /// Current host name, if known.
        pub host_name: Option<String>,
    }
    test_size_of![RunSystemInfo = 72]; // 576 bits
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
