//! AppCraft Core Library & Universal Command Bus.
//!
//! Handles project initialization, blueprint injection from DatabaseCraft,
//! stigmergic worker coordination, and standalone app compilation.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

pub mod error;
pub mod i18n;
pub mod stigmergy;

use crate::stigmergy::StigmergyEngine;
use databasecraft_core::blueprints::{available_blueprints, construct_blueprint};
use databasecraft_store::DatabaseStore;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const CONTROL_PORT: u16 = 14115;
pub const APP_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", content = "params")]
pub enum Command {
    Status,
    ListTemplates,
    ListBlueprints,
    InitApp {
        name: String,
        template_id: String,
        blueprint_id: Option<String>,
        seed_data: Option<bool>,
        out_dir: Option<String>,
    },
    StigmergyEmit {
        task_type: String,
        target_entity: String,
        payload: String,
    },
    StigmergyClaim {
        task_type: String,
        agent_id: String,
    },
    StigmergyComplete {
        task_id: i64,
        success: bool,
    },
    StigmergySummary,
    CompileArtifact {
        app_name: String,
        target_platform: String,
        out_dir: Option<String>,
    },
}

/// Universal Command Dispatch Bus following the ArtCraft Fleet standard.
pub async fn dispatch(cmd: Command, store: &DatabaseStore) -> Value {
    match cmd {
        Command::Status => {
            json!({
                "app": "appcraft",
                "version": APP_VERSION,
                "control_port": CONTROL_PORT,
                "status": "ready",
                "success": true
            })
        }
        Command::ListTemplates => {
            let tmpls = appcraft_templates::available_templates();
            json!({
                "success": true,
                "templates": tmpls
            })
        }
        Command::ListBlueprints => {
            let bps = available_blueprints();
            json!({
                "success": true,
                "blueprints": bps
            })
        }
        Command::InitApp {
            name,
            template_id,
            blueprint_id,
            seed_data,
            out_dir,
        } => {
            let bp_name = blueprint_id.unwrap_or_else(|| "small_business".to_string());
            let seed = seed_data.unwrap_or(true);

            // Initialize Stigmergy ledger
            if let Err(e) = StigmergyEngine::init_ledger(store) {
                return json!({ "success": false, "error": e.to_string() });
            }

            // File-backed generation: the schema is constructed into the
            // app's own .dbcraft file so the emitted app is self-contained.
            // In-memory (default): construct into the caller's store, no files.
            let (constr, emitted) = match out_dir {
                Some(dir) => {
                    if template_id != "desktop_egui" {
                        return json!({ "success": false, "error": format!("template bodies not yet implemented for '{template_id}' (only desktop_egui emits files)") });
                    }
                    let slug = appcraft_templates::slugify(&name);
                    let db_file = format!("{slug}.dbcraft");
                    if let Err(e) = std::fs::create_dir_all(&dir) {
                        return json!({ "success": false, "error": e.to_string() });
                    }
                    let db_path = std::path::PathBuf::from(&dir).join(&db_file);
                    let db_str = match db_path.to_str() {
                        Some(s) => s.to_string(),
                        None => return json!({ "success": false, "error": "out_dir is not valid UTF-8" }),
                    };
                    let file_store = match DatabaseStore::open(&db_str) {
                        Ok(s) => s,
                        Err(e) => return json!({ "success": false, "error": e.to_string() }),
                    };
                    let constr = match construct_blueprint(&file_store, &bp_name, seed, true) {
                        Ok(c) => c,
                        Err(e) => return json!({ "success": false, "error": e.to_string() }),
                    };
                    let files = appcraft_templates::render_desktop_egui(
                        &name,
                        &slug,
                        &db_file,
                        &constr.tables_created,
                        appcraft_templates::DEFAULT_PORTAL_PORT,
                    );
                    let mut written = Vec::new();
                    for (rel, content) in &files {
                        let dest = std::path::PathBuf::from(&dir).join(rel);
                        if let Some(parent) = dest.parent() {
                            if let Err(e) = std::fs::create_dir_all(parent) {
                                return json!({ "success": false, "error": e.to_string() });
                            }
                        }
                        if let Err(e) = std::fs::write(&dest, content) {
                            return json!({ "success": false, "error": e.to_string() });
                        }
                        written.push(rel.clone());
                    }
                    let emitted = json!({ "dir": dir, "db_file": db_file, "files": written });
                    (constr, Some(emitted))
                }
                None => {
                    let constr = match construct_blueprint(store, &bp_name, seed, true) {
                        Ok(c) => c,
                        Err(e) => return json!({ "success": false, "error": e.to_string() }),
                    };
                    (constr, None)
                }
            };

            // Emit initial stigmergic tasks for each constructed table
            let mut emitted_count = 0;
            for tbl in &constr.tables_created {
                let _ = StigmergyEngine::emit_task(
                    store,
                    "API_SCAFFOLD",
                    tbl,
                    &format!("Generate CRUD endpoints for table '{tbl}'"),
                );
                let _ = StigmergyEngine::emit_task(
                    store,
                    "UI_CARD_VIEW",
                    tbl,
                    &format!("Generate responsive card & table view for table '{tbl}'"),
                );
                emitted_count += 2;
            }

            let mut res = json!({
                "success": true,
                "app_name": name,
                "template": template_id,
                "blueprint": constr.blueprint_name,
                "tables_scaffolded": constr.tables_created,
                "sample_records_seeded": constr.sample_records_seeded,
                "stigmergic_tasks_emitted": emitted_count,
                "status": "INITIALIZED"
            });
            if let Some(e) = emitted {
                res["emitted"] = e;
            }
            res
        }
        Command::StigmergyEmit {
            task_type,
            target_entity,
            payload,
        } => {
            match StigmergyEngine::emit_task(store, &task_type, &target_entity, &payload) {
                Ok(id) => json!({ "success": true, "task_id": id }),
                Err(e) => json!({ "success": false, "error": e.to_string() }),
            }
        }
        Command::StigmergyClaim {
            task_type,
            agent_id,
        } => {
            match StigmergyEngine::claim_task(store, &task_type, &agent_id) {
                Ok(Some(task)) => json!({ "success": true, "task": task }),
                Ok(None) => json!({ "success": true, "task": null, "message": "no pending tasks matching criteria" }),
                Err(e) => json!({ "success": false, "error": e.to_string() }),
            }
        }
        Command::StigmergyComplete { task_id, success } => {
            match StigmergyEngine::complete_task(store, task_id, success) {
                Ok(()) => json!({ "success": true, "task_id": task_id, "completed": true }),
                Err(e) => json!({ "success": false, "error": e.to_string() }),
            }
        }
        Command::StigmergySummary => {
            match StigmergyEngine::ledger_summary(store) {
                Ok(summary) => json!({ "success": true, "ledger": summary }),
                Err(e) => json!({ "success": false, "error": e.to_string() }),
            }
        }
        Command::CompileArtifact {
            app_name,
            target_platform: _,
            out_dir,
        } => {
            let dir = match out_dir {
                Some(d) => d,
                None => return json!({ "success": false, "error": "refusing emulated build: init with --out-dir first (or pass out_dir) — no fake COMPILED receipts" }),
            };
            let manifest = std::path::PathBuf::from(&dir).join("Cargo.toml");
            let manifest_str = match manifest.to_str() {
                Some(s) => s.to_string(),
                None => return json!({ "success": false, "error": "out_dir is not valid UTF-8" }),
            };
            let started = std::time::Instant::now();
            let build = tokio::process::Command::new("cargo")
                .arg("build")
                .arg("--manifest-path")
                .arg(&manifest_str)
                .output();
            match tokio::time::timeout(std::time::Duration::from_secs(1200), build).await {
                Err(_) => json!({ "success": false, "app_name": app_name, "timeout_secs": 1200, "status": "BUILD_TIMEOUT" }),
                Ok(Err(e)) => json!({ "success": false, "app_name": app_name, "error": e.to_string(), "status": "BUILD_SPAWN_FAILED" }),
                Ok(Ok(out)) => {
                    let ok = out.status.success();
                    let tail = |b: &[u8]| -> String {
                        let s = String::from_utf8_lossy(b);
                        let chars: Vec<char> = s.chars().collect();
                        let n = chars.len();
                        let start = n.saturating_sub(2000);
                        chars[start..].iter().collect()
                    };
                    json!({
                        "success": ok,
                        "app_name": app_name,
                        "exit_code": out.status.code(),
                        "elapsed_secs": started.elapsed().as_secs(),
                        "stdout_tail": tail(&out.stdout),
                        "stderr_tail": tail(&out.stderr),
                        "status": if ok { "COMPILED" } else { "BUILD_FAILED" }
                    })
                }
            }
        }
    }
}
