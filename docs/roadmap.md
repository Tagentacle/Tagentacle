# Roadmap & Status

## Completed

- [x] **Rust Daemon**: Topic Pub/Sub and Service Req/Res message routing.
- [x] **Python SDK (Simple API)**: `Node` class with `connect`, `publish`, `subscribe`, `service`, `call_service`, `spin`.
- [x] **Python SDK Dual-Layer API**: `LifecycleNode` with `on_configure`/`on_activate`/`on_deactivate`/`on_shutdown`.
- [x] ~~**MCP Bridge (Rust)**~~: Removed in v0.3.0 — superseded by `mcp-gateway` (Python Gateway Node with transport-level relay).
- [x] ~~**MCP Transport Layer**~~: Removed in python-sdk-mcp v0.2.0 — replaced by direct Streamable HTTP connections.
- [x] **MCPServerNode Base Class**: `python-sdk-mcp` v0.2.0 — base class for MCP Server Nodes with auto Streamable HTTP + `/mcp/directory` publishing.
- [x] **MCP Gateway**: `mcp-gateway` v0.1.0 — transport-level stdio→HTTP relay + directory service.
- [x] **Tagentacle MCP Server**: Built-in MCP Server exposing bus interaction tools (FastMCP-based, auto-schema from type hints).
- [x] **`tagentacle.toml` Spec**: Define and parse package manifest format.
- [x] **Bringup Configuration Center**: Config-driven topology orchestration with parameter injection.
- [x] **CLI Toolchain**: `daemon`, `run`, `launch`, `topic echo`, `service call`, `doctor`, `setup dep`, `setup clean`.
- [x] **Environment Management**: uv-based per-package `.venv` isolation, workspace `install/` structure with symlinks.
- [x] **Secrets Management**: `secrets.toml` auto-loading, bringup environment variable injection.
- [x] **SDK Utilities**: `load_pkg_toml`, `discover_packages`, `find_workspace_root`.
- [x] **Workspace Repo Auto-Clone**: `[workspace.repos]` in bringup `tagentacle.toml` — `setup dep --all` auto-clones missing git repos.
- [x] **Example Packages**: `example-agent`, `example-mcp-server`, `example-bringup` as independent repos.
- [x] **TACL (Tagentacle Access Control Layer)**: `python-sdk-mcp` v0.3.0 — MCP-level JWT authentication with `auth_required` on MCPServerNode, `AuthMCPClient`, `PermissionMCPServerNode` (SQLite agent registry + credential issuer).
- [x] **Standard System Services**: Daemon-intercepted `/tagentacle/ping`, `/tagentacle/list_nodes`, `/tagentacle/list_topics`, `/tagentacle/list_services`, `/tagentacle/get_node_info`.
- [x] **Node Registration & Heartbeat**: `Register` handshake, periodic ping/pong, automatic stale-node cleanup (90s timeout).
- [x] **Node Disconnect Cleanup**: Automatic removal of subscriptions, services, and node entries on disconnect, with `/tagentacle/node_events` publishing.
- [x] **JSON Schema Validation**: `python-sdk-core` v0.3.0 — `SchemaRegistry` with auto-discovery from interface packages, configurable per-node (`strict`/`warn`/`off`), integrated into `Node.publish()` and `Node._dispatch()`. Requires optional `jsonschema>=4.0`.
- [x] **TACL `space` Claim**: `python-sdk-mcp` v0.4.0 — JWT `space` field binding agents to isolated execution environments. Full stack: `CallerIdentity.space`, `sign_credential(space=...)`, `PermissionMCPServerNode.register_agent(space=...)`.
- [x] **Container Orchestration Pkg**: `container-orchestrator` v0.1.0 — LifecycleNode managing Docker containers via bus services (`/containers/create`, `stop`, `list`, `exec`, etc.).
- [x] **Shell Server Pkg**: `shell-server` v0.1.0 — MCPServerNode exposing `exec_command` tool with TACL `space`-aware dynamic container routing (JWT → container → local fallback).

## Planned

- [ ] **Standard Topics (SDK-side)**: SDK auto-publish to `/tagentacle/log`, `/tagentacle/diagnostics`.
- [ ] **SDK Log Integration**: Auto-publish node logs to `/tagentacle/log` via `get_logger()`.
- [ ] **Flattened Topic Tools API**: SDK API to auto-generate flattened MCP tools from Topic JSON Schema definitions (e.g., a registered `/chat/input` schema auto-generates a `publish_chat_input(text, sender)` tool with expanded parameters).
- [ ] **Interface Package**: Cross-node JSON Schema contract definition packages.
- [ ] **Action Mode**: Long-running async tasks with progress feedback.
- [ ] **Parameter Server**: Global parameter store with `/tagentacle/parameter_events` notifications.
- [ ] **Web Dashboard**: Real-time topology, message flow, and node status visualizer.
