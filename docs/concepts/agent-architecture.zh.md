# Agent 架构：IO + Inference 分离

Tagentacle 采用 **Agent Node**（上下文工程 + agentic loop）与 **Inference Node**（无状态 LLM 网关）的分离设计。

## Agent Node = 完整的 Agentic Loop

Agent Node 是一个独立 Pkg，在内部完成整个 agentic loop：

- 订阅 Topic → 接收用户消息/事件通知
- 管理 context window（消息队列、上下文工程）
- 通过 Service RPC 调用 Inference Node 获取 completion
- 解析 `tool_calls` → 通过 MCP Session（Streamable HTTP 直连）执行工具 → 回填结果 → 再推理

这个 loop 是一个紧耦合的顺序控制流（类似 ROS 2 的 nav2 导航栈），**不应**被拆分到多个 Node 中。

## Inference Node = 无状态 LLM 网关

一个独立的 Pkg（官方示例，位于 org 级别，**非**核心库组成部分），提供：

- Service（如 `/inference/chat`），接受 OpenAI 兼容格式：`{model, messages, tools?, temperature?}`
- 返回标准 completion：`{choices: [{message: {role, content, tool_calls?}}]}`
- 多个 Agent Node 可并发调用同一个 Inference Node

## 数据流

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
