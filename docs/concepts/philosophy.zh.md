# 核心哲学：一切皆包 (Everything is a Pkg)

Tagentacle 继承了 ROS 2 最根本的软件组织哲学：**将系统功能彻底模块化**。这种哲学在抽象智能体组件时展现出极佳的工程性质：

- **Agent Package**：每个智能体作为独立的包，包含其行为逻辑、Prompt 模板、状态管理和通信接口。
- **Tool/Service Package**：封装智能体需要调用的工具或服务（如数据库访问、网页爬取），支持 MCP 协议实现插件化。
- **Interface Package**：专门定义跨节点通信的消息契约（JSON Schema），确保不同开发者编写的包能"说同一种语言"。
- **Bringup Package**：负责系统的启动和配置，定义 Workspace 中的 Package 集合、各节点的启动参数、以及环境凭据。

## 核心优势

*   **高度可重用性**：成熟的工具包或智能体包可以像乐高积木一样在不同项目中无缝迁移。
*   **版本与依赖隔离**：借鉴 ROS 2 的隔离机制，为每个 Package 自动管理独立的 Python 虚拟环境，彻底解决依赖冲突。
*   **黑盒开发模式**：开发者只需关注包的输入输出契约，无需关心内部实现是基于何种模型或框架。

## 为什么不直接用 Linux？

一个合理的问题：Linux 已经有 IPC（socket、pipe、D-Bus）、用户权限（uid/gid、DAC、SELinux）、进程隔离（namespace、cgroup、Docker）、服务发现（DNS、mDNS、systemd）。Tagentacle 是在重新造轮子吗？

**不是 — Tagentacle 不是在替代 Linux，而是在 Linux 之上构建「领域 Shell」** — 正如 HTTP 没有替代 TCP，ROS 2 没有替代共享内存，Bash 没有替代 `write(2)` 系统调用。

| 原语 | Linux 原生 | Tagentacle 新增的领域语义 |
| :--- | :--- | :--- |
| **IPC** | socket, pipe, shm, D-Bus | **命名 Topic Pub/Sub + Service RPC** — Linux IPC 是点对点管道，没有「发布一次，所有订阅者自动收到」的语义。D-Bus 最接近，但它是面向桌面应用的 XML 协议，不是面向多节点系统的 JSON 消息总线。 |
| **权限** | uid/gid, capabilities | **动态身份 + 工具级授权** — Linux 用户是静态的（root 创建，/etc/passwd 存储）。Tagentacle 节点是动态的（运行时创建/销毁），权限粒度是「节点 X 能调用服务器 Z 的工具 Y」，而非「用户 1000 能读 /home/foo」。 |
| **隔离** | namespace, cgroup, Docker | **节点 ↔ 容器的语义绑定** — Docker 只知道「容器 abc123」。Tagentacle 知道「这个容器是节点 X 的私有空间，它应该自动连接总线，并只接受授权调用者的请求」。 |
| **发现** | DNS, mDNS, systemd | **能力感知的服务发现** — DNS 只告诉你「服务 X 在 192.168.1.5:8080」。`/mcp/directory` 告诉你「MCP Server X 提供工具 [read_file, exec_command]，需要 JWT 认证，支持 Streamable HTTP」。 |

Tagentacle 真正新增的不是原语本身，而是原语之间的**「胶水语义」**：节点身份 → 绑定容器 → 授权工具访问 → 总线服务发现 → 生命周期管理。Linux 提供了所有砖块；Tagentacle 提供了将它们组装成 AI agent 基础设施的蓝图。

### 足够智能的 AI 会怎么选？

三种场景推演：

**场景 A：AI 直接用 Linux API。** fork、clone namespace、设置 cgroup、创建 socket pair、自己实现消息协议、自己实现权限检查逻辑…… *每次操作都要从头推理整条链路。* 就像每次写 Web 应用都从 TCP socket 开始。

**场景 B：AI 用 Docker + 临时编排。** 好一些，但*每个 AI 会发明不同的通信协议*。这正是 ROS 出现之前机器人领域的状态——每个实验室各自造轮子，成果无法复用。

**场景 C：AI 用 Tagentacle。** `node.publish("/task/result", data)` — 一行代码替代几十步推理。标准化协议意味着不同 AI（或人类）编写的节点自动可互操作。

**AI 始终会选择 token 消耗最少的路径。** `node.publish(topic, msg)` 永远比推导等效的 `socket() + connect() + send() + 自定义协议编解码` 消耗更少的 token。正如人类有能力写汇编，但选择用 Python。

### ROS 的历史教训

ROS 封装 Linux 而非直接暴露，核心原因是：

1. **协议统一** — 没有 ROS 时，每个机器人团队用不同的 IPC 方案（CORBA、Ice、自制协议），算法无法跨团队复用。ROS Topic/Service 统一了通信契约。
2. **发现即可用** — `rostopic list` 就能看到系统中所有数据流。没有 ROS，你得知道每个进程的 socket 地址、协议格式、数据编码方式。
3. **组合性** — 任意两个 ROS 节点只要 topic 类型匹配就能通信，无需双方知道对方存在。Linux IPC 需要显式连接两端。

**AI agent 领域正处于 ROS 之前的状态** —— 每个 agent 框架（LangChain、CrewAI、AutoGen）各自发明通信方式，agent 之间无法互操作。Tagentacle 要做的，正是这个领域的 ROS。
