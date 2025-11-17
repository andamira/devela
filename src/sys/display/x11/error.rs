// devela::sys::display::x11::error
//
//! Defines [`XError`].
//

use crate::{Display, Error, FmtResult, Formatter};

/// XCB/X11 error categories.
///
/// Represents the main classes of failures that can occur when interacting with
/// the X server through XCB. These variants cover connection issues, setup failures,
/// missing extensions, raw protocol errors, and other unclassified conditions.
#[derive(Debug)]
pub enum XError {
    /// Failed to establish a connection to the X server.
    ConnectionFailed,
    /// Failed to retrieve the server setup information.
    SetupFailed,
    /// No screens were reported by the X server.
    NoScreensFound,
    /// A required X11 extension is not available.
    ExtensionUnavailable(&'static str),
    /// Protocol-level error code returned by the X server.
    ProtocolError(u8),
    /// Any other error condition not covered by the variants above.
    Other(&'static str),
}

impl Error for XError {}
impl Display for XError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        use XError as E;
        match self {
            E::ConnectionFailed => f.write_str("Cannot open display"),
            E::SetupFailed => f.write_str("Xcb::get_setup failed"),
            E::NoScreensFound => f.write_str("no screens found"),
            E::ExtensionUnavailable(ext) => write!(f, "Extension unavailable: '{ext}'"),
            E::ProtocolError(err) => write!(f, "Protocol error: '{err}'"),
            E::Other(s) => f.write_str(s),
        }
    }
}
