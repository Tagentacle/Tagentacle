# Tagentacle：The ROS of AI Agents 🐙

[![GitHub](https://img.shields.io/badge/GitHub-Tagentacle-blue)](https://github.com/Tagentacle/tagentacle)

**Tagentacle** 是一个去中心化、配置中心化、生产级可用的多智能体框架。它深度引入 **ROS 2**（Robot Operating System）的软件组织模式，结合现代 AI 生态（MCP 协议）与动态 Schema 技术，为大语言模型（LLM）多智能体协同提供工业级基础设施。

> **Everything is a Pkg. Managed. Verifiable. Scalable.**

---

## 🌟 核心哲学：一切皆包 (Everything is a Pkg)

Tagentacle 继承了 ROS 2 最根本的软件组织哲学：**将系统功能彻底模块化**。这种哲学在抽象智能体组件时展现出极佳的工程性质：

- **Agent Package**：每个智能体作为独立的包，包含其行为逻辑、Prompt 模板、状态管理和通信接口。
- **Tool/Service Package**：封装智能体需要调用的工具或服务（如数据库访问、网页爬取），支持 MCP 协议实现插件化。
- **Interface Package**：专门定义跨节点通信的消息契约（JSON Schema），确保不同开发者编写的包能"说同一种语言"。
- **Bringup Package**：负责系统的启动和配置，定义 Workspace 中的 Package 集合、各节点的启动参数、以及环境凭据。

核心优势：
*   **高度可重用性**：成熟的工具包或智能体包可以像乐高积木一样在不同项目中无缝迁移。
*   **版本与依赖隔离**：借鉴 ROS 2 的隔离机制，为每个 Package 自动管理独立的 Python 虚拟环境，彻底解决依赖冲突。
*   **黑盒开发模式**：开发者只需关注包的输入输出契约，无需关心内部实现是基于何种模型或框架。

---

## ⚔️ 为什么选择 Tagentacle？

在单体网关和 CLI 工具统治的当下，Tagentacle 为下一代 AI 提供了"工业级"的基础设施。

| 特性 | 单体网关 (如 OpenClaw) | 命令行工具 (如 Claude Code) | **Tagentacle** |
| :--- | :--- | :--- | :--- |
| **架构** | Node.js 单体巨石 | 单进程 CLI | **分布式微服务 (Rust)** |
| **拓扑** | 星型 (一个中心) | 树状调用栈 (主→子) | **网状 Mesh (Pub/Sub)** |
| **稳定性** | 一处崩溃，全量掉线 | 随进程结束而终止 | **进程隔离 (容错性极强)** |
| **生命周期** | 绑定聊天窗口 (TG/WA) | 任务型 (一次性) | **持续运行 (24/7 事件驱动)** |
| **交互模式** | 聊天气泡 (ChatOps) | 终端输出 | **任务指挥中心 (实时仪表盘)** |
| **组件角色** | 技能 (绑定宿主) | 插件/Sub-Agent (从属于主 Agent) | **独立微服务 (对等节点)** |
| **覆盖范围** | 单一服务器 | 本地文件系统 | **跨设备 / 跨平台协同** |

### 1. 鲁棒性：分布式总线 vs 单体网关
*   **单体软肋**：在一个 Node.js 进程里跑 50 个技能，任何一个技能的内存溢出都导致整个系统重启。
*   **Tagentacle 的绝对隔离**：每一个 Node 都是独立进程。"推特爬虫"崩溃不影响"运维 Agent"。Rust 编写的 Broker 提供永不掉线的高并发消息路由。

### 2. 自主性：事件驱动生命体 vs 任务型工具
*   **超越请求-响应**：CLI 工具只有在你输入指令时才动作，命令结束就"死"了。
*   **活着的系统**：Tagentacle Agent 24 小时在线。凌晨 3 点监控日志、发现异常、触发告警 Agent 并自主修复——它不是工具，而是**数字生命体**。

### 3. 专业级：任务指挥中心 vs 聊天气泡
*   **态大于流**：目前的框架大多被迫将信息挤进 Telegram 或 WhatsApp。
*   **专业可视化**：Tagentacle 暴露原生数据总线，允许构建"任务指挥中心"——实时 Agent 拓扑、CPU 波形图、代码流——由同一套消息总线驱动。

### 4. 架构分水岭：网状拓扑 vs 树状调用栈

这是 Tagentacle 与 Claude Code 插件/Sub-Agent 模式之间最深层的 _架构鸿沟_。

#### 世界观差异

| | **Claude Code (以项目为中心)** | **Tagentacle (以系统为中心)** |
| :--- | :--- | :--- |
| **宇宙** | 当前 Git 仓库 | 运行中的多实体环境 |
| **`.claude.md` / `tagentacle.toml`** | 项目的"物理法则" | 每个微服务的身份证 |
| **Plugin / Pkg** | 挂载在项目上的工具 | 独立存活的软件实体 |
| **Sub-Agent / Node** | 主 Agent 的外包小弟 | 对等的网络公民 |
| **AI 是什么** | Guest（服务于代码库的客人） | Host（管理一切的操作系统） |

#### 拓扑结构：树 vs 网

Claude Code 即便引入了 Sub-Agent（独立系统提示词、独立工具集、物理隔离上下文），其控制流依然是 **树状调用栈 (Call Stack)**：

```
用户 ──▶ 主 Claude ──▶ SQL Agent ──▶ 数据库工具
                    └──▶ 前端 Agent ──▶ 文件系统

控制流自上而下，结果必须原路返回。
SQL Agent 无法主动联系前端 Agent。
```

Tagentacle 是**网状拓扑 (Mesh)**：

```
┌──────────────┐     /social/alerts     ┌──────────────┐
│ 爬虫 Node    │ ── publish ──────────▶ │ 分析师 Node  │
└──────────────┘                        └──────┬───────┘
                                               │ /pr/drafts
┌──────────────┐                               ▼
│ 仪表盘 Node  │ ◀── subscribe ──── ┌──────────────┐
│ (旁路观测)   │ ◀── /mcp/traffic   │ 公关 Node    │
└──────────────┘                    └──────────────┘
                                         │ call_service
                                         ▼
                                   ┌──────────────┐
                                   │ 邮件服务 Node │
                                   └──────────────┘

没有主角。任何节点可以发布到任何 Topic，
订阅任何 Topic，调用任何 Service。
仪表盘节点可以越过所有 Agent 直接旁听底层流量。
```

#### 三道不可逾越的鸿沟

| 维度 | Claude Code 插件 / Sub-Agent | Tagentacle Node |
| :--- | :--- | :--- |
| **拓扑** | 树状 (调用栈，结果必须原路返回) | 网状 (Pub/Sub，任意节点互通) |
| **生命周期** | 短暂 (依附于一轮对话，对话结束即休眠) | 持续守护 (独立进程，24/7 事件驱动) |
| **依赖关系** | 组件从属于宿主项目 (Guest) | 组件是对等的独立微服务 (Peer) |

#### 场景适配

*   **"帮我修这个 React 项目的 Bug"** → **Claude Code 最佳**。AI 是 Guest，代码库是宇宙，`.claude.md` 提供上下文，Sub-Agent 分工查库/写码，一切顺滑。
*   **"24 小时舆情监控：爬虫→分析师→公关→自动发邮件"** → **Tagentacle 不可替代**。事件驱动、持续运行、节点独立崩溃/重启、无中心主脑——这是树状调用栈根本无法表达的架构。

Tagentacle 不是另一个 Claude Code，**它是管理无数个 "Claude 级别智能体" 的基础设施**。

---

## 🏗️ 系统架构

本项目由三个核心部分组成：

1.  **`tagentacle` (Rust)**：高性能消息路由器 (Daemon/Broker) 与命令行工具。
2.  **`tagentacle-py` (Python)**：官方 Python SDK (类比 ROS 的 `rclpy`)，提供双层异步 API。
3.  **`tagentacle-ecosystem` (成长中)**：官方示例 Pkg 集合，包含完整聊天机器人系统（`example-agent`、`example-inference`、`example-memory`、`example-frontend`、`example-mcp-server`、`example-bringup`）。

### 🧩 ROS 2 概念映射

| ROS 2 概念 | Tagentacle 映射 | AI 场景说明 |
| :--- | :--- | :--- |
| **Workspace** | **Agent 工作空间** | 包含多个 Pkg 的目录，代表一个复杂智能体系统（如"私人助理"）。 |
| **Node** | **Agent Node / General Node** | 运行实体。**智能体节点** 由 LLM 驱动，具备自主决策；**一般节点** 运行确定性逻辑（监控、硬件接口）。 |
| **Topic** | **带 Schema 校验的通道** | 异步数据通道，**必须关联 JSON Schema**。不符合格式的"幻觉输出"在入口处即被拦截。 |
| **Service** | **工具调用 (RPC)** | 同步 RPC。用于高频 MCP 工具调用（读写文件、查询数据库）。 |
| **Interface Pkg** | **JSON Schema 契约包** | 专门定义跨节点消息契约，确保互操作性。 |
| **Bringup Pkg** | **配置中心** | 拓扑编排、参数注入（API_KEY、Base_URL、工具允许列表）、节点启动配置。 |
| **Library Pkg** | **纯提示词 / 代码库** | 包含代码库或 Skills，不启动独立节点。 |

### 📦 包管理与编排

#### `tagentacle.toml`：轻量级元数据声明
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

#### Bringup：中心化配置与拓扑管控
Bringup Package 不仅是启动脚本，更是系统的"配置中心"：
*   **拓扑编排**：通过配置文件声明系统由哪些节点组成。
*   **参数注入**：启动时动态分发 API_KEY、Base_URL 和"工具允许列表"等敏感或易变配置。

### 通信流

- **Topic (Pub/Sub)**：实时广播、时间线更新、流式输出（如 LLM 打字机效果）。**经 JSON Schema 校验。**
- **Service (Req/Res)**：节点间 RPC 调用（如 Agent 调用 Inference Node 获取 completion）。
- **MCP (Streamable HTTP)**：Agent 通过原生 MCP SDK HTTP Client 直连 MCP Server，工具调用不经过总线。总线仅承担服务发现（`/mcp/directory` Topic）。
- **Action (计划中)**：长程异步任务，支持进度反馈。

---

## 🔌 Python SDK：双层设计

### Simple API（适用于一般程序节点）
为已有软件快速接入总线提供简易接口——只需 `publish()` 和 `subscribe()`：
```python
from tagentacle_py import Node
import asyncio

async def main():
    node = Node("sensor_node")
    await node.connect()

    @node.subscribe("/data/temperature")
    async def on_temp(msg):
        print(f"温度: {msg['payload']['value']}°C")

    await node.publish("/status/online", {"node": "sensor_node"})
    await node.spin()

asyncio.run(main())
```

### Node API（适用于智能体节点，带生命周期管理）
完善的生命周期管理，支持 `on_configure`、`on_activate` 等钩子，适用于 CLI 启动并接受 Bringup 配置的节点：
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
        self.logger.info("Alice 正在优雅关闭。")
```

### 预制节点：TagentacleMCPServer
SDK 内置两个关键节点：
*   **TagentacleMCPServer**：将总线的 `publish`、`subscribe`、`call_service` 等能力暴露为标准 MCP Tool。继承 `MCPServerNode`，自行运行 Streamable HTTP 端点。
*   **MCPGatewayNode**：传输层中继 — 将仅支持 stdio 的传统 MCP Server 适配为 Streamable HTTP，发布远程服务器 URL 到 `/mcp/directory`。

---

## 🛠️ MCP 集成：本地会话 + HTTP 直连

借鉴 ROS 2 TF2 的设计理念，Tagentacle 将 MCP 会话管理完全本地化于 Agent 节点：

### 设计原则
*   **会话本地化**：MCP Client Session 保持在 Agent 节点内存中。Agent 通过原生 MCP SDK HTTP Client 直连 MCP Server 的 Streamable HTTP 端点。
*   **MCPServerNode 基类**：MCP Server 继承 `MCPServerNode`（LifecycleNode 子类），自动运行 Streamable HTTP 服务并在激活时向 `/mcp/directory` Topic 发布 `MCPServerDescription`。
*   **统一发现**：Agent 订阅 `/mcp/directory` Topic 即可自动发现所有可用 MCP Server（包括原生 HTTP Server 和 Gateway 代理的 stdio Server）。
*   **完整协议支持**：因 MCP 会话直接在 Agent ↔ Server 之间建立，所有 MCP 功能（sampling、notifications、resources 等）原生可用。
*   **MCP Gateway**：独立 `mcp-gateway` 包提供传输层 stdio→HTTP 中继，不解析 MCP 语义。

### Agent 侧连接示例
```python
from mcp import ClientSession
from mcp.client.streamable_http import streamable_http_client

# 通过 Streamable HTTP 直连 MCP Server
async with streamable_http_client("http://127.0.0.1:8100/mcp") as (r, w, _):
    async with ClientSession(r, w) as session:
        await session.initialize()
        result = await session.call_tool("query", {"sql": "SELECT * FROM users"})
```

### 双向调用与可观测性
- **双向调用**：因 MCP 会话直接在 Agent ↔ Server 之间建立（HTTP 长连接），完整支持 MCP 规范中的 **Sampling**（Server 反向调用 Agent）等双向能力。
- **透明观测**：MCP Server 的服务发现信息发布到 `/mcp/directory` Topic，任何节点可订阅获取系统中所有可用工具的实时视图。

---

## 📜 标准 Topic 与 Service

当 Daemon 启动时，会自动创建一组 `/tagentacle/` 命名空间下的**系统保留 Topic 和 Service** —— 类比 ROS 2 的 `/rosout`、`/parameter_events` 和节点内省服务。这些提供内置的可观测性、日志聚合和系统内省能力，无需用户侧任何配置。

### 保留命名空间约定

| 前缀 | 用途 | 管理者 |
|---|---|---|
| `/tagentacle/*` | **系统保留。** Daemon 与 SDK 核心功能 | 核心库 |
| `/mcp/*` | MCP 发现和网关服务 | MCPServerNode / Gateway |

用户自定义 Topic **不应**使用以上前缀。

### 标准 Topic（Daemon 管理）

| Topic | ROS 2 对应 | 说明 | 发布者 |
|---|---|---|---|
| `/tagentacle/log` | `/rosout` | 全局日志聚合。所有节点通过 SDK 自动发布日志；Daemon 也发布系统事件。 | SDK 节点（自动）+ Daemon |
| `/tagentacle/node_events` | 生命周期事件 | 节点生命周期事件：上线、下线、状态转换。支撑 Dashboard 实时拓扑图。 | Daemon（自动）+ `LifecycleNode`（自动）|
| `/tagentacle/diagnostics` | `/diagnostics` | 节点健康诊断：心跳、运行时长、消息计数、错误计数。 | SDK `Node.spin()`（定时）|
| `/mcp/directory` | _（无）_ | MCP 服务器发现。`MCPServerDescription` 由 MCP Server Node 和 Gateway 在激活时发布。Agent 订阅后自动发现服务器。 | MCPServerNode / Gateway |

### 标准 Service（Daemon 内置）

Daemon 以 `_daemon_` 作为内部节点 ID，直接从 Router 内部状态提供以下内省 Service：

| Service | ROS 2 对应 | 说明 |
|---|---|---|
| `/tagentacle/ping` | `ros2 doctor` | Daemon 健康检测。返回 `{status, uptime_s, version, node_count, topic_count}` |
| `/tagentacle/list_nodes` | `ros2 node list` | 返回所有已连接节点：`{nodes: [{node_id, connected_at}]}` |
| `/tagentacle/list_topics` | `ros2 topic list` | 返回所有活跃 Topic 及其订阅者：`{topics: [{name, subscribers}]}` |
| `/tagentacle/list_services` | `ros2 service list` | 返回所有已注册 Service：`{services: [{name, provider}]}` |
| `/tagentacle/get_node_info` | `ros2 node info` | 获取单个节点详情：`{node_id, subscriptions, services, connected_at}` |

可以直接通过 CLI 测试：
```bash
tagentacle service call /tagentacle/ping '{}'
tagentacle service call /tagentacle/list_nodes '{}'
tagentacle topic echo /tagentacle/log
tagentacle topic echo /tagentacle/node_events
```

### 日志消息格式 (`/tagentacle/log`)
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

### 节点事件格式 (`/tagentacle/node_events`)
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

## 🤖 Agent 架构：IO + Inference 分离

Tagentacle 采用 **Agent Node**（上下文工程 + agentic loop）与 **Inference Node**（无状态 LLM 网关）的分离设计：

### Agent Node = 完整的 Agentic Loop

Agent Node 是一个独立 Pkg，在内部完成整个 agentic loop：
- 订阅 Topic → 接收用户消息/事件通知
- 管理 context window（消息队列、上下文工程）
- 通过 Service RPC 调用 Inference Node 获取 completion
- 解析 `tool_calls` → 通过 MCP Session（Streamable HTTP 直连）执行工具 → 回填结果 → 再推理

这个 loop 是一个紧耦合的顺序控制流（类似 ROS 2 的 nav2 导航栈），**不应**被拆分到多个 Node 中。

### Inference Node = 无状态 LLM 网关

一个独立的 Pkg（官方示例，位于 org 级别，**非**核心库组成部分），提供：
- Service（如 `/inference/chat`），接受 OpenAI 兼容格式：`{model, messages, tools?, temperature?}`
- 返回标准 completion：`{choices: [{message: {role, content, tool_calls?}}]}`
- 多个 Agent Node 可并发调用同一个 Inference Node

### 数据流
```
UI Node ──publish──▶ /chat/input ──▶ Agent Node (agentic loop)
                                        │
                                        ├─ call_service("/inference/chat") ──▶ Inference Node ──▶ OpenRouter/OpenAI
                                        │                                           │
                                        │◀── completion (with tool_calls) ◀─────────┘
                                        │
                                        ├─ MCP Session (HTTP) ──▶ Tool Server Node
                                        │◀── tool result ◀────────┘
                                        │
                                        └─ publish ──▶ /chat/output ──▶ UI Node
```

---

## � 天然容器化架构

Tagentacle 的"一切皆 Pkg"哲学使其**天然适配容器化部署**。每个包都是独立进程、拥有独立依赖，完美适配一容器一包的部署模式。

### 为什么要容器化？

容器化对 **Agent Node** 尤其强大，赋予其前所未有的自由度：

- **Agent 的最大自由度**：每个 Agent 运行在独立容器中，拥有完全隔离的文件系统、网络栈和依赖树。Agent 可以自由 `pip install` 库、写文件、启动子进程，而不影响其他 Agent —— 真正的 AI 节点自治。
- **操作系统级故障隔离**：失控的 Agent（死循环、内存泄漏、对抗性工具输出）被 cgroup 限制约束。其他服务不受影响。
- **动态伸缩**：在负载均衡器后面启动多个 Inference Node 或 MCP Server 实例。Agent Node 可在运行时动态增删。
- **可复现部署**：每个包的 `Dockerfile` + `pyproject.toml` = 从开发笔记本到云集群的字节级一致环境。
- **安全边界**：处理不可信工具输出或用户输入的 Agent Node 在容器级别被沙箱化 —— 在 TACL JWT 认证之上提供纵深防御。

### 部署演进路径

```
开发阶段              → docker-compose           → K3s / K8s
─────────────────      ─────────────────────      ──────────────────
tagentacle daemon      tagentacle-core 容器       Deployment + Service
python 进程/包          一容器一包                  HPA 自动伸缩
localhost:19999        Docker bridge 网络          K8s Service DNS
bringup.py / launch    docker-compose.yml         Helm Chart
```

### 近零开销

容器**不是虚拟机** —— 它们通过 Linux namespaces 和 cgroups 共享宿主机内核：
- **CPU / 内存**：<1% 开销（原生执行，无 hypervisor）
- **网络**：bridge 模式 2–5% 开销；`host` 网络模式 = 零开销
- **磁盘 IO**：bind mount 卷 = 原生速度；OverlayFS 写入有约 5% 的写时复制开销
- **GPU**：通过 NVIDIA Container Toolkit 直通，零开销

对于 Tagentacle 的主要负载（WebSocket 总线消息 + LLM API 调用），容器网络开销（~50μs）相比推理延迟（~200ms+）完全可以忽略。

### 最小化代码改动

SDK 已经天然兼容容器化。唯一需要的改动是通过环境变量注入总线地址：

```python
import os
bus_host = os.environ.get("TAGENTACLE_BUS_HOST", "localhost")
bus_port = int(os.environ.get("TAGENTACLE_BUS_PORT", "19999"))
```

所有上层抽象 —— `Node`、`LifecycleNode`、`MCPServerNode`、TACL 认证、`/mcp/directory` 发现 —— 在容器内的行为与裸跑完全一致。

---

## �📜 通信协议规范

Tagentacle Daemon 默认监听 `TCP 19999` 端口。所有通信均为换行符分割的 JSON 字符串（JSON Lines）。

### 话题 (Topic)
*   **订阅**: `{"op": "subscribe", "topic": "/chat/global", "node_id": "alice_node"}`
*   **发布**: `{"op": "publish", "topic": "/chat/global", "sender": "bob_node", "payload": {"text": "Hello!"}}`
*   **消息推送 (Daemon -> Client)**: `{"op": "message", "topic": "/chat/global", "sender": "bob_node", "payload": {"text": "Hello!"}}`

### 服务 (Service)
*   **注册服务**: `{"op": "advertise_service", "service": "/tool/read_file", "node_id": "fs_node"}`
*   **发起请求**: `{"op": "call_service", "service": "/tool/read_file", "request_id": "req-1", "payload": {"path": "a.txt"}}`
*   **返回响应**: `{"op": "service_response", "service": "/tool/read_file", "request_id": "req-1", "payload": {"content": "..."}}`

---

## 🛠️ 命令行工具 (`tagentacle`)

CLI 是开发者的主要交互入口：
- `tagentacle daemon`：启动本地 TCP 消息总线。
- `tagentacle run --pkg <dir>`：激活包的 `.venv` 并启动其 Node。
- `tagentacle launch <config.toml>`：根据拓扑配置编排多节点，每个节点独立 venv；自动 `git clone` `[workspace]` 声明的仓库，实现一键工作空间引导。
- `tagentacle topic echo <topic>`：订阅并实时打印消息。
- `tagentacle service call <srv> <json>`：从命令行测试服务。
- ~~`tagentacle bridge`~~：已在 v0.3.0 移除。请使用 `mcp-gateway` 包替代。
- `tagentacle setup dep --pkg <dir>`：对单个包执行 `uv sync`。
- `tagentacle setup dep --all <workspace>`：扫描工作空间所有包并安装依赖，生成 `install/` 结构。
- `tagentacle setup clean --workspace <dir>`：移除生成的 `install/` 目录。
- `tagentacle doctor`：健康检查（守护进程状态、节点连通性）。

### 环境管理

每个包都是一个 **uv 项目**（`pyproject.toml` + `uv.lock`）。不使用 pip。

```bash
# 初始化整个工作空间
tagentacle setup dep --all .
# → 在每个包中执行 uv sync
# → 创建 install/src/<pkg>/.venv 符号链接
# → 生成 install/setup_env.bash

# 加载环境（将所有 .venv 添加到 PATH）
source install/setup_env.bash

# 清理
tagentacle setup clean --workspace .
```

---

## 📝 路线图与状态

### 已完成
- [x] **Rust Daemon**：Topic Pub/Sub 和 Service Req/Res 消息路由。
- [x] **Python SDK (Simple API)**：`Node` 类，含 `connect`、`publish`、`subscribe`、`service`、`call_service`、`spin`。
- [x] **Python SDK 双层 API**：实现 `LifecycleNode`，含 `on_configure`/`on_activate`/`on_deactivate`/`on_shutdown`。
- [x] ~~**MCP Bridge (Rust)**~~：已在 v0.3.0 移除 — 由 `mcp-gateway`（Python Gateway Node，传输层中继）替代。
- [x] ~~**MCP Transport 层**~~：已在 python-sdk-mcp v0.2.0 移除 — 由 Streamable HTTP 直连替代。
- [x] **MCPServerNode 基类**：python-sdk-mcp v0.2.0 — MCP Server Node 基类，自动 Streamable HTTP + `/mcp/directory` 发布。
- [x] **MCP Gateway**：mcp-gateway v0.1.0 — 传输层 stdio→HTTP 中继 + 目录服务。
- [x] **Tagentacle MCP Server**：内置 MCP Server，暴露总线交互工具（`publish_to_topic`、`subscribe_topic`、`list_nodes`、`list_topics`、`list_services`、`call_bus_service`、`ping_daemon`、`describe_topic_schema`）。
- [x] **`tagentacle.toml` 规范**：定义并解析包清单格式。
- [x] **Bringup 配置中心**：配置驱动的拓扑编排与参数注入。
- [x] **CLI 工具链**：`daemon`、`run`、`launch`、`topic echo`、`service call`、`doctor`、`setup dep`、`setup clean`。
- [x] **环境管理**：基于 uv 的逐包 `.venv` 隔离，工作空间 `install/` 结构与符号链接。
- [x] **秘钥管理**：`secrets.toml` 自动加载，Bringup 环境变量注入。
- [x] **SDK 工具函数**：`load_pkg_toml`、`discover_packages`、`find_workspace_root`。
- [x] **工作空间 Repo 自动克隆**：`tagentacle launch` 读取 `[workspace]` 配置段，启动前自动 `git clone` 所有声明的仓库。
- [x] **示例聊天机器人系统**：5 节点完整系统（`example-agent`、`example-inference`、`example-memory`、`example-frontend`、`example-mcp-server`），通过 `example-bringup` 一键启动，端到端验证通过。
- [x] **示例 Workspace**：`examples/src/` 包含 agent_pkg、mcp_server_pkg、bringup_pkg，均为独立 uv 项目。

### 计划中
- [ ] **标准 Topic 与 Service**：Daemon 内置 `/tagentacle/log`、`/tagentacle/node_events`、`/tagentacle/diagnostics`、`/tagentacle/ping`、`/tagentacle/list_nodes` 等。
- [ ] **SDK 日志集成**：通过 `get_logger()` 自动发布节点日志到 `/tagentacle/log`。
- [ ] **JSON Schema 校验**：Topic 级别 Schema 契约，实现确定性消息校验。
- [ ] **展平 Topic 工具 API**：SDK 提供 API，根据 Topic JSON Schema 定义自动生成展平参数的 MCP 工具（如注册了 `/chat/input` 的 Schema 后，自动生成 `publish_chat_input(text, sender)` 工具）。
- [ ] **节点生命周期追踪**：通过 `/tagentacle/diagnostics` 实现 Daemon 侧心跳/存活监控。
- [ ] **Interface Package**：跨节点 JSON Schema 契约定义包。
- [ ] **Action 模式**：长程异步任务，支持进度反馈。
- [ ] **Parameter Server**：全局参数存储，配合 `/tagentacle/parameter_events` 通知。
- [ ] **vcstool 兼容**：支持 `.repos` 文件格式，与 ROS 生态工具链互操作。
- [ ] **Web Dashboard**：实时拓扑、消息流和节点状态可视化。

---

## 🚀 快速开始

### 安装

```bash
# 从源码安装（编译并复制到 ~/.cargo/bin/）
cd tagentacle
cargo install --path .

# 验证
tagentacle --help

# 卸载
cargo uninstall tagentacle
```

> **提示**：确保 `~/.cargo/bin` 在你的 `PATH` 中（rustup 默认已添加）。

### 快速上手

安装完成后，以下命令均在**工作空间目录**（如 `tagentacle-py/example_ws/`）下运行：

1. **启动守护进程**（在另一个终端中）：
   ```bash
   tagentacle daemon
   ```

2. **初始化工作空间**（安装所有包的依赖）：
   ```bash
   cd tagentacle-py/example_ws
   tagentacle setup dep --all .
   ```

3. **运行节点**：
   ```bash
   tagentacle run --pkg src/mcp_server_pkg
   ```
