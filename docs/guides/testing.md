# Testing & E2E

## Overview

Tagentacle uses a **dedicated test package** (`test-bringup`) for end-to-end integration testing across the entire stack.

```
┌─────────────┐     ┌──────────────────┐     ┌──────────────┐
│  test-bringup│────▶│ tagentacle-py-core│────▶│   Daemon     │
│  (pytest)    │     │ (Python SDK)      │     │ (Rust binary) │
│              │────▶│ tagentacle-py-mcp │     │              │
└─────────────┘     └──────────────────┘     └──────────────┘
```

## Repo Structure

| Repository | Tests | What it covers |
|---|---|---|
| [tagentacle](https://github.com/Tagentacle/tagentacle) | `cargo test` | Daemon unit tests (Rust) |
| [tagentacle-py-core](https://github.com/Tagentacle/tagentacle-py-core) | `pytest` | SDK unit tests (Python) |
| [tagentacle-py-mcp](https://github.com/Tagentacle/tagentacle-py-mcp) | `pytest` | MCP bridge unit tests |
| [**test-bringup**](https://github.com/Tagentacle/test-bringup) | `pytest` | **E2E integration tests** |

## Running E2E Tests Locally

### Prerequisites

- Python ≥ 3.10
- Compiled `tagentacle` daemon binary

### Quick Start

```bash
# Build daemon
cd tagentacle && cargo build --release

# Install SDK (editable)
pip install -e tagentacle-py-core[validation]
pip install -e tagentacle-py-mcp

# Install test deps
pip install -e test-bringup[dev]

# Run!
TAGENTACLE_BIN=tagentacle/target/release/tagentacle pytest test-bringup -v
```

### Daemon Auto-Discovery

The test fixture searches for the daemon binary in this order:

1. `TAGENTACLE_BIN` environment variable
2. `../tagentacle/target/release/tagentacle` (workspace layout)
3. System `PATH`

## E2E Test Coverage

| Test File | Scenarios |
|---|---|
| `test_pubsub.py` | Single subscriber, multi-subscriber fanout, topic isolation, message ordering |
| `test_service.py` | System services (ping, list_nodes, list_topics), user-defined services |
| `test_node_events.py` | Node connect event, node disconnect event |
| `test_schema.py` | Strict mode validation, invalid message rejection, no-schema passthrough |

## CI Pipelines

### Per-Repo CI

Each repo runs its own CI on push/PR:

- **tagentacle**: `cargo check` → `clippy` → `test` → `fmt`
- **tagentacle-py-core**: `ruff` → `pytest` → `build`
- **tagentacle-py-mcp**: `ruff` → `pytest` → `build`

### E2E CI (test-bringup)

The `test-bringup` repo has a GitHub Actions workflow that:

1. Checks out all dependent repos
2. Builds the Rust daemon from source
3. Installs Python SDKs
4. Runs the full E2E test suite

It can be triggered manually with custom git refs:

```bash
gh workflow run e2e.yml \
  -f daemon_ref=v0.4.0 \
  -f sdk_core_ref=v0.3.0 \
  -f sdk_mcp_ref=v0.4.0
```

## Dependency Management Strategy

`test-bringup` uses version constraints in `pyproject.toml` to pin upstream SDK versions:

```toml
dependencies = [
    "tagentacle-py-core >= 0.3.0",
]
```

**Workflow for upstream releases:**

1. Upstream repo tags a new version (e.g., `tagentacle-py-core` v0.4.0)
2. Create a branch in `test-bringup`: `bump/sdk-core-0.4.0`
3. Update the version constraint in `pyproject.toml`
4. CI runs E2E tests against the new version
5. Green → merge to main

This ensures compatibility is validated **before** declaring a release stable.
