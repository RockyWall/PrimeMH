# PrimeMH

# THIS TOOL IS DETECTED BY BLIZZARD, RUNNING THIS MAY RESULT IN A BAN

A real-time map overlay tool for **Diablo II: Resurrected**, written in Rust. PrimeMH reads game memory to render an interactive map overlay with monster tracking, item filtering, pathfinding, etc all as a transparent, always-on-top window.

First released 2024-02-12, it received a ban wave on 2026-02-26.

Discord server https://discord.gg/CGmHS3xFWc


## Features

- **Map Overlay** — Renders the current area map as a transparent overlay on top of D2R. Supports center, top-left, and top-right positioning with configurable opacity and scale.
- **Monster Display** — Shows normal, minion, champion, unique, and boss monsters on the map with customizable colors, sizes, and immunity indicators.
- **Missile Tracking** — Visualises incoming projectiles (fire, cold, poison, lightning, physical, magic) with per-element color and size settings.
- **Item Log & Alerts** — On-screen item log with configurable duration, ground item alerts, sound notifications, and text-to-speech voice callouts.
- **Item Filter** — YAML-based item filter (`itemfilter.yml`) to define rules by item quality, sockets, ethereal status, and more.
- **Buff Bar** — Displays active buffs/debuffs with icons and countdown timers.
- **Lines & Pathfinding** — Draws lines and computed paths to waypoints, level exits, quest objectives, and bosses.
- **Points of Interest** — Marks portals (with area names), shrines (with labels), and chests on the map.
- **Party Info** — Shows information about other players in your party, including hostile status detection.
- **Item Tooltips** — Enhanced hover tooltips for items.
- **Configurable Hotkeys** — Toggle map, toggle menu, and exit hotkeys with modifier key support (Ctrl, Alt, Shift, Win).
- **Multi-Language Support** — English, German, Spanish, French, Italian, Korean, Polish, and Chinese.
- **Map Caching** — Generated map data is cached per seed/difficulty for fast subsequent loads.

## Requirements

- **Windows** (uses Win32 API for overlay and memory reading)
- **Rust** (2021 edition) — install via [rustup](https://rustup.rs/)
- **Diablo II: Lord of Destruction data files** — classic D2 LoD `.mpq` files placed in the `bin/d2lod/` directory
- **d2-mapgen.exe** — map generation binary (expected at `bin/d2-mapgen.exe` by default)

## Getting Started

### 1. Clone the repository

```bash
git clone <repo-url>
cd PrimeMH
```

### 2. Set up game data

Place the required Diablo II: Lord of Destruction `.mpq` files into `bin/d2lod/`:

### 3. Build & Run

```bash
# Debug build
cargo build
cargo run

# Release build (optimized, smaller binary)
cargo build --release
cargo run --release
```

## Configuration

All settings are managed through **`settings.toml`**. The file is auto-generated with defaults on first run if it doesn't exist.

### General

| Setting | Default | Description |
|---|---|---|
| `d2lodpath` | `"bin/d2lod"` | Path to D2 LoD data files |
| `blacha_exe` | `"bin/d2-mapgen.exe"` | Path to the map generation binary |
| `render_scale` | `1.0` | Map render scale multiplier |
| `fps_limit` | `60` | Frame rate cap |
| `map_position` | `"Center"` | Map position: `Center`, `TopLeft`, or `TopRight` |
| `multisampling` | `8` | Anti-aliasing sample count |
| `vsync` | `true` | Vertical sync |
| `overlay_mode` | `true` | Transparent always-on-top overlay |
| `language` | `"enUS"` | UI language code |

### Visual

| Setting | Default | Description |
|---|---|---|
| `scale` | `3.3` | Map zoom scale |
| `map_opacity` | `0.65` | Map transparency (0.0–1.0) |
| `always_show_map` | `true` | Keep map visible at all times |
| `hide_map_menus_open` | `true` | Hide map when game menus are open |

### Hotkeys

| Setting | Default | Description |
|---|---|---|
| `hotkey_toggle_map` | `"PageDown"` | Toggle map visibility |
| `hotkey_toggle_menu` | `"Home"` | Toggle settings menu |
| `hotkey_exit` | `"^End"` | Exit PrimeMH (`^` = Ctrl) |

Modifier prefixes: `^` Ctrl, `!` Alt, `+` Shift, `#` Win.

### Additional Sections

See `settings.toml` for full configuration of: **monsters**, **missiles**, **item_log**, **buffbar**, **lines**, **chests**, **portals**, **shrines**, **item_hover**, and **party_info**.

## Item Filter

Define item filter rules in **`itemfilter.yml`**. Each entry is an item base name with a list of matching conditions:

```yaml
Shako:
  - quality: [unique]     # Harlequin Crest
    ethereal: false

Archon Plate:
  - quality: [normal, superior]
    sockets: [3, 4]
  - quality: [normal]
    sockets: [0]
    ethereal: true         # Eth base for Cube socketing
```

Supported filter fields: `quality`, `sockets`, `ethereal`.

## Project Structure

```
src/
├── main.rs                 # Entry point — logging, icon setup, UI launch
├── settings.rs             # Settings deserialization from TOML
├── logger.rs               # Logging configuration
├── gui/                    # Rendering and UI
│   ├── ui.rs               # Main render loop (notan + egui)
│   ├── egui.rs             # Settings panel UI
│   ├── draw_map.rs         # Map tile rendering
│   ├── draw_units.rs       # Players, monsters, pets
│   ├── draw_objects.rs     # Chests, shrines, portals
│   ├── draw_lines.rs       # Lines to objectives
│   ├── draw_path.rs        # Pathfinding visualisation
│   ├── draw_presets.rs     # Preset map markers
│   ├── draw_buff_bar.rs    # Buff/debuff bar
│   ├── draw_item_log.rs    # Item drop log
│   ├── draw_item_tooltip.rs# Item hover tooltips
│   ├── draw_party_info.rs  # Party member info
│   ├── hotkeys.rs          # Hotkey parsing and detection
│   ├── play_sound.rs       # Audio alerts
│   └── images.rs           # Image/icon loading
├── memory/                 # D2R process memory reading
│   ├── process.rs          # Process attachment and memory reads
│   ├── gamedata.rs         # High-level game state assembly
│   ├── structs.rs          # Memory structure definitions
│   ├── decrypt.rs          # Value decryption
│   ├── instance_manager.rs # D2R window detection
│   └── types/              # Typed game data (items, NPCs, buffs, etc.)
├── mapgeneration/          # Map data generation and processing
│   ├── blacha.rs           # d2-mapgen integration
│   ├── cache.rs            # Seed data caching
│   ├── jsondata.rs         # Map JSON data structures
│   ├── mapgrid.rs          # Walkable grid processing
│   ├── mapimages.rs        # Map image generation
│   ├── pathfind.rs         # A* pathfinding
│   ├── pois.rs             # Points of interest
│   └── seeddata.rs         # Seed/difficulty request handling
└── localisation/           # Multi-language support
    ├── localisation.rs     # Translation lookup
    ├── localisation_file_parser.rs
    └── reference/          # JSON translation files
```

## Tech Stack

- **[Rust](https://www.rust-lang.org/)** — systems language for performance and safety
- **[notan](https://crates.io/crates/notan)** — cross-platform graphics/windowing (OpenGL)
- **[egui](https://crates.io/crates/egui)** — immediate-mode GUI for the settings panel
- **[winapi](https://crates.io/crates/winapi)** — Win32 API for overlay and memory access
- **[pathfinding](https://crates.io/crates/pathfinding)** — A* pathfinding algorithms
- **[rodio](https://crates.io/crates/rodio)** — audio playback for item alerts
- **[sapi-lite](https://crates.io/crates/sapi-lite)** — Windows Speech API for voice alerts
- **[serde](https://crates.io/crates/serde)** / **[toml](https://crates.io/crates/toml)** / **[serde_yaml](https://crates.io/crates/serde_yaml)** — configuration and data serialization

## Supported Languages

| Code | Language |
|---|---|
| `enUS` | English |
| `deDE` | German |
| `esES` | Spanish |
| `frFR` | French |
| `itIT` | Italian |
| `koKR` | Korean |
| `plPL` | Polish |
| `zhTW` | Traditional Chinese |
| `enBG` | Bulgarian |

## License

This project is private and not licensed for redistribution.