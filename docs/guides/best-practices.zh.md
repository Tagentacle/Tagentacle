# 最佳实践

## TACL：推荐服务器级访问控制

TACL 通过 `tool_grants` 同时支持**服务器级**和**工具级**授权。但我们推荐主要使用 TACL 做**服务器级控制** —— 即授予或拒绝 Agent 对整个 MCP Server 的访问权限。

如果你发现自己需要在单个服务器内做工具级访问控制，这通常意味着该服务器承担了过多职责。遵循 Unix 哲学，将其拆分为更小、更专注的 MCP Server Pkg —— 每个只做好一件事。当每个服务器都是单一职责时，服务器级 ACL 自然提供了恰当的控制粒度。

| 方式 | 推荐度 | 示例 |
|------|--------|------|
| 服务器级 | ✅ 推荐 | Agent A 可以访问 `shell-server` 但不能访问 `wallet-server` |
| 工具级 | ⚠️ 可行但不推荐 | Agent A 可以调用 `exec_command` 但不能调用同一服务器上的 `list_files` |

!!! tip "为什么？"
    更小的服务器更易于理解、独立部署和边界安全防护。工具级 ACL 在服务器内部增加了复杂性，但并未改善整体安全边界。

## MCP Server 传输层：仅限 Streamable HTTP

Tagentacle 中所有 MCP Server Node Pkg 均使用 **Streamable HTTP** 作为传输协议。这是 `MCPServerComponent` 的强制要求，具备以下优势：

- Agent↔Server 直连会话，完整支持 MCP 协议（sampling、notifications、resources）
- 通过标准 HTTP `Authorization` 头实现 TACL JWT 认证
- 标准的健康检查、负载均衡和容器网络

!!! warning "不推荐使用 stdio MCP Server"
    `mcp-gateway` 包提供的 stdio→HTTP 中继是面向第三方仅支持 stdio 传输的 MCP Server 的**传统兼容层**。这类似于在 ROS 2 节点内桥接 Linux 管道 —— 能用，但打破了标准通信模型：

    - stdio Server 无法参与 TACL 认证（没有 HTTP 头）
    - stdio 会话由 Gateway 进程管理，而非 Agent
    - 每个 HTTP 会话一个子进程，可扩展性受限

    如果你能控制 MCP Server 代码，请始终将其实现为 Streamable HTTP 的 `LifecycleNode` + `MCPServerComponent` Pkg。

## Server 设计：Unix 哲学

设计 MCP Server Pkg 时遵循 Unix 哲学：

| 原则 | 应用 |
|------|------|
| **只做好一件事** | 每个 MCP Server Pkg 暴露一组聚焦的相关工具 |
| **自由组合** | Agent 发现并连接多个小型服务器 |
| **独立故障** | 一个服务器崩溃不影响其他服务器 |
| **独立部署** | 每个服务器是独立容器，有独立依赖 |

### 示例：拆分巨型 Server

不要把所有工具塞进一个 Server：

```
❌  mega-server
    ├── exec_command
    ├── read_file
    ├── write_file
    ├── query_balance
    ├── transfer_funds
    └── send_email
```

拆分为专注的小 Server：

```
✅  shell-server       → exec_command
    file-server        → read_file, write_file
    wallet-server      → query_balance, transfer_funds
    email-server       → send_email
```

这样 TACL 服务器级 ACL 就能提供精确的控制粒度：Agent A 获得 `shell-server` + `file-server`，Agent B 仅获得 `wallet-server`。
