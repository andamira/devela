// devela::run::driver::error
//
//! Defines [`RunDriverError`], [`RunDriverFrameError`].
//

use crate::RunPhase;

#[doc = crate::_tags!(runtime error)]
/// Errors returned while driving a runtime step.
#[doc = crate::_doc_location!("run")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RunDriverError<BE, AE> {
    /// The runtime cannot advance in its current phase.
    InvalidPhase(RunPhase),
    /// The backend failed while collecting events or exposing frame context.
    Backend(BE),
    /// The app failed while processing a runtime step.
    App(AE),
}

#[doc = crate::_tags!(runtime error)]
/// Errors returned while driving a runtime frame step.
#[doc = crate::_doc_location!("run")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RunDriverFrameError<BE, AE, RE, PE> {
    /// The runtime cannot advance in its current phase.
    InvalidPhase(RunPhase),
    /// The backend failed while collecting events or exposing frame context.
    Backend(BE),
    /// The app failed while processing a runtime step.
    App(AE),
    /// Rendering failed for the current frame.
    Render(RE),
    /// Presentation failed for the current frame.
    Present(PE),
}
