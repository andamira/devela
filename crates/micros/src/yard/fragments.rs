//
//! Docs headings for modules.
//

#![allow(missing_docs, reason = "hidden internals for the workspace")]

crate::CONST! { hidden macro_export,
    _DOC_ALL  = "All public crate items re-exported in a single flat namespace.";
    _DOC_ALL_ = "All public crate items re-exported, grouped by their root modules.";
    _DOC_ALL_PLUS_ = "\n\nEach root module appears here and provides
its own flat view of all its public children.

Click on the `▽` symbol to switch to hierarchical view, and `◉` to switch back to flat view.
";

    _DOC_MCU           = "Microcontrollers and their integrated hardware foundations.";
    _DOC_MCU_AVR       = "AVR microcontrollers and peripheral foundations.";
    _DOC_MCU_AVR_TIMER = "AVR timer/counter peripherals and timing concepts.";
    _DOC_MCU_ESP32     = "ESP32 microcontrollers and peripheral foundations.";
    _DOC_MCU_SAM       = "Microchip SAM microcontrollers and peripheral foundations.";

    _DOC_BOARD         = "Development boards and their fixed hardware configurations.";
    _DOC_BOARD_AVR     = "Boards based on AVR microcontrollers.";
    _DOC_BOARD_ESP32   = "Boards based on ESP32 microcontrollers.";
    _DOC_BOARD_SAM     = "Boards based on Microchip SAM microcontrollers.";

    _DOC_DEVICE         = "Reusable drivers for discrete hardware devices.";
    _DOC_DEVICE_AUDIO   = "Audio converters, amplifiers and interface devices.";
    _DOC_DEVICE_COMM    = "Communication transceivers, modems, and interface devices.";
    _DOC_DEVICE_DISPLAY = "Display controllers and display-device drivers.";
    _DOC_DEVICE_INPUT   = "Input controllers and human-interface devices.";
    _DOC_DEVICE_MOTION  = "Motor, actuator, and motion-control devices.";
    _DOC_DEVICE_POWER   = "Power-management, conversion, and control devices.";
    _DOC_DEVICE_SENSOR  = "Sensors and measurement devices.";
    _DOC_DEVICE_VISION  = "Cameras, imagers, and vision-oriented devices.";

    _DOC_YARD       = "Scaffolding, taxonomy, and documentation support.";
}
