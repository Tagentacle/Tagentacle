# Tagentacle: ROS for the AI Era 🐙

[![GitHub](https://img.shields.io/badge/GitHub-Tagentacle-blue)](https://github.com/Tagentacle/tagentacle)

**Tagentacle** is a decentralized, configuration-centralized, production-grade multi-agent framework. It deeply adopts the software organization patterns of **ROS 2**, combined with the modern AI ecosystem (MCP Protocol) and dynamic Schema technologies, to build a robust infrastructure for LLM multi-agent collaboration.

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

| Feature | Monolithic Gateways (e.g., OpenClaw) | CLI-Based Tools (e.g., Claude Code/CC) | **Tagentacle** |
| :--- | :--- | :--- | :--- |
| **Architecture** | Monolithic Gateway (Node.js) | Single-Process CLI | **Distributed Microservices (Rust)** |
| **Stability** | Single Failure = Full Crash | Process-bound | **Isolated Processes (Fault Tolerant)** |
| **Lifecycle** | Chat-bound (TG/WA) | Task-bound (One-shot) | **Continuous (24/7 Event-Driven)** |
| **Interaction** | Chat Bubble (ChatOps) | Terminal Output | **Mission Control (Real-time Dashboards)** |
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
- `tagentacle run <pkg_name>`: Parses `tagentacle.toml`, sets up venv, and launches a Node.
- `tagentacle launch <config.yaml>`: Orchestrates multiple Nodes from topology config.
- `tagentacle topic list`: Lists all active Topics on the bus.
- `tagentacle topic echo <topic>`: Subscribes and prints real-time messages.
- `tagentacle service list`: Lists all registered Services.
- `tagentacle service call <srv> <json>`: Tests a service from the command line.
- `tagentacle bridge --mcp <cmd>`: Bridges an external MCP Server (stdio) to the bus.
- `tagentacle setup/dep`: Environment management (venv creation, dependency installation).
- `tagentacle doctor`: Health check (daemon status, node connectivity).

---

## 📝 Roadmap & Status

### Completed
- [x] **Rust Daemon**: Topic Pub/Sub and Service Req/Res message routing.
- [x] **Python SDK (Simple API)**: `Node` class with `connect`, `publish`, `subscribe`, `service`, `call_service`, `spin`.
- [x] **MCP Bridge (Rust)**: `tagentacle bridge --mcp` command tunneling stdio MCP servers through the bus.
- [x] **Examples**: `talker/listener` (Pub/Sub) and `service_server/service_client` (Service) demos.

### In Progress
- [x] **Fix Build**: Add missing `clap` and `uuid` dependencies to `Cargo.toml`.
- [x] **Python SDK Dual-Layer API**: `LifecycleNode` with `on_configure`/`on_activate`/`on_deactivate`/`on_shutdown`.
- [x] **MCP Transport Layer**: Implement `TagentacleClientTransport` and `TagentacleServerTransport` in `tagentacle-py`.
- [x] **`tagentacle.toml` Spec**: Define and parse package manifest format.
- [ ] **JSON Schema Validation**: Topic-level schema contracts for deterministic message validation.

### Planned
- [x] **MCP-Publish Bridge Node**: Built-in MCP Server that exposes `publish()` as an MCP Tool.
- [x] **Bringup Configuration Center**: Config-driven topology orchestration with parameter injection.
- [x] **CLI Expansion (partial)**: `topic echo`, `service call`, `doctor` implemented. `run`, `launch`, `topic list`, `service list`, `setup/dep` pending.
- [ ] **Node Lifecycle Tracking**: Heartbeat/liveliness monitoring in the Daemon.
- [ ] **Interface Package**: Cross-node JSON Schema contract definition packages.
- [ ] **Action Mode**: Long-running async tasks with progress feedback.
- [ ] **vcstool + `.repos`**: Multi-repo one-click workspace pull and build.
- [ ] **Web Dashboard**: Real-time topology, message flow, and node status visualizer.

---

## 🚀 Getting Started

1. **Start the Daemon** (Rust Core):
   ```bash
   cd tagentacle
   cargo run -- daemon
   ```

2. **Run a Node** (Python SDK):
   ```python
   from tagentacle_py import Node
   import asyncio

   async def main():
       node = Node("example_node")
       await node.connect()
       await node.publish("/hello", {"data": "world"})
       await node.spin()

   asyncio.run(main())
   ```
