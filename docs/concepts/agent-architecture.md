# Agent Architecture: IO + Inference Separation

Tagentacle adopts a clean separation between **Agent Nodes** (context engineering + agentic loop) and **Inference Nodes** (stateless LLM gateway).

## Agent Node = Complete Agentic Loop

An Agent Node is a single Pkg that owns the entire agentic loop internally:

- Subscribe to Topics → receive user messages / events
- Manage the context window (message queue, context engineering)
- Call Inference Node's Service for LLM completion
- Parse `tool_calls` → execute tools via MCP Session (Streamable HTTP direct connection) → backfill results → re-infer

This loop is a tightly-coupled sequential control flow (like ROS 2's nav2 stack) and should **not** be split across multiple Nodes.

## Inference Node = Stateless LLM Gateway

A separate Pkg (official example at org level, **not** part of the core library) that provides:

- A Service (e.g., `/inference/chat`) accepting OpenAI-compatible format: `{model, messages, tools?, temperature?}`
- Returns standard completion: `{choices: [{message: {role, content, tool_calls?}}]}`
- Multiple Agent Nodes can call the same Inference Node concurrently

## Data Flow

```
UI Node ──publish──▶ /chat/input ──▶ Agent Node (agentic loop)
                                        │
                                        ├─ call_service("/inference/chat") ──▶ Inference Node ──▶ OpenRouter/OpenAI
                                        │                                           │
                                        │◀── completion (with tool_calls) ◀─────────┘
                                        │
                                        ├─ MCP Session (HTTP) ──▶ Tool Server Node
                                        │◀── tool result ◀────────┘
                                        │
                                        └─ publish ──▶ /chat/output ──▶ UI Node
```
