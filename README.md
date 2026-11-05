# AppCraft

**AppCraft** is the autonomous stigmergic application factory for the sandraschi fleet. It synthesizes local-first, zero-runtime-tax native desktop and web applications from structured domain blueprints (powered by **DatabaseCraft**).

## Key Architecture
- **Stigmergic Coordination**: Decentralized environment-mediated task routing (`_stigmergy_ledger`).
- **Zero-Runtime-Tax**: Standalone single-binary `.exe` packaging with embedded SQLite B-Tree transactional storage.
- **Universal 3-Tier ArtCraft Standard**:
  - `crates/appcraft-core`: Universal Command Bus (`Command::dispatch`).
  - `crates/appcraft-templates`: Architecture starter stacks (Desktop egui, FastAPI React, Axum Webview).
  - `crates/appcraft-mcp`: Hand-rolled line-delimited MCP stdio server + CLI (`appcraft-cli`).
  - `crates/appcraft-app`: Native immediate-mode Studio IDE (`appcraft`).
- **Standard Control Port**: `14115`.

## Lineage — the Storytold craft-app pattern

Storytold ArtCraft ([storytold/artcraft](https://github.com/storytold/artcraft) — "the IDE for artists") was the starting point. The 16 craft apps in the Storytold stable each ship one binary that is both the human CLI and the agent MCP server (stdio), headless-first, with `run.json` + report artifacts. That modus operandi and harness is what this repo implements; the code here is independent, the pattern is shared.
