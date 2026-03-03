# CLI Tools (`tagentacle`)

The CLI provides the primary interface for developers:

| Command | Description |
|---|---|
| `tagentacle daemon` | Starts the local TCP message bus |
| `tagentacle run --pkg <dir>` | Activates the package's `.venv` and launches its Node |
| `tagentacle launch <config.toml>` | Orchestrates multiple Nodes from topology config, each with isolated venvs |
| `tagentacle topic echo <topic>` | Subscribes and prints real-time messages |
| `tagentacle service call <srv> <json>` | Tests a service from the command line |
| `tagentacle setup dep --pkg <dir>` | Runs `uv sync` for a single package |
| `tagentacle setup dep --all <workspace>` | Installs all packages and generates `install/` structure |
| `tagentacle setup clean --workspace <dir>` | Removes the generated `install/` directory |
| `tagentacle doctor` | Health check (daemon status, node connectivity) |

!!! note
    `tagentacle bridge` was removed in v0.3.0. Use the `mcp-gateway` package instead.

## Environment Management

Every package is a **uv project** (`pyproject.toml` + `uv.lock`). No pip is used.

```bash
# Initialize the full workspace
tagentacle setup dep --all .
# → runs uv sync in each package
# → creates install/src/<pkg>/.venv symlinks
# → generates install/setup_env.bash

# Source environment (adds all .venvs to PATH)
source install/setup_env.bash

# Clean up
tagentacle setup clean --workspace .
```

## Dependency Resolution Capabilities

| Source Type | Status | Description |
| :--- | :---: | :--- |
| **Git Repos** | ✅ | `[workspace.repos]` in `tagentacle.toml` — auto-clone missing repos on `setup dep --all` |
| **Python (uv)** | ✅ | Per-package `uv sync` with `.venv` isolation |
| **apt packages** | ❌ | Planned — system-level dependencies |
| **PyPI packages** | ❌ | Planned — system-level SDK distribution via `pip install tagentacle-py-core` |
| **npm packages** | ❌ | Planned — Node.js tool and MCP server dependencies |
| **Build commands** | ❌ | Planned — custom build steps (e.g., `cargo build`, `make`) |
