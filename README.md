# Tagentacle: ROS for the AI Era 🐙

[![GitHub](https://img.shields.io/badge/GitHub-Tagentacle-blue)](https://github.com/Tagentacle/tagentacle)

**Tagentacle** is a lightweight, decentralized message bus and package management system designed for Large Language Model (LLM) multi-agent collaboration and Model Context Protocol (MCP) tool integration.

Drawing inspiration from the architectural soul of **ROS 2** (Robot Operating System)—where "Everything is a Package" and "Pub/Sub Communication with Service Discovery" rule—Tagentacle discards the heavy C++ dependencies, DDS protocols, and complex build systems of its predecessor to embrace the speed and simplicity required for the AI era.

---

## 🌟 Project Vision

Tagentacle's core philosophy is **AI-Native Middleware**:
- **Protocol First**: Communication relies solely on **JSON over TCP**. Any language capable of handling JSON can become a Tagentacle Node.
- **Everything is a Package (Pkg)**: LLM roles, MCP toolkits, and system prompts are all encapsulated as standard Pkg directories with a `tagentacle.toml` manifest.
- **Node Decoupling**: Nodes are runtime process entities isolated from each other. They communicate asynchronously through the Tagentacle Daemon (Broker).

---

## ⚔️ Why Tagentacle? (The Pitch)

In a world dominated by monolithic gateways and CLI-bound tools, Tagentacle provides the "Industrial Grade" infrastructure for the next generation of AI.

| Feature | Monolithic Gateways (e.g., OpenClaw) | CLI-Based Tools (e.g., Claude Code/CC) | **Tagentacle** |
| :--- | :--- | :--- | :--- |
| **Architecture** | Monolithic Gateway (Node.js) | Single-Process CLI | **Distributed Microservices (Rust)** |
| **Stability** | Single Failure = Full Crash | Process-bound | **Isolated Processes (Fault Tolerant)** |
| **Lifecycle** | Chat-bound (TG/WA) | Task-bound (One-shot) | **Continuous (24/7 Event-Driven)** |
| **Interaction** | Chat Bubble (ChatOps) | Terminal Output | **Mission Control (Real-time Dashboards)** |
| **Scope** | Single Server | Local Filesystem | **Multi-Device / Cross-Platform** |

### 1. Robustness: Distributed Bus vs. Monolithic Gateway
*   **OpenClaw's Achilles' Heel**: Running 50 skills in one Node.js process means a memory leak in one skill reboots your entire system.
*   **Tagentacle's Absolute Isolation**: Since Every Node is a separate process, if your "Twitter Scraper" crashes, your "SRE Agent" continues fixing the server. The Rust-based Broker provides high-concurrency message routing that never sleeps.

### 2. Autonomy: Event-Driven Lifeforms vs. Task-Based Tools
*   **Beyond Request/Response**: Tools like Claude Code only move when you ask them to. They are "dead" when the command ends.
*   **The Living System**: Tagentacle agents are "alive" 24/7. They can watch logs at 3 AM, detect a crash, notify a recovery agent, and fix the issue before you wake up. It's not a tool; it's a **Digital Persona**.

### 3. Professional Grade: Mission Control vs. Chat Bubbles
*   **State over Stream**: Most frameworks force everything into a Telegram/WhatsApp chat.
*   **Visualization**: Tagentacle exposes a raw data bus, allowing for "Mission Control" dashboards. Imagine seeing a real-time topology of your agents, live CPU graphs from your servers, and an interactive code editor—all driven by the same message bus.

---

## 🏗️ Architecture

The system consists of three main pillars:

1.  **`tagentacle` (Rust)**: High-performance message router (Daemon/Broker) and CLI tools. (Core repo: `tagentacle-core`)
2.  **`tagentacle-py` (Python)**: Official Python SDK (similar to ROS's `rclpy`), providing a minimalist async API for developers.
3.  **`tagentacle-ecosystem` (Planned)**: A collection of standard Pkgs (e.g., `chatbot_ui_pkg`, `mcp_sqlite_wrapper_pkg`, `alice_agent_pkg`).

### 🧩 The ROS 2 Analogy (Updated)

Tagentacle deeply reuses the engineering philosophy of ROS 2, mapping it to AI Agent development:

| ROS 2 Concept | Tagentacle Mapping | AI Scenario Description |
| :--- | :--- | :--- |
| **Workspace** | **Agent Workspace** | A directory containing multiple Pkgs, representing a complex robotic agent (e.g., "Personal Assistant"). |
| **Node** | **MCP Server / Agent** | A running entity. Can be an MCP Server providing tool services, or a standalone Agent with its own context (e.g. Zeroclaw). |
| **Topic** | **Config / Identity / Skill / Stream** | Async data flux. E.g. Loading Config/Identity on startup, or a Web UI (a Node) publishing user input. |
| **Service** | **Tool Call (In Progress)** | Synchronous RPC. Typically used for high-frequency MCP tool calls (e.g., reading files, DB queries). |
| **Bringup Pkg** | **Config Pkg** | Handles the startup parameters of the entire system, customizing sub-agents' skills, permissions, and runtime vars. |
| **Library Pkg** | **Pure Prompt / Code** | Contains code libraries or Skills without starting an independent node. |
| **Msg/Srv Pkg** | *(Simplified)* | Tagentacle embraces **JSON First**, eliminating the need for pre-defined IDL message packages. |

#### Use Case Mapping
*   **Node (Runtime Entity)**: 
    - **MCP Server**: Executes specific tools (e.g. SQLite, Filesystem) as a decoupled service.
    - **Single Agent**: A brain with its own reasoning loop and context (e.g., **Zeroclaw**).
    - **Web UI**: An interface node that interacts with the bus.
*   **Topic (Asynchronous Channel)**:
    - **Skill/Identity**: Agent broadcasts capabilities or identity proof (similar to `/camera/parameters`).
    - **User Interaction**: Web UI publishes user messages to a topic; Agents subscribe and respond.
*   **Bringup Pkg**: A meta-package containing configurations (e.g. `launch.yaml`) that define which agents to pull, their permission levels, and API key mappings at runtime.

### Communication Flow
- **Topic (Pub/Sub)**: Used for real-time broadcasts, timeline updates, and streaming output (e.g., LLM response streams).
- **Service (Req/Res) (In Progress)**: Used for fast tool calling (e.g., MCP tool execution).
- **Action (In Progress)**: Designed for long-running asynchronous tasks with progress feedback.

---

## 📜 Original Communication Protocol

The Tagentacle Daemon listens on `TCP 19999` by default. All communication uses newline-delimited JSON strings (JSON Lines).

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
- `tagentacle run <pkg_name>`: Launches a Node within the workspace.
- `tagentacle launch <config.yaml>`: Orchestrates multiple Nodes simultaneously.
- `tagentacle topic list/echo`: Interrogates the bus for debugging.

---

## 📝 Roadmap & Todo

- [x] **Core Service Support**: Implement `advertise_service` and `call_service` in the Rust Daemon.
- [x] **Service API in SDK**: Add `node.call_service()` and `@node.service()` support to `tagentacle-py`.
- [ ] **CLI Implementation**: Complete major commands (`run`, `launch`, `list`, `echo`) in the `tagentacle` binary.
- [ ] **MCP Bridge**: Built-in `tagentacle bridge --mcp` to map standard MCP servers into the bus.
- [ ] **Action Mode**: Implement long-running task patterns with feedback loops.
- [ ] **Web Dashboard**: A visualizer for topics, nodes, and message flow.

---

## 🚀 Getting Started

1. **Start the Daemon** (Rust Core):
   ```bash
   cd tagentacle-core
   cargo run
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
