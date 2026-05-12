# elthen-cats/ — cat sprite source

This folder is intentionally empty in the repo. The cat sprite sheet is a
purchased itch.io asset whose license prohibits redistribution, so it lives
outside git.

## Setup

1. Buy/download the pack: https://elthen.itch.io/2d-pixel-art-cat-sprites
2. Place the sheet at:

   ```
   elthen-cats/Cat Sprite Sheet.png
   ```

   It should be a 320×320 grid of 32×32 cells — see
   `firmware/assets/README.md` for the row → animation mapping.

`firmware/build/cat.rs` and `pygame/main.py` both read from this path.

See `ATTRIBUTIONS.md` at the repo root for credits and license terms.
