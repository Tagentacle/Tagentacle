# 路线图与状态

## 已完成

- [x] **Rust Daemon**：Topic Pub/Sub 和 Service Req/Res 消息路由。
- [x] **Python SDK (Simple API)**：`Node` 类，含 `connect`、`publish`、`subscribe`、`service`、`call_service`、`spin`。
- [x] **Python SDK 双层 API**：实现 `LifecycleNode`，含 `on_configure`/`on_activate`/`on_deactivate`/`on_shutdown`。
- [x] ~~**MCP Bridge (Rust)**~~：已在 v0.3.0 移除 — 由 `mcp-gateway`（Python Gateway Node，传输层中继）替代。
- [x] ~~**MCP Transport 层**~~：已在 python-sdk-mcp v0.2.0 移除 — 由 Streamable HTTP 直连替代。
- [x] **MCPServerNode 基类**（现为 MCPServerComponent）：python-sdk-mcp v0.2.0 — MCP Server Node 基类，自动 Streamable HTTP + `/mcp/directory` 发布。
- [x] **MCP Gateway**：mcp-gateway v0.1.0 — 传输层 stdio→HTTP 中继 + 目录服务。
- [x] **Tagentacle MCP Server**：内置 MCP Server，暴露总线交互工具。
- [x] **`tagentacle.toml` 规范**：定义并解析包清单格式。
- [x] **Bringup 配置中心**：配置驱动的拓扑编排与参数注入。
- [x] **CLI 工具链**：`daemon`、`run`、`launch`、`topic echo`、`service call`、`doctor`、`setup dep`、`setup clean`。
- [x] **环境管理**：基于 uv 的逐包 `.venv` 隔离，工作空间 `install/` 结构与符号链接。
- [x] **秘钥管理**：`secrets.toml` 自动加载，Bringup 环境变量注入。
- [x] **SDK 工具函数**：`load_pkg_toml`、`discover_packages`、`find_workspace_root`。
- [x] **工作空间 Repo 自动克隆**：`tagentacle launch` 读取 `[workspace]` 配置段，启动前自动 `git clone` 所有声明的仓库。
- [x] **示例聊天机器人系统**：5 节点完整系统，通过 `example-bringup` 一键启动。
- [x] **TACL（Tagentacle 访问控制层）**：`python-sdk-mcp` v0.3.0 — MCP 级别 JWT 认证。
- [x] **标准系统 Service**：Daemon 拦截式 `/tagentacle/ping`、`/tagentacle/list_nodes`、`/tagentacle/list_topics`、`/tagentacle/list_services`、`/tagentacle/get_node_info`。
- [x] **节点注册与心跳**：`Register` 握手、周期性 ping/pong、自动清理超时节点（90 秒超时）。
- [x] **节点断开清理**：断开连接时自动清理订阅、服务和节点条目，并发布 `/tagentacle/node_events`。
- [x] **JSON Schema 校验**：python-sdk-core v0.3.0 — `SchemaRegistry` 自动发现 + 逐节点校验模式。
- [x] **TACL `space` 声明**：python-sdk-mcp v0.4.0 — JWT `space` 字段将 Agent 绑定到隔离执行环境。
- [x] **容器编排生态包**：`container-orchestrator` v0.1.0 — LifecycleNode 通过总线服务管理 Docker 容器。
- [x] **Shell Server 生态包**：`shell-server` v0.1.0 — MCPServerNode（现为 LifecycleNode + MCPServerComponent）暴露 `exec_command` 工具，支持 TACL `space` 感知的动态容器路由。

## 计划中

- [ ] **标准 Topic（SDK 侧）**：SDK 自动发布到 `/tagentacle/log`、`/tagentacle/diagnostics`。
- [ ] **SDK 日志集成**：通过 `get_logger()` 自动发布节点日志到 `/tagentacle/log`。
- [ ] **展平 Topic 工具 API**：SDK 提供 API，根据 Topic JSON Schema 定义自动生成展平参数的 MCP 工具。
- [ ] **Interface Package**：跨节点 JSON Schema 契约定义包。
- [ ] **Action 模式**：长程异步任务，支持进度反馈。
- [ ] **Parameter Server**：全局参数存储，配合 `/tagentacle/parameter_events` 通知。
- [ ] **Web Dashboard**：实时拓扑、消息流和节点状态可视化。
