# Tagentacle: The ROS of AI Agents 🐙

[![GitHub](https://img.shields.io/badge/GitHub-Tagentacle-blue)](https://github.com/Tagentacle/tagentacle)
[![Docs](https://img.shields.io/badge/Docs-tagentacle.github.io-green)](https://tagentacle.github.io/docs/)

**Tagentacle** is a decentralized, configuration-centralized, production-grade multi-agent framework. It deeply adopts the software organization patterns of **ROS 2** (Robot Operating System), combined with the modern AI ecosystem (MCP Protocol), to build robust infrastructure for LLM multi-agent collaboration.

> **Everything is a Pkg. Managed. Verifiable. Scalable.**

📖 **[Full Documentation](https://tagentacle.github.io/docs/)** · 📖 **[中文文档](https://tagentacle.github.io/docs/zh/)** · 📖 **[中文 README](README_CN.md)**

---

## 🌟 Core Philosophy

Tagentacle inherits the most fundamental philosophy from ROS 2: **thorough modularization of system capabilities**.

- **Agent Pkg** — Each agent as an independent package with behavior logic, prompts, state, and communication interfaces.
- **Tool/Service Pkg** — Encapsulates tools agents call (DB, web scraping, etc.), supporting MCP protocol plug-and-play.
- **Interface Pkg** — Cross-node JSON Schema contracts ensuring interoperability.
- **Bringup Pkg** — System startup configuration, topology orchestration, and credential injection.

Key advantages: **High reusability** (LEGO-like migration), **dependency isolation** (per-package `.venv`), **black-box development** (focus on I/O contracts only).

→ [Learn more: Philosophy](https://tagentacle.github.io/docs/concepts/philosophy/)

---

## ⚔️ Why Tagentacle?

| Feature | Monolithic Gateways | CLI Tools (e.g., Claude Code) | **Tagentacle** |
| :--- | :--- | :--- | :--- |
| **Architecture** | Monolithic (Node.js) | Single-Process CLI | **Distributed Microservices (Rust)** |
| **Topology** | Star (single hub) | Tree call-stack (main→sub) | **Mesh (Pub/Sub)** |
| **Stability** | Single failure = full crash | Process-bound | **Isolated Processes (Fault Tolerant)** |
| **Lifecycle** | Chat-bound | Task-bound (one-shot) | **Continuous (24/7 Event-Driven)** |
| **Component Role** | Skills (bound to host) | Plugin (subordinate) | **Independent microservices (Peers)** |
| **Scope** | Single server | Local filesystem | **Multi-Device / Cross-Platform** |

Tagentacle is not another Claude Code. **It is the infrastructure for managing countless "Claude-grade agents."**

→ [Full comparison & deep analysis](https://tagentacle.github.io/docs/concepts/why-tagentacle/)

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│  Application Layer (Nodes, Agents, MCP Servers)      │  ← iterates fastest
├─────────────────────────────────────────────────────┤
│  Tagentacle (Domain Shell / Middleware)               │  ← domain semantics
│  Topic Pub/Sub, Service RPC, /mcp/directory,         │
│  Node identity, Lifecycle, TACL, Launch              │
├─────────────────────────────────────────────────────┤
│  Linux (Kernel + Docker)                             │  ← "laws of physics"
│  socket, namespace, cgroup, filesystem, signals      │
└─────────────────────────────────────────────────────┘
```

Three pillars:

1. **`tagentacle` (Rust)** — High-performance message Daemon/Broker + CLI tools.
2. **`tagentacle-py-core` / `tagentacle-py-mcp` (Python)** — Official dual-layer SDK (Simple API + LifecycleNode).
3. **Ecosystem Pkgs** — `example-agent`, `example-inference`, `example-memory`, `example-frontend`, `example-mcp-server`, `mcp-gateway`, `container-orchestrator`, `shell-server`.

→ [Architecture details](https://tagentacle.github.io/docs/concepts/architecture/) · [Agent architecture](https://tagentacle.github.io/docs/concepts/agent-architecture/)

---

## 🚀 Quick Start

```bash
# 1. Install from source
cd tagentacle && cargo install --path .

# 2. Create workspace & clone bringup
mkdir -p my_workspace/src && cd my_workspace/src
git clone https://github.com/Tagentacle/example-bringup.git

# 3. Setup all dependencies (auto-clones repos + uv sync)
cd .. && tagentacle setup dep --all src

# 4. Start daemon (separate terminal)
tagentacle daemon

# 5. Launch the system
tagentacle launch src/example-bringup/launch/system_launch.toml
```

→ [Full getting started guide](https://tagentacle.github.io/docs/getting-started/)

---

## 📚 Documentation

| Topic | Link |
|-------|------|
| **Getting Started** | [Installation, workspace setup, first launch](https://tagentacle.github.io/docs/getting-started/) |
| **Core Concepts** | [Philosophy](https://tagentacle.github.io/docs/concepts/philosophy/) · [Architecture](https://tagentacle.github.io/docs/concepts/architecture/) · [Agent Architecture](https://tagentacle.github.io/docs/concepts/agent-architecture/) · [Why Tagentacle](https://tagentacle.github.io/docs/concepts/why-tagentacle/) |
| **Guides** | [Python SDK](https://tagentacle.github.io/docs/guides/python-sdk/) · [MCP Integration](https://tagentacle.github.io/docs/guides/mcp-integration/) · [TACL Auth](https://tagentacle.github.io/docs/guides/tacl/) · [Containers](https://tagentacle.github.io/docs/guides/containers/) · [Best Practices](https://tagentacle.github.io/docs/guides/best-practices/) |
| **Reference** | [Protocol (TCP JSON Lines)](https://tagentacle.github.io/docs/reference/protocol/) · [Topics & Services](https://tagentacle.github.io/docs/reference/topics-services/) · [CLI](https://tagentacle.github.io/docs/reference/cli/) |
| **Roadmap** | [Completed & planned features](https://tagentacle.github.io/docs/roadmap/) |

---

## 📝 Roadmap Highlights

### ✅ Completed
- Rust Daemon (Topic Pub/Sub + Service RPC + node registration/heartbeat)
- Python SDK dual-layer API (`Node` + `LifecycleNode`)
- MCPServerNode base class (auto Streamable HTTP + `/mcp/directory`)
- MCP Gateway (stdio→HTTP relay)
- TACL (JWT auth with `space` claim for container binding)
- JSON Schema validation (`SchemaRegistry`)
- Container Orchestrator + Shell Server ecosystem pkgs
- CLI toolchain (`daemon`, `run`, `launch`, `setup dep`, `doctor`)

### 🔜 Planned
- SDK auto-logging to `/tagentacle/log`
- Flattened Topic Tools API
- Interface Packages (cross-node schema contracts)
- Action Mode (long-running tasks with progress)
- Parameter Server
- Web Dashboard

→ [Full roadmap](https://tagentacle.github.io/docs/roadmap/)

---

## 🤝 Contributing

We welcome contributions! See our CHANGELOG files for recent changes:
- [tagentacle (Rust core)](CHANGELOG.md)
- [python-sdk-core](https://github.com/Tagentacle/python-sdk-core/blob/main/CHANGELOG.md)
- [python-sdk-mcp](https://github.com/Tagentacle/python-sdk-mcp/blob/main/CHANGELOG.md)

## 📄 License

MIT
