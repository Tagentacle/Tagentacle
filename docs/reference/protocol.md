# Communication Protocol

The Tagentacle Daemon listens on `TCP 19999` by default. All communication uses newline-delimited JSON (JSON Lines).

## Topics

**Subscribe:**
```json
{"op": "subscribe", "topic": "/chat/global", "node_id": "alice_node"}
```

**Publish:**
```json
{"op": "publish", "topic": "/chat/global", "sender": "bob_node", "payload": {"text": "Hello!"}}
```

**Message (Daemon Push):**
```json
{"op": "message", "topic": "/chat/global", "sender": "bob_node", "payload": {"text": "Hello!"}}
```

## Services

**Advertise:**
```json
{"op": "advertise_service", "service": "/tool/read_file", "node_id": "fs_node"}
```

**Call:**
```json
{"op": "call_service", "service": "/tool/read_file", "request_id": "req-1", "payload": {"path": "a.txt"}}
```

**Response:**
```json
{"op": "service_response", "service": "/tool/read_file", "request_id": "req-1", "payload": {"content": "..."}}
```
