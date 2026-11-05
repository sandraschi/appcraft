//! AppCraft MCP Server & Universal CLI.
//!
//! Follows the MCP 2024-11-05 standard: stdout is 100% JSON-RPC, diagnostics go to stderr.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use appcraft_core::{dispatch, Command, APP_VERSION, CONTROL_PORT};
use clap::{Parser, Subcommand};
use databasecraft_store::DatabaseStore;
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use tokio::runtime::Runtime;

#[derive(Parser)]
#[command(name = "appcraft-cli")]
#[command(about = "AppCraft: Stigmergic Local Application Factory & CLI", version = APP_VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Run as MCP 2024-11-05 stdio server
    #[arg(long)]
    mcp: bool,
    /// Custom database workspace file path
    #[arg(long)]
    db: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check AppCraft engine status and control port
    Status,
    /// List available UI and stack architecture templates
    Templates,
    /// List available turnkey domain blueprints (from DatabaseCraft)
    Blueprints,
    /// Initialize a new application workspace with domain blueprints
    Init {
        name: String,
        #[arg(long, default_value = "desktop_egui")]
        template: String,
        #[arg(long, default_value = "small_business")]
        blueprint: String,
        #[arg(long, default_value_t = true)]
        seed: bool,
        /// Emit generated app files here (enables real builds later)
        #[arg(long)]
        out_dir: Option<String>,
    },
    /// Inspect or manage stigmergic task ledger
    Stigmergy {
        #[arg(long)]
        summary: bool,
        #[arg(long)]
        claim_type: Option<String>,
        #[arg(long)]
        agent_id: Option<String>,
    },
    /// Compile a previously emitted application (real cargo build, no emulation)
    Compile {
        name: String,
        #[arg(long, default_value = "windows-x86_64")]
        target: String,
        /// Directory the app was emitted to (must contain Cargo.toml)
        #[arg(long)]
        out_dir: Option<String>,
    },
}

fn open_store(path: Option<&str>) -> DatabaseStore {
    match path {
        Some(p) => DatabaseStore::open(p).unwrap_or_else(|e| {
            eprintln!("Failed to open database at {p}: {e}");
            std::process::exit(1);
        }),
        None => DatabaseStore::open_in_memory().unwrap_or_else(|e| {
            eprintln!("Failed to open in-memory store: {e}");
            std::process::exit(1);
        }),
    }
}

fn main() {
    let cli = Cli::parse();
    let rt = Runtime::new().unwrap_or_else(|e| {
        eprintln!("Failed to create tokio runtime: {e}");
        std::process::exit(1);
    });

    let store = open_store(cli.db.as_deref());

    if cli.mcp {
        run_mcp_stdio(&rt, &store);
        return;
    }

    match cli.command {
        Some(Commands::Status) => {
            let res = rt.block_on(dispatch(Command::Status, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Templates) => {
            let res = rt.block_on(dispatch(Command::ListTemplates, &store));
            println!("{}", serde_json::to_string_pretty(&res["templates"]).unwrap_or_default());
        }
        Some(Commands::Blueprints) => {
            let res = rt.block_on(dispatch(Command::ListBlueprints, &store));
            println!("{}", serde_json::to_string_pretty(&res["blueprints"]).unwrap_or_default());
        }
        Some(Commands::Init { name, template, blueprint, seed, out_dir }) => {
            let res = rt.block_on(dispatch(
                Command::InitApp {
                    name,
                    template_id: template,
                    blueprint_id: Some(blueprint),
                    seed_data: Some(seed),
                    out_dir,
                },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Stigmergy { summary, claim_type, agent_id }) => {
            if let (Some(t_type), Some(a_id)) = (claim_type, agent_id) {
                let res = rt.block_on(dispatch(
                    Command::StigmergyClaim {
                        task_type: t_type,
                        agent_id: a_id,
                    },
                    &store,
                ));
                println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
            } else if summary {
                let res = rt.block_on(dispatch(Command::StigmergySummary, &store));
                println!("{}", serde_json::to_string_pretty(&res["ledger"]).unwrap_or_default());
            } else {
                let res = rt.block_on(dispatch(Command::StigmergySummary, &store));
                println!("{}", serde_json::to_string_pretty(&res["ledger"]).unwrap_or_default());
            }
        }
        Some(Commands::Compile { name, target, out_dir }) => {
            let res = rt.block_on(dispatch(
                Command::CompileArtifact {
                    app_name: name,
                    target_platform: target,
                    out_dir,
                },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        None => {
            println!("AppCraft CLI v{APP_VERSION} on Port {CONTROL_PORT}");
            println!("Use --help to view available commands or --mcp to start JSON-RPC server.");
        }
    }
}

fn run_mcp_stdio(rt: &Runtime, store: &DatabaseStore) {
    eprintln!("[appcraft-mcp] Starting MCP stdio loop on port {CONTROL_PORT}");
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let req: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let id = req.get("id").cloned().unwrap_or(Value::Null);
        let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");

        let resp = match method {
            "initialize" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "appcraft-mcp", "version": APP_VERSION }
                }
            }),
            "tools/list" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": [
                        { "name": "app_status", "description": "Check AppCraft engine status", "inputSchema": { "type": "object", "properties": {} } },
                        { "name": "app_list_templates", "description": "List architecture templates", "inputSchema": { "type": "object", "properties": {} } },
                        { "name": "app_list_blueprints", "description": "List turnkey domain database blueprints", "inputSchema": { "type": "object", "properties": {} } },
                        {
                            "name": "app_init",
                            "description": "Initialize a new local application workspace with blueprints and stigmergic tasks",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": { "type": "string" },
                                    "template": { "type": "string" },
                                    "blueprint": { "type": "string" },
                                    "seed": { "type": "boolean" },
                                    "out_dir": { "type": "string" }
                                },
                                "required": ["name"]
                            }
                        },
                        { "name": "app_stigmergy_summary", "description": "Get summary of pending/claimed/completed stigmergic tasks", "inputSchema": { "type": "object", "properties": {} } },
                        {
                            "name": "app_compile",
                            "description": "Real cargo build of a previously emitted app (out_dir must contain Cargo.toml) — never emulated",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": { "type": "string" },
                                    "target": { "type": "string" },
                                    "out_dir": { "type": "string" }
                                },
                                "required": ["name", "out_dir"]
                            }
                        }
                    ]
                }
            }),
            "tools/call" => {
                let params = req.get("params").cloned().unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                let tool_res = match tool_name {
                    "app_status" => rt.block_on(dispatch(Command::Status, store)),
                    "app_list_templates" => rt.block_on(dispatch(Command::ListTemplates, store)),
                    "app_list_blueprints" => rt.block_on(dispatch(Command::ListBlueprints, store)),
                    "app_init" => {
                        let name = args["name"].as_str().unwrap_or("App").to_string();
                        let template = args["template"].as_str().unwrap_or("desktop_egui").to_string();
                        let blueprint = args["blueprint"].as_str().map(|s| s.to_string());
                        let seed = args["seed"].as_bool();
                        let out_dir = args["out_dir"].as_str().map(|s| s.to_string());
                        rt.block_on(dispatch(Command::InitApp { name, template_id: template, blueprint_id: blueprint, seed_data: seed, out_dir }, store))
                    }
                    "app_compile" => {
                        let name = args["name"].as_str().unwrap_or("App").to_string();
                        let target = args["target"].as_str().unwrap_or("windows-x86_64").to_string();
                        let out_dir = args["out_dir"].as_str().map(|s| s.to_string());
                        rt.block_on(dispatch(Command::CompileArtifact { app_name: name, target_platform: target, out_dir }, store))
                    }
                    "app_stigmergy_summary" => rt.block_on(dispatch(Command::StigmergySummary, store)),
                    unknown => json!({ "success": false, "error": format!("Unknown tool '{unknown}'") })
                };

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "content": [{ "type": "text", "text": tool_res.to_string() }] }
                })
            }
            _ => json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32601, "message": "Method not found" } })
        };

        let _ = writeln!(stdout, "{}", resp.to_string());
        let _ = stdout.flush();
    }
}
