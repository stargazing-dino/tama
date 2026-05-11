"""Render a 64×64-cell prop sheet with grid coords + trimmed AABB labels.

Usage:
  python3 tools/render_sheet.py kitchen   # → sheet_kitchen.png
  python3 tools/render_sheet.py bedroom   # → sheet_bedroom.png

Each cell shows the original 64×64 cell with a magenta box around the trimmed
opaque bbox and "(col,row)  WxH" text below it. Lets you see what asset is
actually where, and whether neighboring cells bleed into each other.
"""

import argparse
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
DAYOFF = ROOT.parent / "DayOff"
SHEETS = {
    "bedroom": DAYOFF / "Objects" / "Bedroom" / "64x64 Bedroom.png",
    "kitchen": DAYOFF / "Objects" / "Kitchen" / "64x64 Kitchen.png",
    "bath": DAYOFF / "Objects" / "Bathroom" / "64x64 Bathroom.png",
}
CELL = 64
SCALE = 3
LABEL_H = 28
PADDING = 4
BG = (30, 30, 36, 255)
GRID = (90, 90, 100, 255)
BOX = (255, 0, 255, 255)


def trim_bbox(cell: Image.Image) -> tuple[int, int, int, int] | None:
    return cell.getbbox()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sheet", choices=SHEETS.keys())
    args = parser.parse_args()

    sheet = Image.open(SHEETS[args.sheet]).convert("RGBA")
    cols = sheet.width // CELL
    rows = sheet.height // CELL

    cell_screen = CELL * SCALE
    tile_w = cell_screen + PADDING * 2
    tile_h = cell_screen + LABEL_H + PADDING * 2

    canvas = Image.new("RGBA", (tile_w * cols, tile_h * rows), BG)
    draw = ImageDraw.Draw(canvas)

    try:
        font = ImageFont.truetype("/System/Library/Fonts/Menlo.ttc", 13)
    except OSError:
        font = ImageFont.load_default()

    for row in range(rows):
        for col in range(cols):
            cell = sheet.crop((col * CELL, row * CELL, (col + 1) * CELL, (row + 1) * CELL))
            cx = col * tile_w + PADDING
            cy = row * tile_h + PADDING

            # Faint grid box
            draw.rectangle(
                [cx, cy, cx + cell_screen - 1, cy + cell_screen - 1],
                outline=GRID, width=1,
            )
            scaled = cell.resize((cell_screen, cell_screen), Image.NEAREST)
            canvas.paste(scaled, (cx, cy), scaled)

            bbox = trim_bbox(cell)
            if bbox:
                x0, y0, x1, y1 = bbox
                w, h = x1 - x0, y1 - y0
                draw.rectangle(
                    [
                        cx + x0 * SCALE,
                        cy + y0 * SCALE,
                        cx + x1 * SCALE - 1,
                        cy + y1 * SCALE - 1,
                    ],
                    outline=BOX, width=2,
                )
                label = f"({col},{row}) {w}×{h}"
            else:
                label = f"({col},{row}) empty"

            draw.text(
                (cx, cy + cell_screen + 2),
                label,
                fill=(230, 230, 230, 255),
                font=font,
            )

    out = ROOT / f"sheet_{args.sheet}.png"
    canvas.save(out)
    print(f"wrote {out} ({canvas.width}×{canvas.height})")


if __name__ == "__main__":
    main()
