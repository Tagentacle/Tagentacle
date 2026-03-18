# MCP 集成：本地会话 + HTTP 直连

借鉴 ROS 2 TF2 的设计理念，Tagentacle 将 MCP 会话管理完全本地化于 Agent 节点。

## 设计原则

*   **会话本地化**：MCP Client Session 保持在 Agent 节点内存中。Agent 通过原生 MCP SDK HTTP Client 直连 MCP Server 的 Streamable HTTP 端点。
*   **MCPServerComponent 组件**：MCP Server 使用 `LifecycleNode` + `MCPServerComponent`（组合模式），自动运行 Streamable HTTP 服务并在激活时向 `/mcp/directory` Topic 发布 `MCPServerDescription`。
*   **统一发现**：Agent 订阅 `/mcp/directory` Topic 即可自动发现所有可用 MCP Server（包括原生 HTTP Server 和 Gateway 代理的 stdio Server）。
*   **完整协议支持**：因 MCP 会话直接在 Agent ↔ Server 之间建立，所有 MCP 功能（sampling、notifications、resources 等）原生可用。
*   **MCP Gateway**：独立 `mcp-gateway` 包提供传输层 stdio→HTTP 中继，不解析 MCP 语义。

## Agent 侧连接示例

```python
from mcp import ClientSession
from mcp.client.streamable_http import streamable_http_client

# 通过 Streamable HTTP 直连 MCP Server
async with streamable_http_client("http://127.0.0.1:8100/mcp") as (r, w, _):
    async with ClientSession(r, w) as session:
        await session.initialize()
        result = await session.call_tool("query", {"sql": "SELECT * FROM users"})
```

## 双向调用与可观测性

- **双向调用**：因 MCP 会话直接在 Agent ↔ Server 之间建立（HTTP 长连接），完整支持 MCP 规范中的 **Sampling**（Server 反向调用 Agent）等双向能力。
- **透明观测**：MCP Server 的服务发现信息发布到 `/mcp/directory` Topic，任何节点可订阅获取系统中所有可用工具的实时视图。

## 控制面 vs 数据面

理解总线通信与 MCP 工具调用的分离至关重要：

| | 控制面 (Bus) | 数据面 (MCP HTTP) |
|---|---|---|
| **内容** | 发现、生命周期、事件 | 工具调用、资源、sampling |
| **传输** | TCP JSON Lines via Daemon | Streamable HTTP 直连 |
| **路径** | Node → Daemon → Node | Agent MCP Client → MCP Server |
| **示例** | `/mcp/directory`、`/tagentacle/node_events` | `call_tool("exec_command", ...)` |

总线**绝不**接触 MCP 工具调用内容。它只发布服务器 URL 用于发现。所有工具调用都直接从 Agent 通过 HTTP 到达 MCP Server。
