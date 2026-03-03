# Tagentacle：The ROS of AI Agents 🐙

**Tagentacle** 是一个去中心化、配置中心化、生产级可用的多智能体框架。它深度引入 **ROS 2**（Robot Operating System）的软件组织模式，结合现代 AI 生态（MCP 协议）与动态 Schema 技术，为大语言模型（LLM）多智能体协同提供工业级基础设施。

> **Everything is a Pkg. Managed. Verifiable. Scalable.**

---

## 快速导航

<div class="grid cards" markdown>

- 🚀 **[快速开始](getting-started.md)** — 安装并运行第一个工作区
- 💡 **[设计哲学](concepts/philosophy.md)** — 核心设计理念
- ⚔️ **[为什么选择 Tagentacle](concepts/why-tagentacle.md)** — 与替代方案的对比
- 🏗️ **[系统架构](concepts/architecture.md)** — 系统总览
- 🤖 **[Agent 架构](concepts/agent-architecture.md)** — IO + Inference 分离
- 🐍 **[Python SDK](guides/python-sdk.md)** — 双层 API 设计
- 🔌 **[MCP 集成](guides/mcp-integration.md)** — 本地会话 + 直接 HTTP
- 🔐 **[TACL 访问控制](guides/tacl.md)** — JWT 权限管理
- 📋 **[最佳实践](guides/best-practices.md)** — 推荐模式
- 🐳 **[容器化](guides/containers.md)** — 容器化部署

</div>

## 架构层次

```
┌─────────────────────────────────────────────────────┐
│  应用层 (Node, Agent, MCP Server)                     │  ← 迭代最快
├─────────────────────────────────────────────────────┤
│  Tagentacle（领域 Shell / 中间件）                     │  ← 领域语义
│  Topic Pub/Sub, Service RPC, /mcp/directory,         │    （语法糖，但有真正的
│  节点身份, 生命周期, TACL, Launch                      │     工程价值）
├─────────────────────────────────────────────────────┤
│  Linux（内核 + Docker）                               │  ← 「物理定律」
│  socket, namespace, cgroup, filesystem, signals      │
└─────────────────────────────────────────────────────┘
```
