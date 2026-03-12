# Tagentacle Core — CI/CD & Development Instructions

## Project Overview

- **Language**: Rust (edition 2024)
- **Build**: `cargo build`
- **Binary**: `tagentacle` daemon + CLI (`src/main.rs`)
- **Version**: Tracked in `Cargo.toml` — must match latest `CHANGELOG.md` release
- **Tests**: `cargo test` (currently no tests — adding them is a priority)

## CI Pipeline

The GitHub Actions workflow (`.github/workflows/ci.yml`) runs on every push and PR:

### Jobs

1. **check** — `cargo check` (fast compile check, catches type errors)
2. **clippy** — `cargo clippy -- -D warnings` (Rust linter, treat warnings as errors)
3. **test** — `cargo test` (unit + integration tests)
4. **fmt** — `cargo fmt --check` (formatting consistency)

### Adding Tests

Test files go alongside source in `src/` using `#[cfg(test)]` modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_something() {
        // ...
    }
}
```

For integration tests, create `tests/` directory at crate root.

### Release Process

1. Update `CHANGELOG.md` with new version section
2. Update `version` in `Cargo.toml` to match
3. Commit: `chore: bump version to X.Y.Z`
4. Tag: `git tag vX.Y.Z`
5. Push: `git push && git push --tags`

## Commit Convention

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — new feature (e.g., new system service)
- `fix:` — bug fix
- `docs:` — documentation only
- `refactor:` — code change that neither fixes a bug nor adds a feature
- `chore:` — tooling, CI, version bumps
- `ci:` — CI configuration changes

## E2E Testing

End-to-end integration tests live in a separate repo: [test-bringup](https://github.com/Tagentacle/test-bringup)

To run E2E tests locally against your daemon build:

```bash
TAGENTACLE_BIN=target/release/tagentacle pytest ../test-bringup -v
```

See `test-bringup/README.md` for details on the dependency management strategy.

## Architecture Notes

- Daemon is a pure message router (like an IP router) — no schema validation
- System services (`/tagentacle/ping`, `list_nodes`, etc.) are intercepted in `handle_system_service()`
- Node events are published to `/tagentacle/node_events`
- MCP is application layer — Daemon does not understand MCP
