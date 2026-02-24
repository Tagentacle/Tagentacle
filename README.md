# Tagentacle: ROS for AI Agent 🐙

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
3.  **`tagentacle-ecosystem` (Planned)**: A collection of standard Pkgs (e.g., `chatbot_ui_pkg`, `mcp_sqlite_wrapper_pkg`).

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
- **Service (Req/Res)**: Fast tool calling (e.g., MCP tool execution).
- **MCP Tunneling**: "Double-Track" mechanism—MCP JSON-RPC tunneled through Tagentacle Services for reliability, mirrored to Topics for observability.
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

### Built-in Node: MCP-Publish Bridge
The SDK includes a special **MCP Server Node** that abstracts the bus `publish()` capability as a standard MCP Tool, allowing Agent Nodes to autonomously send messages to bus Topics via tool calls:
```python
# The MCP-Publish bridge exposes bus topics as MCP tools:
# Tool: "publish_to_topic"
# Args: {"topic": "/alerts/critical", "payload": {"msg": "Server down!"}}
```

---

## 🛠️ MCP Integration: Bus-as-Transport

Tagentacle solves the problem of MCP Sessions being non-serializable across processes:

### Design Principles
*   **Session Localization**: MCP Client/Server Sessions stay in node memory—never transmitted cross-process.
*   **Bus Traffic Forwarding**: Custom `TagentacleTransport` wraps JSON-RPC instructions into Tagentacle Service requests. This achieves "Bus-as-Transport", making tool calls transparent and efficient in distributed environments.
*   **Dual-Track Integration**: MCP's raw JSON payload is simultaneously mirrored to a dedicated Topic (e.g., `/mcp/traffic`) for non-intrusive observation and debugging.

### Seamless SDK Integration
```python
from mcp import ClientSession
from tagentacle_py.mcp import TagentacleClientTransport

# Connect through Tagentacle bus instead of stdio
transport = TagentacleClientTransport(node, server_node_id="sqlite_server")
async with ClientSession(transport) as session:
    await session.initialize()
    result = await session.call_tool("query", {"sql": "SELECT * FROM users"})
```

### Bidirectional & Observability
- **Bidirectional**: Full MCP spec support including **Sampling** (Server calling Agent). Agent nodes also register `/rpc` services for callbacks.
- **Observability**: All tunneled traffic auto-mirrored to `/mcp/traffic` Topic. Any node (e.g., Logger) can audit tool-calling flow without intrusive proxies.

---

## 📜 Standard Topics & Services

When the Daemon starts, it automatically creates a set of **system-reserved Topics and Services** under the `/tagentacle/` namespace — analogous to ROS 2's `/rosout`, `/parameter_events`, and node introspection services. These provide built-in observability, logging, and introspection without requiring any user-side setup.

### Reserved Namespace Convention

| Prefix | Purpose | Managed By |
|---|---|---|
| `/tagentacle/*` | **System reserved.** Daemon & SDK core functionality | Core library |
| `/mcp/*` | MCP protocol (audit trail, RPC tunnels) | MCP Transport / Bridge |

User-defined Topics should **not** use the above prefixes.

### Standard Topics (Daemon-managed)

| Topic | Analogy | Description | Published By |
|---|---|---|---|
| `/tagentacle/log` | ROS `/rosout` | Global log aggregation. All nodes auto-publish logs here via SDK; Daemon also publishes system events. | SDK Nodes (auto) + Daemon |
| `/tagentacle/node_events` | ROS lifecycle events | Node lifecycle events: connected, disconnected, lifecycle state transitions. Powers real-time topology in dashboards. | Daemon (auto) + `LifecycleNode` (auto) |
| `/tagentacle/diagnostics` | ROS `/diagnostics` | Node health reports: heartbeat, uptime, message counters, error counts.  | SDK `Node.spin()` (periodic) |
| `/mcp/traffic` | _(none)_ | MCP JSON-RPC audit stream. All tunneled MCP traffic is mirrored here for non-intrusive observation. **(Already implemented in Bridge.)** | Bridge / MCP Transport |

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
- Parse `tool_calls` → execute tools via MCP Transport → backfill results → re-infer

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
                                        ├─ MCP Transport ──▶ Tool Server Node
                                        │◀── tool result ◀──┘
                                        │
                                        └─ publish ──▶ /chat/output ──▶ UI Node
```

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
- `tagentacle bridge --mcp <cmd>`: Bridges an external MCP Server (stdio) to the bus.
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

### Completed
- [x] **Rust Daemon**: Topic Pub/Sub and Service Req/Res message routing.
- [x] **Python SDK (Simple API)**: `Node` class with `connect`, `publish`, `subscribe`, `service`, `call_service`, `spin`.
- [x] **Python SDK Dual-Layer API**: `LifecycleNode` with `on_configure`/`on_activate`/`on_deactivate`/`on_shutdown`.
- [x] **MCP Bridge (Rust)**: `tagentacle bridge --mcp` command tunneling stdio MCP servers through the bus.
- [x] **MCP Transport Layer**: `TagentacleClientTransport` and `TagentacleServerTransport` in `tagentacle-py`.
- [x] **Tagentacle MCP Server**: Built-in MCP Server exposing bus interaction tools (`publish_to_topic`, `subscribe_topic`, `list_nodes`, `list_topics`, `list_services`, `call_bus_service`, `ping_daemon`, `describe_topic_schema`).
- [x] **`tagentacle.toml` Spec**: Define and parse package manifest format.
- [x] **Bringup Configuration Center**: Config-driven topology orchestration with parameter injection.
- [x] **CLI Toolchain**: `daemon`, `run`, `launch`, `topic echo`, `service call`, `doctor`, `bridge`, `setup dep`, `setup clean`.
- [x] **Environment Management**: uv-based per-package `.venv` isolation, workspace `install/` structure with symlinks.
- [x] **Secrets Management**: `secrets.toml` auto-loading, bringup environment variable injection.
- [x] **SDK Utilities**: `load_pkg_toml`, `discover_packages`, `find_workspace_root`.
- [x] **Example Workspace**: `examples/src/` with agent_pkg, mcp_server_pkg, bringup_pkg as independent uv projects.

### Planned
- [ ] **Standard Topics & Services**: Daemon built-in `/tagentacle/log`, `/tagentacle/node_events`, `/tagentacle/diagnostics`, `/tagentacle/ping`, `/tagentacle/list_nodes`, etc.
- [ ] **SDK Log Integration**: Auto-publish node logs to `/tagentacle/log` via `get_logger()`.
- [ ] **JSON Schema Validation**: Topic-level schema contracts for deterministic message validation.
- [ ] **Flattened Topic Tools API**: SDK API to auto-generate flattened MCP tools from Topic JSON Schema definitions (e.g., a registered `/chat/input` schema auto-generates a `publish_chat_input(text, sender)` tool with expanded parameters).
- [ ] **Node Lifecycle Tracking**: Heartbeat/liveliness monitoring in the Daemon via `/tagentacle/diagnostics`.
- [ ] **Interface Package**: Cross-node JSON Schema contract definition packages.
- [ ] **Action Mode**: Long-running async tasks with progress feedback.
- [ ] **Parameter Server**: Global parameter store with `/tagentacle/parameter_events` notifications.
- [ ] **vcstool + `.repos`**: Multi-repo one-click workspace pull and build.
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

After installation, all commands below are run from a **workspace directory** (e.g., `tagentacle-py/example_ws/`):

1. **Start the Daemon** (in a separate terminal):
   ```bash
   tagentacle daemon
   ```

2. **Set up the workspace** (install all package dependencies):
   ```bash
   cd tagentacle-py/example_ws
   tagentacle setup dep --all .
   ```

3. **Run a Node**:
   ```bash
   tagentacle run --pkg src/mcp_server_pkg
   ```
