// devela::ui::service::miniquad::reexports
//
//!
//

use crate::reexport;

/* structs */

reexport! { "dep_miniquad", "miniquad", miniquad::conf,
    doc: "Describes a hardware and platform-specific setup.",
    @Conf as MiniquadConf
}
reexport! { "dep_miniquad", "miniquad", miniquad::conf,
    doc: "Platform-specific settings.",
    @Platform as MiniquadPlatform
}

/* traits */

reexport! { "dep_miniquad", "miniquad", miniquad,
    doc: "Defines how an application responds to events in miniquad.",
    @EventHandler as MiniquadEventHandler
}
reexport! { "dep_miniquad", "miniquad", miniquad::graphics,
    doc: "Low-level interface for rendering operations in miniquad.",
    @RenderingBackend as MiniquadRenderingBackend
}
