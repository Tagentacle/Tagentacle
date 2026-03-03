# Python SDK：双层设计

## Simple API（适用于一般程序节点）

为已有软件快速接入总线提供简易接口——只需 `publish()` 和 `subscribe()`：

```python
from tagentacle_py import Node
import asyncio

async def main():
    node = Node("sensor_node")
    await node.connect()

    @node.subscribe("/data/temperature")
    async def on_temp(msg):
        print(f"温度: {msg['payload']['value']}°C")

    await node.publish("/status/online", {"node": "sensor_node"})
    await node.spin()

asyncio.run(main())
```

## Node API（适用于智能体节点，带生命周期管理）

完善的生命周期管理，支持 `on_configure`、`on_activate` 等钩子，适用于 CLI 启动并接受 Bringup 配置的节点：

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
        self.logger.info("Alice 正在优雅关闭。")
```

## 预制节点

SDK 内置两个关键节点：

*   **TagentacleMCPServer**：将总线的 `publish`、`subscribe`、`call_service` 等能力暴露为标准 MCP Tool。继承 `MCPServerNode`，自行运行 Streamable HTTP 端点。
*   **MCPGatewayNode**：传输层中继 — 将仅支持 stdio 的传统 MCP Server 适配为 Streamable HTTP，发布远程服务器 URL 到 `/mcp/directory`。
