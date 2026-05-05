# assets

Source art for tama. Everything in here gets turned into bytes at build time —
nothing is loaded at runtime, there is no filesystem on the device.

## Pipeline

`build.rs` reads PNGs from this directory, converts pixels to RGB565, and emits
Rust `const` arrays into `$OUT_DIR/tama_sprite.rs`. `src/sprites.rs` re-exports
them with `include!`. No PNG decoder ships in the firmware — the bytes are
already laid out the way the GC9A01 wants them.

The framebuffer treats `0x07E0` (pure green in RGB565) as a transparency
sentinel. Source pixels that happen to encode to that exact value are nudged
by `+1` so they don't accidentally vanish. If you change the sentinel,
change it in `build.rs` and `src/fb.rs` together.

## `Cat Sprite Sheet.png`

320×320 grid of 32×32 cells, one animation per row:

| row | name     | frames |
|----:|----------|-------:|
|   0 | IDLE_A   |      4 |
|   1 | IDLE_B   |      4 |
|   2 | CLEAN_A  |      4 |
|   3 | CLEAN_B  |      4 |
|   4 | WALK_A   |      8 |
|   5 | WALK_B   |      8 |
|   6 | SLEEP    |      4 |
|   7 | PAW      |      6 |
|   8 | JUMP     |      7 |
|   9 | SCARED   |      8 |

Each frame becomes a `[u16; 1024]` in flash — no deduplication. At ~57 frames
total that's ~117 KB of cat, which is fine for now.

Recommended frame duration from the artist: 60–75ms.

Source: see `attributions.md`.

## `themes/themeNN.png` — wall tile kits

One 96×16 spritesheet per shelf-0 theme (14 in total: `theme00.png` through
`theme13.png`). Each sheet packs the six unique tiles of a theme horizontally,
in this fixed order:

| index | label | tile                                       |
|------:|:-----:|--------------------------------------------|
|     0 | A     | black ceiling w/ trim                      |
|     1 | C     | wall top band (slight highlight)           |
|     2 | D     | wall body                                  |
|     3 | E     | wall + baseboard (with shadow)             |
|     4 | F     | wall body, no-baseboard variant            |
|     5 | G     | floor (rim + black base)                   |

Pixel x-offset of tile `i` is `i * 16`. `build.rs` reads `theme11.png` and
stacks tiles A, C, F, G top-to-bottom into a 16×64 wall image, which it emits
as `WALL_PIXELS`.

Switching wallpapers later is just changing which theme file `build.rs` opens
and which indices it stacks. The generation script is at the bottom of this
file.

## Source: `../DayOff/Tileset/16x16 Tileset.png`

560×368 sheet of 16×16 tiles → a nominal 35×23 = **805 tile slots**, but only
**106 are unique**. The other 699 are duplicates. The big ones:

- 528 copies of the empty/transparent tile (separators between rooms)
- 29 copies of one wall fill
- 18, 17, 12, ... down a long tail

Why? **It's a catalog page, not a tile atlas.** The artist arranged the
unique tiles into mini wall mockups so a human can browse the kits.

### Layout

Two horizontal "shelves":

- **Top shelf** (rows 0–6): 14 wallpaper-style themes — hearts, stripes,
  bricks, etc. Each is **2 columns wide**, 7 rows tall. These are what
  `themes/` covers.
- **Bottom shelf** (rows 7–13): ~12 more themes, 3 columns wide — gray brick,
  red brick, ornate yellow wallpaper with a castle cutout, etc. Not yet
  imported; needs a different layout because of the extra column.

Each top-shelf theme is composed from a tiny tile kit. A worked example:

### Theme 11 (purple/lavender brick) — source layout

Source columns 22–23, rows 0–6. The 14 tile slots in this room only contain
6 unique tiles + transparent:

```
col 0  col 1
  A     .     A = black ceiling w/ purple trim
  C     .     C = wall top band (slight highlight)
  D     .     D = wall body (repeats 3×)
  D     .     E = wall + baseboard (with shadow)
  D     .     F = wall body, no-baseboard variant (col 1)
  E     F     G = floor (lavender rim + black base)
  G     G     . = transparent
```

Mapping to the spritesheet: tile at `theme11.png[16*i, 0]` for `i ∈ 0..6` is
A, C, D, E, F, G respectively.

### Generating the theme sheets

```python
from PIL import Image
src = Image.open("../DayOff/Tileset/16x16 Tileset.png").convert("RGBA")
def t(c, r): return src.crop((c*16, r*16, (c+1)*16, (r+1)*16))
for n in range(14):
    c0, c1 = 2*n, 2*n + 1
    sheet = Image.new("RGBA", (96, 16), (0,0,0,0))
    for i, tile in enumerate([t(c0,0), t(c0,1), t(c0,3), t(c0,5), t(c1,5), t(c0,6)]):
        sheet.paste(tile, (i*16, 0))
    sheet.save(f"themes/theme{n:02d}.png")
```

## Attributions

See `attributions.md` for source links and licenses.
