# devela_ffi examples

Small C and Odin programs that use the generated `devela_ffi` libraries.

## Requirements

### Rust

Install Rust through rustup:

```sh
rustup --version
cargo --version
````

### C compiler

Linux:

```sh
sudo apt install clang
```

or:

```sh
sudo apt install gcc
```

The examples use `cc`, so either compiler is fine if available through that name.

### Odin

Install Odin from:

```text
https://odin-lang.org/
```

Check:

```sh
odin version
```

### Linux dynamic loading

For the C runtime-loading example, link with `dl`:

```sh
-ldl
```

This is already handled by the example script.

## Examples

```text
c_link       C, link-time symbols through the C header
c_load       C, runtime loading with dlopen/dlsym
odin_import  Odin, compile/link-time foreign import
odin_load    Odin, runtime loading with core:dynlib
```

## Run all

From the `devela_ffi/examples` directory:

```sh
./run_all.sh
```

## Run one

```sh
cd examples/c_link
./run.sh
```

```sh
cd examples/odin_import
./run.sh
```

Build scripts keep the produced example binaries in each example directory.
