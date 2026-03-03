# Best Practices

## TACL: Prefer Server-Level Access Control

TACL supports both **server-level** and **tool-level** authorization via `tool_grants`. However, we recommend using TACL primarily for **server-level control** — i.e., granting or denying an agent access to an entire MCP Server.

If you find yourself needing tool-level ACL within a single server, it's a signal that the server is doing too much. Following the Unix philosophy, split it into smaller, focused MCP Server Pkgs — each doing one thing well. When each server is single-purpose, server-level ACL naturally provides the right granularity.

| Approach | Recommendation | Example |
|----------|---------------|--------|
| Server-level | ✅ Recommended | Agent A can access `shell-server` but not `wallet-server` |
| Tool-level | ⚠️ Possible but not preferred | Agent A can call `exec_command` but not `list_files` on the same server |

!!! tip "Why?"
    Smaller servers are easier to reason about, deploy independently, and secure at the perimeter. Tool-level ACL adds complexity inside the server without improving the overall security boundary.

## MCP Server Transport: Streamable HTTP Only

All MCP Server Node Pkgs in Tagentacle use **Streamable HTTP** as their transport. This is required by the `MCPServerNode` base class and enables:

- Direct Agent↔Server sessions with full MCP protocol support (sampling, notifications, resources)
- TACL JWT authentication via standard HTTP `Authorization` headers
- Standard health checks, load balancing, and container networking

!!! warning "stdio MCP Servers are not recommended"
    The `mcp-gateway` package provides stdio→HTTP relay as a **legacy compatibility layer** for third-party MCP servers that only support stdio transport. This is analogous to bridging a Linux pipe inside a ROS 2 node — it works, but breaks the standard communication model:

    - stdio servers cannot participate in TACL authentication (no HTTP headers)
    - stdio sessions are managed by the Gateway process, not the Agent
    - One subprocess per HTTP session limits scalability

    If you control the MCP Server code, always implement it as a Streamable HTTP `MCPServerNode` Pkg.

## Server Design: Unix Philosophy

Follow the Unix philosophy when designing MCP Server Pkgs:

| Principle | Application |
|-----------|-------------|
| **Do one thing well** | Each MCP Server Pkg exposes a focused set of related tools |
| **Compose freely** | Agents discover and connect to multiple small servers |
| **Fail independently** | A crash in one server doesn't affect others |
| **Deploy independently** | Each server is its own container with its own dependencies |

### Example: Splitting a Monolithic Server

Instead of one server with many tools:

```
❌  mega-server
    ├── exec_command
    ├── read_file
    ├── write_file
    ├── query_balance
    ├── transfer_funds
    └── send_email
```

Split into focused servers:

```
✅  shell-server       → exec_command
    file-server        → read_file, write_file
    wallet-server      → query_balance, transfer_funds
    email-server       → send_email
```

Now TACL server-level ACL gives you exactly the right granularity: Agent A gets `shell-server` + `file-server`, Agent B gets `wallet-server` only.
