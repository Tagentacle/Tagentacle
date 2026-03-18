# TACL: Tagentacle Access Control Layer

**TACL** (Tagentacle Access Control Layer) provides MCP-level JWT authentication and authorization. It is implemented entirely in the Python SDK (`python-sdk-mcp`) — the Daemon knows nothing about access control, staying true to the "mechanisms only" principle.

## Architecture

TACL is built around three roles:

| Role | Component | Responsibility |
|---|---|---|
| **Issuer** | `TACLAuthority` | SQLite-backed agent registry. Issues JWT credentials with tool-level grants. |
| **Verifier** | `MCPServerComponent` (with `auth_required=True`) | Validates Bearer JWT on every request. Sets `CallerIdentity` contextvar. |
| **Carrier** | `AuthMCPClient` | Authenticates with the permission server, obtains JWT, attaches it to all MCP requests. |

## JWT Payload Schema

```json
{
    "agent_id": "agent_alpha",
    "tool_grants": {
        "shell_server": ["exec_command"],
        "tao_wallet_server": ["query_balance", "transfer"]
    },
    "space": "agent_space_1",
    "iat": 1740000000,
    "exp": 1740086400
}
```

| Field | Type | Description |
|---|---|---|
| `agent_id` | `string` | Unique agent identifier. |
| `tool_grants` | `{server_id: [tool_names]}` | Per-server tool whitelist. Only listed tools are callable. |
| `space` | `string?` | **Execution environment binding** — identifies the isolated space (e.g., Docker container) assigned to this agent. |
| `iat` / `exp` | `int` | Issued-at / expiration timestamps (UNIX epoch). Default TTL: 24h. |

## The `space` Claim: Binding Agents to Containers

The `space` field is the key that connects TACL authentication with container isolation. When an admin registers an agent, they assign a `space`:

```python
# Admin registers agent with a bound container
await permission_node.register_agent(
    agent_id="agent_alpha",
    raw_token="secret_token",
    tool_grants={"shell_server": ["exec_command"]},
    space="agent_space_1"   # ← bound to this container
)
```

When the agent authenticates and calls an MCP tool (e.g., `exec_command` on the shell-server), the server reads `CallerIdentity.space` from the JWT and routes the command to the bound container — **no global config, no static mapping**. Each agent's JWT carries its own container binding.

## Authentication Flow

```
Admin                   TACLAuthority                     MCPServerComponent (auth_required)
  │                              │                                │
  ├─ register_agent ────────────▶│                                │
  │  (agent_id, token,           │                                │
  │   tool_grants, space)        │                                │
  │                              │                                │
  │          Agent (AuthMCPClient)                                │
  │              │               │                                │
  │              ├─ authenticate ▶│                                │
  │              │  (raw_token)  │                                │
  │              │◀── JWT ───────┘                                │
  │              │                                                │
  │              ├─────── call_tool (Bearer: JWT) ───────────────▶│
  │              │                                  verify JWT     │
  │              │                                  check grants   │
  │              │                                  set CallerIdentity
  │              │◀──────────── tool result ─────────────────────┘
```

## Design Principles

- **Zero external dependencies**: JWT signing/verification uses pure Python stdlib (HS256 via `hmac` + `hashlib`).
- **Shared secret**: Both issuer and verifiers read `TAGENTACLE_AUTH_SECRET` from environment.
- **Contextvar-based**: `CallerIdentity` is set per-request via Python `contextvars`, enabling tool handlers to read caller info without parameter threading.
- **Granularity**: Authorization supports **tool-level** per server. However, prefer **server-level** control with small, focused servers (see [Best Practices](best-practices.md)).
- **Optional**: Auth is opt-in. `MCPServerComponent(auth_required=False)` (default) accepts all callers.
