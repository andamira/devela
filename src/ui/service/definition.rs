// devela::ui::service::definition

use crate::ui::UiCap;

/// Common trait for all UI services.
pub trait UiService {
    /// Returns the service capabilities.
    fn capabilities(&self) -> UiCap;

    /// Returns the service inner version numbers [major, minor, patch].
    fn version(&self) -> [u32; 3];

    /* auto impls */

    /// Returns the service version string.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    fn version_string(&self) -> crate::_liballoc::string::String {
        let v = self.version();
        crate::_liballoc::format!["v{}.{}.{}", v.0, v.1, v.2]
    }
}
