# AppCraft — Tiered Execution Plan

**Parent Standard:** [`RUST_ARTCRAFT_STANDARD.md`](../../mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md)  
**Menu Bar Standard:** [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md)  
**Architecture Model:** 4-Tier ArtCraft Incremental Delivery  
**Status:** Active  

---

## Architecture Flow

```
Tier 1: Core Engine ──> Tier 2: MCP / CLI ──> Tier 3: Desktop UI ──> Tier 4: Menu Bar, Help & Packaging
(appcraft-core)         (appcraft-mcp)         (appcraft-app)         (Hardened Workstation)
```

---

## Tier Specifications

### Tier 1 — Core Domain Engine & Bus (`appcraft-core`, `appcraft-templates`)
- [x] L1 `Command::dispatch` universal execution bus.
- [x] Stigmergic ledger helpers and SQLite B-tree storage integration.
- [x] Domain blueprint intake from DatabaseCraft.
- [x] `#![forbid(unsafe_code)]` and zero-panic clippy lints.

### Tier 2 — Headless Driver & FastMCP (`appcraft-mcp`)
- [x] Single CLI binary `appcraft-cli` with `--json` output.
- [x] FastMCP stdio server communicating over JSON-RPC 2.0.
- [x] Tool catalog mirroring `Command::dispatch` operations.
- [x] Stdio isolation: JSON-RPC on stdout, logs strictly on stderr.

### Tier 3 — Desktop Workstation UI (`appcraft-app`)
- [x] Native immediate-mode `egui` 0.37 / `wgpu` workstation.
- [x] Interactive blueprint editor and template manifest browser.
- [x] Stigmergy task ledger viewer and generator preview dock.

### Tier 4 — Craft Menu Bar Standard, Help Suite & Hardened Verification
- [ ] Universal 8-menu bar (`File`, `Edit`, `View`, `Insert`, `Modify`, `Commands`, `Window`, `Help`) with `switch_on_hover`.
- [ ] Mandatory 5-dialog Help suite (`Command Palette...`, `Keyboard Shortcuts...`, `Project Files & Docs...`, `Licenses & Notices...`, `About AppCraft`).
- [ ] Fleet UI/UX Pattern 2 adherence: viewport-centric navigation, top button row mapped to View toggles, panel visibility managed via `View` / `Window`.
- [ ] Tofu Guard verification (100% font glyph coverage, zero tofu placeholder boxes).
- [ ] Headless `--smoke` test runner verifying zero-window command dispatch parity.
- [ ] Release packaging and installer manifests.
