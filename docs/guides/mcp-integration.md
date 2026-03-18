# MCP Integration: Local Sessions + Direct HTTP

Inspired by ROS 2's TF2 pattern, Tagentacle localizes MCP session management within Agent Nodes.

## Design Principles

*   **Session Localization**: MCP Client Sessions stay in Agent node memory. Agents connect directly to MCP Servers via native MCP SDK Streamable HTTP client.
*   **MCPServerComponent**: MCP Servers use `LifecycleNode` + `MCPServerComponent` (composition), which auto-runs Streamable HTTP and publishes `MCPServerDescription` to `/mcp/directory`.
*   **Unified Discovery**: Agents subscribe to `/mcp/directory` to auto-discover all available MCP servers (native HTTP + Gateway-proxied stdio servers).
*   **Full Protocol Support**: Direct Agent↔Server sessions preserve all MCP capabilities including sampling, notifications, and resources.
*   **MCP Gateway**: A separate `mcp-gateway` package provides transport-level stdio→HTTP relay for legacy MCP servers, without parsing MCP semantics.

## Agent-Side Connection Example

```python
from mcp import ClientSession
from mcp.client.streamable_http import streamable_http_client

# Connect directly to MCP Server via Streamable HTTP
async with streamable_http_client("http://127.0.0.1:8100/mcp") as (r, w, _):
    async with ClientSession(r, w) as session:
        await session.initialize()
        result = await session.call_tool("query", {"sql": "SELECT * FROM users"})
```

## Bidirectional & Observability

- **Bidirectional**: Because MCP sessions are established directly between Agent ↔ Server (HTTP long-lived connections), full MCP spec bidirectional capabilities are natively supported, including **Sampling** (Server calling back to Agent).
- **Observability**: MCP Server discovery info is published to the `/mcp/directory` Topic. Any node can subscribe to get a real-time view of all available tools in the system.

## Control Plane vs. Data Plane

Understanding the separation between bus communication and MCP tool calls is fundamental:

| | Control Plane (Bus) | Data Plane (MCP HTTP) |
|---|---|---|
| **What** | Discovery, lifecycle, events | Tool calls, resources, sampling |
| **Transport** | TCP JSON Lines via Daemon | Streamable HTTP direct connection |
| **Path** | Node → Daemon → Node | Agent MCP Client → MCP Server |
| **Examples** | `/mcp/directory`, `/tagentacle/node_events` | `call_tool("exec_command", ...)` |

The bus **never** sees MCP tool call content. It only publishes server URLs for discovery. All tool calls go directly from Agent to MCP Server via HTTP.
