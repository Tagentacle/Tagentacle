# 包管理

Tagentacle 采用**逐包虚拟环境**模型——灵感来源于 ROS 2 的 colcon 工作空间——`src/` 中的每个包通过 [uv](https://docs.astral.sh/uv/) 获得独立的 `.venv`。

## 工作空间布局

运行 `tagentacle setup dep --all src` 后，工作空间结构如下：

```
my_workspace/
├── src/                          ← 包源码（git clone）
│   ├── python-sdk-core/.venv/    ← 独立 venv（零依赖）
│   ├── python-sdk-mcp/.venv/     ← 独立 venv（mcp, pydantic, …）
│   ├── example-agent/.venv/      ← 独立 venv
│   ├── example-mcp-server/.venv/
│   ├── mcp-gateway/.venv/
│   └── …
├── install/                      ← 自动生成（Phase 2）
│   ├── example_agent/.venv → ../../src/example-agent/.venv
│   ├── tagentacle_py_core/.venv → ../../src/python-sdk-core/.venv
│   ├── …
│   └── setup_env.bash            ← 将所有 .venv/bin 加入 $PATH
└── （无根级 pyproject.toml）
```

## 工作原理

### Phase 0 — Git Clone

bringup 包在 `tagentacle.toml` 中声明工作空间依赖：

```toml
[workspace.repos.python_sdk_core]
git = "https://github.com/Tagentacle/python-sdk-core.git"

[workspace.repos.python_sdk_mcp]
git = "https://github.com/Tagentacle/python-sdk-mcp.git"
```

CLI 自动将缺失的仓库克隆到 `src/`。

### Phase 1 — 逐包 `uv sync`

对 `src/` 中每个含 `pyproject.toml` 的包，CLI 执行：

```bash
cd src/<pkg> && uv sync
```

这将创建 `src/<pkg>/.venv/`，包含：

- 包本身（editable install）
- `[project.dependencies]` 中声明的所有依赖
- SDK 包通过 `[tool.uv.sources]` 的 **editable install**

### Phase 2 — `install/` 结构

CLI 生成：

- **符号链接**：`install/<pkg_name>/.venv → ../../src/<pkg-dir>/.venv`
- **`setup_env.bash`**：将每个包的 `.venv/bin` 加入 `$PATH`

```bash
source install/setup_env.bash
# 现在所有包的入口点都在 PATH 中可用
```

## SDK 共享——Editable Install

SDK 包（如 `python-sdk-core`）**不会被复制**。每个消费者包在 `pyproject.toml` 中声明 editable source：

```toml
[project]
dependencies = ["tagentacle-py-core>=0.1.0"]

[tool.uv.sources]
tagentacle-py-core = { path = "../python-sdk-core", editable = true }
```

`uv sync` 运行时，会在消费者的 `.venv/lib/pythonX.Y/site-packages/` 中创建 `.pth` 文件：

```
_tagentacle_py_core.pth → /path/to/workspace/src/python-sdk-core
```

所有消费者包指向**同一个源目录**。修改 SDK 代码立即对所有包生效。

## 三方依赖隔离

每个包独立解析自己的 `uv.lock`。这意味着：

- 三方包（pydantic, starlette, httpx, …）在每个 `.venv` 中独立安装
- 不同包**可以**有不同的可选依赖（如 `podman` vs `docker`）
- 在同一时期 sync 的包，版本一致性自然保持

!!! tip "磁盘开销"
    每个 MCP 消费者包约有 ~30 个共享传递依赖。实际上，标准工作空间总开销约 ~70 MB（不含 ML 框架等重量级包）。

## CLI 命令

| 命令 | 功能 |
|---|---|
| `tagentacle setup dep --all src` | 克隆仓库 + `uv sync` 所有包 + 生成 `install/` |
| `tagentacle setup dep --pkg src/<pkg>` | 单个包的 `uv sync` |
| `tagentacle setup clean --workspace .` | 移除 `install/` 目录 |
| `tagentacle run --pkg src/<pkg>` | 激活包的 `.venv` 并运行入口点 |
| `tagentacle launch <config.toml>` | 启动多个节点，各自使用独立 `.venv` |

## 包规范

每个 Tagentacle Python 包必须包含：

1. **`pyproject.toml`**——`[project]` 元数据和 `[build-system]`
2. **`[tool.uv.sources]`**——工作空间内 editable 依赖声明
3. **`tagentacle.toml`**——`[package]` 元数据（名称、类型、入口点）
4. **`uv.lock`**——提交到 git（可复现构建）

### 依赖声明模式

```toml
[project]
name = "my-package"
dependencies = [
    "tagentacle-py-core>=0.1.0",
    "tagentacle-py-mcp>=0.2.0",
]

[tool.uv.sources]
tagentacle-py-core = { path = "../python-sdk-core", editable = true }
tagentacle-py-mcp = { path = "../python-sdk-mcp", editable = true }
```

`path` 使用 `src/` 内的相对路径——因为所有包是同级目录，`../` 总是有效。

## 设计决策

| 决策 | 理由 |
|---|---|
| 逐包 venv（非 monorepo） | 节点间隔离；与 ROS 2 模型一致 |
| Editable SDK 链接 | SDK 零重复；代码修改即时生效 |
| `uv`（非 pip） | 快速、可复现、基于 lockfile |
| `install/` 符号链接 | 统一 PATH 无需复制；清晰分层 |
| 无根级 `pyproject.toml` | 工作空间不是 Python 项目；包是独立的 |
