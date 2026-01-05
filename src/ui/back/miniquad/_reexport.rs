// devela::ui::back::miniquad::_reexport
//
//!
//

use crate::{_TAG_EVENT, _TAG_PLATFORM, _TAG_RUNTIME, _TAG_UI, _reexport};

/* structs */

_reexport! { "dep_miniquad", "miniquad", miniquad::conf, location: "ui/back/miniquad",
    tag: _TAG_PLATFORM!() _TAG_RUNTIME!(),
    doc: "Describes a hardware and platform-specific setup.",
    @Conf as MiniquadConf
}
_reexport! { "dep_miniquad", "miniquad", miniquad::conf, location: "ui/back/miniquad",
    tag: _TAG_PLATFORM!() _TAG_RUNTIME!(),
    doc: "Platform-specific settings.",
    @Platform as MiniquadPlatform
}

/* traits */

_reexport! { "dep_miniquad", "miniquad", miniquad, location: "ui/back/miniquad",
    tag: _TAG_EVENT!() _TAG_UI!(),
    doc: "Defines how an application responds to events in miniquad.",
    @EventHandler as MiniquadEventHandler
}
_reexport! { "dep_miniquad", "miniquad", miniquad::graphics, location: "ui/back/miniquad",
    tag: _TAG_UI!() _TAG_RUNTIME!(),
    doc: "Low-level interface for rendering operations in miniquad.",
    @RenderingBackend as MiniquadRenderingBackend
}
