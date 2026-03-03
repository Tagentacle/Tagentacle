# 命令行工具 (`tagentacle`)

CLI 是开发者的主要交互入口：

| 命令 | 说明 |
|---|---|
| `tagentacle daemon` | 启动本地 TCP 消息总线 |
| `tagentacle run --pkg <dir>` | 激活包的 `.venv` 并启动其 Node |
| `tagentacle launch <config.toml>` | 根据拓扑配置编排多节点，每个节点独立 venv |
| `tagentacle topic echo <topic>` | 订阅并实时打印消息 |
| `tagentacle service call <srv> <json>` | 从命令行测试服务 |
| `tagentacle setup dep --pkg <dir>` | 对单个包执行 `uv sync` |
| `tagentacle setup dep --all <workspace>` | 扫描工作空间所有包并安装依赖 |
| `tagentacle setup clean --workspace <dir>` | 移除生成的 `install/` 目录 |
| `tagentacle doctor` | 健康检查（守护进程状态、节点连通性） |

!!! note
    `tagentacle bridge` 已在 v0.3.0 移除。请使用 `mcp-gateway` 包替代。

## 环境管理

每个包都是一个 **uv 项目**（`pyproject.toml` + `uv.lock`）。不使用 pip。

```bash
# 初始化整个工作空间
tagentacle setup dep --all .
# → 在每个包中执行 uv sync
# → 创建 install/src/<pkg>/.venv 符号链接
# → 生成 install/setup_env.bash

# 加载环境（将所有 .venv 添加到 PATH）
source install/setup_env.bash

# 清理
tagentacle setup clean --workspace .
```

## 依赖解析能力

| 来源类型 | 状态 | 说明 |
| :--- | :---: | :--- |
| **Git 仓库** | ✅ | `tagentacle.toml` 中的 `[workspace.repos]` —— `setup dep --all` 时自动克隆缺失仓库 |
| **Python (uv)** | ✅ | 逐包 `uv sync`，`.venv` 隔离 |
| **apt 包** | ❌ | 计划中 —— 系统级依赖 |
| **PyPI 包** | ❌ | 计划中 —— 通过 `pip install tagentacle-py-core` 进行系统级 SDK 分发 |
| **npm 包** | ❌ | 计划中 —— Node.js 工具和 MCP 服务器依赖 |
| **构建命令** | ❌ | 计划中 —— 自定义构建步骤（如 `cargo build`、`make`） |
