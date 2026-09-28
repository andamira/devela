#!/usr/bin/env python3

from pathlib import Path
import struct
import sys


def block(flag: int, payload: bytes) -> bytes:
    body = bytes([flag]) + payload

    checksum = 0
    for byte in body:
        checksum ^= byte

    body += bytes([checksum])
    return struct.pack("<H", len(body)) + body


def header(
    kind: int,
    name: bytes,
    length: int,
    param1: int,
    param2: int,
) -> bytes:
    return (
        bytes([kind])
        + name[:10].ljust(10, b" ")
        + struct.pack("<HHH", length, param1, param2)
    )


if len(sys.argv) not in (3, 4):
    raise SystemExit(
        f"usage: {sys.argv[0]} INPUT.bin OUTPUT.tap [ORIGIN]"
    )

source = Path(sys.argv[1])
output = Path(sys.argv[2])

origin = int(sys.argv[3], 0) if len(sys.argv) == 4 else 0x8000
data = source.read_bytes()

if not 1 <= origin <= 0xFFFF:
    raise SystemExit(f"invalid load address: {origin:#x}")

if len(data) > 0x10000 - origin:
    raise SystemExit(
        f"binary does not fit from {origin:#06x} to 0xffff"
    )

# Sinclair BASIC tokens used by the loader.
CODE = 0xAF
VAL = 0xB0
USR = 0xC0
LOAD = 0xEF
RANDOMIZE = 0xF9
CLEAR = 0xFD

ramtop = origin - 1

# 10 CLEAR VAL "32767":LOAD "" CODE:RANDOMIZE USR VAL "32768"
line = (
    bytes([CLEAR, VAL])
    + f'"{ramtop}"'.encode()
    + b":"
    + bytes([LOAD])
    + b'""'
    + bytes([CODE])
    + b":"
    + bytes([RANDOMIZE, USR, VAL])
    + f'"{origin}"'.encode()
    + b"\r"
)

# BASIC lines store the line number big-endian and line length little-endian.
program = (
    struct.pack(">H", 10)
    + struct.pack("<H", len(line))
    + line
)

loader_name = b"DEVELA"
code_name = source.stem.upper().encode("ascii", "replace")

tap = (
    # Auto-starting BASIC loader.
    block(
        0x00,
        header(0, loader_name, len(program), 10, len(program)),
    )
    + block(0xFF, program)

    # Machine-code payload.
    + block(
        0x00,
        header(3, code_name, len(data), origin, 32768),
    )
    + block(0xFF, data)
)

output.write_bytes(tap)

print(f"code:   {len(data)} bytes")
print(f"origin: {origin:#06x}")
print(f"tap:    {len(tap)} bytes")
