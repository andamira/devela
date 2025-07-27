# `js_web_api` example

This example demonstrates how to use the `web_api` module in a web browser with Rust-generated WASM.

## build

Run the following script to build the `.wasm` file and copy necessary assets:
```sh
./build-web.sh
```
This script:
- Compiles the Rust library to WebAssembly (`wasm32-unknown-unknown`).
- Copies the generated `.wasm` file and `web_api.js` into the `web/` directory.


## run

1. Start a local web server to serve the web/ directory:
```sh
python3 -m http.server 4000 --directory web
```
2. Open http://localhost:4000 in your browser.
