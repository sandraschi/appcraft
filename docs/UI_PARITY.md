# AppCraft UI Parity

**Stack:** `eframe` / `egui` 0.37 (L3 `appcraft-app`), calls L1
`appcraft-core::dispatch` directly in-process. No WebView2, no localhost
HTTP sidecar for the desktop path.

## 1. Oracle loop

The generated app is the oracle. For each template body v1:

1. Generate the starter (desktop egui CRUD skeleton).
2. Run it, screenshot the window, screenshot the reference layout.
3. Diff the frames; fix the template, not the screenshot.
4. Lock with a golden PNG test (`egui_kittest`) so regressions fail CI.

Floor rises one widget at a time. A lone golden of our own window with no
reference is not the loop.

## 2. L3 requirements

- `--smoke` headless launch mode for CI (no window, exits 0 on success).
- `egui_kittest` golden widget PNG snapshots for the Studio shell.
- `glyph_source_scan` tofu guard: no missing-glyph boxes in the checked-in
  UI strings.
- Light/dark follows the fleet egui pattern; `Ctrl+Wheel` zoom where a
  canvas view exists.
- L3 never spawns `appcraft-cli.exe` as a child and never routes GUI
  actions through JSON-RPC.

## 3. Current status

Template bodies are manifests without compiled screens, so there are no
goldens to lock yet. This file defines the bar the first body v1 must
clear: one CRUD skeleton, one golden tree test, one smoke run.

## 4. Screenshots

`docs/screenshots/` (to be added with body v1):

- `studio-shell.png` - L3 app shell with blueprint picker and ledger view.
- `generated-crud.png` - output of the first desktop egui template.
