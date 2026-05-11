#!/usr/bin/env python3
"""Build emotes.png from the vf-traveller animated emotes pack.

Expects the pack unzipped at itch/187x187/<N>/Scene1_*.png in its
original numbered layout. EMOTES holds the (number, name, first, last)
mapping: name is the row label, (first, last) is the inclusive index
range of visible frames (skipping leading/trailing fully-transparent
frames that the pack pads with).

Output: itch/emotes.png, 8 cols (frames) x 16 rows (emotes), nearest-
neighbor downscaled to CELL x CELL.
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ITCH = Path(__file__).resolve().parent
SRC = ITCH / "187x187"
OUT = ITCH / "emotes.png"
CELL = 24
FRAMES = 8

# (folder_number, name, first_visible, last_visible)
EMOTES = [
    ( 4, "anger",       1,  99),
    (13, "confused",    1,  63),
    ( 6, "crying",      1,  63),
    ( 2, "dizzy",       1,  89),
    ( 5, "frustrated",  1,  79),
    ( 8, "interrobang", 1,  63),
    (10, "love",        1,  78),
    (11, "music",       1,  79),
    ( 1, "shine",       1,  89),
    ( 9, "shock",       1,  63),
    ( 7, "shy",         1,  79),
    (14, "sigh",        1,  59),
    (15, "sparkle",     1,  79),
    (12, "speech",      1,  89),
    ( 3, "stink",       1,  96),
    (16, "sweat",       1,  59),
]


def sample(first: int, last: int) -> list[int]:
    span = last - first + 1
    return [first + i * span // FRAMES for i in range(FRAMES)]


def build_row(num: int, name: str, first: int, last: int) -> Path:
    folder = SRC / str(num)
    sources = sorted(folder.glob("Scene1_*.png"))
    if not sources:
        sys.exit(f"no source frames in {folder} (did you unzip the pack into itch/?)")
    picks = [sources[i] for i in sample(first, last)]
    row_path = SRC / f".row_{name}.png"
    subprocess.run(
        ["magick", *map(str, picks), "-filter", "point",
         "-resize", f"{CELL}x{CELL}", "+append", str(row_path)],
        check=True,
    )
    return row_path


def main() -> None:
    if not SRC.is_dir():
        sys.exit(
            f"missing {SRC}\n"
            "Download https://vf-traveller.itch.io/animated-emotes-for-visual-novels "
            "and unzip so the 187x187/ folder lands in itch/."
        )
    rows = [build_row(num, name, first, last) for num, name, first, last in EMOTES]
    subprocess.run(["magick", *map(str, rows), "-append", str(OUT)], check=True)
    for r in rows:
        r.unlink()
    print(f"wrote {OUT}  ({len(EMOTES)} rows x {FRAMES} frames @ {CELL}x{CELL})")


if __name__ == "__main__":
    main()
