// devela::sys::display::x11::raw::xcb_values
//
//! Values from `xproto.h`
//
// TOC
//  - event masks
//  - window attributes

/* event masks */

pub(crate) const XCB_EVENT_MASK_NO_EVENT: u32 = 0;
pub(crate) const XCB_EVENT_MASK_KEY_PRESS: u32 = 1 << 0;
pub(crate) const XCB_EVENT_MASK_KEY_RELEASE: u32 = 1 << 1;
pub(crate) const XCB_EVENT_MASK_BUTTON_PRESS: u32 = 1 << 2;
pub(crate) const XCB_EVENT_MASK_BUTTON_RELEASE: u32 = 1 << 3;
pub(crate) const XCB_EVENT_MASK_ENTER_WINDOW: u32 = 1 << 4;
pub(crate) const XCB_EVENT_MASK_LEAVE_WINDOW: u32 = 1 << 5;
pub(crate) const XCB_EVENT_MASK_POINTER_MOTION: u32 = 1 << 6;
pub(crate) const XCB_EVENT_MASK_POINTER_MOTION_HINT: u32 = 1 << 7;
pub(crate) const XCB_EVENT_MASK_BUTTON_1_MOTION: u32 = 1 << 8;
pub(crate) const XCB_EVENT_MASK_BUTTON_2_MOTION: u32 = 1 << 9;
pub(crate) const XCB_EVENT_MASK_BUTTON_3_MOTION: u32 = 1 << 10;
pub(crate) const XCB_EVENT_MASK_BUTTON_4_MOTION: u32 = 1 << 11;
pub(crate) const XCB_EVENT_MASK_BUTTON_5_MOTION: u32 = 1 << 12;
pub(crate) const XCB_EVENT_MASK_BUTTON_MOTION: u32 = 1 << 13;
pub(crate) const XCB_EVENT_MASK_KEYMAP_STATE: u32 = 1 << 14;
pub(crate) const XCB_EVENT_MASK_EXPOSURE: u32 = 1 << 15;
pub(crate) const XCB_EVENT_MASK_VISIBILITY_CHANGE: u32 = 1 << 16;
pub(crate) const XCB_EVENT_MASK_STRUCTURE_NOTIFY: u32 = 1 << 17;
pub(crate) const XCB_EVENT_MASK_RESIZE_REDIRECT: u32 = 1 << 18;
pub(crate) const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: u32 = 1 << 19;
pub(crate) const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: u32 = 1 << 20;
pub(crate) const XCB_EVENT_MASK_FOCUS_CHANGE: u32 = 1 << 21;
pub(crate) const XCB_EVENT_MASK_PROPERTY_CHANGE: u32 = 1 << 22;
pub(crate) const XCB_EVENT_MASK_COLOR_MAP_CHANGE: u32 = 1 << 23;
pub(crate) const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: u32 = 1 << 24;

/* window attribute value-mask bits (`xcb_cw_t`). */

/**< Overrides the default background-pixmap. The background pixmap and window must
have the same root and same depth. Any size pixmap can be used, although some
sizes may be faster than others.

If `XCB_BACK_PIXMAP_NONE` is specified, the window has no defined background.
The server may fill the contents with the previous screen contents or with
contents of its own choosing.

If `XCB_BACK_PIXMAP_PARENT_RELATIVE` is specified, the parent's background is
used, but the window must have the same depth as the parent (or a Match error
results).   The parent's background is tracked, and the current version is
used each time the window background is required. */
pub(crate) const XCB_CW_BACK_PIXMAP: u32 = 1 << 0;

/// Overrides `BackPixmap`.
///
/// A pixmap of undefined size filled with the specified background pixel is used for the
/// background. Range-checking is not performed, the background pixel is truncated
/// to the appropriate number of bits.
pub(crate) const XCB_CW_BACK_PIXEL: u32 = 1 << 1;

/// Overrides the default border-pixmap.
///
/// The border pixmap and window must have the same root and the same depth.
/// Any size pixmap can be used, although some sizes may be faster than others.
///
/// The special value `XCB_COPY_FROM_PARENT` means the parent's border pixmap is copied
/// (subsequent changes to the parent's border attribute do not affect the child),
/// but the window must have the same depth as the parent.
pub(crate) const XCB_CW_BORDER_PIXMAP: u32 = 1 << 2;


/// Overrides `BorderPixmap`.
///
/// A pixmap of undefined size filled with the specified border pixel is used for the border.
/// Range checking is not performed on the border-pixel value,
/// it is truncated to the appropriate number of bits.
pub(crate) const XCB_CW_BORDER_PIXEL: u32 = 1 << 3;

/// Defines which region of the window should be retained if the window is resized.
pub(crate) const XCB_CW_BIT_GRAVITY: u32 = 1 << 4;

/// Defines how the window should be repositioned if the parent is resized.
/// (see `ConfigureWindow`)
pub(crate) const XCB_CW_WIN_GRAVITY: u32 = 1 << 5;

/**< A backing-store of `WhenMapped` advises the server that maintaining contents of
obscured regions when the window is mapped would be beneficial. A backing-store
of `Always` advises the server that maintaining contents even when the window
is unmapped would be beneficial. In this case, the server may generate an
exposure event when the window is created. A value of `NotUseful` advises the
server that maintaining contents is unnecessary, although a server may still
choose to maintain contents while the window is mapped. Note that if the server
maintains contents, then the server should maintain complete contents not just
the region within the parent boundaries, even if the window is larger than its
parent. While the server maintains contents, exposure events will not normally
be generated, but the server may stop maintaining contents at any time. */
pub(crate) const XCB_CW_BACKING_STORE: u32 = 1 << 6;

/// The backing-planes indicates (with bits set to 1) which bit planes of the window
/// hold dynamic data that must be preserved in backing-stores and during save-unders.
pub(crate) const XCB_CW_BACKING_PLANES: u32 = 1 << 7;

/// The backing-pixel specifies what value to use in planes not covered by backing-planes.
///
/// The server is free to save only the specified bit planes in the backing-store or save-under
/// and regenerate the remaining planes with the specified pixel value.
/// Any bits beyond the specified depth of the window in these values are simply ignored.
pub(crate) const XCB_CW_BACKING_PIXEL: u32 = 1 << 8;

/// The override-redirect specifies whether map and configure requests
/// on this window should override a SubstructureRedirect on the parent,
/// typically to inform a window manager not to tamper with the window.
pub(crate) const XCB_CW_OVERRIDE_REDIRECT: u32 = 1 << 9;

/// If 1, the server is advised that when this window is mapped,
/// saving the contents of windows it obscures would be beneficial.
pub(crate) const XCB_CW_SAVE_UNDER: u32 = 1 << 10;

/// The event-mask defines which events the client is interested in for this window
/// (or for some event types, inferiors of the window).
pub(crate) const XCB_CW_EVENT_MASK: u32 = 1 << 11;

/// The do-not-propagate-mask defines which events should not be propagated to
/// ancestor windows when no client has the event type selected in this window.
pub(crate) const XCB_CW_DONT_PROPAGATE: u32 = 1 << 12;

/// The colormap specifies the colormap that best reflects the true colors of the window.
///
/// Servers capable of supporting multiple hardware colormaps may use this information,
/// and window managers may use it for InstallColormap requests.
///
/// The colormap must have the same visual type and root as the window (or a Match error results).
/// If CopyFromParent is specified, the parent's colormap is copied (subsequent changes to the
/// parent's colormap attribute do not affect the child).
///
/// However, the window must have the same visual type as the parent (or a Match error results),
/// and the parent must not have a colormap of None (or a Match error results). For an explanation
/// of None, see FreeColormap request. The colormap is copied by sharing the colormap object
/// between the child and the parent, not by making a complete copy of the colormap contents.
pub(crate) const XCB_CW_COLORMAP: u32 = 1 << 13;

/// If a cursor is specified, it will be used whenever the pointer is in the window.
///
/// If None is specified, the parent's cursor will be used when the pointer is in the window,
/// and any change in the parent's cursor will cause an immediate change in the displayed cursor.
pub(crate) const XCB_CW_CURSOR: u32 = 1 << 14;
