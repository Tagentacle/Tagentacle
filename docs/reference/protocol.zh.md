# 通信协议规范

Tagentacle Daemon 默认监听 `TCP 19999` 端口。所有通信均为换行符分割的 JSON 字符串（JSON Lines）。

## 话题 (Topic)

**订阅：**
```json
{"op": "subscribe", "topic": "/chat/global", "node_id": "alice_node"}
```

**发布：**
```json
{"op": "publish", "topic": "/chat/global", "sender": "bob_node", "payload": {"text": "Hello!"}}
```

**消息推送 (Daemon → Client)：**
```json
{"op": "message", "topic": "/chat/global", "sender": "bob_node", "payload": {"text": "Hello!"}}
```

## 服务 (Service)

**注册服务：**
```json
{"op": "advertise_service", "service": "/tool/read_file", "node_id": "fs_node"}
```

**发起请求：**
```json
{"op": "call_service", "service": "/tool/read_file", "request_id": "req-1", "payload": {"path": "a.txt"}}
```

**返回响应：**
```json
{"op": "service_response", "service": "/tool/read_file", "request_id": "req-1", "payload": {"content": "..."}}
```
