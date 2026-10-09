#!/usr/bin/env python3
"""Convert a raw ARM application binary into nRF52840-family UF2.

Uses only the Python standard library. The caller supplies the actual
Flash origin and application-end boundary for the resident bootloader.
"""
import argparse
from pathlib import Path
import struct

UF2_MAGIC0 = 0x0A324655
UF2_MAGIC1 = 0x9E5D5157
UF2_MAGIC_END = 0x0AB16F30
NRF52840_FAMILY = 0xADA52840
BLOCK_PAYLOAD = 256

def convert(binary: bytes, base: int, end: int) -> bytes:
    if not binary:
        raise ValueError("empty firmware")
    if base & 0xFF or end <= base:
        raise ValueError("invalid or unaligned Flash region")
    if base + len(binary) > end:
        raise ValueError("firmware exceeds application Flash partition")
    if len(binary) < 8:
        raise ValueError("missing ARM vector table")
    sp, reset = struct.unpack_from("<II", binary)
    if not 0x20000000 <= sp <= 0x20040000 or reset & 1 == 0:
        raise ValueError("unexpected initial stack pointer or reset vector")
    if not base <= (reset & ~1) < base + len(binary):
        raise ValueError("reset vector does not point into the application")
    count = (len(binary) + BLOCK_PAYLOAD - 1) // BLOCK_PAYLOAD
    out = bytearray()
    for n in range(count):
        target = base + n * BLOCK_PAYLOAD
        payload = binary[n * BLOCK_PAYLOAD:(n + 1) * BLOCK_PAYLOAD]
        header = struct.pack("<8I", UF2_MAGIC0, UF2_MAGIC1, 0x2000,
                             target, BLOCK_PAYLOAD, n, count, NRF52840_FAMILY)
        out.extend(header)
        out.extend(payload.ljust(BLOCK_PAYLOAD, b"\xFF"))
        out.extend(bytes(476 - BLOCK_PAYLOAD))
        out.extend(struct.pack("<I", UF2_MAGIC_END))
    return bytes(out)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path)
    parser.add_argument("uf2", type=Path)
    parser.add_argument("--base", type=lambda x: int(x, 0), required=True)
    parser.add_argument("--end", type=lambda x: int(x, 0), required=True)
    args = parser.parse_args()
    firmware = convert(args.binary.read_bytes(), args.base, args.end)
    args.uf2.write_bytes(firmware)
    print(f"{args.uf2}: {len(firmware) // 512} blocks, {len(firmware)} bytes")

if __name__ == "__main__":
    main()
