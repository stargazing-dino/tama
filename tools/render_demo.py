"""Render a flat panorama of the world — all rooms side-by-side, or a single room.

Used for iterating on prop placement before wiring rooms up in src/world.rs.
Mirrors the world geometry: wall art is 16×WALL_NATIVE_H native, floor seam at
y=48, room widths in native pixels.

Usage:
  python3 tools/render_demo.py             # full panorama with labels
  python3 tools/render_demo.py bedroom     # just one room

Edit BEDROOM_PROPS / KITCHEN_PROPS / BATH_PROPS below and re-run to preview.
"""

import argparse
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
SPRITESHEET = ROOT / "assets" / "Cat Sprite Sheet.png"
THEMES = {
    "bedroom": ROOT / "assets" / "themes" / "theme09.png",
    "kitchen": ROOT / "assets" / "themes" / "theme10.png",
    "bath": ROOT / "assets" / "themes" / "theme12.png",
}
BEDROOM_SHEET = ROOT / "DayOff" / "Objects" / "Bedroom" / "64x64 Bedroom.png"
KITCHEN_SHEET = ROOT / "DayOff" / "Objects" / "Kitchen" / "64x64 Kitchen.png"
BATH_SHEET = ROOT / "DayOff" / "Objects" / "Bathroom" / "64x64 Bathroom.png"
OUT = ROOT / "demo.png"

# Match src/main.rs + build.rs.
SCALE = 6
CELL = 32  # cat sprite cell
TILE = 16  # theme tile
WALL_TILE_INDICES = [2, 2, 4, 5]  # D plain wall ×2, F wall, G floor
WALL_NATIVE_H = TILE * len(WALL_TILE_INDICES)  # 64
FLOOR_SEAM_NATIVE_Y = 48
ROOM_NATIVE_W = 160
ROOM_ORDER = ["bedroom", "kitchen", "bath"]
BG_565 = 0xFEDD
LABEL_BAND = 24

# Bedroom prop sheet is a 12×7 grid of 64×64 cells; only first 2 rows used.
PROP_CELL = 64
BEDROOM_PROP_INDEX = {
    "bed": (0, 0), "plant_lg": (1, 0), "plant_md": (2, 0), "plant_sm": (3, 0),
    "stool": (4, 0), "book_side": (5, 0), "lamp": (6, 0), "bookshelf": (7, 0),
    "portrait_woman": (8, 0), "portrait_man": (9, 0), "neon_fab": (10, 0), "door": (11, 0),
    "wardrobe": (0, 1), "wardrobe_open": (1, 1), "mini_fridge": (2, 1), "mini_fridge_open": (3, 1),
    "window_lg": (4, 1), "window": (5, 1), "chair": (6, 1), "mirror": (7, 1),
    "bedside_table": (8, 1),
}

KITCHEN_PROP_INDEX = {
    "counter_set": (0, 0), "counter_set_open": (1, 0),
    "counter": (2, 0), "counter_open": (3, 0),
    "fridge": (4, 0),
    "spatula": (5, 0), "spatula_sm": (6, 0), "spatula_holes": (7, 0), "knife": (8, 0),
    "dish": (9, 0), "dish_apple": (10, 0), "apple": (11, 0),
    "dish_carrot": (0, 1), "carrot": (1, 1), "dish_meat": (2, 1),
    "bowl": (3, 1), "bowl_food": (4, 1),
    "hanging_drawer": (5, 1), "hanging_drawer_open": (6, 1),
    "dish_rack": (7, 1), "cutting_board": (8, 1), "painting": (9, 1),
}

BATH_PROP_INDEX = {
    "toilet": (0, 0), "tp_roll": (1, 0), "towel_rack": (2, 0),
    "bathtub": (3, 0), "mirror": (4, 0), "sink_long": (5, 0),
    "trashcan": (6, 0), "shower_shelf": (7, 0), "shower_shelf_full": (8, 0),
    "soap_red": (9, 0), "soap_orange": (10, 0), "soap_green": (11, 0),
    "sink": (0, 1), "bath_hanging_drawer": (1, 1), "bath_hanging_drawer_open": (2, 1),
}

# (prop_name, x_native, y_native). x is room-local; y is native px down from top
# of wall art (so floor seam is at y=48). Floor-standing → y = 48 - prop_h.
# Starter layouts — edit freely.
BEDROOM_PROPS = [
    ("bookshelf", 2, 13),         # 30×35 floor (2..32)
    ("plant_md", 36, 32),         # 17×16 floor (36..53)
    ("neon_fab", 56, 8),          # 22×21 wall above bed (56..78)
    ("bed", 54, 32),              # 40×16 floor (54..94)
    ("portrait_woman", 96, 4),    # 24×22 wall (96..120)
    ("bedside_table", 96, 29),    # 22×19 floor (96..118, lamp baked in)
    ("window", 126, 6),           # 31×31 wall (126..157)
    ("chair", 122, 33),           # 13×15 floor in front of window
    ("plant_sm", 145, 37),        # 10×11 floor (145..155)
]
KITCHEN_PROPS = [
    # Wall items (y < 23, above counter)
    ("painting", 68, 2),           # 39×30 big TV-style wall art (68..107)
    ("hanging_drawer", 113, 4),    # 19×11 wall cabinet (113..132)
    ("spatula_holes", 8, 3),       # 7×14 hanging spatula (8..15)
    ("spatula", 20, 4),            # 7×14 (20..27)
    ("spatula_sm", 32, 6),         # 5×12 (32..37)
    ("knife", 44, 4),              # 5×14 (44..49)
    # Counter unit (floor) — sink, stove, dishes baked in
    ("counter_set", 0, 23),        # 64×25 (0..64)
    # Floor zone right of counter
    ("bowl_food", 80, 39),         # 11×9 cat food bowl (80..91)
    ("fridge", 138, 27),           # 18×21 fridge (138..156)
]
BATH_PROPS = [
    # Wall items (y < 30)
    ("shower_shelf_full", 6, 8),      # 28×18 shelf w/ bottles above tub (6..34, y=8..26)
    ("towel_rack", 44, 22),           # 17×14 towel rack near tub (44..61, y=22..36)
    ("mirror", 101, 4),               # 16×22 vanity mirror above sink (101..117, y=4..26)
    ("bath_hanging_drawer", 140, 14), # 19×11 wall shelf above trashcan (140..159, y=14..25)
    ("tp_roll", 78, 31),              # 9×10 TP roll right of toilet (78..87, y=31..41)
    # Floor items (bottom at y=48)
    ("bathtub", 4, 34),               # 35×14 bathtub (4..39, y=34..48)
    ("toilet", 63, 33),               # 13×15 toilet (63..76, y=33..48)
    ("sink", 98, 32),                 # 20×16 sink (98..118, y=32..48)
    ("trashcan", 145, 34),            # 9×14 trashcan (145..154, y=34..48)
]

PROPS_BY_ROOM = {
    "bedroom": BEDROOM_PROPS,
    "kitchen": KITCHEN_PROPS,
    "bath": BATH_PROPS,
}


def rgb565_to_rgb(v: int) -> tuple[int, int, int]:
    r = (v >> 11) & 0x1F
    g = (v >> 5) & 0x3F
    b = v & 0x1F
    return (r << 3) | (r >> 2), (g << 2) | (g >> 4), (b << 3) | (b >> 2)


def build_wall(theme_path: Path) -> Image.Image:
    theme = Image.open(theme_path).convert("RGBA")
    wall = Image.new("RGBA", (TILE, WALL_NATIVE_H), (0, 0, 0, 0))
    for slot, idx in enumerate(WALL_TILE_INDICES):
        tile = theme.crop((idx * TILE, 0, (idx + 1) * TILE, TILE))
        wall.paste(tile, (0, slot * TILE), tile)
    return wall


def trim(img: Image.Image) -> Image.Image:
    bbox = img.getbbox()
    return img.crop(bbox) if bbox else img


def load_props(sheet_path: Path, index: dict[str, tuple[int, int]]) -> dict[str, Image.Image]:
    sheet = Image.open(sheet_path).convert("RGBA")
    props = {}
    for name, (col, row) in index.items():
        cell = sheet.crop(
            (col * PROP_CELL, row * PROP_CELL, (col + 1) * PROP_CELL, (row + 1) * PROP_CELL)
        )
        props[name] = trim(cell)
    return props


def upscale(img: Image.Image, scale: int) -> Image.Image:
    return img.resize((img.width * scale, img.height * scale), Image.NEAREST)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "room", nargs="?", choices=ROOM_ORDER,
        help="render only this room (default: full panorama)",
    )
    args = parser.parse_args()

    walls = {name: build_wall(path) for name, path in THEMES.items()}
    props_by_room = {
        "bedroom": load_props(BEDROOM_SHEET, BEDROOM_PROP_INDEX),
        "kitchen": load_props(KITCHEN_SHEET, KITCHEN_PROP_INDEX),
        "bath": load_props(BATH_SHEET, BATH_PROP_INDEX),
    }

    cat_sheet = Image.open(SPRITESHEET).convert("RGBA")
    cat = cat_sheet.crop((0, 0, CELL, CELL))  # IDLE_A frame 0

    rooms = [args.room] if args.room else ROOM_ORDER
    show_labels = args.room is None

    world_w = ROOM_NATIVE_W * len(rooms) * SCALE
    world_h = WALL_NATIVE_H * SCALE
    band = LABEL_BAND if show_labels else 0
    canvas = Image.new("RGBA", (world_w, world_h + band), (*rgb565_to_rgb(BG_565), 255))

    for room_idx, room_name in enumerate(rooms):
        room_x0_native = room_idx * ROOM_NATIVE_W
        wall_scaled = upscale(walls[room_name], SCALE)
        for tx in range(0, ROOM_NATIVE_W, TILE):
            canvas.paste(wall_scaled, ((room_x0_native + tx) * SCALE, 0), wall_scaled)

        room_props = props_by_room.get(room_name, {})
        for prop_name, px, py in PROPS_BY_ROOM[room_name]:
            p = upscale(room_props[prop_name], SCALE)
            canvas.paste(p, ((room_x0_native + px) * SCALE, py * SCALE), p)

    # Cat for scale — at the start of the first rendered room.
    cat_native_x = 30
    cat_native_y = FLOOR_SEAM_NATIVE_Y - CELL + 4  # +4 foot nudge
    cat_scaled = upscale(cat, SCALE)
    canvas.paste(cat_scaled, (cat_native_x * SCALE, cat_native_y * SCALE), cat_scaled)

    if show_labels:
        draw = ImageDraw.Draw(canvas)
        try:
            font = ImageFont.truetype("/System/Library/Fonts/Menlo.ttc", 16)
        except OSError:
            font = ImageFont.load_default()
        for room_idx, room_name in enumerate(rooms):
            x = room_idx * ROOM_NATIVE_W * SCALE
            if room_idx > 0:
                draw.line([(x, 0), (x, world_h)], fill=(255, 0, 255, 200), width=1)
            draw.text((x + 8, world_h + 4), room_name, fill=(40, 40, 40, 255), font=font)

    canvas.save(OUT)
    print(f"wrote {OUT} ({canvas.width}×{canvas.height})")


if __name__ == "__main__":
    main()
