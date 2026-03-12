# Tagentacle：The ROS of AI Agents 🐙

[![GitHub](https://img.shields.io/badge/GitHub-Tagentacle-blue)](https://github.com/Tagentacle/tagentacle)
[![Docs](https://img.shields.io/badge/Docs-tagentacle.github.io-green)](https://tagentacle.github.io/docs/)

**Tagentacle** 是一个去中心化、配置中心化、生产级可用的多智能体框架。它深度引入 **ROS 2**（Robot Operating System）的软件组织模式，结合现代 AI 生态（MCP 协议），为大语言模型多智能体协同提供工业级基础设施。

> **Everything is a Pkg. Managed. Verifiable. Scalable.**

📖 **[完整文档](https://tagentacle.github.io/docs/zh/)** · 📖 **[English Docs](https://tagentacle.github.io/docs/)** · 📖 **[English README](README.md)**

---

## 🌟 核心哲学

Tagentacle 继承了 ROS 2 最根本的软件组织哲学：**将系统功能彻底模块化**。

- **Agent Pkg** — 每个智能体作为独立包，包含行为逻辑、Prompt 模板、状态管理和通信接口。
- **Tool/Service Pkg** — 封装智能体调用的工具（数据库、爬虫等），支持 MCP 协议插件化。
- **Interface Pkg** — 跨节点 JSON Schema 消息契约，确保互操作性。
- **Bringup Pkg** — 系统启动配置、拓扑编排与凭据注入。

核心优势：**高度可重用**（乐高积木式迁移）、**依赖隔离**（逐包 `.venv`）、**黑盒开发**（只关注 I/O 契约）。

→ [详细了解：设计哲学](https://tagentacle.github.io/docs/zh/concepts/philosophy/)

---

## ⚔️ 为什么选择 Tagentacle？

| 特性 | 单体网关 | 命令行工具 (如 Claude Code) | **Tagentacle** |
| :--- | :--- | :--- | :--- |
| **架构** | Node.js 单体 | 单进程 CLI | **分布式微服务 (Rust)** |
| **拓扑** | 星型 (一个中心) | 树状调用栈 (主→子) | **网状 Mesh (Pub/Sub)** |
| **稳定性** | 一处崩溃全量掉线 | 随进程终止 | **进程隔离 (容错)** |
| **生命周期** | 绑定聊天窗口 | 任务型 (一次性) | **持续运行 (24/7 事件驱动)** |
| **组件角色** | 技能 (绑定宿主) | 插件 (从属) | **独立微服务 (对等节点)** |
| **覆盖范围** | 单一服务器 | 本地文件系统 | **跨设备 / 跨平台** |

Tagentacle 不是另一个 Claude Code，**它是管理无数个 "Claude 级别智能体" 的基础设施**。

→ [竞品对比与深度分析](https://tagentacle.github.io/docs/zh/concepts/why-tagentacle/)

---

## 🏗️ 系统架构

```
┌─────────────────────────────────────────────────────┐
│  应用层 (Node, Agent, MCP Server)                     │  ← 迭代最快
├─────────────────────────────────────────────────────┤
│  Tagentacle（领域 Shell / 中间件）                     │  ← 领域语义
│  Topic Pub/Sub, Service RPC, /mcp/directory,         │
│  节点身份, 生命周期, TACL, Launch                      │
├─────────────────────────────────────────────────────┤
│  Linux（内核 + Podman/Docker）                         │  ← 「物理定律」
│  socket, namespace, cgroup, filesystem, signals      │
└─────────────────────────────────────────────────────┘
```

三大支柱：

1. **`tagentacle` (Rust)** — 高性能消息 Daemon/Broker + CLI 工具。
2. **`tagentacle-py-core` / `tagentacle-py-mcp` (Python)** — 官方双层 SDK（Simple API + LifecycleNode）。
3. **生态包** — `example-agent`、`example-inference`、`example-memory`、`example-frontend`、`example-mcp-server`、`mcp-gateway`、`container-orchestrator`、`shell-server`。

→ [架构详解](https://tagentacle.github.io/docs/zh/concepts/architecture/) · [Agent 架构](https://tagentacle.github.io/docs/zh/concepts/agent-architecture/)

---

## 🚀 快速开始

```bash
# 1. 从源码安装
cd tagentacle && cargo install --path .

# 2. 创建工作空间并克隆 bringup
mkdir -p my_workspace/src && cd my_workspace/src
git clone https://github.com/Tagentacle/example-bringup.git

# 3. 安装所有依赖（自动克隆仓库 + uv sync）
cd .. && tagentacle setup dep --all src

# 4. 启动守护进程（另开终端）
tagentacle daemon

# 5. 启动系统
tagentacle launch src/example-bringup/launch/system_launch.toml
```

→ [完整入门指南](https://tagentacle.github.io/docs/zh/getting-started/)

---

## 📚 文档导航

| 主题 | 链接 |
|------|------|
| **快速入门** | [安装、工作空间配置、首次启动](https://tagentacle.github.io/docs/zh/getting-started/) |
| **核心概念** | [设计哲学](https://tagentacle.github.io/docs/zh/concepts/philosophy/) · [系统架构](https://tagentacle.github.io/docs/zh/concepts/architecture/) · [Agent 架构](https://tagentacle.github.io/docs/zh/concepts/agent-architecture/) · [为什么选择 Tagentacle](https://tagentacle.github.io/docs/zh/concepts/why-tagentacle/) |
| **使用指南** | [Python SDK](https://tagentacle.github.io/docs/zh/guides/python-sdk/) · [MCP 集成](https://tagentacle.github.io/docs/zh/guides/mcp-integration/) · [TACL 认证](https://tagentacle.github.io/docs/zh/guides/tacl/) · [容器化](https://tagentacle.github.io/docs/zh/guides/containers/) · [最佳实践](https://tagentacle.github.io/docs/zh/guides/best-practices/) |
| **参考手册** | [通信协议 (TCP JSON Lines)](https://tagentacle.github.io/docs/zh/reference/protocol/) · [Topics & Services](https://tagentacle.github.io/docs/zh/reference/topics-services/) · [CLI](https://tagentacle.github.io/docs/zh/reference/cli/) |
| **路线图** | [已完成与计划中功能](https://tagentacle.github.io/docs/zh/roadmap/) |

---

## 📝 路线图摘要

### ✅ 已完成
- Rust Daemon（Topic Pub/Sub + Service RPC + 节点注册/心跳）
- Python SDK 双层 API（`Node` + `LifecycleNode`）
- MCPServerNode 基类（自动 Streamable HTTP + `/mcp/directory`）
- MCP Gateway（stdio→HTTP 中继）
- TACL（JWT 认证 + `space` 容器绑定）
- JSON Schema 校验（`SchemaRegistry`）
- Container Orchestrator + Shell Server 生态包
- CLI 工具链（`daemon`、`run`、`launch`、`setup dep`、`doctor`）

### 🔜 计划中
- SDK 自动日志发布到 `/tagentacle/log`
- 展平 Topic 工具 API
- Interface Package（跨节点 Schema 契约包）
- Action 模式（长程异步任务 + 进度反馈）
- Parameter Server
- Web Dashboard

→ [完整路线图](https://tagentacle.github.io/docs/zh/roadmap/)

---

## 🤝 参与贡献

欢迎贡献！各组件变更日志：
- [tagentacle (Rust 核心)](CHANGELOG.md)
- [python-sdk-core](https://github.com/Tagentacle/python-sdk-core/blob/main/CHANGELOG.md)
- [python-sdk-mcp](https://github.com/Tagentacle/python-sdk-mcp/blob/main/CHANGELOG.md)

## 📄 许可证

MIT
