# itch/ — emote source assets

This folder is intentionally empty in the repo. Contents are purchased
itch.io assets that cannot be redistributed.

## Setup

1. Download the pack: https://vf-traveller.itch.io/animated-emotes-for-visual-novels
2. Unzip it so that `187x187/` lands directly in this folder. Expected:

   ```
   itch/187x187/{1..16}/Scene1_*.png
   ```

3. From the repo root: `python3 itch/build_emotes.py` → writes
   `itch/emotes.png` (192×384, 8 cols × 16 rows @ 24×24).

The number → emote-name mapping and per-emote visible-frame ranges live
at the top of `build_emotes.py`.

See `ATTRIBUTIONS.md` at the repo root for credits and terms.
