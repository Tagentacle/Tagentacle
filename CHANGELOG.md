# Changelog

All notable changes to the **Tagentacle** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-22

### Added
- **Core MVP**: Implement a minimal Pub/Sub messaging core protocol.
  - Basic `Action` enum for `Subscribe` and `Publish`.
  - Tokio-based TCP server with `LinesCodec` framing.
  - In-memory `Router` for managing subscriptions.
- **Project Structure**:
  - `vibe coding` initial setup.
  - Rust project scaffolding with `Cargo.toml`.
- **Documentation**:
  - Added [README.md](README.md) (English).
  - Added [README_CN.md](README_CN.md) (Chinese).
