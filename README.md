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

## 🏗️ Architecture

The system consists of three main pillars:

1.  **`tagentacle` (Rust)**: High-performance message router (Daemon/Broker) and CLI tools. (Core repo: `tagentacle-core`)
2.  **`tagentacle-py` (Python)**: Official Python SDK (similar to ROS's `rclpy`), providing a minimalist async API for developers.
3.  **`tagentacle-ecosystem` (Planned)**: A collection of standard Pkgs (e.g., `chatbot_ui_pkg`, `mcp_sqlite_wrapper_pkg`, `alice_agent_pkg`).

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

- [ ] **Core Service Support**: Implement `advertise_service` and `call_service` in the Rust Daemon.
- [ ] **Service API in SDK**: Add `node.call_service()` and `@node.service()` support to `tagentacle-py`.
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
