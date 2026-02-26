# Changelog — Tagentacle Core (Rust)

All notable changes to the **Tagentacle Core** (Rust daemon & CLI) will be documented in this file.
For Python SDK changes see [`tagentacle-py/CHANGELOG.md`](../tagentacle-py/CHANGELOG.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **`find_sdk_path()`**: Updated to search for `python-sdk-core/` and `python-sdk-mcp/` directories (new 1-repo-1-pkg layout), joining both into `PYTHONPATH`. Removed legacy `tagentacle-py/` fallback.

### Added
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

### Fixed
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
