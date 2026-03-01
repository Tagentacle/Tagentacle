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
| **What is AI?** | Guest (serving the codebase) | Host (the operating system managing everything) |

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

### Standard Services (Daemon built-in)

The Daemon registers itself as the `_daemon_` node and provides these introspection Services directly from its internal Router state:

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

## � Container-Ready Architecture

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

---

## �📜 Communication Protocol

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
- [ ] **Standard Topics & Services**: Daemon built-in `/tagentacle/log`, `/tagentacle/node_events`, `/tagentacle/diagnostics`, `/tagentacle/ping`, `/tagentacle/list_nodes`, etc.
- [ ] **SDK Log Integration**: Auto-publish node logs to `/tagentacle/log` via `get_logger()`.
- [ ] **JSON Schema Validation**: Topic-level schema contracts for deterministic message validation.
- [ ] **Flattened Topic Tools API**: SDK API to auto-generate flattened MCP tools from Topic JSON Schema definitions (e.g., a registered `/chat/input` schema auto-generates a `publish_chat_input(text, sender)` tool with expanded parameters).
- [ ] **Node Lifecycle Tracking**: Heartbeat/liveliness monitoring in the Daemon via `/tagentacle/diagnostics`.
- [ ] **Interface Package**: Cross-node JSON Schema contract definition packages.
- [ ] **Action Mode**: Long-running async tasks with progress feedback.
- [ ] **Parameter Server**: Global parameter store with `/tagentacle/parameter_events` notifications.
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
