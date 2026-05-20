# biggit Logo & Icon Assets

This directory contains the official logo and icon assets for the biggit project.

## Structure
- `biggit.svg` — Main app symbol (bare repo hub + worktree branches, squircle, Catppuccin Macchiato)
- `biggit-micro.svg` — Hand-tuned 16/24px variant
- `gows.svg` — gows app variant (adds graph topology triad badge)
- `gows-micro.svg` — gows micro variant
- `generate.sh` — Asset generation script (SVG → PNG, ICO, ICNS)


## Generation
Run `cargo run --package logo` (or build the workspace) to generate all PNG and ICO outputs from SVG using Rust (resvg, image, ico crates). No external graphics tools required.

ICNS is not generated automatically; use a manual tool if needed for macOS packaging.

The old `generate.sh` is deprecated and only for reference.

## Integration
- GUI: Set window icon to `biggit.png` (or platform-specific)
- gows: Use `gows.png`

