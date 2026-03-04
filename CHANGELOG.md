# Changelog — Tagentacle Core (Rust)

All notable changes to the **Tagentacle Core** (Rust daemon & CLI) will be documented in this file.
For Python SDK changes see [`python-sdk-core`](https://github.com/Tagentacle/python-sdk-core) and [`python-sdk-mcp`](https://github.com/Tagentacle/python-sdk-mcp) changelogs.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Docs: "Why Not Just Use an Agent Framework?"** — new section in `philosophy.md/.zh.md` comparing Tagentacle's OS-level approach against super-app frameworks (OpenClaw, Claude Code, ADK), with litmus test and historical parallels.
- **Docs: "The Plugin Trap"** — new section 5 in `why-tagentacle.md/.zh.md` analyzing in-process plugin systems (category confusion, patch accumulation, ADK shared-dict multi-agent) vs OS-level composition.
- **Fix: "Role of AI" row** — corrected worldview table across READMEs and docs: Tagentacle is the OS, AI agents are processes running on it (not "Host managing everything").
- **Fix: engineering comparison table** — removed inaccurate claim that OC/CC require forking to upgrade plugins; accurately describes shared-process-fate restart cost instead.

## [0.4.0] - 2026-03-03

### Added
- **Node Registration & Heartbeat**:
  - New `Register` action: nodes send `{op: register, node_id}` on connect; daemon responds with `register_ack`.
  - `NodeEntry` tracks `connected_at`, `registered`, `last_pong` per node.
  - `Pong` action: nodes reply to daemon heartbeat pings.
  - Daemon sends heartbeat ping every 30s; nodes unresponsive for 90s are automatically removed.
- **Node Disconnect Cleanup**:
  - On connection close, daemon removes all subscriptions, services, and node entries.
  - Publishes `{event: "disconnected"}` to `/tagentacle/node_events` on disconnect and timeout.
  - Publishes `{event: "connected"}` to `/tagentacle/node_events` on registration.
- **System Service Interception** (`/tagentacle/*`):
  - `/tagentacle/ping` — daemon health check (uptime, version, node/topic/service counts).
  - `/tagentacle/list_nodes` — list all connected nodes with metadata.
  - `/tagentacle/list_topics` — list all topics with subscriber details.
  - `/tagentacle/list_services` — list all registered services with providers.
  - `/tagentacle/get_node_info` — detailed info for a specific node.
  - These are daemon-intercepted (like `/proc`), not advertised by any node.
- **Ecosystem Packages**:
  - `container-orchestrator` v0.1.0 — LifecycleNode managing Docker containers (`/containers/create`, `stop`, `remove`, `list`, `inspect`, `exec`).
  - `shell-server` v0.1.0 — MCPServerNode exposing `exec_command`, `read_file`, `write_file`, `list_dir` MCP tools targeting containers.

### Changed
- **Router struct**: `nodes` HashMap now stores `NodeEntry` (with metadata) instead of bare `mpsc::Sender`.
- **Documentation**: Added system service asymmetry explanation (daemon interception vs normal advertise_service); updated roadmap in both EN and CN READMEs.

## [0.3.0] - 2026-03-15

### Removed
- **BREAKING**: Removed `bridge` subcommand (`tagentacle bridge`).
  - The stdio→HTTP MCP bridging functionality is superseded by [mcp-gateway](https://github.com/Tagentacle/mcp-gateway), a dedicated Python-based Gateway Node with transport-level relay.
  - Removed `run_bridge()` function (~110 lines) and `Bridge` CLI variant.
  - Cleaned unused `tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader}` imports.

## [0.2.0] - 2026-02-26

### Added
- **Workspace Repo Auto-Clone**: `tagentacle launch` now reads the `[workspace]` section in `tagentacle.toml` and automatically `git clone`s declared repos into the workspace before starting nodes. Enables one-command multi-repo workspace bootstrap from a single bringup config.
- **CLI Tools Expansion**:
  - `tagentacle topic echo <topic>` — subscribe and print messages from a topic.
  - `tagentacle service call <service> <payload>` — call a service and print the response.
  - `tagentacle doctor` — check daemon connectivity.
  - `tagentacle run --pkg <path>` — run a single package node, auto-sourcing per-pkg `.venv`.
  - `tagentacle launch <config.toml>` — multi-node topology launch from TOML config, per-node venv activation.
  - `tagentacle setup dep --pkg <path>` / `--all` — uv-based dependency installation for single pkg or entire workspace.
  - `tagentacle setup clean` — remove all `.venv` directories under the workspace.
- **Environment Management**:
  - Per-package `.venv` isolation via `uv sync`.
  - Workspace `install/` directory generation with symlinks to all discovered packages.
  - `setup_env.bash` auto-generation for `PATH` injection.
  - `find_all_packages()` recursive workspace scanner, `generate_install_structure()`.
- **`tagentacle.toml` Package Manifest** (specification):
  - Defined `[package]` (name, version, type, description, entry_point) and `[dependencies]` sections.

### Removed
- **`find_sdk_path()`**: Removed SDK auto-discovery helper. SDK packages are now managed as explicit workspace repos via the `[workspace]` section in `tagentacle.toml` and cloned automatically by `tagentacle launch`.

### Fixed
- **TOML Parser**: Replaced ad-hoc manual TOML parsing with `serde` + `toml` crate for robust, correct config file handling. Fixes panics and silent misparses in bringup configs with complex values.
- **Cargo.toml**: Added missing `clap` (with `derive` feature) and `uuid` (with `v4` feature) dependencies.
- **Cargo.toml / Cargo.lock**: Lowercase package name (`Tagentacle` → `tagentacle`) for Cargo convention compliance; added `[[bin]]` section (`name = "tagentacle"`, `path = "src/main.rs"`) so `cargo install --path .` correctly produces `~/.cargo/bin/tagentacle`; added `description` and `license` fields.
- **Quick start**: Commands now run from workspace root instead of subdirectories; documented `cargo install --path .` installation workflow; added `cargo uninstall tagentacle` uninstall step.
- **CLI**: Running `tagentacle` without a subcommand now prints full help text instead of blocking as a daemon; use `tagentacle daemon` to start the message bus.

### Changed
- **Workspace Restructure**: Moved example packages from `examples/` to `examples/src/` for cleaner separation.
- **Build System**: Switched from pip to uv as sole Python package manager for all packages.
- **Documentation Overhaul (NEW_ARCHITECTURE alignment)**:
  - Rewrote [README.md](README.md) (EN) and [README_CN.md](README_CN.md) (CN) to reflect the new architecture:
    - Core philosophy: "Everything is a Pkg" with 4 package types (Agent, Tool/Service, Interface, Bringup).
    - Node model: Agent Node vs General Node distinction, JSON Schema-validated Topics.
    - MCP Bus-as-Transport with dual-track integration (Service tunnel + Topic mirroring).
    - Bringup as configuration center with topology orchestration and parameter injection.
    - `tagentacle.toml` package manifest specification.
    - Full CLI toolchain: `run`, `launch`, `topic`, `service`, `bridge`, `setup dep/clean`, `doctor`.
  - Added comprehensive Roadmap & Status section with categorized task tracking.
- **Branding**: Renamed project tagline to "The ROS of AI Agents"; added ROS full name for context.
- **Documentation**: Added Claude Code vs Tagentacle architectural comparison; added Standard Topics/Services specification; added Agent IO+Inference architecture design.
- **Documentation**: Added standalone Installation section to README / README_CN with `cargo install --path .` / `cargo uninstall` instructions and PATH note.
- **Documentation (Roadmap)**: Added `describe_topic_schema` as a planned MCP Server tool (TODO); added Flattened Topic Tools API roadmap item (auto-generate expanded MCP tools from Topic JSON Schema definitions).

## [0.1.1] - 2026-02-22

### Added
- **Core Service Mechanism**:
  - Implemented `AdvertiseService`, `CallService`, and `ServiceResponse` in Rust Core.
  - Enhanced `Router` to support point-to-point service routing and node-ID tracking.

### Changed
- **Documentation**:
  - Updated [README.md](README.md) Roadmap.
  - Translated all Python SDK and example code comments to English.

## [0.1.0] - 2026-02-22
