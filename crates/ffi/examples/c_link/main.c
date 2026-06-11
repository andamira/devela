/* C link-time use of devela_ffi through the generated header. */

#include <stdint.h>
#include <stddef.h>
#include <stdio.h>

#include "../../include/devela_ffi.h"

int main(void) {
    uint32_t version = devela_abi_version();
    int32_t sum = devela_add_i32(20, 22);

    const uint8_t msg[] = "hello from C link (static)";
    size_t out_len = 0;
    devela_status status = devela_bytes_len(msg, sizeof(msg) - 1, &out_len);

    printf("%s\n", msg);
    printf("abi version: 0x%08x\n", version);
    printf("20 + 22 = %d\n", sum);
    printf("bytes_len status: %d (%s), len: %zu\n",
        status, devela_error_string(status), out_len);

    return 0;
}
