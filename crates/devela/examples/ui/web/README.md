# `ui_web` example

This example demonstrates devela's UI abstractions
running in a web browser through WebAssembly and the web backend.

It is intended to evolve alongside the corresponding UI examples
for other backends while preserving a comparable overall structure.

## Build

Run the build script:

```sh
./build.sh
```

The script:

* Compiles the Rust crate to WebAssembly using `wasm32-unknown-unknown`.
* Copies the generated `.wasm` module into `public_html/`.
* Copies the required JavaScript bridge modules into `public_html/`.

## Run

Start a local web server from the example directory. For example:

```sh
python3 -m http.server 4000 --directory public_html
```

Then open `http://localhost:4000` in a web browser.
