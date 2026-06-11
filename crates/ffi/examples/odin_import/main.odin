// Odin compile/link-time foreign import of devela_ffi.

package main

import "core:fmt"
import ffi "../../odin"

main :: proc() {
    version := ffi.devela_abi_version()
    sum := ffi.devela_add_i32(20, 22)

    msg := "hello from Odin import"
    out_len: uint

    status := ffi.devela_bytes_len(raw_data(msg), len(msg), &out_len)

	fmt.printf("%s\n", msg)
    fmt.printf("abi version: 0x%08x\n", version)
    fmt.printf("20 + 22 = %d\n", sum)
    fmt.printf(
        "bytes_len status: %d (%s), len: %d\n",
        status,
        ffi.devela_error_string(status),
        out_len,
    )
}
