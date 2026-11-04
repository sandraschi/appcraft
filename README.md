# AppCraft

**AppCraft** is the autonomous stigmergic application factory for the sandraschi fleet. It synthesizes sovereign, zero-runtime-tax native desktop and web applications from structured domain blueprints (powered by **DatabaseCraft**).

## Key Architecture
- **Stigmergic Coordination**: Decentralized environment-mediated task routing (`_stigmergy_ledger`).
- **Zero-Runtime-Tax**: Standalone single-binary `.exe` packaging with embedded SQLite B-Tree transactional storage.
- **Universal 3-Tier ArtCraft Standard**:
  - `crates/appcraft-core`: Universal Command Bus (`Command::dispatch`).
  - `crates/appcraft-templates`: Architecture starter stacks (Desktop egui, FastAPI React, Axum Webview).
  - `crates/appcraft-mcp`: Hand-rolled line-delimited MCP stdio server + CLI (`appcraft-cli`).
  - `crates/appcraft-app`: Native immediate-mode Studio IDE (`appcraft`).
- **Standard Control Port**: `14115`.
