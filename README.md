# Palette Matcher

A color-matching game built with Rust and Bevy. Match the target color as closely as you can before the timer runs out!

## Prerequisites

- [Rust](https://rustup.rs/) (installs `cargo` and `rustc`)
- Windows: [Visual Studio 2022 Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) with the **Desktop development with C++** workload

## How to run

```
cd palette_matcher_game
cargo run
```

The first build will take several minutes while it compiles Bevy and its dependencies. Subsequent builds are much faster.

## How to play

1. Click **Start Game** on the title screen
2. You have **30 seconds** per round to match the target color
3. Use the **R**, **G**, **B** arrow buttons to adjust your color
4. Click **Submit** when you're happy with your match
5. After **3 rounds**, your total score is shown alongside the high score leaderboard

## Scoring

- Each round scores up to **1000 points** based on how closely your color matches the target
- Points are multiplied by how much time you have remaining — submit quickly for a higher score
- Let the timer expire and you score **0** for that round
- High scores are saved to `palette_matcher_game/scores.txt` between sessions
