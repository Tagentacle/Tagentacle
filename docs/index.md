# Tagentacle: The ROS of AI Agents 🐙

**Tagentacle** is a decentralized, configuration-centralized, production-grade multi-agent framework. It deeply adopts the software organization patterns of **ROS 2** (Robot Operating System), combined with the modern AI ecosystem (MCP Protocol) and dynamic Schema technologies, to build a robust infrastructure for LLM multi-agent collaboration.

> **Everything is a Pkg. Managed. Verifiable. Scalable.**

---

## Quick Links

<div class="grid cards" markdown>

- 🚀 **[Getting Started](getting-started.md)** — Install and run your first workspace
- 💡 **[Philosophy](concepts/philosophy.md)** — Core design principles
- ⚔️ **[Why Tagentacle](concepts/why-tagentacle.md)** — Comparison with alternatives
- 🏗️ **[Architecture](concepts/architecture.md)** — System overview
- 🤖 **[Agent Architecture](concepts/agent-architecture.md)** — IO + Inference separation
- 🐍 **[Python SDK](guides/python-sdk.md)** — Dual-layer API
- 🔌 **[MCP Integration](guides/mcp-integration.md)** — Local sessions + Direct HTTP
- 🔐 **[TACL](guides/tacl.md)** — JWT access control
- 📋 **[Best Practices](guides/best-practices.md)** — Recommended patterns
- 🐳 **[Containers](guides/containers.md)** — Container-ready deployment

</div>

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
