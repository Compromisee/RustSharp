<div align="center">

# 🃏 SHARP

**A high-stakes, pixel-art roguelike card game of deception and strategy.**

![Rust](https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Macroquad](https://img.shields.io/badge/Engine-Macroquad-blue?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-In_Development-orange?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

*Built with Rust and featuring a custom CRT Balatro-esque shader.*

</div>

---

## 📖 Overview

**SHARP** is a deep, highly replayable deck-building roguelike where you play a fictional underground card game called **"CUT"**. 

You'll face off against 30 unique CPU opponents and 10 ruthless Bosses. But here is the catch: **your opponents will cheat.** You must learn their behavioral "tells" (visual and auditory cues), call out their sleight of hand, manage your Credibility, and strategically build your deck and passives to survive all 10 rounds and reach **The House**.

## ✨ Key Features

- **The Game of "CUT"**: A fast-paced 40-card game (4 suits: ⚙️ Iron, 🔥 Ember, 🦴 Bone, 🪟 Glass; Ranks 1-10). Draw 5, play 3. First to 12 points wins.
- **Dynamic Zone Cards**: 25 unique Zone Cards flip each round to change the scoring rules (e.g., *Peak Zone* = highest card wins, *Balance Zone* = closest to 15 wins, *Iron Wall* = Iron cards score double).
- **Cheat Detection System**: Every opponent has a unique cheating style and a specific "tell" (e.g., screen shakes, glowing cards, hesitation). Catch them to steal points and earn bonus cash, but call wrong and you'll lose your Credibility.
- **Deep Meta-Progression**: 
  - 🪙 **Coins**: Buy Action Cards, Wenter Cards (economy buffs), and Control Cards (passive buffs) during the run.
  - 💎 **Gems**: Unlock permanent Skill Tree nodes and Ascensions between runs.
- **30 Unique CPUs & 10 Bosses**: From *Benny the Beginner* who clumsily swaps cards, to *The Algorithm* who uses pure statistical anomaly detection, to *The House* who cheats seamlessly.
- **Ascension System**: 15 levels of escalating difficulty modifiers for endless replayability.

---

## 🛠️ Architecture & Tech Stack

This game is built from the ground up in **Rust** using the [**Macroquad**](https://github.com/not-fl3/macroquad) immediate-mode rendering engine. 

- **State Machine Driven**: Clean separation between `RunState`, `MatchState`, and `GamePhase`.
- **Procedural Fallbacks**: The game is designed to work completely procedurally (drawing shapes and text) if assets are missing, gracefully swapping to `.png` textures once placed in the `assets/` folder.
- **Custom Shaders**: Features a highly optimized GLSL fragment shader applied to a Render Target, achieving barrel distortion, scanlines, and vignette for that perfect retro-CRT monitor feel.

---

## 🚀 Getting Started

### Prerequisites
You need to have **Rust** and **Cargo** installed. If you don't have them, install them via [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build and Run
Clone the repository and run the game natively:
```bash
git clone https://github.com/compromisee/RustSharp.git
cd sharp_game
cargo run --release
```

*Note: The first compilation might take a minute as it builds the engine and dependencies.*

---

## 🎨 Asset Integration (For Artists)

The codebase is fully primed to accept art. See `DEV_README.md` for exact sprite dimensions and requirements. To inject art into the game, simply drop the following files into the `assets/` directory:
- `cards.png` (800x480 spritesheet)
- `card_back.png` (80x120 pattern)
- `table_bg.png` (1920x1080 felt background)
- `characters.png` (64x64 or 128x128 grid)
- `googleicons.ttf` (or any pixel font)

---

## 🗺️ Roadmap

- [x] Core "CUT" card game logic.
- [x] Initial CPU AI and Cheat Tell architecture.
- [x] GLSL CRT Shader integration.
- [ ] Implement all 25 Zone Cards.
- [ ] Populate all 30 CPUs and 10 Boss algorithms.
- [ ] Finalize the interactive Shop UI (Action/Control/Wenter cards).
- [ ] Connect the Skill Tree & Gem Economy.
- [ ] Audio integration (Card snaps, chip clinks, synthwave ambient tracks).

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/yourusername/sharp_game/issues). 

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.
