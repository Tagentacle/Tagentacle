# Changelog

All notable changes to the **Tagentacle** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-02-22

### Added
- **Core Service Mechanism**:
  - Implemented `AdvertiseService`, `CallService`, and `ServiceResponse` in Rust Core.
  - Enhanced `Router` to support point-to-point service routing and node-ID tracking.
- **Python SDK Enhancements**:
  - Added `@node.service` decorator for declaring service handlers (supporting both sync and async).
  - Implemented `node.call_service()` for asynchronous RPC-style calls using `asyncio.Future`.
  - Updated `Node.spin()` to handle service request dispatching and response routing.
- **Examples**:
  - Added `service_server.py` and `service_client.py` for service mechanism demonstration.

### Changed
- **Documentation**:
  - Updated [README.md](README.md) Roadmap.
  - Translated all Python SDK and example code comments to English.

## [0.1.0] - 2026-02-22
