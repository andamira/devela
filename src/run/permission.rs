// devela/src/run/permission.rs
//
//! Defines [`PermissionState`], [`PermissionError`].
//

use crate::AsyncPoll;

#[doc = crate::_tags!(error runtime)]
/// An error while determining or changing a permission state.
#[doc = crate::_doc_meta!{location("run/permission")}]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PermissionError {
    /// The permission or requested permission operation is unsupported.
    Unsupported,

    /// The permission operation could not be completed.
    Failed,
}
crate::impl_trait![fmt::Display+Error for PermissionError |self, f| match *self {
    Self::Unsupported => f.write_str(""),
    Self::Failed => f.write_str(""),
}];

#[doc = crate::_tags!(runtime result)]
/// The result of polling a non-blocking permission query.
#[doc = crate::_doc_meta!{location("run/permission")}]
///
/// - [`Pending`][AsyncPoll::Pending] means that no result is available yet.
/// - [`Ready(Ok(_))`][AsyncPoll::Ready] contains the current permission state.
/// - [`Ready(Err(_))`][AsyncPoll::Ready] means the state could not be determined.
pub type PermissionQuery<E = PermissionError> = AsyncPoll<Result<PermissionState, E>>;

#[doc = crate::_tags!(runtime state)]
/// The effective authorization state of a permission.
#[doc = crate::_doc_meta!{location("run/permission")}]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PermissionState {
    /// The protected operation is currently authorized.
    Granted,

    /// Authorization remains undecided or requestable.
    ///
    /// Requesting the permission may require interaction with the user,
    /// a policy provider, or another authorizing authority.
    Prompt,

    /// The protected operation is currently not authorized.
    Denied,
}
crate::_impl_init![Self::Prompt => PermissionState];
