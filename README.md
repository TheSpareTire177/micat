# MICAT – Minecraft Instance Control and Automation Toolkit

MICAT (Minecraft Instance Control and Automation Toolkit) is a dual Minecraft server management suite built on top of the Ferium/libium codebase.

It is designed to manage **two** Minecraft servers on a single host:
- one long‑lived, **permanent** world
- one **seasonal / temporary** world for events or limited‑time runs

MICAT focuses on safe automation, predictable updates, and tight integration with containerised servers.

## Layout
- `core/` – Rust-based core application (forked and adapted from Ferium) that runs directly on the host. It is responsible for modpack management, automation, and orchestration of Dockerised Minecraft servers.
- `planner/` – Local design and planning documents for MICAT features and architecture (kept out of the public repo).

## High-Level Goals
- Manage two servers (permanent + seasonal) using Docker Compose and the `itzg/minecraft-server` image.
- Run MICAT on the host machine to control the two Minecraft containers (e.g. by writing mods/modpacks into their bound `/data` volumes and orchestrating restarts).
- Provide both a CLI and (eventually) a locally hosted GUI/web interface.
- Automate modpack fetching/updates (via Ferium/libium integration), backups, and safe shutdown flows.

## Features (current & planned)

- **Dual server orchestration** – Treat a permanent and a seasonal server as first‑class citizens.
- **Profile‑driven modpack management** – Use libium’s profile system to target the two servers’ `/data` directories.
- **Forge / NeoForge support** – Aim to integrate cleanly with popular modded server stacks via `itzg/minecraft-server`.
- **Safe maintenance windows** (planned) – Broadcast shutdown warnings in‑game / online before host reboots or updates.
- **Backups & recovery** (planned) – Snapshot worlds regularly and provide simple restore paths.
- **Web management surface** (planned) – A small web UI (backed by the WEBSURFACE project) for monitoring and control.

## Requirements

- Linux host (tested on a Debian/Ubuntu‑style environment).
- [Docker](https://docs.docker.com/get-docker/) and Docker Compose for running the Minecraft servers.
- [Rust toolchain](https://www.rust-lang.org/tools/install) (via `rustup`) for building MICAT.

Optional but recommended:
- `make` for using the provided `Makefile` shortcuts.

## Quickstart

From the MICAT project root:

```bash
git clone https://github.com/TheSpareTire177/micat.git
cd micat

# Build the MICAT binary
make build

# Initialise MICAT config pointing at your two server data directories
make init
```

By default, `make init` configures two profiles in `micat-config.json`:
- **Permanent Server** → `core/../data/mc-permanent`
- **Seasonal Server** → `core/../data/mc-seasonal`

You should bind these directories into your `itzg/minecraft-server` containers as `/data` so that MICAT can control mods, configs, and worlds.

## CLI Overview

The main entrypoint is the `micat` binary (built from `core/src/bin/micat.rs`).

Basic usage:

```bash
cd core
. "$HOME/.cargo/env"

cargo run --bin micat -- --help

# Initialise config with defaults
cargo run --bin micat -- init

# Customise game version and loader
cargo run --bin micat -- init \
	--game-version 1.20.1 \
	--loader forge
```

Over time, additional subcommands will be added for:
- managing modpacks for each server profile
- triggering safe restart/shutdown sequences
- exposing status to the web UI

## Acknowledgements & License
- Micat’s core functionality is built on top of the `libium` and original Ferium codebase, which are licensed under the Mozilla Public License 2.0.
- The upstream Ferium README and licensing details are preserved in `core/README.md` and `core/LICENSE.txt`.
