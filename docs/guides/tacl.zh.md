# TACL：Tagentacle 访问控制层

**TACL**（Tagentacle Access Control Layer）提供 MCP 级别的 JWT 认证与授权。它完全实现在 Python SDK（`python-sdk-mcp`）中 —— Daemon 对访问控制一无所知，忠实遵循「只提供机制，不制定策略」的原则。

## 架构

TACL 围绕三个角色构建：

| 角色 | 组件 | 职责 |
|---|---|---|
| **签发者** | `TACLAuthority` | SQLite 支撑的 Agent 注册中心。签发携带工具级授权的 JWT 凭证。 |
| **验证者** | `MCPServerComponent`（`auth_required=True`） | 每次请求验证 Bearer JWT。设置 `CallerIdentity` 上下文变量。 |
| **携带者** | `AuthMCPClient` | 向权限服务器认证，获取 JWT，附加到所有 MCP 请求。 |

## JWT 负载格式

```json
{
    "agent_id": "agent_alpha",
    "tool_grants": {
        "shell_server": ["exec_command"],
        "tao_wallet_server": ["query_balance", "transfer"]
    },
    "space": "agent_space_1",
    "iat": 1740000000,
    "exp": 1740086400
}
```

| 字段 | 类型 | 说明 |
|---|---|---|
| `agent_id` | `string` | Agent 唯一标识符。 |
| `tool_grants` | `{server_id: [tool_names]}` | 按服务器的工具白名单。仅列出的工具可被调用。 |
| `space` | `string?` | **执行环境绑定** —— 标识分配给此 Agent 的隔离空间（如 Docker 容器名）。 |
| `iat` / `exp` | `int` | 签发时间 / 过期时间（UNIX 时间戳）。默认有效期：24 小时。 |

## `space` 声明：将 Agent 绑定到容器

`space` 字段是连接 TACL 认证与容器隔离的关键。管理员注册 Agent 时指定 `space`：

```python
# 管理员注册 Agent 并绑定容器
await permission_node.register_agent(
    agent_id="agent_alpha",
    raw_token="secret_token",
    tool_grants={"shell_server": ["exec_command"]},
    space="agent_space_1"   # ← 绑定到此容器
)
```

当 Agent 认证后调用 MCP 工具（如 shell-server 的 `exec_command`），服务端从 JWT 中读取 `CallerIdentity.space` 并将命令路由到绑定的容器 —— **无全局配置，无静态映射**。每个 Agent 的 JWT 携带自己的容器绑定信息。

## 认证流程

```
管理员                  TACLAuthority                     MCPServerComponent (auth_required)
  │                              │                                │
  ├─ register_agent ────────────▶│                                │
  │  (agent_id, token,           │                                │
  │   tool_grants, space)        │                                │
  │                              │                                │
  │          Agent (AuthMCPClient)                                │
  │              │               │                                │
  │              ├─ authenticate ▶│                                │
  │              │  (raw_token)  │                                │
  │              │◀── JWT ───────┘                                │
  │              │                                                │
  │              ├─────── call_tool (Bearer: JWT) ───────────────▶│
  │              │                                  验证 JWT       │
  │              │                                  检查授权       │
  │              │                                  设置 CallerIdentity
  │              │◀──────────── 工具结果 ────────────────────────┘
```

## 设计原则

- **零外部依赖**：JWT 签名/验证使用纯 Python 标准库（HS256 via `hmac` + `hashlib`）。
- **共享密钥**：签发者和所有验证者均从环境变量 `TAGENTACLE_AUTH_SECRET` 读取密钥。
- **基于 Contextvar**：`CallerIdentity` 通过 Python `contextvars` 按请求设置，工具处理函数可直接读取调用者信息，无需参数透传。
- **细粒度**：授权支持每个服务器的**工具级别**。但推荐通过小而专注的服务器实现**服务器级**控制（参见[最佳实践](best-practices.md)）。
- **可选启用**：认证默认关闭。`MCPServerComponent(auth_required=False)`（默认值）接受所有调用者。
