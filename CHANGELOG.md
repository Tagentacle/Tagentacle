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
- **Bringup Configuration Center** (`examples/bringup_pkg/launch/`):
  - `system_launch.toml` — TOML-based topology definition with `depends_on`, `startup_delay`, parameters.
  - `system_launch.py` — config-driven launcher using `tomllib`, topological ordering, env var injection.
- **CLI Tools Expansion** (Rust Daemon):
  - `tagentacle topic echo <topic>` — subscribe and print messages from a topic.
  - `tagentacle service call <service> <payload>` — call a service and print the response.
  - `tagentacle doctor` — check daemon connectivity.
- **Examples**:
  - `mcp_server_pkg/server.py` — MCP weather server over bus transport.
  - Updated `agent_pkg/client.py` — MCP client using `tagentacle_client_transport`.
  - `mcp_seamless_demo.py` — end-to-end MCP-over-bus pipeline demo.

### Fixed
- **Cargo.toml**: Added missing `clap` (with `derive` feature) and `uuid` (with `v4` feature) dependencies.

### Changed
- **Documentation Overhaul (NEW_ARCHITECTURE alignment)**:
  - Rewrote [README.md](README.md) (EN) and [README_CN.md](README_CN.md) (CN) to reflect the new architecture:
    - Core philosophy: "Everything is a Pkg" with 4 package types (Agent, Tool/Service, Interface, Bringup).
    - Node model: Agent Node vs General Node distinction, JSON Schema-validated Topics.
    - Python SDK dual-layer design: Simple API + LifecycleNode API.
    - MCP Bus-as-Transport with dual-track integration (Service tunnel + Topic mirroring).
    - MCP-Publish Bridge Node concept.
    - Bringup as configuration center with topology orchestration and parameter injection.
    - `tagentacle.toml` package manifest specification.
    - Full CLI toolchain roadmap: `run`, `launch`, `topic`, `service`, `bridge`, `setup/dep`, `doctor`.
  - Added comprehensive Roadmap & Status section with categorized task tracking.

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
