// devela::ui::service::definition
//
//! Defines the [`UiService`] trait.
//

use crate::UiCap;

/// Common trait for all UI services.
pub trait UiService {
    /// Returns the service capabilities.
    fn capabilities(&self) -> UiCap;

    /// Returns the service inner version numbers (major, minor, patch).
    fn version(&self) -> (u32, u32, u32);

    /* auto impls */

    /// Returns the service version string.
    // IMPROVE: Use StringU8<16>
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    fn version_string(&self) -> crate::String {
        let v = self.version();
        crate::format!["v{}.{}.{}", v.0, v.1, v.2]
    }
}
