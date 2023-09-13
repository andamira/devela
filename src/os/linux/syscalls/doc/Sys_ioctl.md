Performs a generic I/O control operation (ioctl) on the given file descriptor.

The operation to perform and the data to use is determined by the `request`
argument, which is a device-specific request code, and the `argp` argument,
which is a pointer to the data.