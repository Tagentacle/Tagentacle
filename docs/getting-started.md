# Getting Started

## Installation

```bash
# Install from source (compiles and copies to ~/.cargo/bin/)
cd tagentacle
cargo install --path .

# Verify
tagentacle --help

# Uninstall
cargo uninstall tagentacle
```

!!! note
    Ensure `~/.cargo/bin` is in your `PATH` (rustup adds it by default).

## Quick Start

The recommended workflow is to clone a **bringup package** into a workspace's `src/` directory and let the CLI handle everything:

### 1. Create a workspace and clone the bringup repo

```bash
mkdir -p my_workspace/src && cd my_workspace/src
git clone https://github.com/Tagentacle/example-bringup.git
```

### 2. Set up workspace dependencies

Auto-clones repos & installs Python deps:

```bash
cd ..  # back to my_workspace/
tagentacle setup dep --all src
```

This reads `[workspace.repos]` from `example-bringup/tagentacle.toml`, clones missing repos (SDK + app packages) into `src/`, then runs `uv sync` in each package.

### 3. Start the Daemon

In a separate terminal:

```bash
tagentacle daemon
```

### 4. Launch the system

```bash
tagentacle launch src/example-bringup/launch/system_launch.toml
```
