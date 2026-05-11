"""Pygame port of nineapt — a tiny preview of the cat-in-an-apartment game.

Mirrors the Rust target on the XIAO board: 240×240 native canvas, three rooms
side-by-side (bedroom, kitchen, bath), a scrolling camera that follows the cat,
and the same cat state machine (idle / walk / sleep / clean / paw / scared /
feed / pet). Keyboard maps to the three hardware buttons.

Controls:
  A → Feed     S → Pet      D → Play     Esc → Quit
"""

from __future__ import annotations

import random
import sys
from dataclasses import dataclass
from pathlib import Path

import pygame

ROOT = Path(__file__).resolve().parent
NINEAPT = ROOT.parent / "firmware"
DAYOFF = ROOT.parent / "DayOff"

SPRITESHEET = NINEAPT / "assets" / "Cat Sprite Sheet.png"
THEMES = {
    "bedroom": NINEAPT / "assets" / "themes" / "theme09.png",
    "kitchen": NINEAPT / "assets" / "themes" / "theme10.png",
    "bath": NINEAPT / "assets" / "themes" / "theme12.png",
}
PROP_SHEETS = {
    "bedroom": DAYOFF / "Objects" / "Bedroom" / "64x64 Bedroom.png",
    "kitchen": DAYOFF / "Objects" / "Kitchen" / "64x64 Kitchen.png",
    "bath": DAYOFF / "Objects" / "Bathroom" / "64x64 Bathroom.png",
}

# Match the Rust target.
W = H = 240
SCALE = 6
VIEW_NATIVE_W = W // SCALE  # 40
CELL = 32  # cat sprite cell
PROP_CELL = 64
TILE = 16
WALL_TILE_INDICES = [2, 2, 4, 5]
WALL_NATIVE_H = TILE * len(WALL_TILE_INDICES)  # 64
FLOOR_SEAM_NATIVE_Y = 48
ROOM_WIDTH = 160
BG = (0xFF, 0xDB, 0xE5)  # ≈ 0xFEDD in rgb565
CAT_FOOT_NUDGE = 4

PROP_INDEX = {
    "bedroom": {
        "bed": (0, 0), "plant_md": (2, 0), "plant_sm": (3, 0),
        "bookshelf": (7, 0), "portrait_woman": (8, 0), "neon_fab": (10, 0),
        "window": (5, 1), "chair": (6, 1), "bedside_table": (8, 1),
    },
    "kitchen": {
        "counter_set": (0, 0), "fridge": (4, 0),
        "spatula": (5, 0), "spatula_sm": (6, 0), "spatula_holes": (7, 0),
        "knife": (8, 0), "bowl_food": (4, 1),
        "hanging_drawer": (5, 1), "painting": (9, 1),
    },
    "bath": {
        "toilet": (0, 0), "tp_roll": (1, 0), "towel_rack": (2, 0),
        "bathtub": (3, 0), "mirror": (4, 0), "trashcan": (6, 0),
        "shower_shelf_full": (8, 0), "sink": (0, 1),
        "bath_hanging_drawer": (1, 1),
    },
}

# (name, x_native, y_native) — pulled from src/world.rs.
LAYOUTS = {
    "bedroom": [
        ("bookshelf", 2, 13),
        ("plant_md", 36, 32),
        ("neon_fab", 56, 8),
        ("bed", 54, 32),
        ("portrait_woman", 96, 4),
        ("bedside_table", 96, 29),
        ("window", 126, 6),
        ("chair", 122, 33),
        ("plant_sm", 145, 37),
    ],
    "kitchen": [
        ("painting", 68, 2),
        ("hanging_drawer", 113, 4),
        ("spatula_holes", 8, 3),
        ("spatula", 20, 4),
        ("spatula_sm", 32, 6),
        ("knife", 44, 4),
        ("counter_set", 0, 23),
        ("bowl_food", 80, 39),
        ("fridge", 138, 27),
    ],
    "bath": [
        ("shower_shelf_full", 6, 8),
        ("towel_rack", 44, 22),
        ("mirror", 101, 4),
        ("bath_hanging_drawer", 140, 14),
        ("tp_roll", 78, 31),
        ("bathtub", 4, 34),
        ("toilet", 63, 33),
        ("sink", 98, 32),
        ("trashcan", 145, 34),
    ],
}

ROOMS = ["bedroom", "kitchen", "bath"]


# --- Anim definitions: (row, frames, frame_ms). Mirrors build/cat.rs + src/cat.rs.
ANIMS = {
    "idle":   (0, 4, 250),
    "walk":   (4, 8, 120),
    "sleep":  (6, 4, 400),
    "paw":    (7, 6, 150),
    "clean":  (2, 4, 200),
    "scared": (9, 8, 100),
    "feed":   (8, 7, 120),   # JUMP row
    "pet":    (1, 4, 250),   # IDLE_B row
}

WALK_STEP_MS = 50
WALK_PX_PER_STEP = 1


def trim(surf: pygame.Surface) -> pygame.Surface:
    rect = surf.get_bounding_rect(min_alpha=1)
    if rect.width == 0 or rect.height == 0:
        return surf
    return surf.subsurface(rect).copy()


def upscale(surf: pygame.Surface) -> pygame.Surface:
    return pygame.transform.scale(surf, (surf.get_width() * SCALE, surf.get_height() * SCALE))


def load_walls() -> dict[str, pygame.Surface]:
    walls = {}
    for name, path in THEMES.items():
        theme = pygame.image.load(str(path)).convert_alpha()
        wall = pygame.Surface((TILE, WALL_NATIVE_H), pygame.SRCALPHA)
        for slot, idx in enumerate(WALL_TILE_INDICES):
            tile = theme.subsurface(pygame.Rect(idx * TILE, 0, TILE, TILE))
            wall.blit(tile, (0, slot * TILE))
        walls[name] = upscale(wall)
    return walls


def load_props() -> dict[str, dict[str, pygame.Surface]]:
    out = {}
    for room, path in PROP_SHEETS.items():
        sheet = pygame.image.load(str(path)).convert_alpha()
        room_props = {}
        for name, (col, row) in PROP_INDEX[room].items():
            cell = sheet.subsurface(
                pygame.Rect(col * PROP_CELL, row * PROP_CELL, PROP_CELL, PROP_CELL)
            )
            room_props[name] = upscale(trim(cell.copy()))
        out[room] = room_props
    return out


def load_cat_frames() -> dict[str, list[pygame.Surface]]:
    sheet = pygame.image.load(str(SPRITESHEET)).convert_alpha()
    frames = {}
    for state, (row, count, _ms) in ANIMS.items():
        frames[state] = [
            upscale(sheet.subsurface(pygame.Rect(f * CELL, row * CELL, CELL, CELL)).copy())
            for f in range(count)
        ]
    return frames


@dataclass
class Cat:
    world_x: int
    facing: int = 1
    state: str = "idle"
    entered_ms: int = 0
    last_roll_ms: int = 0
    last_step_ms: int = 0
    frame: int = 0
    last_frame_ms: int = 0

    DWELLS = {
        "walk": 4000, "sleep": 10000, "paw": 2000, "clean": 3000,
        "scared": 2000, "feed": 3000, "pet": 3000,
    }

    def transition(self, next_state: str, now: int) -> None:
        if next_state == self.state:
            return
        self.state = next_state
        self.entered_ms = now
        self.last_roll_ms = now
        self.frame = 0
        self.last_frame_ms = now

    def idle_dice(self, now: int) -> str | None:
        if now - self.last_roll_ms < 1000:
            return None
        self.last_roll_ms = now
        roll = random.randint(0, 23)
        if roll == 0:
            self.facing = random.choice([-1, 1])
            return "walk"
        if roll == 1:
            return "clean"
        if roll == 2:
            return "sleep"
        return None

    def tick(self, now: int, action: str | None, world_w: int) -> pygame.Surface:
        if action is not None:
            self.transition(action, now)

        dwell = now - self.entered_ms
        if self.state == "idle":
            nxt = self.idle_dice(now)
            if nxt:
                self.transition(nxt, now)
        else:
            limit = self.DWELLS.get(self.state)
            if limit is not None and dwell > limit:
                self.transition("idle", now)

        if self.state == "walk":
            while now - self.last_step_ms >= WALK_STEP_MS:
                self.world_x += self.facing * WALK_PX_PER_STEP
                if self.world_x <= 0:
                    self.world_x = 0
                    self.facing = 1
                elif self.world_x >= world_w - 1:
                    self.world_x = world_w - 1
                    self.facing = -1
                self.last_step_ms += WALK_STEP_MS
        else:
            self.last_step_ms = now

        row, count, frame_ms = ANIMS[self.state]
        if now - self.last_frame_ms >= frame_ms:
            self.frame = (self.frame + 1) % count
            self.last_frame_ms = now

        return CAT_FRAMES[self.state][self.frame]


CAT_FRAMES: dict[str, list[pygame.Surface]] = {}


def draw_world(
    surface: pygame.Surface,
    walls: dict[str, pygame.Surface],
    props: dict[str, dict[str, pygame.Surface]],
    cam_x: int,
    wall_screen_y: int,
) -> None:
    room_x0 = 0
    wall_tile_native = TILE
    for room in ROOMS:
        room_x1 = room_x0 + ROOM_WIDTH
        if room_x1 > cam_x and room_x0 < cam_x + VIEW_NATIVE_W:
            wall = walls[room]
            tx = room_x0
            while tx < room_x1:
                surface.blit(wall, ((tx - cam_x) * SCALE, wall_screen_y))
                tx += wall_tile_native
            for name, px, py in LAYOUTS[room]:
                sprite = props[room][name]
                sx = (room_x0 + px - cam_x) * SCALE
                sy = wall_screen_y + py * SCALE
                surface.blit(sprite, (sx, sy))
        room_x0 = room_x1


def main() -> None:
    pygame.init()
    screen = pygame.display.set_mode((W, H))
    pygame.display.set_caption("nineapt — preview")
    clock = pygame.time.Clock()

    walls = load_walls()
    props = load_props()
    CAT_FRAMES.update(load_cat_frames())

    sprite_px = CELL * SCALE
    dy_centered = (H - sprite_px) // 2
    cat_screen_y = dy_centered + CAT_FOOT_NUDGE
    floor_anchor_y = dy_centered + CELL * SCALE
    wall_screen_y = floor_anchor_y - FLOOR_SEAM_NATIVE_Y * SCALE

    world_w = ROOM_WIDTH * len(ROOMS)
    cat = Cat(world_x=world_w // 2)
    cat.entered_ms = pygame.time.get_ticks()
    cat.last_roll_ms = cat.entered_ms
    cat.last_step_ms = cat.entered_ms
    cat.last_frame_ms = cat.entered_ms

    flip_cache: dict[int, pygame.Surface] = {}

    while True:
        action: str | None = None
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                sys.exit(0)
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_ESCAPE:
                    pygame.quit()
                    sys.exit(0)
                elif event.key == pygame.K_a:
                    action = "feed"
                elif event.key == pygame.K_s:
                    action = "pet"
                elif event.key == pygame.K_d:
                    action = "paw"

        now = pygame.time.get_ticks()
        frame = cat.tick(now, action, world_w)

        max_cam = max(world_w - VIEW_NATIVE_W, 0)
        cam_x = max(0, min(cat.world_x - VIEW_NATIVE_W // 2, max_cam))

        screen.fill(BG)
        draw_world(screen, walls, props, cam_x, wall_screen_y)

        if cat.facing < 0:
            key = id(frame)
            flipped = flip_cache.get(key)
            if flipped is None:
                flipped = pygame.transform.flip(frame, True, False)
                flip_cache[key] = flipped
            blit_sprite = flipped
        else:
            blit_sprite = frame

        cat_screen_x = (cat.world_x - cam_x) * SCALE - sprite_px // 2
        screen.blit(blit_sprite, (cat_screen_x, cat_screen_y))

        pygame.display.flip()
        clock.tick(60)


if __name__ == "__main__":
    main()
