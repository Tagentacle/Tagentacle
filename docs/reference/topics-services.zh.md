# 标准 Topic 与 Service

当 Daemon 启动时，会自动创建一组 `/tagentacle/` 命名空间下的**系统保留 Topic 和 Service** —— 类比 ROS 2 的 `/rosout`、`/parameter_events` 和节点内省服务。这些提供内置的可观测性、日志聚合和系统内省能力，无需用户侧任何配置。

## 保留命名空间约定

| 前缀 | 用途 | 管理者 |
|---|---|---|
| `/tagentacle/*` | **系统保留。** Daemon 与 SDK 核心功能 | 核心库 |
| `/mcp/*` | MCP 发现和网关服务 | MCPServerNode / Gateway |

用户自定义 Topic **不应**使用以上前缀。

## 标准 Topic（Daemon 管理）

| Topic | ROS 2 对应 | 说明 | 发布者 |
|---|---|---|---|
| `/tagentacle/log` | `/rosout` | 全局日志聚合。所有节点通过 SDK 自动发布日志；Daemon 也发布系统事件。 | SDK 节点（自动）+ Daemon |
| `/tagentacle/node_events` | 生命周期事件 | 节点生命周期事件：上线、下线、状态转换。支撑 Dashboard 实时拓扑图。 | Daemon（自动）+ `LifecycleNode`（自动）|
| `/tagentacle/diagnostics` | `/diagnostics` | 节点健康诊断：心跳、运行时长、消息计数、错误计数。 | SDK `Node.spin()`（定时）|
| `/mcp/directory` | _（无）_ | MCP 服务器发现。`MCPServerDescription` 由 MCP Server Node 和 Gateway 在激活时发布。Agent 订阅后自动发现服务器。 | MCPServerNode / Gateway |

## 标准 Service（Daemon 拦截式）

!!! info "架构说明 —— 有意的不对称性"
    这些 `/tagentacle/*` Service **不是**由某个正经 Node 通过 `advertise_service` 发布的。而是由 Daemon **拦截** `call_service` 请求——当 service 名以 `/tagentacle/` 开头时，Daemon 直接从 Router 内部状态生成响应——类似 Linux 的 `/proc` 文件系统由内核合成，而非由真实磁盘支撑。

    这意味着 `/tagentacle/list_services` **不会列出自己**或任何其他 `/tagentacle/*` service——它们存在于正常服务注册表之外。这是故意为之：Daemon 以*机制*而非*策略*的方式提供只读内省能力，而不作为普通 Node 参与其所管理的总线拓扑。

    从调用者视角看，API 与普通 service 完全一致——`call_service("/tagentacle/ping", {})` 用法相同。这种不对称性对消费者透明，但对架构至关重要。

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

## 日志消息格式 (`/tagentacle/log`)

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

## 节点事件格式 (`/tagentacle/node_events`)

```json
{
  "event": "connected",
  "node_id": "alice_agent",
  "timestamp": "2026-02-24T12:00:00.000Z",
  "state": "active",
  "prev_state": "inactive"
}
```
