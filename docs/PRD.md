# AppCraft PRD

**Repo:** `appcraft` (Pure-Rust ArtCraft 3-tier workspace)
**Version:** 0.1.0 (workspace.package)
**Control port:** 14115 (`appcraft --control 14115`)
**Crates:** `appcraft-core` (L1 bus) / `appcraft-templates` (starter manifests) / `appcraft-mcp` (L2 CLI + stdio) / `appcraft-app` (L3 egui Studio)

## 1. Problem

Fleet app ideas start as domain blueprints (DatabaseCraft tables) and die as
one-off scripts. There is no factory that turns a blueprint into a runnable
local-first desktop or web app with storage, ledger tasks, and a control
surface wired on day one.

## 2. Solution

AppCraft is the autonomous stigmergic application factory. It consumes a
structured domain blueprint (via `databasecraft-core` / `databasecraft-store`),
emits a starter stack (desktop egui or web), and coordinates follow-up work
through an environment-mediated `_stigmergy_ledger` instead of direct
agent-to-agent calls.

## 3. Scope

- L1 `appcraft-core`: universal `Command::dispatch`, blueprint intake,
  stigmergy ledger helpers, SQLite B-Tree storage via DatabaseCraft.
- `appcraft-templates`: architecture starter stacks (desktop egui, web).
  Today these are manifests without bodies; bodies are the next milestone.
- L2 `appcraft-mcp`: hand-rolled line-delimited MCP stdio server + CLI
  (`appcraft-cli`). Stdout is pure JSON-RPC; logs go to stderr.
- L3 `appcraft-app`: native immediate-mode Studio IDE (`appcraft` binary).

### 3.1 UI/UX Architecture & Menu Bar Standard
In accordance with [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md):
- **Pattern 2 (Viewport-Centric Craft Workstation):** Persistent factory canvas/blueprint editor surrounded by dockable inspectors, with top quick button row mapped as accelerators into `View` toggles. New generator views or template panels are registered as visibility toggles under `View` / `Window` and in the toolbox, wired to `Command::dispatch`.
- **Top Menu Bar Hierarchy:**
  - `File`: New Blueprint (`Ctrl+N`), Open Blueprint (`Ctrl+O`), Save (`Ctrl+S`), Export Scaffolding (`Ctrl+E`), Quit (`Ctrl+Q`).
  - `Edit`: Undo (`Ctrl+Z`), Redo (`Ctrl+Y`), Cut, Copy, Paste, Preferences (`Ctrl+,`).
  - `View`: Blueprint Canvas, Stigmergy Ledger Dock, Generated Code Preview, Theme (`Dark`/`Light`).
  - `Insert`: Add Table, Add Relation, Add Template Manifest, Add Ledger Task.
  - `Modify`: Mutate Schema, Reorder Fields, Refactor Blueprint.
  - `Commands`: Synthesize Artifacts, Run Code Generator, Verify Manifests, Clean Output.
  - `Window`: Reset Layout, Blueprint Viewport, Output Console, Stigmergy Inspector.
  - `Help`: Command Palette (`Ctrl+K`), Keyboard Shortcuts (`F1`), Project Files & Docs, Licenses & Notices, About AppCraft.
- **Behavioral Mechanics:** Smooth `switch_on_hover` between top-level menus, zero-unsafe immediate-mode rendering, and headless `--smoke` parity.

## 4. Does today / next week / non-goals

**Does today:** 4-crate workspace, dispatch wired to DatabaseCraft blueprints;
surgery blueprint yields 6 tables + 12 ledger tasks (proven by tests); three
surfaces (CLI, MCP, app shell) share one bus.

**Next week:** template bodies v1 (desktop egui CRUD skeleton + golden tree
test); honest compile path (real `cargo build` or renamed receipt);
first generated recipe with smoke check.

**Non-goals:** AppCraft generates files, it does not host a platform. No
second template before the first one compiles. No cloud deploy, no auth
service, no multi-tenant story.

## 5. Safety and honesty

- Zero-runtime-tax single binary; embedded SQLite, no sidecar ports for the
  desktop path (control port is opt-in for external tooling).
- `CompileArtifact` is currently a receipt stub, not proof of a build. It is
  labelled as such until the real `cargo build` path lands.
- No stubs presented as features in MCP `tools/list`.

## 6. Gates

`cargo clippy --workspace --all-targets --all-features -- -D warnings`,
`cargo test --workspace`, `cargo fmt --all --check`, byte-exact MCP stdio
probe (`initialize` -> `tools/list` -> `tools/call`), `Cargo.lock` committed,
no `target/` / `*.bak` / `*.exe` in `git status`.
