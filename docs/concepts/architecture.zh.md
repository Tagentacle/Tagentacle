# 系统架构

本项目由三个核心部分组成：

1.  **`tagentacle` (Rust)**：高性能消息路由器 (Daemon/Broker) 与命令行工具。
2.  **`tagentacle-py` (Python)**：官方 Python SDK (类比 ROS 的 `rclpy`)，提供双层异步 API。
3.  **`tagentacle-ecosystem` (成长中)**：官方示例 Pkg 集合，包含完整聊天机器人系统（`example-agent`、`example-inference`、`example-memory`、`example-frontend`、`example-mcp-server`、`example-bringup`）。

## ROS 2 概念映射

| ROS 2 概念 | Tagentacle 映射 | AI 场景说明 |
| :--- | :--- | :--- |
| **Workspace** | **Agent 工作空间** | 包含多个 Pkg 的目录，代表一个复杂智能体系统（如"私人助理"）。 |
| **Node** | **Agent Node / General Node** | 运行实体。**智能体节点** 由 LLM 驱动，具备自主决策；**一般节点** 运行确定性逻辑（监控、硬件接口）。 |
| **Topic** | **带 Schema 校验的通道** | 异步数据通道，**必须关联 JSON Schema**。不符合格式的"幻觉输出"在入口处即被拦截。 |
| **Service** | **工具调用 (RPC)** | 同步 RPC。用于高频 MCP 工具调用（读写文件、查询数据库）。 |
| **Interface Pkg** | **JSON Schema 契约包** | 专门定义跨节点消息契约，确保互操作性。 |
| **Bringup Pkg** | **配置中心** | 拓扑编排、参数注入（API_KEY、Base_URL、工具允许列表）、节点启动配置。 |
| **Library Pkg** | **纯提示词 / 代码库** | 包含代码库或 Skills，不启动独立节点。 |

## 包管理与编排

### `tagentacle.toml`：轻量级元数据声明

每个 Pkg 根目录必须包含此清单文件：

```toml
[package]
name = "alice_agent"
version = "0.1.0"
description = "一个对话式 AI 智能体"
authors = ["dev@example.com"]

[entry_points]
node = "main:AliceNode"  # 导出的 Node 类，便于 CLI 自动加载

[dependencies]
python = ["openai", "tagentacle-py>=0.1.0"]
```

### Bringup：中心化配置与拓扑管控

Bringup Package 不仅是启动脚本，更是系统的"配置中心"：

*   **拓扑编排**：通过配置文件声明系统由哪些节点组成。
*   **参数注入**：启动时动态分发 API_KEY、Base_URL 和"工具允许列表"等敏感或易变配置。

## 通信流

- **Topic (Pub/Sub)**：实时广播、时间线更新、流式输出（如 LLM 打字机效果）。**经 JSON Schema 校验。**
- **Service (Req/Res)**：节点间 RPC 调用（如 Agent 调用 Inference Node 获取 completion）。
- **MCP (Streamable HTTP)**：Agent 通过原生 MCP SDK HTTP Client 直连 MCP Server，工具调用不经过总线。总线仅承担服务发现（`/mcp/directory` Topic）。
- **Action (计划中)**：长程异步任务，支持进度反馈。

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

真正的风险不是「AI 跳过 Tagentacle 直接用 Linux」，而是「另一个框架提供了更好的语义压缩」。这就是为什么 Daemon 应该只实现**机制 (mechanism)**（IPC 路由、进程启动、容器生命周期），永远不实现**策略 (policy)**（如何编排、如何调度）—— 机制是稳定的，策略随竞争迭代。
