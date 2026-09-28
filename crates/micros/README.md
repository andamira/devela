# devela_micros

Microcontroller, board, and embedded hardware support for devela.

This crate contains concrete hardware realizations built on devela's
portable abstractions and low-level mechanisms.

## Structure

- `board`: fixed board configurations and onboard hardware.
- `computer`: microcomputer systems and machine-specific hardware.
- `device`: independently addressable components and reusable drivers.
- `mcu`: microcontrollers and their integrated hardware foundations.
- `processor`: processor families and instruction-level facilities.

## Documentation

- [API documentation][api]: browse the library by module.
- [WIP documentation][wip]: current development API.

[api]: https://docs.rs/devela_micros/latest/devela_micros/
[wip]: https://andamira.github.io/devela_micros/wip/devela_micros/
