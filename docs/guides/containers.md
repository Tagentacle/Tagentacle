# Container-Ready Architecture

Tagentacle's "Everything is a Pkg" philosophy makes it **naturally container-friendly**. Each package is an independent process with its own dependencies — a perfect fit for one-container-per-package deployment.

## Why Containerize?

Containerization is especially powerful for **Agent Nodes**, where it provides:

- **Maximum Freedom for Agents**: Each Agent runs in its own container with a fully isolated filesystem, network stack, and dependency tree. An Agent can `pip install` libraries, write files, or spawn subprocesses without affecting other Agents — true autonomy for AI-driven nodes.
- **Fault Isolation at the OS Level**: A runaway Agent (infinite loop, memory leak, adversarial tool output) is contained by cgroup limits. Other services continue unaffected.
- **Dynamic Scaling**: Spin up multiple instances of Inference Nodes or MCP Servers behind a load balancer. Agent Nodes can be added/removed at runtime.
- **Reproducible Deployment**: Each package's `Dockerfile` + `pyproject.toml` = byte-identical environments everywhere, from dev laptops to cloud clusters.
- **Security Boundaries**: Agent Nodes handling untrusted tool outputs or user inputs are sandboxed at the container level — defense-in-depth beyond TACL's JWT authentication.

## Deployment Progression

```
Development          → docker-compose           → K3s / K8s
─────────────────      ─────────────────────      ──────────────────
tagentacle daemon      tagentacle-core container   Deployment + Service
python process/pkg     one container per pkg       HPA auto-scaling
localhost:19999        Docker bridge network        K8s Service DNS
bringup.py / launch    docker-compose.yml          Helm Chart
```

## Near-Zero Overhead

Containers are **not VMs** — they share the host kernel via Linux namespaces and cgroups:

- **CPU / Memory**: <1% overhead (native execution, no hypervisor)
- **Network**: 2–5% overhead on bridge mode; `host` network mode = zero overhead
- **Disk I/O**: Bind-mounted volumes = native speed; OverlayFS writes have ~5% copy-on-write cost
- **GPU**: Zero overhead via NVIDIA Container Toolkit (passthrough)

For Tagentacle's primary workload (WebSocket bus messages + LLM API calls), the container network overhead (~50μs) is negligible compared to inference latency (~200ms+).

## Minimal Code Changes

The SDK is already container-compatible. The only change needed is injecting the bus address via environment variable:

```python
import os
bus_host = os.environ.get("TAGENTACLE_BUS_HOST", "localhost")
bus_port = int(os.environ.get("TAGENTACLE_BUS_PORT", "19999"))
```

All higher-level abstractions — `Node`, `LifecycleNode`, `MCPServerComponent`, TACL authentication, `/mcp/directory` discovery — work identically inside containers.

## Container Orchestrator: Bus-Level Container Management

The `container-orchestrator` package is a `LifecycleNode` that manages Docker containers via the bus — **not** part of the Daemon core. Like Docker is a userspace program on Linux, this orchestrator is an ecosystem package.

| Bus Service | Description |
|---|---|
| `/containers/create` | Create and start a container from an image |
| `/containers/stop` | Stop a running container |
| `/containers/remove` | Remove a container |
| `/containers/list` | List all managed containers |
| `/containers/inspect` | Get container details |
| `/containers/exec` | Execute a command inside a container |

```bash
# Create an agent space
tagentacle service call /containers/create \
  '{"image": "ubuntu:22.04", "name": "agent_space_1"}'

# Execute a command inside it
tagentacle service call /containers/exec \
  '{"name": "agent_space_1", "command": "ls -la /workspace"}'
```

Key design decisions:

- All containers are labeled with `tagentacle.managed=true` for filtering.
- Auto-injects `TAGENTACLE_DAEMON_URL` env var so containerized nodes can connect back to the bus.
- Default network mode: `host` (simplest for bus connectivity).
- No ACL logic — access control is handled by TACL at the MCP layer.

## Shell Server: TACL-Aware Dynamic Routing

The `shell-server` package is a `LifecycleNode` + `MCPServerComponent` that provides `exec_command` as an MCP tool. It supports three execution modes, resolved per-request:

```
Container Resolution Order:
  1. TACL JWT space claim → docker exec into caller's bound container
  2. Static TARGET_CONTAINER env → single fixed container
  3. Local subprocess fallback
```

In production (TACL mode), a single shell-server instance serves **multiple agents simultaneously**, each routed to their own container based on their JWT `space` claim:

```
Agent Alpha (JWT: space=agent_space_1) ──▶ Shell Server ──▶ docker exec agent_space_1
Agent Beta  (JWT: space=agent_space_2) ──▶ Shell Server ──▶ docker exec agent_space_2
Agent Gamma (JWT: no space)            ──▶ Shell Server ──▶ local subprocess
```

## End-to-End: From Registration to Isolated Execution

```
1. Admin registers agent:          PermissionNode.register_agent(space="agent_space_1")
2. Orchestrator creates container: /containers/create → docker run agent_space_1
3. Agent authenticates:            AuthMCPClient → JWT {agent_id, space: "agent_space_1"}
4. Agent calls exec_command:       Shell Server reads JWT.space → docker exec agent_space_1
```

No node trusts another node's self-reported identity. Every tool call goes through JWT verification. The container binding is **cryptographically attested** in the token, not passed as a mutable parameter.
