// Odin runtime symbol loading of devela_ffi.

package main

import "core:fmt"
import ffi "../../odin"

main :: proc() {
    lib, ok := ffi.load()
    if !ok {
        fmt.eprintln("failed to load devela_ffi")
        return
    }
    defer ffi.unload(&lib)

    version := lib.abi_version()
    sum := lib.add_i32(20, 22)

    msg := "hello from Odin load"
    out_len: uint

    status := lib.bytes_len(raw_data(msg), len(msg), &out_len)

    fmt.printf("%s\n", msg)
    fmt.printf("abi version: 0x%08x\n", version)
    fmt.printf("20 + 22 = %d\n", sum)
    fmt.printf(
        "bytes_len status: %d (%s), len: %d\n",
        status,
        lib.error_string(status),
        out_len,
    )
}
