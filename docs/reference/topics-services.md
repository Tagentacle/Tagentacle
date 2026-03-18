# Standard Topics & Services

When the Daemon starts, it automatically creates a set of **system-reserved Topics and Services** under the `/tagentacle/` namespace — analogous to ROS 2's `/rosout`, `/parameter_events`, and node introspection services. These provide built-in observability, logging, and introspection without requiring any user-side setup.

## Reserved Namespace Convention

| Prefix | Purpose | Managed By |
|---|---|---|
| `/tagentacle/*` | **System reserved.** Daemon & SDK core functionality | Core library |
| `/mcp/*` | MCP discovery and gateway services | MCPServerComponent / Gateway |

User-defined Topics should **not** use the above prefixes.

## Standard Topics (Daemon-managed)

| Topic | Analogy | Description | Published By |
|---|---|---|---|
| `/tagentacle/log` | ROS `/rosout` | Global log aggregation. All nodes auto-publish logs here via SDK; Daemon also publishes system events. | SDK Nodes (auto) + Daemon |
| `/tagentacle/node_events` | ROS lifecycle events | Node lifecycle events: connected, disconnected, lifecycle state transitions. Powers real-time topology in dashboards. | Daemon (auto) + `LifecycleNode` (auto) |
| `/tagentacle/diagnostics` | ROS `/diagnostics` | Node health reports: heartbeat, uptime, message counters, error counts. | SDK `Node.spin()` (periodic) |
| `/mcp/directory` | _(none)_ | MCP server discovery. `MCPServerDescription` messages published by MCP Server Nodes and Gateway on activation. Agents subscribe to auto-discover servers. | MCPServerComponent / Gateway |

## Standard Services (Daemon-intercepted)

!!! info "Architectural Note — Intentional Asymmetry"
    These `/tagentacle/*` Services are **not** published by a real Node via `advertise_service`. Instead, the Daemon **intercepts** `call_service` requests whose name starts with `/tagentacle/` and generates responses directly from its Router's internal state — much like Linux's `/proc` filesystem, which is synthesized by the kernel rather than backed by a real disk.

    This means `/tagentacle/list_services` will **not** list itself or any other `/tagentacle/*` service — they exist outside the normal service registry. This is by design: the Daemon provides *read-only introspection as a mechanism*, without participating as a regular Node in the bus topology it manages.

    From a caller's perspective, the API is identical to any other service call — `call_service("/tagentacle/ping", {})` works the same way. The asymmetry is invisible to consumers but fundamental to the architecture.

| Service | Analogy | Description |
|---|---|---|
| `/tagentacle/ping` | `ros2 doctor` | Daemon health check. Returns `{status, uptime_s, version, node_count, topic_count}` |
| `/tagentacle/list_nodes` | `ros2 node list` | Returns all connected nodes: `{nodes: [{node_id, connected_at}]}` |
| `/tagentacle/list_topics` | `ros2 topic list` | Returns all active Topics and their subscribers: `{topics: [{name, subscribers}]}` |
| `/tagentacle/list_services` | `ros2 service list` | Returns all registered Services: `{services: [{name, provider}]}` |
| `/tagentacle/get_node_info` | `ros2 node info` | Returns details for a specific node: `{node_id, subscriptions, services, connected_at}` |

These Services can be tested directly from the CLI:

```bash
tagentacle service call /tagentacle/ping '{}'
tagentacle service call /tagentacle/list_nodes '{}'
tagentacle topic echo /tagentacle/log
tagentacle topic echo /tagentacle/node_events
```

## Log Message Schema (`/tagentacle/log`)

```json
{
  "level": "info",
  "timestamp": "2026-02-24T12:00:00.000Z",
  "node_id": "alice_agent",
  "message": "Connected to OpenAI API successfully",
  "file": "main.py",
  "line": 42,
  "function": "on_configure"
}
```

## Node Event Schema (`/tagentacle/node_events`)

```json
{
  "event": "connected",
  "node_id": "alice_agent",
  "timestamp": "2026-02-24T12:00:00.000Z",
  "state": "active",
  "prev_state": "inactive"
}
```
