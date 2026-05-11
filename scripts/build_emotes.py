#!/usr/bin/env python3
"""Build emotes.png from the per-emote Scene1_*.png source frames.

Layout: 8 cols (frames) x N rows (emotes), nearest-neighbor downscaled to
CELL x CELL. Source folders contain only the visible frames — leading and
trailing transparent frames have been trimmed.
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent / "itch"
CELL = 24
FRAMES = 8
OUT = ROOT / "emotes.png"

EMOTES = [
    "anger", "confused", "crying", "dizzy",
    "frustrated", "interrobang", "love", "music",
    "shine", "shock", "shy", "sigh",
    "sparkle", "speech", "stink", "sweat",
]


def sample(total: int) -> list[int]:
    return [i * total // FRAMES for i in range(FRAMES)]


def build_row(emote: str) -> Path:
    folder = ROOT / emote
    sources = sorted(folder.glob("Scene1_*.png"))
    if not sources:
        sys.exit(f"no source frames in {folder}")
    picks = [sources[i] for i in sample(len(sources))]
    row_path = folder / f".row_{emote}.png"
    subprocess.run(
        ["magick", *map(str, picks), "-filter", "point",
         "-resize", f"{CELL}x{CELL}", "+append", str(row_path)],
        check=True,
    )
    return row_path


def main() -> None:
    rows = [build_row(e) for e in EMOTES]
    subprocess.run(
        ["magick", *map(str, rows), "-append", str(OUT)],
        check=True,
    )
    for r in rows:
        r.unlink()
    print(f"wrote {OUT}  ({len(EMOTES)} rows x {FRAMES} frames @ {CELL}x{CELL})")


if __name__ == "__main__":
    main()
