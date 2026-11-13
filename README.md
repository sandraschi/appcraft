# AppCraft

![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Rust 1.90+](https://img.shields.io/badge/rust-1.90%2B-orange?logo=rust)
![MCP stdio](https://img.shields.io/badge/MCP-stdio-purple)
![ArtCraft harness](https://img.shields.io/badge/ArtCraft-harness-green)

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

## Status — does today / next week / non-goals

**Does today:** 4-crate workspace (templates as a crate), dispatch wired
to DatabaseCraft blueprints; surgery blueprint → 6 tables + 12 ledger
tasks, proven by test (3 fns); stigmergy ledger; 3 surfaces. Templates are
manifests without bodies; CompileArtifact is a receipt stub (see hub
assessment — honest, not finished).

**Next week:** template bodies v1 (`desktop_egui` CRUD skeleton + golden
tree test); honest compile (real `cargo build` or rename); generate
`bohr-ordination` + smoke + recipe.

**Non-goals:** factory generates files, doesn't host a platform; no second
template before the first compiles.

## Lineage — the Storytold craft-app pattern

Storytold ArtCraft ([storytold/artcraft](https://github.com/storytold/artcraft) — "the IDE for artists") was the starting point. The 16 craft apps in the Storytold stable each ship one binary that is both the human CLI and the agent MCP server (stdio), headless-first, with `run.json` + report artifacts. That modus operandi and harness is what this repo implements; the code here is independent, the pattern is shared.
