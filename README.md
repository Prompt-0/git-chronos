# ⚡ git-chronos

> **High-Performance Animated Git History Visualization Utility in Rust**

`git-chronos` is a production-quality CLI tool written in Rust that transforms git commit histories into animated timeline visual representations (GIF / MP4). It renders node graph DAG connections, diff activity sparklines, and contributor rank badges frame-by-frame.

---

## 🎨 Features

- **Frame-by-Frame Timeline Animation**: Generates smooth frame sequences tracking repository evolution step-by-step.
- **Node Graph Visualization**: Highlights active commits, historical branches, and DAG relationships with glowing halos and hash badges.
- **Diff Activity & Churn Sparks**: Visualizes additions (+lines) and deletions (-lines) per commit alongside cumulative repository churn trends.
- **Contributor Badges**: Ranks authors dynamically with avatar initials, commit counts, and additions/deletions breakdowns.
- **Multiple Color Themes**: Built-in support for `dark`, `light`, `cyber`, `dracula`, `solarized`, and `neon` palettes.
- **Flexible Export**: Renders animated **GIF** natively and supports **MP4** video generation via `ffmpeg`.
- **Author & Commit Filtering**: Filter history by author pattern (`--author-filter`) or cap total frames (`--max-commits`).
- **Self-Contained & Fast**: Zero external font dependencies at runtime (TTF font embedded directly into binary).

---

## 📐 Architecture Overview

```
+-----------------------------------------------------------------------+
|                             git-chronos CLI                           |
+-----------------------------------------------------------------------+
                                   |
                                   v
+------------------+     +-------------------+     +--------------------+
|    cli::Cli      | --> |  git::GitParser   | --> | model::Commit      |
|  (Clap Options)  |     |  (Git Log Runner) |     | (Domain Entities)  |
+------------------+     +-------------------+     +--------------------+
                                                             |
                                                             v
+------------------+     +-------------------+     +--------------------+
|   theme::Theme   | --> |  render::Engine   | --> |   export::Encoder  |
| (Palette Loader) |     | (Canvas & Layout) |     |  (GIF / MP4 Output)|
+------------------+     +-------------------+     +--------------------+
                                                             |
                                                             v
                                                    +------------------+
                                                    |  chronos.gif     |
                                                    |  chronos.mp4     |
                                                    +------------------+
```

---

## 💻 CLI Usage

### Basic Command

```bash
# Generate chronos.gif for current repository
git-chronos

# Specify repository path and output file
git-chronos --repo /path/to/repo --output timeline.gif

# Select a cyber theme and custom frame rate
git-chronos --theme cyber --fps 15 --output cyber_history.gif

# Export to MP4 video format
git-chronos --output history.mp4 --format mp4
```

### CLI Command Options

```
Usage: git-chronos [OPTIONS]

Options:
  -r, --repo <REPO>            Path to target git repository [default: .]
  -o, --output <OUTPUT>        Output file path (e.g. chronos.gif, chronos.mp4) [default: chronos.gif]
  -f, --format <FORMAT>        Output animation format [possible values: gif, mp4]
      --fps <FPS>              Animation frame rate (frames per second) [default: 10]
  -t, --theme <THEME>          Visual color theme [default: dark] [possible values: dark, light, cyber, dracula, solarized, neon]
      --author-filter <FILTER> Filter commits by author (name or email pattern)
  -n, --max-commits <MAX>      Maximum number of commits to process
      --width <WIDTH>          Canvas width in pixels [default: 1280]
      --height <HEIGHT>        Canvas height in pixels [default: 720]
      --title <TITLE>          Custom title for visualization header
  -h, --help                   Print help information
  -V, --version                Print version information
```

---

## 🚀 Building & Testing

### Build from Source

```bash
cargo build --release
```

### Run Tests & Validation

```bash
# Run unit & integration tests
cargo test

# Run Clippy checks with zero warnings tolerance
cargo clippy -- -D warnings

# Format code
cargo fmt
```

---

## 📄 License

Dual-licensed under MIT or Apache 2.0.
