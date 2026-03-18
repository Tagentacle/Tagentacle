# Package Management

Tagentacle uses a **per-package virtual environment** model — inspired by ROS 2's colcon workspaces — where each package in `src/` gets its own `.venv` via [uv](https://docs.astral.sh/uv/).

## Workspace Layout

After running `tagentacle setup dep --all src`, a workspace looks like:

```
my_workspace/
├── src/                          ← Package source (git-cloned)
│   ├── python-sdk-core/.venv/    ← Independent venv (zero deps)
│   ├── python-sdk-mcp/.venv/     ← Independent venv (mcp, pydantic, …)
│   ├── example-agent/.venv/      ← Independent venv
│   ├── example-mcp-server/.venv/
│   ├── mcp-gateway/.venv/
│   └── …
├── install/                      ← Auto-generated (Phase 2)
│   ├── example_agent/.venv → ../../src/example-agent/.venv
│   ├── tagentacle_py_core/.venv → ../../src/python-sdk-core/.venv
│   ├── …
│   └── setup_env.bash            ← Adds all .venv/bin to $PATH
└── (no root-level pyproject.toml)
```

## How It Works

### Phase 0 — Git Clone

The bringup package declares workspace dependencies in `tagentacle.toml`:

```toml
[workspace.repos.python_sdk_core]
git = "https://github.com/Tagentacle/python-sdk-core.git"

[workspace.repos.python_sdk_mcp]
git = "https://github.com/Tagentacle/python-sdk-mcp.git"
```

The CLI clones any missing repos into `src/`.

### Phase 1 — Per-Package `uv sync`

For each package in `src/` that has a `pyproject.toml`, the CLI runs:

```bash
cd src/<pkg> && uv sync
```

This creates `src/<pkg>/.venv/` with:

- The package itself (editable install)
- All declared dependencies from `[project.dependencies]`
- SDK packages as **editable installs** via `[tool.uv.sources]`

### Phase 2 — `install/` Structure

The CLI generates:

- **Symlinks**: `install/<pkg_name>/.venv → ../../src/<pkg-dir>/.venv`
- **`setup_env.bash`**: Adds every package's `.venv/bin` to `$PATH`

```bash
source install/setup_env.bash
# Now all package entry points are available in PATH
```

## SDK Sharing via Editable Installs

SDK packages (like `python-sdk-core`) are **not duplicated**. Each consumer package declares an editable source in `pyproject.toml`:

```toml
[project]
dependencies = ["tagentacle-py-core>=0.1.0"]

[tool.uv.sources]
tagentacle-py-core = { path = "../python-sdk-core", editable = true }
```

When `uv sync` runs, it creates a `.pth` file in the consumer's `.venv/lib/pythonX.Y/site-packages/`:

```
_tagentacle_py_core.pth → /path/to/workspace/src/python-sdk-core
```

All consumer packages point to the **same source directory**. Editing SDK code takes immediate effect across all packages.

## Third-Party Dependency Isolation

Each package independently resolves its own `uv.lock`. This means:

- Third-party packages (pydantic, starlette, httpx, …) are installed separately in each `.venv`
- Different packages **can** have different optional dependencies (e.g., `podman` vs `docker`)
- Version consistency is maintained naturally when packages are synced in the same time period

!!! tip "Disk Cost"
    Typical overhead is ~30 shared transitive dependencies per MCP-consuming package. In practice this amounts to ~70 MB total for a standard workspace (excluding heavy packages like ML frameworks).

## CLI Commands

| Command | What It Does |
|---|---|
| `tagentacle setup dep --all src` | Clone repos + `uv sync` all + generate `install/` |
| `tagentacle setup dep --pkg src/<pkg>` | `uv sync` for a single package |
| `tagentacle setup clean --workspace .` | Remove the `install/` directory |
| `tagentacle run --pkg src/<pkg>` | Activate the package's `.venv` and run its entry point |
| `tagentacle launch <config.toml>` | Launch multiple nodes, each with its own `.venv` |

## Package Conventions

Every Tagentacle Python package must have:

1. **`pyproject.toml`** with `[project]` metadata and `[build-system]`
2. **`[tool.uv.sources]`** for intra-workspace editable dependencies
3. **`tagentacle.toml`** with `[package]` metadata (name, type, entry point)
4. **`uv.lock`** committed to git (reproducible builds)

### Dependency Declaration Pattern

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

The `path` uses relative references from within `src/` — since all packages are siblings, `../` always works.

## Design Decisions

| Decision | Rationale |
|---|---|
| Per-pkg venv (not monorepo) | Isolation between nodes; matches ROS 2 model |
| Editable SDK links | Zero SDK duplication; instant code changes |
| `uv` (not pip) | Fast, reproducible, lockfile-based |
| `install/` symlinks | Unified PATH without copying; clean separation |
| No root `pyproject.toml` | Workspace is not a Python project; packages are independent |
