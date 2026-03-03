# Python SDK: Dual-Layer Design

## Simple API (for General Nodes)

Quick integration for existing software — just `publish()` and `subscribe()`:

```python
from tagentacle_py import Node
import asyncio

async def main():
    node = Node("sensor_node")
    await node.connect()

    @node.subscribe("/data/temperature")
    async def on_temp(msg):
        print(f"Temperature: {msg['payload']['value']}°C")

    await node.publish("/status/online", {"node": "sensor_node"})
    await node.spin()

asyncio.run(main())
```

## Node API (for Agent Nodes with Lifecycle)

Full lifecycle management with `on_configure`, `on_activate`, etc., suitable for CLI-launched nodes accepting Bringup configuration:

```python
from tagentacle_py import LifecycleNode

class AliceAgent(LifecycleNode):
    def on_configure(self, config):
        self.api_key = config.get("api_key")
        self.allowed_tools = config.get("tools", [])

    def on_activate(self):
        self.subscribe("/task/inbox", self.handle_task)

    async def handle_task(self, msg):
        result = await self.call_service("/tool/search", msg["payload"])
        await self.publish("/task/result", result)

    def on_shutdown(self):
        self.logger.info("Alice shutting down gracefully.")
```

## Built-in Nodes

The SDK includes two key built-in nodes:

*   **TagentacleMCPServer**: Exposes the bus's `publish`, `subscribe`, `call_service` and other capabilities as standard MCP Tools. Inherits `MCPServerNode` and runs its own Streamable HTTP endpoint.
*   **MCPGatewayNode**: Transport-level relay — adapts stdio-only legacy MCP Servers to Streamable HTTP, and publishes remote server URLs to `/mcp/directory`.
