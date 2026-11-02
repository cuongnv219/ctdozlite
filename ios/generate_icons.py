#!/usr/bin/env python3
"""Generate crisp PNG app icons for ctdozlite iOS 12.0+ bundle without external image deps."""
import struct
import sys
import zlib
from pathlib import Path


def make_png(width: int, height: int) -> bytes:
    raw_rows = bytearray()
    cx, cy = width / 2.0, height / 2.0
    radius = width * 0.36
    for y in range(height):
        raw_rows.append(0)  # Filter type 0
        for x in range(width):
            # Gradient Zalo Blue background (#005AE0 -> #0091FF)
            t = (x + y) / float(width + height)
            r = int(0 + 12 * t)
            g = int(82 + 78 * t)
            b = int(218 + 37 * t)
            # Draw central white circle badge
            dx, dy = x - cx, y - cy
            dist = (dx * dx + dy * dy) ** 0.5
            if dist <= radius:
                # Inside white badge, draw stylized 'Z' in Zalo blue
                nx = (x - (cx - radius * 0.55)) / (radius * 1.1)
                ny = (y - (cy - radius * 0.55)) / (radius * 1.1)
                in_z = False
                if 0.0 <= nx <= 1.0 and 0.0 <= ny <= 1.0:
                    if ny < 0.22 or ny > 0.78:
                        in_z = True
                    elif abs((1.0 - ny) - nx) < 0.16:
                        in_z = True
                if in_z:
                    r, g, b = 0, 104, 255
                else:
                    r, g, b = 255, 255, 255
            raw_rows.extend((r, g, b, 255))

    def chunk(tag: bytes, data: bytes) -> bytes:
        return (
            struct.pack(">I", len(data))
            + tag
            + data
            + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)
        )

    ihdr = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    idat = zlib.compress(bytes(raw_rows), 9)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr) + chunk(b"IDAT", idat) + chunk(b"IEND", b"")


def main() -> None:
    out_dir = Path(sys.argv[1])
    out_dir.mkdir(parents=True, exist_ok=True)
    for name, size in [
        ("AppIcon60x60@2x.png", 120),
        ("AppIcon60x60@3x.png", 180),
        ("AppIcon76x76@2x~ipad.png", 152),
        ("AppIcon83.5x83.5@2x~ipad.png", 167),
    ]:
        (out_dir / name).write_bytes(make_png(size, size))


if __name__ == "__main__":
    main()
