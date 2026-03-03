# 天然容器化架构

Tagentacle 的"一切皆 Pkg"哲学使其**天然适配容器化部署**。每个包都是独立进程、拥有独立依赖，完美适配一容器一包的部署模式。

## 为什么要容器化？

容器化对 **Agent Node** 尤其强大，赋予其前所未有的自由度：

- **Agent 的最大自由度**：每个 Agent 运行在独立容器中，拥有完全隔离的文件系统、网络栈和依赖树。Agent 可以自由 `pip install` 库、写文件、启动子进程，而不影响其他 Agent —— 真正的 AI 节点自治。
- **操作系统级故障隔离**：失控的 Agent（死循环、内存泄漏、对抗性工具输出）被 cgroup 限制约束。其他服务不受影响。
- **动态伸缩**：在负载均衡器后面启动多个 Inference Node 或 MCP Server 实例。Agent Node 可在运行时动态增删。
- **可复现部署**：每个包的 `Dockerfile` + `pyproject.toml` = 从开发笔记本到云集群的字节级一致环境。
- **安全边界**：处理不可信工具输出或用户输入的 Agent Node 在容器级别被沙箱化 —— 在 TACL JWT 认证之上提供纵深防御。

## 部署演进路径

```
开发阶段              → docker-compose           → K3s / K8s
─────────────────      ─────────────────────      ──────────────────
tagentacle daemon      tagentacle-core 容器       Deployment + Service
python 进程/包          一容器一包                  HPA 自动伸缩
localhost:19999        Docker bridge 网络          K8s Service DNS
bringup.py / launch    docker-compose.yml         Helm Chart
```

## 近零开销

容器**不是虚拟机** —— 它们通过 Linux namespaces 和 cgroups 共享宿主机内核：

- **CPU / 内存**：<1% 开销（原生执行，无 hypervisor）
- **网络**：bridge 模式 2–5% 开销；`host` 网络模式 = 零开销
- **磁盘 IO**：bind mount 卷 = 原生速度；OverlayFS 写入有约 5% 的写时复制开销
- **GPU**：通过 NVIDIA Container Toolkit 直通，零开销

对于 Tagentacle 的主要负载（WebSocket 总线消息 + LLM API 调用），容器网络开销（~50μs）相比推理延迟（~200ms+）完全可以忽略。

## 最小化代码改动

SDK 已经天然兼容容器化。唯一需要的改动是通过环境变量注入总线地址：

```python
import os
bus_host = os.environ.get("TAGENTACLE_BUS_HOST", "localhost")
bus_port = int(os.environ.get("TAGENTACLE_BUS_PORT", "19999"))
```

所有上层抽象 —— `Node`、`LifecycleNode`、`MCPServerNode`、TACL 认证、`/mcp/directory` 发现 —— 在容器内的行为与裸跑完全一致。

## 容器编排器：总线级容器管理

`container-orchestrator` 包是一个 `LifecycleNode`，通过总线管理 Docker 容器 —— **不是** Daemon 核心的一部分。正如 Docker 是 Linux 上的用户态程序（而非内核模块），此编排器是一个生态包。

| 总线服务 | 说明 |
|---|---|
| `/containers/create` | 从镜像创建并启动容器 |
| `/containers/stop` | 停止运行中的容器 |
| `/containers/remove` | 移除容器 |
| `/containers/list` | 列出所有受管容器 |
| `/containers/inspect` | 获取容器详情 |
| `/containers/exec` | 在容器内执行命令 |

```bash
# 创建 Agent 空间
tagentacle service call /containers/create \
  '{"image": "ubuntu:22.04", "name": "agent_space_1"}'

# 在容器内执行命令
tagentacle service call /containers/exec \
  '{"name": "agent_space_1", "command": "ls -la /workspace"}'
```

核心设计决策：

- 所有容器标记 `tagentacle.managed=true` 以便筛选。
- 自动注入 `TAGENTACLE_DAEMON_URL` 环境变量，使容器化节点可连回总线。
- 默认网络模式：`host`（总线连接最简方案）。
- 不包含 ACL 逻辑 —— 访问控制由 TACL 在 MCP 层处理。

## Shell Server：TACL 感知的动态路由

`shell-server` 包是一个 `MCPServerNode`，将 `exec_command` 作为 MCP 工具暴露。支持三种执行模式，按请求动态解析：

```
容器解析优先级：
  1. TACL JWT space 声明 → docker exec 到调用者绑定的容器
  2. 静态 TARGET_CONTAINER 环境变量 → 单一固定容器
  3. 本地 subprocess 兜底
```

在生产环境（TACL 模式）下，单个 shell-server 实例**同时服务多个 Agent**，每个请求根据 JWT `space` 声明路由到各自的容器：

```
Agent Alpha (JWT: space=agent_space_1) ──▶ Shell Server ──▶ docker exec agent_space_1
Agent Beta  (JWT: space=agent_space_2) ──▶ Shell Server ──▶ docker exec agent_space_2
Agent Gamma (JWT: 无 space)            ──▶ Shell Server ──▶ 本地 subprocess
```

## 端到端：从注册到隔离执行

```
1. 管理员注册 Agent：           PermissionNode.register_agent(space="agent_space_1")
2. 编排器创建容器：             /containers/create → docker run agent_space_1
3. Agent 认证：                 AuthMCPClient → JWT {agent_id, space: "agent_space_1"}
4. Agent 执行命令：             Shell Server 读取 JWT.space → docker exec agent_space_1
```

没有节点信任其他节点的自我声明身份。每次工具调用都经过 JWT 验证。容器绑定关系以**密码学方式证明**于 Token 中，而非作为可篡改的参数传递。
