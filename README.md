# biggit

A desktop tool for managing large Git repositories using **bare clones** and **worktrees**.

Instead of cloning full repositories multiple times, biggit clones once as a bare repo with partial clone filtering, then creates lightweight worktrees for each branch or task you work on. 
This saves disk space and clone time — especially useful for monorepos and large codebases.

## Features

- **Bare clone with partial-clone filtering** — `git clone --bare --filter=blob:none` fetches only the metadata you need upfront
- **Worktree management** — create, list, and remove worktrees from a single bare repo
- **Sync** — fetch all remotes and prune stale branches in one command
- **GUI** (iced) — visual dashboard with repository setup, worktree management, and native file dialogs
- **CLI** — scriptable command-line interface for all operations
- **Cross-platform** — Linux, macOS, and Windows

## Prerequisites

- [Git](https://git-scm.com/) must be installed and available on your `PATH`
- [Rust](https://www.rust-lang.org/tools/install) 1.85+ (edition 2024) to build from source

## Building

```sh
git clone https://github.com/mwallner/biggit.git
cd biggit
cargo build --workspace --release
```

The release binaries are placed in `target/release/`:
- `biggit-cli` — command-line interface
- `biggit` — graphical interface

### System dependencies (Linux)

The GUI uses GTK for native file dialogs. On Debian/Ubuntu:

```sh
sudo apt-get install libgtk-3-dev libxdo-dev libayatana-appindicator3-dev
```

## Usage

### CLI

```sh
# Clone a repo as a bare repo with partial clone
biggit-cli clone https://github.com/mwallner/repo.git ./repo.git

# Fetch all remotes
biggit-cli sync repo

# Create a worktree for a branch
biggit-cli worktree add repo ./worktrees/feature-x feature-x

# Create a worktree on a new branch
biggit-cli worktree add repo ./worktrees/my-fix main -b my-fix

# List worktrees
biggit-cli worktree list repo

# Remove a worktree
biggit-cli worktree remove repo ./worktrees/feature-x
```

### GUI

```sh
biggit
```

The GUI provides:
- **Setup screen** — enter a repository URL, pick a destination directory, and clone
- **Dashboard** — view repo info, sync, open the repo folder
- **Worktree management** — browse existing worktrees, create new ones with branch selection, remove worktrees

## Project Structure

```
crates/
  core/     — git operations (clone, sync, worktree, branch) via std::process::Command
  config/   — platform-aware config path resolution and JSON persistence
  cli/      — clap-based command-line interface
  gui/      — iced-based graphical interface with async git operations
```

## Running Tests

```sh
cargo test --workspace
```

## CI / Releases

GitHub Actions runs tests on Linux, macOS, and Windows on every push and pull request to `main`. Release builds produce artifacts for all three platforms — see the [Actions tab](https://github.com/mwallner/biggit/actions/) for build artifacts or [Releases](https://github.com/mwallner/biggit/releases) for published builds.

## License

This project is provided as-is. See the repository for license details.
