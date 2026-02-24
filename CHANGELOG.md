# Changelog

All notable changes to the **Tagentacle** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Python SDK Dual-Layer API** (`tagentacle-py`):
  - `LifecycleNode(Node)` with state machine (UNCONFIGURED → INACTIVE → ACTIVE → FINALIZED).
  - Lifecycle hooks: `on_configure`, `on_activate`, `on_deactivate`, `on_shutdown`.
  - `bringup()` convenience method for one-call startup.
  - Improved `Node`: `disconnect()`, `_connected` flag, `call_service()` timeout, error responses on handler failure.
- **MCP Bus-as-Transport** (`tagentacle-py/tagentacle_py/mcp/`):
  - `tagentacle_client_transport(node, server_node_id)` — async context manager bridging MCP ClientSession over the bus.
  - `tagentacle_server_transport(node, server_node_id)` — async context manager exposing MCP Server as a bus service.
  - Automatic traffic mirroring to `/mcp/traffic` topic.
  - Backward-compatible class aliases `TagentacleClientTransport` / `TagentacleServerTransport`.
- **MCP-Publish Bridge Node** (`tagentacle_py/mcp/publish_bridge.py`):
  - Pre-built MCP Server exposing `publish_to_topic` and `list_available_topics` as MCP Tools.
  - Topic allow-list support. Standalone `main()` entrypoint.
- **`tagentacle.toml` Package Manifest** (specification + examples):
  - Defined `[package]` (name, version, type, description, entry_point) and `[dependencies]` sections.
  - Created example manifests for `agent_pkg`, `mcp_server_pkg`, `bringup_pkg`.
- **Bringup Configuration Center** (`examples/src/bringup_pkg/launch/`):
  - `system_launch.toml` — TOML-based topology definition with `depends_on`, `startup_delay`, parameters.
  - `system_launch.py` — config-driven launcher using `tomllib`, topological ordering, env var injection.
- **CLI Tools Expansion** (Rust Daemon):
  - `tagentacle topic echo <topic>` — subscribe and print messages from a topic.
  - `tagentacle service call <service> <payload>` — call a service and print the response.
  - `tagentacle doctor` — check daemon connectivity.
  - `tagentacle run --pkg <path>` — run a single package node, auto-sourcing per-pkg `.venv`.
  - `tagentacle launch <config.toml>` — multi-node topology launch from TOML config, per-node venv activation.
  - `tagentacle setup dep --pkg <path>` / `--all` — uv-based dependency installation for single pkg or entire workspace.
  - `tagentacle setup clean` — remove all `.venv` directories under the workspace.
- **Environment Management** (Rust Daemon):
  - Per-package `.venv` isolation via `uv sync`.
  - Workspace `install/` directory generation with symlinks to all discovered packages.
  - `setup_env.bash` auto-generation for `PATH` injection.
  - `find_all_packages()` recursive workspace scanner, `generate_install_structure()`.
- **Secrets Management** (`tagentacle-py`):
  - `secrets.toml` auto-loading via `TAGENTACLE_SECRETS_FILE` environment variable.
  - `_load_secrets_file()` and `_parse_toml_fallback()` in Node.
  - Bringup launcher injects `TAGENTACLE_SECRETS_FILE` into child node environments.
- **SDK Bringup Utilities** (`tagentacle-py`):
  - `load_pkg_toml(pkg_dir)` — parse and return `tagentacle.toml` as dict.
  - `discover_packages(workspace_root)` — recursively find all packages with `tagentacle.toml`.
  - `find_workspace_root(start_path)` — locate workspace root by traversing parent directories.
- **Examples**:
  - `mcp_server_pkg/server.py` — MCP weather server over bus transport.
  - Updated `agent_pkg/client.py` — MCP client using `tagentacle_client_transport`.
  - `mcp_seamless_demo.py` — end-to-end MCP-over-bus pipeline demo.
  - All example packages converted to independent uv projects (`pyproject.toml` + `uv.lock`).

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
    - Python SDK dual-layer design: Simple API + LifecycleNode API.
    - MCP Bus-as-Transport with dual-track integration (Service tunnel + Topic mirroring).
    - MCP-Publish Bridge Node concept.
    - Bringup as configuration center with topology orchestration and parameter injection.
    - `tagentacle.toml` package manifest specification.
    - Full CLI toolchain: `run`, `launch`, `topic`, `service`, `bridge`, `setup dep/clean`, `doctor`.
  - Added comprehensive Roadmap & Status section with categorized task tracking.
  - Updated bilingual SDK READMEs with uv environment workflow and workspace documentation.
- **Branding**: Renamed project tagline to "ROS for AI Agent"; added ROS full name for context.
- **Documentation**: Added Claude Code vs Tagentacle architectural comparison; added Standard Topics/Services specification; added Agent IO+Inference architecture design.
- **Documentation**: Added standalone Installation section to README / README_CN with `cargo install --path .` / `cargo uninstall` instructions and PATH note.
- **Documentation (Roadmap)**: Added `describe_topic_schema` as a planned MCP Server tool (TODO); added Flattened Topic Tools API roadmap item (auto-generate expanded MCP tools from Topic JSON Schema definitions).

## [0.1.1] - 2026-02-22

### Added
- **Core Service Mechanism**:
  - Implemented `AdvertiseService`, `CallService`, and `ServiceResponse` in Rust Core.
  - Enhanced `Router` to support point-to-point service routing and node-ID tracking.
- **Python SDK Enhancements**:
  - Added `@node.service` decorator for declaring service handlers (supporting both sync and async).
  - Implemented `node.call_service()` for asynchronous RPC-style calls using `asyncio.Future`.
  - Updated `Node.spin()` to handle service request dispatching and response routing.
- **Examples**:
  - Added `service_server.py` and `service_client.py` for service mechanism demonstration.

### Changed
- **Documentation**:
  - Updated [README.md](README.md) Roadmap.
  - Translated all Python SDK and example code comments to English.

## [0.1.0] - 2026-02-22
