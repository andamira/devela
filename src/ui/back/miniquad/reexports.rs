// devela::ui::back::miniquad::reexports
//
//!
//

use crate::_reexport;

/* structs */

_reexport! { "dep_miniquad", "miniquad", miniquad::conf,
    doc: "Describes a hardware and platform-specific setup.",
    @Conf as MiniquadConf
}
_reexport! { "dep_miniquad", "miniquad", miniquad::conf,
    doc: "Platform-specific settings.",
    @Platform as MiniquadPlatform
}

/* traits */

_reexport! { "dep_miniquad", "miniquad", miniquad,
    doc: "Defines how an application responds to events in miniquad.",
    @EventHandler as MiniquadEventHandler
}
_reexport! { "dep_miniquad", "miniquad", miniquad::graphics,
    doc: "Low-level interface for rendering operations in miniquad.",
    @RenderingBackend as MiniquadRenderingBackend
}
