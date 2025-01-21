This is derived work from the
[`etcetera`](https://crates.io/crates/etcetera/0.8.0) crate,
including the following modifications:

- make the trait `dyn` compatible.
- rename `AppStrategyArgs` to `AppData`.
- unify all `Xdg` structs into `DirXdg`.
- unify all `Apple` structs into `DirApple`.
- unify all `Windows` structs into `DirWindows`.
- unify `BaseStrategy` and `AppStrategy` into `DirEnv`.
- replace `Result` with `Option`, remove `HomeDirError`.
- validate `AppData` construction, made fields private.
- make Apple bundle ID match the `directories` crate.
- replace dashes with underscores in `unixy_name`.
- remove all deprecated windows-specific code.
- remove all unneeded dependencies.
- remove helper macros.
- remove all unsafe.
