# System Architecture

The system consists of three main pillars:

1.  **`tagentacle` (Rust)**: High-performance message router (Daemon/Broker) and CLI tools.
2.  **`tagentacle-py` (Python)**: Official Python SDK (similar to ROS's `rclpy`), providing a dual-layer async API.
3.  **`tagentacle-ecosystem` (Growing)**: Official example Pkg collection, including a complete chatbot system (`example-agent`, `example-inference`, `example-memory`, `example-frontend`, `example-mcp-server`, `example-bringup`).

## The ROS 2 Analogy

| ROS 2 Concept | Tagentacle Mapping | AI Scenario Description |
| :--- | :--- | :--- |
| **Workspace** | **Agent Workspace** | A directory containing multiple Pkgs, representing a complex agent system (e.g., "Personal Assistant"). |
| **Node** | **Agent Node / General Node** | Runtime entity. **Agent Nodes** are LLM-driven with autonomous decision-making; **General Nodes** are deterministic programs (e.g., monitoring, hardware interfaces). |
| **Topic** | **Schema-Validated Channel** | Async data channel with **mandatory JSON Schema** contracts. Non-conforming "hallucination outputs" are rejected at the entry point. |
| **Service** | **Tool Call (RPC)** | Synchronous RPC. Used for high-frequency MCP tool calls (e.g., reading files, DB queries). |
| **Interface Pkg** | **JSON Schema Contract Pkg** | Dedicated package defining cross-node message contracts, ensuring interoperability. |
| **Bringup Pkg** | **Config Center** | Topology orchestration, parameter injection (API_KEY, Base_URL, tool allow-lists), and node launch configuration. |
| **Library Pkg** | **Pure Prompt / Code** | Contains code libraries or Skills without starting an independent node. |

## Package Management & Orchestration

### `tagentacle.toml`: Lightweight Metadata Declaration

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

### Bringup: Centralized Configuration & Topology Control

The Bringup Package serves as the system's "configuration center":

*   **Topology Orchestration**: Declare which nodes compose the system via configuration files.
*   **Parameter Injection**: Dynamically distribute API_KEY, Base_URL, and "tool allow-lists" at launch time.

## Communication Flow

- **Topic (Pub/Sub)**: Real-time broadcasts, timeline updates, and streaming output (e.g., LLM response streams). **Validated against JSON Schema.**
- **Service (Req/Res)**: Inter-node RPC calls (e.g., Agent calling Inference Node for completion).
- **MCP (Streamable HTTP)**: Agents connect directly to MCP Servers via native MCP SDK HTTP client; tool calls bypass the bus. The bus only handles service discovery (`/mcp/directory` Topic).
- **Action (Planned)**: Long-running asynchronous tasks with progress feedback.

## The Architecture Stack

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
