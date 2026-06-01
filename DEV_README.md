# DEV_README.md (Art & Assets Requirements)

This document outlines the assets required to fully swap out the placeholder graphics for final pixel art. The game currently uses procedural shapes and text but is architected to seamlessly drop in `.png` and `.ttf` files.

## Font Needed
- **googleicons.ttf**: Place this in the `assets/` folder. This is used for standard crisp text styling in a pixel-art setting (or use any retro pixel font of choice).

## Textures Needed (All inside `assets/` directory)

### 1. `cards.png` (The Playing Cards)
- **Structure**: 4 rows (suits) x 10 columns (ranks 1 through 10).
- **Individual Sprite Size**: `80px` wide by `120px` tall.
- **Total Image Size**: `800px` wide by `480px` tall.
- **Row Order** (from top to bottom):
  1. Iron (Gray style)
  2. Ember (Red style)
  3. Bone (White style)
  4. Glass (Blue style)
- **Note**: The code mathematically slices the texture into tenths horizontally and quarters vertically based on the card's suit and rank. Keep them packed edge-to-edge (no padding between sprites).

### 2. `card_back.png`
- **Total Image Size**: `80px` wide by `120px` tall.
- **Description**: A single image representing the back of the deck. A distinct, sharp geometric pattern for the deck back.

### 3. `table_bg.png`
- **Total Image Size**: `1920px` wide by `1080px` tall (16:9 ratio).
- **Description**: The background poker/card table. It scales to fit the window, so make it a high-resolution pixel art background (or scaled-up pixel art). Green/dark-red felt with faint wear-and-tear.

### 4. `characters.png`
- **Structure**: Grid of portraits.
- **Individual Sprite Size**: `64px` by `64px` (or scaled up to `128x128` depending on how much detail you want).
- **Total Image Size**: Depends on layout, but ensure they are uniformly gridded.
- **Frames**: Each character needs 2 horizontal frames (Idle, Tell/Cheat).

### 5. `zone_cards.png`
- **Individual Sprite Size**: `200px` wide by `100px` tall (horizontal cards).
- **Total Image Size**: `200px` wide by `2500px` tall (1 column, 25 rows).
- **Description**: Darker, mysterious cards with glowing glyphs representing the zone rules.

## Adding Assets
The game is already built to look for these files in the `assets/` folder relative to where the binary is run.
If you stick to the sizes above, especially for `cards.png`, you won't need to change a single line of code! The engine handles debuffs, shaking, and selection highlighting procedurally.
