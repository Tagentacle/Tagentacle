# 测试与 E2E

## 概述

Tagentacle 使用专用测试包（`test-bringup`）对整个技术栈进行端到端集成测试。

```
┌─────────────┐     ┌──────────────────┐     ┌──────────────┐
│  test-bringup│────▶│ tagentacle-py-core│────▶│   Daemon     │
│  (pytest)    │     │ (Python SDK)      │     │ (Rust 二进制) │
│              │────▶│ tagentacle-py-mcp │     │              │
└─────────────┘     └──────────────────┘     └──────────────┘
```

## 仓库结构

| 仓库 | 测试工具 | 覆盖内容 |
|---|---|---|
| [tagentacle](https://github.com/Tagentacle/tagentacle) | `cargo test` | Daemon 单元测试（Rust） |
| [python-sdk-core](https://github.com/Tagentacle/python-sdk-core) | `pytest` | SDK 单元测试（Python） |
| [python-sdk-mcp](https://github.com/Tagentacle/python-sdk-mcp) | `pytest` | MCP 桥接单元测试 |
| [**test-bringup**](https://github.com/Tagentacle/test-bringup) | `pytest` | **E2E 集成测试** |

## 本地运行 E2E 测试

### 前置条件

- Python ≥ 3.10
- 已编译的 `tagentacle` daemon 二进制

### 快速开始

```bash
# 编译 daemon
cd tagentacle && cargo build --release

# 安装 SDK（可编辑模式）
pip install -e python-sdk-core[validation]
pip install -e python-sdk-mcp

# 安装测试依赖
pip install -e test-bringup[dev]

# 运行！
TAGENTACLE_BIN=tagentacle/target/release/tagentacle pytest test-bringup -v
```

### Daemon 自动发现

测试 fixture 按以下顺序搜索 daemon 二进制：

1. `TAGENTACLE_BIN` 环境变量
2. `../tagentacle/target/release/tagentacle`（workspace 布局）
3. 系统 `PATH`

## E2E 测试覆盖

| 测试文件 | 场景 |
|---|---|
| `test_pubsub.py` | 单订阅、多订阅扇出、话题隔离、消息顺序 |
| `test_service.py` | 系统服务（ping、list_nodes、list_topics）、用户自定义服务 |
| `test_node_events.py` | 节点上线事件、节点离线事件 |
| `test_schema.py` | 严格模式校验、非法消息拒绝、无 schema 透传 |

## CI 流水线

### 各仓库 CI

每个仓库在 push/PR 时运行自己的 CI：

- **tagentacle**: `cargo check` → `clippy` → `test` → `fmt`
- **python-sdk-core**: `ruff` → `pytest` → `build`
- **python-sdk-mcp**: `ruff` → `pytest` → `build`

### E2E CI（test-bringup）

`test-bringup` 仓库的 GitHub Actions 流水线会：

1. 检出所有依赖仓库
2. 从源码编译 Rust daemon
3. 安装 Python SDK
4. 运行完整 E2E 测试套件

支持手动触发，可指定各组件的 git ref：

```bash
gh workflow run e2e.yml \
  -f daemon_ref=v0.4.0 \
  -f sdk_core_ref=v0.3.0 \
  -f sdk_mcp_ref=v0.4.0
```

## 依赖管理策略

`test-bringup` 通过 `pyproject.toml` 中的版本约束管理上游 SDK 版本：

```toml
dependencies = [
    "tagentacle-py-core >= 0.3.0",
]
```

**上游发版工作流：**

1. 上游仓库打新 tag（例如 `python-sdk-core` v0.4.0）
2. 在 `test-bringup` 创建分支：`bump/sdk-core-0.4.0`
3. 更新 `pyproject.toml` 中的版本约束
4. CI 运行 E2E 测试验证新版本
5. 绿灯 → 合并到 main

这样可以在宣布发布稳定之前验证兼容性。
