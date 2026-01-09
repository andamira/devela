// devela::sys::display::x11::raw::xcb_values
//
//! Values from `xproto.h`
//
// TOC
// - xcb_event_code
// - xcb_error_code
// - xcb_request_code
// - xcb_prop_mode
// - window attribute flags

#![allow(non_camel_case_types, clippy::upper_case_acronyms)]

/* codes */

/// X11 core event opcodes.
///
/// These identify the type of an event packet returned by the X server.
/// All event packets start with a `response_type` byte
/// where the low 7 bits select one of these opcodes.
#[repr(u8)]
pub(crate) enum xcb_event_code {
    /// Key down
    XCB_KEY_PRESS = 2,
    /// Key up
    XCB_KEY_RELEASE = 3,
    /// Mouse button down
    XCB_BUTTON_PRESS = 4,
    /// Mouse button up
    XCB_BUTTON_RELEASE = 5,
    /// Mouse/pointer movement
    XCB_MOTION_NOTIFY = 6,
    /// Pointer entered window
    XCB_ENTER_NOTIFY = 7,
    /// Pointer left window
    XCB_LEAVE_NOTIFY = 8,
    /// The window gained keyboard focus.
    XCB_FOCUS_IN = 9,
    /// The window lost keyboard focus.
    XCB_FOCUS_OUT = 10,
    /// Legacy "keyboard mapping changed"
    XCB_KEYMAP_NOTIFY = 11,
    /// Window region became visible, redraw needed
    XCB_EXPOSE = 12,
    /// Obscure, only for certain graphics ops.
    XCB_GRAPHICS_EXPOSURE = 13,
    /// Confirms no Exposure occurred.
    XCB_NO_EXPOSURE = 14,
    /// Visibility has changed.
    XCB_VISIBILITY_NOTIFY = 15,

    /* window life cycle*/
    /// A window was created.
    XCB_CREATE_NOTIFY = 16,
    /// A window was destroyed.
    XCB_DESTROY_NOTIFY = 17,
    /// Window hidden/minimized.
    XCB_UNMAP_NOTIFY = 18,
    /// Window shown.
    XCB_MAP_NOTIFY = 19,

    /* window reparenting */
    /// Window manager asking to map.
    XCB_MAP_REQUEST = 20,
    /// WM moved window into a frame (normal under tiling/stacking WMs).
    XCB_REPARENT_NOTIFY = 21,

    /* geometry changes*/
    /// Position/size changed (your resize event).
    XCB_CONFIGURE_NOTIFY = 22,
    /// WM requesting configure.
    XCB_CONFIGURE_REQUEST = 23,
    /// Rare offset change.
    XCB_GRAVITY_NOTIFY = 24,
    /// Legacy; usually bypassed by modern WMs.
    XCB_RESIZE_REQUEST = 25,

    /* stacking / circulation */
    XCB_CIRCULATE_NOTIFY = 26,
    XCB_CIRCULATE_REQUEST = 27,

    /* properties & selection */
    /// Fires for every change to any window property.
    /// E.g. Title changed, WM hints changed, etc.
    XCB_PROPERTY_NOTIFY = 28,
    /// Lost clipboard ownership.
    XCB_SELECTION_CLEAR = 29,
    /// Another app wants clipboard contents.
    XCB_SELECTION_REQUEST = 30,
    /// Clipboard transfer finished.
    XCB_SELECTION_NOTIFY = 31,

    /* color map */
    /// Obsolete. Color map change.
    XCB_COLORMAP_NOTIFY = 32,

    /* client messages */
    /// Used for WM_DELETE_WINDOW (close button), _NET_WM_STATE (fullscreen, maximized, minimized).
    XCB_CLIENT_MESSAGE = 33,
    /// Obsolete. XT keyboard remap.
    XCB_MAPPING_NOTIFY = 34,

    /// Generic events. For XInput2 (high-resolution mouse, raw input, multi-touch).
    XCB_GE_GENERIC = 35,
}

/// X11 core error codes.
///
/// Sent by the server when a request fails.
/// These populate the `xcb_generic_error_t.error_code` field.
#[repr(u8)]
enum xcb_error_code {
    XCB_REQUEST = 1,
    XCB_VALUE = 2,
    XCB_WINDOW = 3,
    XCB_PIXMAP = 4,
    XCB_ATOM = 5,
    XCB_CURSOR = 6,
    XCB_FONT = 7,
    XCB_MATCH = 8,
    XCB_DRAWABLE = 9,
    XCB_ACCESS = 10,
    XCB_ALLOC = 11,
    XCB_COLORMAP = 12,
    XCB_G_CONTEXT = 13,
    XCB_ID_CHOICE = 14,
    XCB_NAME = 15,
    XCB_LENGTH = 16,
    XCB_IMPLEMENTATION = 17,
}

/// X11 core request opcodes.
///
/// These identify the requests sent from client to server.
/// They occupy the first byte in any request packet
/// and determine how the remaining bytes are interpreted.
#[repr(u8)]
enum xcb_request_code {
    XCB_CREATE_WINDOW = 1,
    XCB_CHANGE_WINDOW_ATTRIBUTES = 2,
    XCB_GET_WINDOW_ATTRIBUTES = 3,
    XCB_DESTROY_WINDOW = 4,
    XCB_DESTROY_SUBWINDOWS = 5,
    XCB_CHANGE_SAVE_SET = 6,
    XCB_REPARENT_WINDOW = 7,
    XCB_MAP_WINDOW = 8,
    XCB_MAP_SUBWINDOWS = 9,
    XCB_UNMAP_WINDOW = 10,
    XCB_UNMAP_SUBWINDOWS = 11,
    XCB_CONFIGURE_WINDOW = 12,
    XCB_CIRCULATE_WINDOW = 13,
    XCB_GET_GEOMETRY = 14,
    XCB_QUERY_TREE = 15,
    XCB_INTERN_ATOM = 16,
    XCB_GET_ATOM_NAME = 17,
    XCB_CHANGE_PROPERTY = 18,
    XCB_DELETE_PROPERTY = 19,
    XCB_GET_PROPERTY = 20,
    XCB_LIST_PROPERTIES = 21,
    XCB_SET_SELECTION_OWNER = 22,
    XCB_GET_SELECTION_OWNER = 23,
    XCB_CONVERT_SELECTION = 24,
    XCB_SEND_EVENT = 25,
    XCB_GRAB_POINTER = 26,
    XCB_UNGRAB_POINTER = 27,
    XCB_GRAB_BUTTON = 28,
    XCB_UNGRAB_BUTTON = 29,
    XCB_CHANGE_ACTIVE_POINTER_GRAB = 30,
    XCB_GRAB_KEYBOARD = 31,
    XCB_UNGRAB_KEYBOARD = 32,
    XCB_GRAB_KEY = 33,
    XCB_UNGRAB_KEY = 34,
    XCB_ALLOW_EVENTS = 35,
    XCB_GRAB_SERVER = 36,
    XCB_UNGRAB_SERVER = 37,
    XCB_QUERY_POINTER = 38,
    XCB_GET_MOTION_EVENTS = 39,
    XCB_TRANSLATE_COORDINATES = 40,
    XCB_WARP_POINTER = 41,
    XCB_SET_INPUT_FOCUS = 42,
    XCB_GET_INPUT_FOCUS = 43,
    XCB_QUERY_KEYMAP = 44,
    XCB_OPEN_FONT = 45,
    XCB_CLOSE_FONT = 46,
    XCB_QUERY_FONT = 47,
    XCB_QUERY_TEXT_EXTENTS = 48,
    XCB_LIST_FONTS = 49,
    XCB_LIST_FONTS_WITH_INFO = 50,
    XCB_SET_FONT_PATH = 51,
    XCB_GET_FONT_PATH = 52,
    XCB_CREATE_PIXMAP = 53,
    XCB_FREE_PIXMAP = 54,
    XCB_CREATE_GC = 55,
    XCB_CHANGE_GC = 56,
    XCB_COPY_GC = 57,
    XCB_SET_DASHES = 58,
    XCB_SET_CLIP_RECTANGLES = 59,
    XCB_FREE_GC = 60,
    XCB_CLEAR_AREA = 61,
    XCB_COPY_AREA = 62,
    XCB_COPY_PLANE = 63,
    XCB_POLY_POINT = 64,
    XCB_POLY_LINE = 65,
    XCB_POLY_SEGMENT = 66,
    XCB_POLY_RECTANGLE = 67,
    XCB_POLY_ARC = 68,
    XCB_FILL_POLY = 69,
    XCB_POLY_FILL_RECTANGLE = 70,
    XCB_POLY_FILL_ARC = 71,
    XCB_PUT_IMAGE = 72,
    XCB_GET_IMAGE = 73,
    XCB_POLY_TEXT_8 = 74,
    XCB_POLY_TEXT_16 = 75,
    XCB_IMAGE_TEXT_8 = 76,
    XCB_IMAGE_TEXT_16 = 77,
    XCB_CREATE_COLORMAP = 78,
    XCB_FREE_COLORMAP = 79,
    XCB_COPY_COLORMAP_AND_FREE = 80,
    XCB_INSTALL_COLORMAP = 81,
    XCB_UNINSTALL_COLORMAP = 82,
    XCB_LIST_INSTALLED_COLORMAPS = 83,
    XCB_ALLOC_COLOR = 84,
    XCB_ALLOC_NAMED_COLOR = 85,
    XCB_ALLOC_COLOR_CELLS = 86,
    XCB_ALLOC_COLOR_PLANES = 87,
    XCB_FREE_COLORS = 88,
    XCB_STORE_COLORS = 89,
    XCB_STORE_NAMED_COLOR = 90,
    XCB_QUERY_COLORS = 91,
    XCB_LOOKUP_COLOR = 92,
    XCB_CREATE_CURSOR = 93,
    XCB_CREATE_GLYPH_CURSOR = 94,
    XCB_FREE_CURSOR = 95,
    XCB_RECOLOR_CURSOR = 96,
    XCB_QUERY_BEST_SIZE = 97,
    XCB_QUERY_EXTENSION = 98,
    XCB_LIST_EXTENSIONS = 99,
    XCB_CHANGE_KEYBOARD_MAPPING = 100,
    XCB_GET_KEYBOARD_MAPPING = 101,
    XCB_CHANGE_KEYBOARD_CONTROL = 102,
    XCB_GET_KEYBOARD_CONTROL = 103,
    XCB_BELL = 104,
    XCB_CHANGE_POINTER_CONTROL = 105,
    XCB_GET_POINTER_CONTROL = 106,
    XCB_SET_SCREEN_SAVER = 107,
    XCB_GET_SCREEN_SAVER = 108,
    XCB_CHANGE_HOSTS = 109,
    XCB_LIST_HOSTS = 110,
    XCB_SET_ACCESS_CONTROL = 111,
    XCB_SET_CLOSE_DOWN_MODE = 112,
    XCB_KILL_CLIENT = 113,
    XCB_ROTATE_PROPERTIES = 114,
    XCB_FORCE_SCREEN_SAVER = 115,
    XCB_SET_POINTER_MAPPING = 116,
    XCB_GET_POINTER_MAPPING = 117,
    XCB_SET_MODIFIER_MAPPING = 118,
    XCB_GET_MODIFIER_MAPPING = 119,
    //
    XCB_NO_OPERATION = 127,
}

///
#[repr(u8)]
pub(crate) enum xcb_atom_enum_t {
    XCB_ATOM_ANY = 0,
    XCB_ATOM_PRIMARY = 1,
    XCB_ATOM_SECONDARY = 2,
    XCB_ATOM_ARC = 3,
    XCB_ATOM_ATOM = 4,
    XCB_ATOM_BITMAP = 5,
    XCB_ATOM_CARDINAL = 6,
    XCB_ATOM_COLORMAP = 7,
    XCB_ATOM_CURSOR = 8,
    XCB_ATOM_CUT_BUFFER0 = 9,
    XCB_ATOM_CUT_BUFFER1 = 10,
    XCB_ATOM_CUT_BUFFER2 = 11,
    XCB_ATOM_CUT_BUFFER3 = 12,
    XCB_ATOM_CUT_BUFFER4 = 13,
    XCB_ATOM_CUT_BUFFER5 = 14,
    XCB_ATOM_CUT_BUFFER6 = 15,
    XCB_ATOM_CUT_BUFFER7 = 16,
    XCB_ATOM_DRAWABLE = 17,
    XCB_ATOM_FONT = 18,
    XCB_ATOM_INTEGER = 19,
    XCB_ATOM_PIXMAP = 20,
    XCB_ATOM_POINT = 21,
    XCB_ATOM_RECTANGLE = 22,
    XCB_ATOM_RESOURCE_MANAGER = 23,
    XCB_ATOM_RGB_COLOR_MAP = 24,
    XCB_ATOM_RGB_BEST_MAP = 25,
    XCB_ATOM_RGB_BLUE_MAP = 26,
    XCB_ATOM_RGB_DEFAULT_MAP = 27,
    XCB_ATOM_RGB_GRAY_MAP = 28,
    XCB_ATOM_RGB_GREEN_MAP = 29,
    XCB_ATOM_RGB_RED_MAP = 30,
    XCB_ATOM_STRING = 31,
    XCB_ATOM_VISUALID = 32,
    XCB_ATOM_WINDOW = 33,
    XCB_ATOM_WM_COMMAND = 34,
    XCB_ATOM_WM_HINTS = 35,
    XCB_ATOM_WM_CLIENT_MACHINE = 36,
    XCB_ATOM_WM_ICON_NAME = 37,
    XCB_ATOM_WM_ICON_SIZE = 38,
    XCB_ATOM_WM_NAME = 39,
    XCB_ATOM_WM_NORMAL_HINTS = 40,
    XCB_ATOM_WM_SIZE_HINTS = 41,
    XCB_ATOM_WM_ZOOM_HINTS = 42,
    XCB_ATOM_MIN_SPACE = 43,
    XCB_ATOM_NORM_SPACE = 44,
    XCB_ATOM_MAX_SPACE = 45,
    XCB_ATOM_END_SPACE = 46,
    XCB_ATOM_SUPERSCRIPT_X = 47,
    XCB_ATOM_SUPERSCRIPT_Y = 48,
    XCB_ATOM_SUBSCRIPT_X = 49,
    XCB_ATOM_SUBSCRIPT_Y = 50,
    XCB_ATOM_UNDERLINE_POSITION = 51,
    XCB_ATOM_UNDERLINE_THICKNESS = 52,
    XCB_ATOM_STRIKEOUT_ASCENT = 53,
    XCB_ATOM_STRIKEOUT_DESCENT = 54,
    XCB_ATOM_ITALIC_ANGLE = 55,
    XCB_ATOM_X_HEIGHT = 56,
    XCB_ATOM_QUAD_WIDTH = 57,
    XCB_ATOM_WEIGHT = 58,
    XCB_ATOM_POINT_SIZE = 59,
    XCB_ATOM_RESOLUTION = 60,
    XCB_ATOM_COPYRIGHT = 61,
    XCB_ATOM_NOTICE = 62,
    XCB_ATOM_FONT_NAME = 63,
    XCB_ATOM_FAMILY_NAME = 64,
    XCB_ATOM_FULL_NAME = 65,
    XCB_ATOM_CAP_HEIGHT = 66,
    XCB_ATOM_WM_CLASS = 67,
    XCB_ATOM_WM_TRANSIENT_FOR = 68,
}
impl xcb_atom_enum_t {
    pub const XCB_ATOM_NONE: Self = Self::XCB_ATOM_ANY;
}

/* property modes */

///
#[repr(u8)]
pub(crate) enum xcb_prop_mode {
    /// Discard the previous property value and store the new data.
    XCB_PROP_MODE_REPLACE = 0,

    /// Insert the new data before the beginning of existing data.
    XCB_PROP_MODE_PREPEND = 1,

    /// Insert the new data after the beginning of existing data.
    XCB_PROP_MODE_APPEND = 2,
}

/* window attribute flags */

/// X11 graphics context attribute: foreground pixel.
pub(crate) const XCB_GC_FOREGROUND: u32 = 4;

/// X11 window class: input/output window.
pub(crate) const XCB_WINDOW_CLASS_INPUT_OUTPUT: u16 = 1;

/// X11 image format: Z-pixmap.
pub(crate) const XCB_IMAGE_FORMAT_Z_PIXMAP: u8 = 2;
