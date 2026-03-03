# Core Philosophy: Everything is a Pkg

Tagentacle inherits the most fundamental software organization philosophy from ROS 2: **thorough modularization of system capabilities**. This philosophy exhibits excellent engineering properties when abstracting agent components:

- **Agent Package**: Each agent as an independent package, containing its behavior logic, prompt templates, state management, and communication interfaces.
- **Tool/Service Package**: Encapsulates tools or services that agents need to call (e.g., database access, web scraping), supporting MCP protocol for plug-and-play integration.
- **Interface Package**: Dedicated to defining cross-node communication contracts (JSON Schema), ensuring packages written by different developers "speak the same language".
- **Bringup Package**: Responsible for system startup and configuration, defining which packages the workspace should include, node launch parameters, and environment credentials.

## Key Advantages

*   **High Reusability**: A mature tool package or agent package can be seamlessly migrated across projects like LEGO bricks.
*   **Version & Dependency Isolation**: Drawing from ROS 2's isolation mechanism, Tagentacle automatically manages independent Python virtual environments per package, eliminating dependency conflicts.
*   **Black-Box Development**: Developers only need to focus on input/output contracts — no need to know whether the internals use GPT, Claude, or a custom model.

## Why Not Just Use Linux Directly?

A reasonable question: Linux already has IPC (sockets, pipes, D-Bus), user permissions (uid/gid, DAC, SELinux), process isolation (namespaces, cgroups, Docker), and service discovery (DNS, mDNS, systemd). Is Tagentacle reinventing the wheel?

**No — Tagentacle is not replacing Linux. It is a _domain shell_ built on top of Linux** — the same way HTTP doesn't replace TCP, ROS 2 doesn't replace shared memory, and Bash doesn't replace `write(2)`.

| Primitive | Linux Native | What Tagentacle Adds |
| :--- | :--- | :--- |
| **IPC** | socket, pipe, shm, D-Bus | **Named Topic Pub/Sub + Service RPC** — Linux IPC is point-to-point. Topic Pub/Sub provides "publish once, all subscribers receive" semantics with named channels. D-Bus is the closest, but it's an XML protocol designed for desktop apps, not a JSON message bus for multi-node systems. |
| **Permissions** | uid/gid, capabilities | **Dynamic identity + tool-level authorization** — Linux users are static (root creates, /etc/passwd stores). Tagentacle nodes are dynamic (created/destroyed at runtime), and permission granularity is "node X can call tool Y on server Z", not "user 1000 can read /home/foo". |
| **Isolation** | namespaces, cgroups, Docker | **Semantic binding between node ↔ container** — Docker only knows "container abc123". Tagentacle knows "this container is a personal space for node X, it should auto-connect to the bus and only accept requests from authorized callers". |
| **Discovery** | DNS, mDNS, systemd | **Capability-aware discovery** — DNS tells you "service X is at 192.168.1.5:8080". `/mcp/directory` tells you "MCP Server X provides tools [read_file, exec_command], requires JWT auth, supports Streamable HTTP". |

What Tagentacle truly adds is not the primitives themselves, but the **glue semantics** between them: node identity → bound container → authorized tool access → bus-based service discovery → lifecycle management. Linux provides every brick; Tagentacle provides the blueprint for assembling them into AI agent infrastructure.

### What Would a Sufficiently Advanced AI Choose?

Three scenarios when AI is smart enough to do anything:

**Scenario A: AI uses raw Linux APIs.** Fork, clone namespaces, set up cgroups, create socket pairs, implement its own message protocol, implement its own permission logic... *Every action requires reasoning through the entire chain from scratch.* Like writing a web app from TCP sockets every time.

**Scenario B: AI uses Docker + ad-hoc orchestration.** Better, but *each AI invents a different communication protocol.* This is exactly the state of robotics before ROS — every lab reinvented IPC, making results non-composable.

**Scenario C: AI uses Tagentacle.** `node.publish("/task/result", data)` — one line replaces dozens of reasoning steps. Standardized protocol means nodes written by different AIs (or humans) are automatically interoperable.

**AI will always choose the path that minimizes token expenditure.** `node.publish(topic, msg)` will always cost fewer tokens than deriving the equivalent `socket() + connect() + send() + custom protocol encode/decode`. Just as humans can write assembly but choose Python.

### The Lesson from ROS

ROS wrapped Linux rather than exposing it directly because:

1. **Protocol unification** — before ROS, every robotics team used different IPC (CORBA, Ice, custom protocols). Algorithms couldn't be shared across teams. ROS Topics/Services unified the communication contract.
2. **Discover-and-use** — `rostopic list` shows all data flows in the system. Without ROS, you need to know every process's socket address, protocol format, and data encoding.
3. **Composability** — any two ROS nodes can communicate as long as their topic types match, without knowing each other exists. Linux IPC requires explicit wiring of both ends.

**The AI agent ecosystem is in a pre-ROS state** — every agent framework (LangChain, CrewAI, AutoGen) invents its own communication model, making agents non-interoperable. Tagentacle aims to be the ROS of this domain.
