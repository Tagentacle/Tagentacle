# 快速开始

## 安装

```bash
# 从源码安装（编译并复制到 ~/.cargo/bin/）
cd tagentacle
cargo install --path .

# 验证
tagentacle --help

# 卸载
cargo uninstall tagentacle
```

!!! note "提示"
    确保 `~/.cargo/bin` 在你的 `PATH` 中（rustup 默认已添加）。

## 快速上手

推荐的工作流程是将一个 **bringup 包** 克隆到工作空间的 `src/` 目录，然后让 CLI 处理一切：

### 1. 创建工作空间并克隆 bringup 仓库

```bash
mkdir -p my_workspace/src && cd my_workspace/src
git clone https://github.com/Tagentacle/example-bringup.git
```

### 2. 安装工作空间依赖

自动克隆仓库并安装 Python 依赖：

```bash
cd ..  # 回到 my_workspace/
tagentacle setup dep --all src
```

这会读取 `example-bringup/tagentacle.toml` 中的 `[workspace.repos]`，将缺失的仓库（SDK + 应用包）克隆到 `src/` 中，然后在每个包中运行 `uv sync`。

### 3. 启动 Daemon

在另一个终端中：

```bash
tagentacle daemon
```

### 4. 启动系统

```bash
tagentacle launch src/example-bringup/launch/system_launch.toml
```
