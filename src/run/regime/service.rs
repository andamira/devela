// devela::run::regime::service
//
//! Runtime service capability traits.
//!
//! Defines [`RunService`] for reporting the currently known capabilities of a
//! runtime service, and [`RunServiceProbe`] for services that can actively
//! refresh that information.
//

use crate::{RunCap, VersionFull};

#[doc = crate::_tags!(runtime)]
/// Reports runtime service metadata.
#[doc = crate::_doc_meta!{location("run/regime")}]
///
/// A run service is an execution-facing backend, frontend, driver,
/// or platform adapter that can describe the capabilities it currently exposes.
///
/// Capability reporting is intentionally passive:
/// [`run_capabilities`] returns the currently known snapshot
/// and should not perform probing, blocking I/O, or backend mutation.
///
/// Use [`RunServiceProbe`] when a service can actively refresh its capability snapshot.
///
/// [`run_capabilities`]: Self::run_capabilities
pub trait RunService {
    /// Returns the currently known service capabilities.
    ///
    /// This should be a cheap, passive snapshot.
    /// Implementations should not perform probing, blocking I/O, or backend mutation here.
    ///
    /// For services whose capabilities may need active detection,
    /// use [`RunServiceProbe::run_capabilities_refresh`].
    fn run_capabilities(&self) -> RunCap;

    /// Returns service version metadata.
    ///
    /// This describes the service or backend implementation,
    /// not necessarily the crate version.
    fn run_version(&self) -> VersionFull<'_>;
}

#[doc = crate::_tags!(runtime)]
/// Actively refreshes runtime service capabilities.
#[doc = crate::_doc_meta!{location("run/regime")}]
///
/// This extension trait is for services whose capabilities may require probing,
/// querying, negotiation, or backend interaction.
///
/// Implementations may perform I/O, mutate cached service state,
/// or fail while refreshing the capability snapshot.
pub trait RunServiceProbe: RunService {
    /// Error returned while refreshing service capabilities.
    type Error;

    /// Refreshes and returns the currently known service capabilities.
    ///
    /// Unlike [`RunService::run_capabilities`], this method may actively probe
    /// the backend, update internal cached state, block briefly, or fail.
    fn run_capabilities_refresh(&mut self) -> Result<RunCap, Self::Error>;
}
