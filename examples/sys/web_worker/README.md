# `web_worker` example

This example demonstrates how to use the `web_api` module in a web browser with Rust-generated WASM.

## build

Run the following script to build the `.wasm` file and copy necessary assets:
```sh
./build.sh
```
This script:
- Compiles the Rust library to WebAssembly (`wasm32-unknown-unknown`).
- Copies the generated `.wasm` file and `web_api.js` into the `public_html/` directory.


## run

1. Start a local web server to serve the public_html/ directory. E.g.:
```sh
python3 -m http.server 4000 --directory public_html
```
2. Open http://localhost:4000 in your browser.
