/* C runtime loading of devela_ffi through dlopen/dlsym. */

#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>

typedef int32_t devela_status;

typedef uint32_t (*devela_abi_version_fn)(void);
typedef int32_t  (*devela_add_i32_fn)(int32_t, int32_t);
typedef devela_status (*devela_bytes_len_fn)(const uint8_t *, size_t, size_t *);
typedef const char *(*devela_error_string_fn)(devela_status);

static void *load_symbol(void *lib, const char *name) {
    dlerror();
    void *sym = dlsym(lib, name);
    const char *err = dlerror();
    if (err != NULL) {
        fprintf(stderr, "dlsym(%s): %s\n", name, err);
        exit(1);
    }
    return sym;
}

int main(void) {
    void *lib = dlopen("../../libs/libdevela_ffi.so", RTLD_NOW | RTLD_LOCAL);
    if (lib == NULL) {
        fprintf(stderr, "dlopen: %s\n", dlerror());
        return 1;
    }

    devela_abi_version_fn devela_abi_version =
        (devela_abi_version_fn)load_symbol(lib, "devela_abi_version");
    devela_add_i32_fn devela_add_i32 =
        (devela_add_i32_fn)load_symbol(lib, "devela_add_i32");
    devela_bytes_len_fn devela_bytes_len =
        (devela_bytes_len_fn)load_symbol(lib, "devela_bytes_len");
    devela_error_string_fn devela_error_string =
        (devela_error_string_fn)load_symbol(lib, "devela_error_string");

    uint32_t version = devela_abi_version();
    int32_t sum = devela_add_i32(20, 22);

    const uint8_t msg[] = "hello from C load (dynamic)";
    size_t out_len = 0;
    devela_status status = devela_bytes_len(msg, sizeof(msg) - 1, &out_len);

    printf("%s\n", msg);
    printf("abi version: 0x%08x\n", version);
    printf("20 + 22 = %d\n", sum);
    printf("bytes_len status: %d (%s), len: %zu\n",
        status, devela_error_string(status), out_len);

    dlclose(lib);
    return 0;
}
