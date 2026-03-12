# 设计谱系：Tagentacle 与互联网

每一代工程师都在解同一个根本问题：**多个独立实体如何可靠通信？** Tagentacle 也不例外——它处于一个源远流长的通信系统谱系之中。理解这个谱系，有助于厘清 Tagentacle 是什么、借鉴了什么、又在哪里分道扬镳。

## 同一个问题，反复出现

```
1970s  多台计算机要互相通信  → 发明了 IP + DNS + TCP
1990s  多个进程要互相通信    → 发明了 D-Bus / CORBA / IPC
2000s  多个微服务要互相通信  → 发明了 Consul / etcd / Service Mesh
2010s  多个机器人节点要通信  → 用了 DDS（经由 ROS 2）
2025s  多个 AI Agent 要通信  → Tagentacle
```

每一代人都重新发现了相同的构建块：**命名、路由、发现、类型安全、授权**。模式之所以趋同，是因为解空间本身有限——拓扑就那么几种（星型/树型/网状/总线），通信模式就那么几种（请求-响应/发布-订阅/流/广播），一致性模型也就那么几种（强一致/最终一致/因果一致）。

**Tagentacle 的价值不在于发明新的原语，**  而是为特定领域——AI Agent 通信——**选出正确的子集和正确的默认值**。

## 逐层对应关系

| 互联网 | Tagentacle | 共同抽象 |
|--------|-----------|---------|
| 物理层（Ethernet、Wi-Fi） | TCP socket | 传输介质 |
| IP（尽力路由，不保证送达） | Daemon（Topic 路由，fire-and-forget） | 分组交换 |
| TCP/UDP（可靠/不可靠传输） | JSON Lines 帧 + 心跳 | 端到端可靠性 |
| DNS（层级命名，最终一致） | Topic 命名空间（树状） | 命名与发现 |
| TLS + CA（身份 + 加密） | TACL JWT（身份 + 授权） | 信任基础设施 |
| HTTP（请求-响应，无状态） | Service RPC | 同步调用 |
| WebSocket / SSE（服务器推送） | Topic Pub/Sub | 异步流 |
| Web App / API | MCP Server / Agent | 应用层 |

### Daemon ≈ IP 路由器

IP 路由器看目的地址、查路由表、转发。它不关心 payload 是 HTTP 还是 SMTP，不做内容校验。没人监听，包就丢弃。

Tagentacle Daemon 做的是完全一样的事：看 topic name、查订阅表、转发。它不关心 payload 是 ToolCall 还是自然语言回复，不做 schema 校验。没人订阅，消息丢弃。

这正是**端到端原则（end-to-end principle）**的体现：智能放在端点（SDK/Node），不放在网络（Daemon）。

### Topic 树 ≈ DNS 层级

```
DNS:                              Topic:
com.                              /
├── google.com                    ├── tagentacle/
│   ├── mail.google.com           │   ├── node_events
│   └── maps.google.com           │   └── list_nodes (service)
└── github.com                    ├── agent/
                                  │   ├── output
                                  │   └── plan
                                  └── mcp/
                                      └── directory
```

一个关键区别：DNS 是分布式的，而 Tagentacle 的 topic 命名空间目前是单 Daemon 集中式的。互联网从 ARPANET 的 `HOSTS.TXT`（一个文件 = 一个中心）演化到分布式 DNS 花了十年。Tagentacle 现在处于 `HOSTS.TXT` 阶段——这不是问题，因为当前的规模还不需要更多。

### MCP ≈ HTTP — 应用层协议，不是系统层

MCP 不在 Tagentacle 系统层。这是有意为之的设计：

- IP 层不知道 HTTP 的存在，路由器不解析 HTTP header。
- Daemon 不知道 MCP 的存在，不解析 MCP JSON-RPC。

HTTP 运行在 TCP 之上，MCP 运行在 Tagentacle bus 之上。两者都是**端点选择使用的应用层协议**，与底层传输无关。

## 为什么不直接用现有基础设施？

如果答案都是"现成的"，为什么 AI Agent 框架不直接用？

| 现有方案 | 为什么 Agent 不用 | 根本矛盾 |
|---------|-----------------|---------|
| IP + DNS | 太底层，Agent 不想管 socket | Agent 需要语义通信，不是字节流 |
| HTTP + REST | 每次调用都要知道对方 URL | Agent 之间是动态拓扑，不是写死的 API |
| Kafka / NATS | 运维重，需要外部集群 | Agent 系统应该 `pip install && 跑起来` |
| ROS 2 / DDS | 静态类型、编译期绑定、C++ 优先 | Agent 是 Python、JSON、动态 schema |
| D-Bus | 单机、同步、session-scoped | Agent 可能跨机器 |
| Consul / etcd | 只做发现，不做通信 | Agent 需要发现 + 通信 + 类型一体 |
| gRPC + Protobuf | 需要 `.proto` 编译 | LLM 输出的是 JSON，不是 protobuf |

**每一个方案都覆盖了 80% 的需求。** 剩下的 20% 恰恰是 AI Agent 最在意的部分。

## Docker 类比

Docker 也没有发明任何东西：

```
cgroups     → 已经存在了十年
namespaces  → 已经存在了十年
overlayfs   → 已经存在了十年
```

Docker 的贡献是把它们包装成了 `docker run`。

Tagentacle 做的是同样的事：

```
pub/sub          → 已经存在了四十年
service RPC      → 已经存在了三十年
JSON Schema      → 已经存在了十五年
```

包装成：

```python
node = Node("my_agent")
node.publish("/output", {"result": "..."})
```

价值在于**精选（curation）**——挑选、组合、设定默认值——而非发明。

## 规模演进轨迹

互联网在每个阶段的解法，以及 Tagentacle 的对应时刻：

| 互联网阶段 | 规模 | 解法 | Tagentacle 对应 |
|-----------|------|------|----------------|
| HOSTS.TXT | ~100 台主机 | 中心化列表 | 单 Daemon + `list_nodes` |
| DNS | ~10K 台主机 | 层级分区 + 缓存 | Topic 树 + namespace 隔离 |
| CDN / 负载均衡 | ~1M 服务 | 就近路由 + 副本 | 多 Daemon + gossip |
| Service Mesh | ~10M 服务 | Sidecar proxy + 中央控制面 | 超出当前视野 |

Tagentacle 目前处于 HOSTS.TXT 时代。这不是局限——这是对当前规模（几十个节点，单机部署）的合理选择。架构本身并不妨碍在规模需要时沿着这条轨迹演进。

## Topic vs. Type：Tagentacle 与 ROS 2 的分歧点

互联网类比在一个重要的地方失效。在 ROS 2 中，**Topic = Type** 是 1:1 绑定：

```
/joint_states  →  永远是 sensor_msgs/JointState,  50Hz
/cmd_vel       →  永远是 geometry_msgs/Twist,      10Hz
```

这在物理传感器和执行器的世界中成立——它们的数据格式在系统运行周期内不变。

AI Agent 的通信模式截然不同：

```
/agent/output  →  可能是 ToolCall、NaturalLanguage、ErrorReport，或者某种全新类型
```

通信模式是**多态的、弹性的、需要协商的**——更接近 HTTP（同一个 URL 可以根据 `Content-Type` 返回 HTML、JSON 或图片），而不是 ROS 2 topic。

这意味着 Tagentacle 的通道（channel）在概念上是一个 **(topic, schema)** 二元组，而不仅仅是一个 topic 名称。Daemon 按 topic 路由（快速字符串匹配），SDK 按 schema 过滤（语义匹配）。这种分离让系统层保持简单，同时赋予应用层灵活性。

## 总结

> Tagentacle 之于 AI Agent = IP + DNS 之于互联网应用。
>
> Daemon 是路由器，Topic 树是 DNS，MCP 是 HTTP，Agent 是 Web App。
>
> 系统层提供 naming + routing + discovery 机制，
> 应用层（MCP、Agent 框架）在上面自由组合。

把 MCP 排除在系统层之外——正如 IP 不理解 HTTP——这个架构决定，与当年让互联网从几百台主机扩展到几十亿设备的决定是同一个。
