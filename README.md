# Micat

Micat is a dual Minecraft server management suite built on top of the Ferium/libium codebase.

## Layout
- `core/` – Rust-based core application (forked and adapted from Ferium) that runs directly on the host. It is responsible for modpack management, automation, and orchestration of the Dockerized Minecraft servers.
- `planner/` – Design and planning documents for Micat features and architecture.

## High-Level Goals
- Manage two servers (permanent + seasonal) using Docker Compose and the `itzg/minecraft-server` image.
- Run Micat on the host machine to control the two Minecraft containers (e.g. by writing mods/modpacks into their bound `/data` volumes and orchestrating restarts).
- Provide both a CLI and a locally hosted GUI/web interface.
- Automate modpack fetching/updates (via Ferium/libium integration), backups, and safe shutdown flows.

See `planner/plan.md` for the current feature and implementation plan.

## Acknowledgements & License
- Micat’s core functionality is built on top of the `libium` and original Ferium codebase, which are licensed under the Mozilla Public License 2.0.
- The upstream Ferium README and licensing details are preserved in `core/README.md` and `core/LICENSE.txt`.
