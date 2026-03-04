# Tagentacle: The ROS of AI Agents 🐙

[![GitHub](https://img.shields.io/badge/GitHub-Tagentacle-blue)](https://github.com/Tagentacle/tagentacle)

**Tagentacle** is a decentralized, configuration-centralized, production-grade multi-agent framework. It deeply adopts the software organization patterns of **ROS 2** (Robot Operating System), combined with the modern AI ecosystem (MCP Protocol) and dynamic Schema technologies, to build a robust infrastructure for LLM multi-agent collaboration.

> **Everything is a Pkg. Managed. Verifiable. Scalable.**

---

## 🌟 Core Philosophy: Everything is a Pkg

Tagentacle inherits the most fundamental software organization philosophy from ROS 2: **thorough modularization of system capabilities**. This philosophy exhibits excellent engineering properties when abstracting agent components:

- **Agent Package**: Each agent as an independent package, containing its behavior logic, prompt templates, state management, and communication interfaces.
- **Tool/Service Package**: Encapsulates tools or services that agents need to call (e.g., database access, web scraping), supporting MCP protocol for plug-and-play integration.
- **Interface Package**: Dedicated to defining cross-node communication contracts (JSON Schema), ensuring packages written by different developers "speak the same language".
- **Bringup Package**: Responsible for system startup and configuration, defining which packages the workspace should include, node launch parameters, and environment credentials.

Key advantages:
*   **High Reusability**: A mature tool package or agent package can be seamlessly migrated across projects like LEGO bricks.
*   **Version & Dependency Isolation**: Drawing from ROS 2's isolation mechanism, Tagentacle automatically manages independent Python virtual environments per package, eliminating dependency conflicts.
*   **Black-Box Development**: Developers only need to focus on input/output contracts—no need to know whether the internals use GPT, Claude, or a custom model.

---

## ⚔️ Why Tagentacle? (The Pitch)

In a world dominated by monolithic gateways and CLI-bound tools, Tagentacle provides "Industrial Grade" infrastructure for the next generation of AI.

| Feature | Monolithic Gateways (e.g., OpenClaw) | CLI-Based Tools (e.g., Claude Code) | **Tagentacle** |
| :--- | :--- | :--- | :--- |
| **Architecture** | Monolithic Gateway (Node.js) | Single-Process CLI | **Distributed Microservices (Rust)** |
| **Topology** | Star (single hub) | Tree-shaped call stack (main→sub) | **Mesh (Pub/Sub)** |
| **Stability** | Single Failure = Full Crash | Process-bound | **Isolated Processes (Fault Tolerant)** |
| **Lifecycle** | Chat-bound (TG/WA) | Task-bound (One-shot) | **Continuous (24/7 Event-Driven)** |
| **Interaction** | Chat Bubble (ChatOps) | Terminal Output | **Mission Control (Real-time Dashboards)** |
| **Component Role** | Skills (bound to host) | Plugin / Sub-Agent (subordinate) | **Independent microservices (Peers)** |
| **Scope** | Single Server | Local Filesystem | **Multi-Device / Cross-Platform** |

### 1. Robustness: Distributed Bus vs. Monolithic Gateway
*   **OpenClaw's Achilles' Heel**: Running 50 skills in one Node.js process means a memory leak in one skill reboots your entire system.
*   **Tagentacle's Absolute Isolation**: Every Node is a separate process. If your "Twitter Scraper" crashes, your "SRE Agent" continues fixing the server. The Rust-based Broker provides high-concurrency message routing that never sleeps.

### 2. Autonomy: Event-Driven Lifeforms vs. Task-Based Tools
*   **Beyond Request/Response**: Tools like Claude Code only move when you ask them to. They are "dead" when the command ends.
*   **The Living System**: Tagentacle agents are "alive" 24/7. They can watch logs at 3 AM, detect a crash, notify a recovery agent, and fix the issue before you wake up. It's not a tool; it's a **Digital Persona**.

### 3. Professional Grade: Mission Control vs. Chat Bubbles
*   **State over Stream**: Most frameworks force everything into a Telegram/WhatsApp chat.
*   **Visualization**: Tagentacle exposes a raw data bus, allowing for "Mission Control" dashboards—real-time topology of agents, live CPU graphs, and interactive code editors—all driven by the same message bus.

### 4. The Architectural Divide: Mesh Topology vs. Tree Call-Stack

This is the deepest _architectural chasm_ between Tagentacle and Claude Code's plugin/sub-agent model.

#### Worldview Divergence

| | **Claude Code (Project-Centric)** | **Tagentacle (System-Centric)** |
| :--- | :--- | :--- |
| **Universe** | The current Git repo | A live multi-entity runtime |
| **`.claude.md` / `tagentacle.toml`** | The project's "laws of physics" | Each microservice's identity card |
| **Plugin / Pkg** | A tool mounted on the project | An independently living software entity |
| **Sub-Agent / Node** | Main Agent's outsourced helper | A first-class network citizen |
| **Role of AI** | Protagonist (all capabilities built around AI) | Just another node (Tagentacle is the OS; AI agents are processes running on it) |

#### Topology: Tree vs. Mesh

Even with Sub-Agents (isolated prompts, isolated tools, physically separated contexts), Claude Code's control flow remains a **tree-shaped call stack**:

```
User ──▶ Main Claude ──▶ SQL Agent ──▶ Database Tool
                     └──▶ Frontend Agent ──▶ Filesystem

Control flows top-down; results must return the same path.
SQL Agent cannot proactively contact Frontend Agent.
```

Tagentacle is a **Mesh topology**:

```
┌──────────────┐     /social/alerts     ┌──────────────┐
│ Scraper Node │ ── publish ──────────▶ │ Analyst Node │
└──────────────┘                        └──────┬───────┘
                                               │ /pr/drafts
┌──────────────┐                               ▼
│ Dashboard    │ ◀── subscribe ──── ┌──────────────┐
│ (side-channel│ ◀── /mcp/traffic   │ PR Node      │
│  observer)   │                    └──────────────┘
└──────────────┘                         │ call_service
                                         ▼
                                   ┌──────────────┐
                                   │ Email Service │
                                   └──────────────┘

No protagonist. Any node can publish to any Topic,
subscribe to any Topic, call any Service.
The dashboard can bypass all Agents to tap raw traffic.
```

#### Three Uncrossable Chasms

| Dimension | Claude Code Plugin / Sub-Agent | Tagentacle Node |
| :--- | :--- | :--- |
| **Topology** | Tree (call-stack; results return up the chain) | Mesh (Pub/Sub; any node can reach any other) |
| **Lifecycle** | Ephemeral (bound to one conversation turn) | Daemon (independent process, 24/7 event-driven) |
| **Dependency** | Components subordinate to host project (Guest) | Components are equal, independent microservices (Peer) |

#### Scenario Fit

*   **"Fix this React bug for me"** → **Claude Code wins.** AI is Guest, the codebase is its universe, `.claude.md` provides context, Sub-Agents split the query/write work. Smooth.
*   **"24/7 sentiment monitoring: scraper → analyst → PR writer → auto-email"** → **Tagentacle is irreplaceable.** Event-driven, continuously running, nodes crash/restart independently, no central brain—a tree call-stack simply cannot express this architecture.

Tagentacle is not another Claude Code. **It is the infrastructure for managing countless "Claude-grade agents."**

### 5. Why Not Just Use Linux Directly?

A reasonable question: Linux already has IPC (sockets, pipes, D-Bus), user permissions (uid/gid, DAC, SELinux), process isolation (namespaces, cgroups, Docker), and service discovery (DNS, mDNS, systemd). Is Tagentacle reinventing the wheel?

**No — Tagentacle is not replacing Linux. It is a _domain shell_ built on top of Linux** — the same way HTTP doesn't replace TCP, ROS 2 doesn't replace shared memory, and Bash doesn't replace `write(2)`.

| Primitive | Linux Native | What Tagentacle Adds |
| :--- | :--- | :--- |
| **IPC** | socket, pipe, shm, D-Bus | **Named Topic Pub/Sub + Service RPC** — Linux IPC is point-to-point. Topic Pub/Sub provides "publish once, all subscribers receive" semantics with named channels. D-Bus is the closest, but it's an XML protocol designed for desktop apps, not a JSON message bus for multi-node systems. |
| **Permissions** | uid/gid, capabilities | **Dynamic identity + tool-level authorization** — Linux users are static (root creates, /etc/passwd stores). Tagentacle nodes are dynamic (created/destroyed at runtime), and permission granularity is "node X can call tool Y on server Z", not "user 1000 can read /home/foo". |
| **Isolation** | namespaces, cgroups, Docker | **Semantic binding between node ↔ container** — Docker only knows "container abc123". Tagentacle knows "this container is a personal space for node X, it should auto-connect to the bus and only accept requests from authorized callers". |
| **Discovery** | DNS, mDNS, systemd | **Capability-aware discovery** — DNS tells you "service X is at 192.168.1.5:8080". `/mcp/directory` tells you "MCP Server X provides tools [read_file, exec_command], requires JWT auth, supports Streamable HTTP". |

What Tagentacle truly adds is not the primitives themselves, but the **glue semantics** between them: node identity → bound container → authorized tool access → bus-based service discovery → lifecycle management. Linux provides every brick; Tagentacle provides the blueprint for assembling them into AI agent infrastructure.

#### What Would a Sufficiently Advanced AI Choose?

Three scenarios when AI is smart enough to do anything:

**Scenario A: AI uses raw Linux APIs.** Fork, clone namespaces, set up cgroups, create socket pairs, implement its own message protocol, implement its own permission logic... *Every action requires reasoning through the entire chain from scratch.* Like writing a web app from TCP sockets every time.

**Scenario B: AI uses Docker + ad-hoc orchestration.** Better, but *each AI invents a different communication protocol.* This is exactly the state of robotics before ROS — every lab reinvented IPC, making results non-composable.

**Scenario C: AI uses Tagentacle.** `node.publish("/task/result", data)` — one line replaces dozens of reasoning steps. Standardized protocol means nodes written by different AIs (or humans) are automatically interoperable.

**AI will always choose the path that minimizes token expenditure.** `node.publish(topic, msg)` will always cost fewer tokens than deriving the equivalent `socket() + connect() + send() + custom protocol encode/decode`. Just as humans can write assembly but choose Python.

#### The Lesson from ROS

ROS wrapped Linux rather than exposing it directly because:
1. **Protocol unification** — before ROS, every robotics team used different IPC (CORBA, Ice, custom protocols). Algorithms couldn't be shared across teams. ROS Topics/Services unified the communication contract.
2. **Discover-and-use** — `rostopic list` shows all data flows in the system. Without ROS, you need to know every process's socket address, protocol format, and data encoding.
3. **Composability** — any two ROS nodes can communicate as long as their topic types match, without knowing each other exists. Linux IPC requires explicit wiring of both ends.

**The AI agent ecosystem is in a pre-ROS state** — every agent framework (LangChain, CrewAI, AutoGen) invents its own communication model, making agents non-interoperable. Tagentacle aims to be the ROS of this domain.

#### The Architecture Stack

```
┌─────────────────────────────────────────────────────┐
│  Application Layer (Nodes, Agents, MCP Servers)      │  ← iterates fastest
├─────────────────────────────────────────────────────┤
│  Tagentacle (Domain Shell / Middleware)               │  ← domain semantics
│  Topic Pub/Sub, Service RPC, /mcp/directory,         │     (syntax sugar with
│  Node identity, Lifecycle, TACL, Launch              │      real engineering value)
├─────────────────────────────────────────────────────┤
│  Linux (Kernel + Docker)                             │  ← "laws of physics"
│  socket, namespace, cgroup, filesystem, signals      │
└─────────────────────────────────────────────────────┘
```

The real risk is not "AI skips Tagentacle and uses Linux directly" — it's "another framework provides better semantic compression." This is why the Daemon should only implement **mechanisms** (IPC routing, process launch, container lifecycle), never **policies** (how to orchestrate, how to schedule) — mechanisms are stable, policies iterate with competition.

---

## 🏗️ Architecture

The system consists of three main pillars:

1.  **`tagentacle` (Rust)**: High-performance message router (Daemon/Broker) and CLI tools.
2.  **`tagentacle-py` (Python)**: Official Python SDK (similar to ROS's `rclpy`), providing a dual-layer async API.
3.  **`tagentacle-ecosystem` (Growing)**: Official example Pkg collection, including a complete chatbot system (`example-agent`, `example-inference`, `example-memory`, `example-frontend`, `example-mcp-server`, `example-bringup`).

### 🧩 The ROS 2 Analogy

| ROS 2 Concept | Tagentacle Mapping | AI Scenario Description |
| :--- | :--- | :--- |
| **Workspace** | **Agent Workspace** | A directory containing multiple Pkgs, representing a complex agent system (e.g., "Personal Assistant"). |
| **Node** | **Agent Node / General Node** | Runtime entity. **Agent Nodes** are LLM-driven with autonomous decision-making; **General Nodes** are deterministic programs (e.g., monitoring, hardware interfaces). |
| **Topic** | **Schema-Validated Channel** | Async data channel with **mandatory JSON Schema** contracts. Non-conforming "hallucination outputs" are rejected at the entry point. |
| **Service** | **Tool Call (RPC)** | Synchronous RPC. Used for high-frequency MCP tool calls (e.g., reading files, DB queries). |
| **Interface Pkg** | **JSON Schema Contract Pkg** | Dedicated package defining cross-node message contracts, ensuring interoperability. |
| **Bringup Pkg** | **Config Center** | Topology orchestration, parameter injection (API_KEY, Base_URL, tool allow-lists), and node launch configuration. |
| **Library Pkg** | **Pure Prompt / Code** | Contains code libraries or Skills without starting an independent node. |

### 📦 Package Management & Orchestration

#### `tagentacle.toml`: Lightweight Metadata Declaration
Each package root must contain a `tagentacle.toml` manifest:
```toml
[package]
name = "alice_agent"
version = "0.1.0"
description = "A conversational AI agent"
authors = ["dev@example.com"]

[entry_points]
node = "main:AliceNode"  # Exported Node class for CLI auto-loading

[dependencies]
python = ["openai", "tagentacle-py>=0.1.0"]
```

#### Bringup: Centralized Configuration & Topology Control
The Bringup Package serves as the system's "configuration center":
*   **Topology Orchestration**: Declare which nodes compose the system via configuration files.
*   **Parameter Injection**: Dynamically distribute API_KEY, Base_URL, and "tool allow-lists" at launch time.

### Communication Flow
- **Topic (Pub/Sub)**: Real-time broadcasts, timeline updates, and streaming output (e.g., LLM response streams). **Validated against JSON Schema.**
- **Service (Req/Res)**: Inter-node RPC calls (e.g., Agent calling Inference Node for completion).
- **MCP (Streamable HTTP)**: Agents connect directly to MCP Servers via native MCP SDK HTTP client; tool calls bypass the bus. The bus only handles service discovery (`/mcp/directory` Topic).
- **Action (Planned)**: Long-running asynchronous tasks with progress feedback.

---

## 🔌 Python SDK: Dual-Layer Design

### Simple API (for General Nodes)
Quick integration for existing software—just `publish()` and `subscribe()`:
```python
from tagentacle_py import Node
import asyncio

async def main():
    node = Node("sensor_node")
    await node.connect()

    @node.subscribe("/data/temperature")
    async def on_temp(msg):
        print(f"Temperature: {msg['payload']['value']}°C")

    await node.publish("/status/online", {"node": "sensor_node"})
    await node.spin()

asyncio.run(main())
```

### Node API (for Agent Nodes with Lifecycle)
Full lifecycle management with `on_configure`, `on_activate`, etc., suitable for CLI-launched nodes accepting Bringup configuration:
```python
from tagentacle_py import LifecycleNode

class AliceAgent(LifecycleNode):
    def on_configure(self, config):
        self.api_key = config.get("api_key")
        self.allowed_tools = config.get("tools", [])

    def on_activate(self):
        self.subscribe("/task/inbox", self.handle_task)

    async def handle_task(self, msg):
        result = await self.call_service("/tool/search", msg["payload"])
        await self.publish("/task/result", result)

    def on_shutdown(self):
        self.logger.info("Alice shutting down gracefully.")
```

### Built-in Nodes: TagentacleMCPServer & MCPGatewayNode
The SDK includes two key built-in nodes:
*   **TagentacleMCPServer**: Exposes the bus's `publish`, `subscribe`, `call_service` and other capabilities as standard MCP Tools. Inherits `MCPServerNode` and runs its own Streamable HTTP endpoint.
*   **MCPGatewayNode**: Transport-level relay — adapts stdio-only legacy MCP Servers to Streamable HTTP, and publishes remote server URLs to `/mcp/directory`.

---

## 🛠️ MCP Integration: Local Sessions + Direct HTTP

Inspired by ROS 2's TF2 pattern, Tagentacle localizes MCP session management within Agent Nodes:

### Design Principles
*   **Session Localization**: MCP Client Sessions stay in Agent node memory. Agents connect directly to MCP Servers via native MCP SDK Streamable HTTP client.
*   **MCPServerNode Base Class**: MCP Servers inherit `MCPServerNode` (LifecycleNode subclass), which auto-runs Streamable HTTP and publishes `MCPServerDescription` to `/mcp/directory`.
*   **Unified Discovery**: Agents subscribe to `/mcp/directory` to auto-discover all available MCP servers (native HTTP + Gateway-proxied stdio servers).
*   **Full Protocol Support**: Direct Agent↔Server sessions preserve all MCP capabilities including sampling, notifications, and resources.
*   **MCP Gateway**: A separate `mcp-gateway` package provides transport-level stdio→HTTP relay for legacy MCP servers, without parsing MCP semantics.

### Agent-Side Connection Example
```python
from mcp import ClientSession
from mcp.client.streamable_http import streamable_http_client

# Connect directly to MCP Server via Streamable HTTP
async with streamable_http_client("http://127.0.0.1:8100/mcp") as (r, w, _):
    async with ClientSession(r, w) as session:
        await session.initialize()
        result = await session.call_tool("query", {"sql": "SELECT * FROM users"})
```

### Bidirectional & Observability
- **Bidirectional**: Because MCP sessions are established directly between Agent ↔ Server (HTTP long-lived connections), full MCP spec bidirectional capabilities are natively supported, including **Sampling** (Server calling back to Agent).
- **Observability**: MCP Server discovery info is published to the `/mcp/directory` Topic. Any node can subscribe to get a real-time view of all available tools in the system.

---

## 📜 Standard Topics & Services

When the Daemon starts, it automatically creates a set of **system-reserved Topics and Services** under the `/tagentacle/` namespace — analogous to ROS 2's `/rosout`, `/parameter_events`, and node introspection services. These provide built-in observability, logging, and introspection without requiring any user-side setup.

### Reserved Namespace Convention

| Prefix | Purpose | Managed By |
|---|---|---|
| `/tagentacle/*` | **System reserved.** Daemon & SDK core functionality | Core library |
| `/mcp/*` | MCP discovery and gateway services | MCPServerNode / Gateway |

User-defined Topics should **not** use the above prefixes.

### Standard Topics (Daemon-managed)

| Topic | Analogy | Description | Published By |
|---|---|---|---|
| `/tagentacle/log` | ROS `/rosout` | Global log aggregation. All nodes auto-publish logs here via SDK; Daemon also publishes system events. | SDK Nodes (auto) + Daemon |
| `/tagentacle/node_events` | ROS lifecycle events | Node lifecycle events: connected, disconnected, lifecycle state transitions. Powers real-time topology in dashboards. | Daemon (auto) + `LifecycleNode` (auto) |
| `/tagentacle/diagnostics` | ROS `/diagnostics` | Node health reports: heartbeat, uptime, message counters, error counts.  | SDK `Node.spin()` (periodic) |
| `/mcp/directory` | _(none)_ | MCP server discovery. `MCPServerDescription` messages published by MCP Server Nodes and Gateway on activation. Agents subscribe to auto-discover servers. | MCPServerNode / Gateway |

### Standard Services (Daemon-intercepted)

> **Architectural Note — Intentional Asymmetry**
>
> These `/tagentacle/*` Services are **not** published by a real Node via `advertise_service`. Instead, the Daemon **intercepts** `call_service` requests whose name starts with `/tagentacle/` and generates responses directly from its Router's internal state — much like Linux's `/proc` filesystem, which is synthesized by the kernel rather than backed by a real disk.
>
> This means `/tagentacle/list_services` will **not** list itself or any other `/tagentacle/*` service — they exist outside the normal service registry. This is by design: the Daemon provides *read-only introspection as a mechanism*, without participating as a regular Node in the bus topology it manages.
>
> From a caller's perspective, the API is identical to any other service call — `call_service("/tagentacle/ping", {})` works the same way. The asymmetry is invisible to consumers but fundamental to the architecture.

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

### Log Message Schema (`/tagentacle/log`)
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

### Node Event Schema (`/tagentacle/node_events`)
```json
{
  "event": "connected",
  "node_id": "alice_agent",
  "timestamp": "2026-02-24T12:00:00.000Z",
  "state": "active",
  "prev_state": "inactive"
}
```

---

## 🤖 Agent Architecture: IO + Inference Separation

Tagentacle adopts a clean separation between **Agent Nodes** (context engineering + agentic loop) and **Inference Nodes** (stateless LLM gateway):

### Agent Node = Complete Agentic Loop

An Agent Node is a single Pkg that owns the entire agentic loop internally:
- Subscribe to Topics → receive user messages / events
- Manage the context window (message queue, context engineering)
- Call Inference Node's Service for LLM completion
- Parse `tool_calls` → execute tools via MCP Session (Streamable HTTP direct connection) → backfill results → re-infer

This loop is a tightly-coupled sequential control flow (like ROS 2's nav2 stack) and should **not** be split across multiple Nodes.

### Inference Node = Stateless LLM Gateway

A separate Pkg (official example at org level, **not** part of the core library) that provides:
- A Service (e.g., `/inference/chat`) accepting OpenAI-compatible format: `{model, messages, tools?, temperature?}`
- Returns standard completion: `{choices: [{message: {role, content, tool_calls?}}]}`
- Multiple Agent Nodes can call the same Inference Node concurrently

### Data Flow
```
UI Node ──publish──▶ /chat/input ──▶ Agent Node (agentic loop)
                                        │
                                        ├─ call_service("/inference/chat") ──▶ Inference Node ──▶ OpenRouter/OpenAI
                                        │                                           │
                                        │◀── completion (with tool_calls) ◀─────────┘
                                        │
                                        ├─ MCP Session (HTTP) ──▶ Tool Server Node
                                        │◀── tool result ◀────────┘
                                        │
                                        └─ publish ──▶ /chat/output ──▶ UI Node
```

---

## 🔐 TACL: Tagentacle Access Control Layer

**TACL** (Tagentacle Access Control Layer) provides MCP-level JWT authentication and authorization. It is implemented entirely in the Python SDK (`python-sdk-mcp`) — the Daemon knows nothing about access control, staying true to the "mechanisms only" principle.

### Architecture

TACL is built around three roles:

| Role | Component | Responsibility |
|---|---|---|
| **Issuer** | `PermissionMCPServerNode` | SQLite-backed agent registry. Issues JWT credentials with tool-level grants. |
| **Verifier** | `MCPServerNode` (with `auth_required=True`) | Validates Bearer JWT on every request. Sets `CallerIdentity` contextvar. |
| **Carrier** | `AuthMCPClient` | Authenticates with the permission server, obtains JWT, attaches it to all MCP requests. |

### JWT Payload Schema

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

### The `space` Claim: Binding Agents to Containers

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

### Authentication Flow

```
Admin                   PermissionMCPServerNode           MCPServerNode (auth_required)
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

### Design Principles

- **Zero external dependencies**: JWT signing/verification uses pure Python stdlib (HS256 via `hmac` + `hashlib`).
- **Shared secret**: Both issuer and verifiers read `TAGENTACLE_AUTH_SECRET` from environment.
- **Contextvar-based**: `CallerIdentity` is set per-request via Python `contextvars`, enabling tool handlers to read caller info without parameter threading.
- **Granularity**: Authorization supports **tool-level** per server. However, prefer **server-level** control with small, focused servers (see [Best Practices](#-best-practices)).
- **Optional**: Auth is opt-in. `MCPServerNode(auth_required=False)` (default) accepts all callers.

---

## � Best Practices

### TACL: Prefer Server-Level Access Control

TACL supports both **server-level** and **tool-level** authorization via `tool_grants`. However, we recommend using TACL primarily for **server-level control** — i.e., granting or denying an agent access to an entire MCP Server.

If you find yourself needing tool-level ACL within a single server, it's a signal that the server is doing too much. Following the Unix philosophy, split it into smaller, focused MCP Server Pkgs — each doing one thing well. When each server is single-purpose, server-level ACL naturally provides the right granularity.

| Approach | Recommendation | Example |
|----------|---------------|--------|
| Server-level | ✅ Recommended | Agent A can access `shell-server` but not `wallet-server` |
| Tool-level | ⚠️ Possible but not preferred | Agent A can call `exec_command` but not `list_files` on the same server |

**Why?** Smaller servers are easier to reason about, deploy independently, and secure at the perimeter. Tool-level ACL adds complexity inside the server without improving the overall security boundary.

### MCP Server Transport: Streamable HTTP Only

All MCP Server Node Pkgs in Tagentacle use **Streamable HTTP** as their transport. This is required by the `MCPServerNode` base class and enables:

- Direct Agent↔Server sessions with full MCP protocol support (sampling, notifications, resources)
- TACL JWT authentication via standard HTTP `Authorization` headers
- Standard health checks, load balancing, and container networking

**stdio MCP Servers are not recommended.** The `mcp-gateway` package provides stdio→HTTP relay as a **legacy compatibility layer** for third-party MCP servers that only support stdio transport. This is analogous to bridging a Linux pipe inside a ROS 2 node — it works, but breaks the standard communication model:

- stdio servers cannot participate in TACL authentication (no HTTP headers)
- stdio sessions are managed by the Gateway process, not the Agent
- One subprocess per HTTP session limits scalability

If you control the MCP Server code, always implement it as a Streamable HTTP `MCPServerNode` Pkg.

---

## �🐳 Container-Ready Architecture

Tagentacle's "Everything is a Pkg" philosophy makes it **naturally container-friendly**. Each package is an independent process with its own dependencies — a perfect fit for one-container-per-package deployment.

### Why Containerize?

Containerization is especially powerful for **Agent Nodes**, where it provides:

- **Maximum Freedom for Agents**: Each Agent runs in its own container with a fully isolated filesystem, network stack, and dependency tree. An Agent can `pip install` libraries, write files, or spawn subprocesses without affecting other Agents — true autonomy for AI-driven nodes.
- **Fault Isolation at the OS Level**: A runaway Agent (infinite loop, memory leak, adversarial tool output) is contained by cgroup limits. Other services continue unaffected.
- **Dynamic Scaling**: Spin up multiple instances of Inference Nodes or MCP Servers behind a load balancer. Agent Nodes can be added/removed at runtime.
- **Reproducible Deployment**: Each package's `Dockerfile` + `pyproject.toml` = byte-identical environments everywhere, from dev laptops to cloud clusters.
- **Security Boundaries**: Agent Nodes handling untrusted tool outputs or user inputs are sandboxed at the container level — defense-in-depth beyond TACL's JWT authentication.

### Deployment Progression

```
Development          → docker-compose           → K3s / K8s
─────────────────      ─────────────────────      ──────────────────
tagentacle daemon      tagentacle-core container   Deployment + Service
python process/pkg     one container per pkg       HPA auto-scaling
localhost:19999        Docker bridge network        K8s Service DNS
bringup.py / launch    docker-compose.yml          Helm Chart
```

### Near-Zero Overhead

Containers are **not VMs** — they share the host kernel via Linux namespaces and cgroups:
- **CPU / Memory**: <1% overhead (native execution, no hypervisor)
- **Network**: 2–5% overhead on bridge mode; `host` network mode = zero overhead
- **Disk I/O**: Bind-mounted volumes = native speed; OverlayFS writes have ~5% copy-on-write cost
- **GPU**: Zero overhead via NVIDIA Container Toolkit (passthrough)

For Tagentacle's primary workload (WebSocket bus messages + LLM API calls), the container network overhead (~50μs) is negligible compared to inference latency (~200ms+).

### Minimal Code Changes

The SDK is already container-compatible. The only change needed is injecting the bus address via environment variable:

```python
import os
bus_host = os.environ.get("TAGENTACLE_BUS_HOST", "localhost")
bus_port = int(os.environ.get("TAGENTACLE_BUS_PORT", "19999"))
```

All higher-level abstractions — `Node`, `LifecycleNode`, `MCPServerNode`, TACL authentication, `/mcp/directory` discovery — work identically inside containers.

### Container Orchestrator: Bus-Level Container Management

The `container-orchestrator` package is a `LifecycleNode` that manages Docker containers via the bus — **not** part of the Daemon core. Like Docker is a userspace program on Linux, this orchestrator is an ecosystem package.

| Bus Service | Description |
|---|---|
| `/containers/create` | Create and start a container from an image |
| `/containers/stop` | Stop a running container |
| `/containers/remove` | Remove a container |
| `/containers/list` | List all managed containers |
| `/containers/inspect` | Get container details |
| `/containers/exec` | Execute a command inside a container |

```bash
# Create an agent space
tagentacle service call /containers/create \
  '{"image": "ubuntu:22.04", "name": "agent_space_1"}'

# Execute a command inside it
tagentacle service call /containers/exec \
  '{"name": "agent_space_1", "command": "ls -la /workspace"}'
```

Key design decisions:
- All containers are labeled with `tagentacle.managed=true` for filtering.
- Auto-injects `TAGENTACLE_DAEMON_URL` env var so containerized nodes can connect back to the bus.
- Default network mode: `host` (simplest for bus connectivity).
- No ACL logic — access control is handled by TACL at the MCP layer.

### Shell Server: TACL-Aware Dynamic Routing

The `shell-server` package is an `MCPServerNode` that provides `exec_command` as an MCP tool. It supports three execution modes, resolved per-request:

```
Container Resolution Order:
  1. TACL JWT space claim → docker exec into caller's bound container
  2. Static TARGET_CONTAINER env → single fixed container
  3. Local subprocess fallback
```

In production (TACL mode), a single shell-server instance serves **multiple agents simultaneously**, each routed to their own container based on their JWT `space` claim:

```
Agent Alpha (JWT: space=agent_space_1) ──▶ Shell Server ──▶ docker exec agent_space_1
Agent Beta  (JWT: space=agent_space_2) ──▶ Shell Server ──▶ docker exec agent_space_2
Agent Gamma (JWT: no space)            ──▶ Shell Server ──▶ local subprocess
```

### End-to-End: From Registration to Isolated Execution

```
1. Admin registers agent:          PermissionNode.register_agent(space="agent_space_1")
2. Orchestrator creates container: /containers/create → docker run agent_space_1
3. Agent authenticates:            AuthMCPClient → JWT {agent_id, space: "agent_space_1"}
4. Agent calls exec_command:       Shell Server reads JWT.space → docker exec agent_space_1
```

No node trusts another node's self-reported identity. Every tool call goes through JWT verification. The container binding is **cryptographically attested** in the token, not passed as a mutable parameter.

---

## 📜 Communication Protocol

The Tagentacle Daemon listens on `TCP 19999` by default. All communication uses newline-delimited JSON (JSON Lines).

### Topics
*   **Subscribe**: `{"op": "subscribe", "topic": "/chat/global", "node_id": "alice_node"}`
*   **Publish**: `{"op": "publish", "topic": "/chat/global", "sender": "bob_node", "payload": {"text": "Hello!"}}`
*   **Message (Daemon Push)**: `{"op": "message", "topic": "/chat/global", "sender": "bob_node", "payload": {"text": "Hello!"}}`

### Services
*   **Advertise**: `{"op": "advertise_service", "service": "/tool/read_file", "node_id": "fs_node"}`
*   **Call**: `{"op": "call_service", "service": "/tool/read_file", "request_id": "req-1", "payload": {"path": "a.txt"}}`
*   **Response**: `{"op": "service_response", "service": "/tool/read_file", "request_id": "req-1", "payload": {"content": "..."}}`

---

## 🛠️ CLI Tools (`tagentacle`)

The CLI provides the primary interface for developers:
- `tagentacle daemon`: Starts the local TCP message bus.
- `tagentacle run --pkg <dir>`: Activates the package's `.venv` and launches its Node.
- `tagentacle launch <config.toml>`: Orchestrates multiple Nodes from topology config, each with isolated venvs.
- `tagentacle topic echo <topic>`: Subscribes and prints real-time messages.
- `tagentacle service call <srv> <json>`: Tests a service from the command line.
- ~~`tagentacle bridge`~~: Removed in v0.3.0. Use `mcp-gateway` package instead.
- `tagentacle setup dep --pkg <dir>`: Runs `uv sync` for a single package.
- `tagentacle setup dep --all <workspace>`: Installs all packages and generates `install/` structure with `.venv` symlinks + `setup_env.bash`.
- `tagentacle setup clean --workspace <dir>`: Removes the generated `install/` directory.
- `tagentacle doctor`: Health check (daemon status, node connectivity).

### Environment Management

Every package is a **uv project** (`pyproject.toml` + `uv.lock`). No pip is used.

```bash
# Initialize the full workspace
tagentacle setup dep --all .
# → runs uv sync in each package
# → creates install/src/<pkg>/.venv symlinks
# → generates install/setup_env.bash

# Source environment (adds all .venvs to PATH)
source install/setup_env.bash

# Clean up
tagentacle setup clean --workspace .
```

---

## 📝 Roadmap & Status

### Dependency Resolution Capabilities

| Source Type | Status | Description |
| :--- | :---: | :--- |
| **Git Repos** | ✅ | `[workspace.repos]` in `tagentacle.toml` — auto-clone missing repos on `setup dep --all` |
| **Python (uv)** | ✅ | Per-package `uv sync` with `.venv` isolation |
| **apt packages** | ❌ | Planned — system-level dependencies |
| **PyPI packages** | ❌ | Planned — system-level SDK distribution via `pip install tagentacle-py-core` |
| **npm packages** | ❌ | Planned — Node.js tool and MCP server dependencies |
| **Build commands** | ❌ | Planned — custom build steps (e.g., `cargo build`, `make`) |

### Completed
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

### Planned
- [x] **Standard System Services**: Daemon-intercepted `/tagentacle/ping`, `/tagentacle/list_nodes`, `/tagentacle/list_topics`, `/tagentacle/list_services`, `/tagentacle/get_node_info`.
- [x] **Node Registration & Heartbeat**: `Register` handshake, periodic ping/pong, automatic stale-node cleanup (90s timeout).
- [x] **Node Disconnect Cleanup**: Automatic removal of subscriptions, services, and node entries on disconnect, with `/tagentacle/node_events` publishing.
- [ ] **Standard Topics (SDK-side)**: SDK auto-publish to `/tagentacle/log`, `/tagentacle/diagnostics`.
- [ ] **SDK Log Integration**: Auto-publish node logs to `/tagentacle/log` via `get_logger()`.
- [x] **JSON Schema Validation**: `python-sdk-core` v0.3.0 — `SchemaRegistry` with auto-discovery from interface packages, configurable per-node (`strict`/`warn`/`off`), integrated into `Node.publish()` and `Node._dispatch()`. Requires optional `jsonschema>=4.0`.
- [x] **TACL `space` Claim**: `python-sdk-mcp` v0.4.0 — JWT `space` field binding agents to isolated execution environments. Full stack: `CallerIdentity.space`, `sign_credential(space=...)`, `PermissionMCPServerNode.register_agent(space=...)`.
- [ ] **Flattened Topic Tools API**: SDK API to auto-generate flattened MCP tools from Topic JSON Schema definitions (e.g., a registered `/chat/input` schema auto-generates a `publish_chat_input(text, sender)` tool with expanded parameters).
- [ ] **Interface Package**: Cross-node JSON Schema contract definition packages.
- [ ] **Action Mode**: Long-running async tasks with progress feedback.
- [ ] **Parameter Server**: Global parameter store with `/tagentacle/parameter_events` notifications.
- [x] **Container Orchestration Pkg**: `container-orchestrator` v0.1.0 — LifecycleNode managing Docker containers via bus services (`/containers/create`, `stop`, `list`, `exec`, etc.).
- [x] **Shell Server Pkg**: `shell-server` v0.1.0 — MCPServerNode exposing `exec_command` tool with TACL `space`-aware dynamic container routing (JWT → container → local fallback).
- [ ] **Web Dashboard**: Real-time topology, message flow, and node status visualizer.

---

## 🚀 Getting Started

### Installation

```bash
# Install from source (compiles and copies to ~/.cargo/bin/)
cd tagentacle
cargo install --path .

# Verify
tagentacle --help

# Uninstall
cargo uninstall tagentacle
```

> **Note**: Ensure `~/.cargo/bin` is in your `PATH` (rustup adds it by default).

### Quick Start

The recommended workflow is to clone a **bringup package** into a workspace's `src/` directory and let the CLI handle everything:

1. **Create a workspace and clone the bringup repo**:
   ```bash
   mkdir -p my_workspace/src && cd my_workspace/src
   git clone https://github.com/Tagentacle/example-bringup.git
   ```

2. **Set up workspace dependencies** (auto-clones repos & installs Python deps):
   ```bash
   cd ..  # back to my_workspace/
   tagentacle setup dep --all src
   ```
   This reads `[workspace.repos]` from `example-bringup/tagentacle.toml`, clones missing repos (SDK + app packages) into `src/`, then runs `uv sync` in each package.

3. **Start the Daemon** (in a separate terminal):
   ```bash
   tagentacle daemon
   ```

4. **Launch the system**:
   ```bash
   tagentacle launch src/example-bringup/launch/system_launch.toml
   ```
