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

    _DOC_BOARD       = "Development boards and their hardware configurations.";
    _DOC_BOARD_AVR   = "AVR-based development boards.";
    _DOC_BOARD_ESP32 = "ESP32-based development boards.";
    _DOC_BOARD_SAM   = "Microchip SAM-based development boards.";

    _DOC_COMPUTER             = "Microcomputer systems, and machine-specific hardware.";
    _DOC_COMPUTER_ZX          = "Sinclair ZX-family microcomputers.";
    _DOC_COMPUTER_ZX_SPECTRUM = "Sinclair ZX Spectrum computers.";

    _DOC_DEVICE         = "Reusable drivers for discrete hardware devices.";
    _DOC_DEVICE_AUDIO   = "Audio conversion, amplification, and interfacing.";
    _DOC_DEVICE_COMM    = "Communication transceivers, modems, and interfaces.";
    _DOC_DEVICE_DISPLAY = "Display controllers and interfaces.";
    _DOC_DEVICE_INPUT   = "Input controllers and human interfaces.";
    _DOC_DEVICE_MOTION  = "Motors, actuators, and motion control.";
    _DOC_DEVICE_POWER   = "Power management, conversion, and control.";
    _DOC_DEVICE_SENSOR  = "Sensing and measurement.";
    _DOC_DEVICE_VISION  = "Cameras, imagers, and machine vision.";

    _DOC_MCU           = "Microcontrollers and their integrated peripherals.";
    _DOC_MCU_AVR       = "AVR-family microcontrollers.";
    _DOC_MCU_AVR_TIMER = "AVR timer/counter peripherals.";
    _DOC_MCU_ESP32     = "Espressif ESP32-family microcontrollers.";
    _DOC_MCU_SAM       = "Microchip SAM-family microcontrollers.";

    _DOC_PROCESSOR     = "Processors and instruction-level facilities.";
    _DOC_PROCESSOR_Z80 = "Z80 processor architecture.";

    _DOC_YARD          = "Scaffolding, taxonomy, and documentation support.";
}
