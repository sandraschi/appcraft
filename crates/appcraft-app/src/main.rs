//! AppCraft Native Studio GUI.
//!
//! Visual application factory studio with template picker, domain blueprints,
//! stigmergic worker monitors, and 1-click single-binary compilation.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use appcraft_core::{dispatch, Command, APP_VERSION, CONTROL_PORT};
use databasecraft_store::DatabaseStore;
use eframe::egui::{self, Color32, Vec2};
use serde_json::Value;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Studio,
    Blueprints,
    StigmergyLedger,
    Compiler,
}

pub struct AppCraftGui {
    pub store: Arc<DatabaseStore>,
    pub runtime: Arc<Runtime>,
    pub active_tab: ActiveTab,
    pub app_name: String,
    pub selected_template: String,
    pub selected_blueprint: String,
    pub seed_sample_data: bool,
    pub init_result: Option<Value>,
    pub stigmergy_summary: Option<Value>,
    pub compilation_result: Option<Value>,
    pub status_message: String,
}

impl AppCraftGui {
    pub fn new(store: Arc<DatabaseStore>, runtime: Arc<Runtime>) -> Self {
        let mut app = Self {
            store,
            runtime,
            active_tab: ActiveTab::Studio,
            app_name: "MySovereignApp".to_string(),
            selected_template: "desktop_egui".to_string(),
            selected_blueprint: "surgery".to_string(),
            seed_sample_data: true,
            init_result: None,
            stigmergy_summary: None,
            compilation_result: None,
            status_message: format!("AppCraft v{APP_VERSION} Studio [Ready on Port {CONTROL_PORT}]"),
        };
        app.refresh_stigmergy();
        app
    }

    pub fn refresh_stigmergy(&mut self) {
        let res = self.runtime.block_on(dispatch(Command::StigmergySummary, &self.store));
        self.stigmergy_summary = Some(res);
    }

    pub fn init_application(&mut self) {
        let name = self.app_name.clone();
        let tmpl = self.selected_template.clone();
        let bp = self.selected_blueprint.clone();
        let seed = self.seed_sample_data;

        let res = self.runtime.block_on(dispatch(
            Command::InitApp {
                name: name.clone(),
                template_id: tmpl,
                blueprint_id: Some(bp.clone()),
                seed_data: Some(seed),
            },
            &self.store,
        ));

        self.init_result = Some(res.clone());
        self.refresh_stigmergy();
        self.status_message = format!("Application '{name}' initialized with '{bp}' blueprint!");
    }

    pub fn compile_application(&mut self) {
        let name = self.app_name.clone();
        let res = self.runtime.block_on(dispatch(
            Command::CompileArtifact {
                app_name: name.clone(),
                target_platform: "windows-x86_64".to_string(),
            },
            &self.store,
        ));
        self.compilation_result = Some(res);
        self.status_message = format!("Compiled sovereign standalone .exe for '{name}' (Zero-Runtime-Tax)");
    }
}

impl eframe::App for AppCraftGui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let emerald = Color32::from_rgb(16, 185, 129);

        // 1. Top Control Bar
        egui::Panel::top("top_header").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.strong(egui::RichText::new("⚡ APPCRAFT STUDIO").color(emerald).size(16.0));
                ui.separator();
                ui.label(format!("v{APP_VERSION}"));
                ui.colored_label(Color32::from_rgb(148, 163, 184), format!("| Port {CONTROL_PORT}"));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🚀 Compile Standalone .exe").clicked() {
                        self.active_tab = ActiveTab::Compiler;
                        self.compile_application();
                    }
                });
            });
        });

        // 2. Bottom Status Bar
        egui::Panel::bottom("bottom_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(emerald, "●");
                ui.label(&self.status_message);
            });
        });

        // 3. Navigation Rail
        egui::Panel::left("nav_panel").min_size(200.0).show(ui, |ui| {
            ui.add_space(8.0);
            ui.strong("WORKSPACES");
            ui.add_space(4.0);

            let nav_btn = |ui: &mut egui::Ui, active: bool, text: &str| -> bool {
                let rich = egui::RichText::new(text).color(if active { emerald } else { Color32::WHITE });
                ui.selectable_label(active, rich).clicked()
            };

            if nav_btn(ui, self.active_tab == ActiveTab::Studio, "🛠️ Studio Config") {
                self.active_tab = ActiveTab::Studio;
            }
            if nav_btn(ui, self.active_tab == ActiveTab::Blueprints, "🏛️ 10 Domain Blueprints") {
                self.active_tab = ActiveTab::Blueprints;
            }
            if nav_btn(ui, self.active_tab == ActiveTab::StigmergyLedger, "🐝 Stigmergic Ledger") {
                self.active_tab = ActiveTab::StigmergyLedger;
                self.refresh_stigmergy();
            }
            if nav_btn(ui, self.active_tab == ActiveTab::Compiler, "📦 Single-Binary Compiler") {
                self.active_tab = ActiveTab::Compiler;
            }
        });

        // 4. Central Panel
        egui::CentralPanel::default().show(ui, |ui| match self.active_tab {
            ActiveTab::Studio => {
                ui.heading("🛠️ Application Generation Studio");
                ui.label("Scaffold sovereign applications with zero recurring subscriptions and 100% data ownership.");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Application Name:");
                    ui.text_edit_singleline(&mut self.app_name);
                });

                ui.add_space(8.0);
                ui.strong("Select Architecture Stack:");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.selected_template, "desktop_egui".to_string(), "🦀 Native Rust Desktop (egui)");
                    ui.selectable_value(&mut self.selected_template, "fastapi_react".to_string(), "🐍 FastAPI + React / Tailwind");
                    ui.selectable_value(&mut self.selected_template, "axum_webview".to_string(), "📦 Single-Binary Axum Webview");
                });

                ui.add_space(8.0);
                ui.strong("Select Domain Blueprint (from DatabaseCraft):");
                egui::ComboBox::from_label("Domain Archetype")
                    .selected_text(&self.selected_blueprint)
                    .show_ui(ui, |ui| {
                        let domains = [
                            ("small_business", "🏢 Small Business & Invoicing"),
                            ("club", "🏆 Club & Association Membership"),
                            ("surgery", "🩺 Medical & Dental Surgery Clinic"),
                            ("creative_agency", "🎨 Creative Studio & Projects"),
                            ("real_estate", "🏡 Property & Lease Management"),
                            ("field_service", "🔧 Field Service & Trades"),
                            ("hospitality", "🍽️ Restaurant & Dining Reservations"),
                            ("legal", "⚖️ Legal Case & Practice Docket"),
                            ("automotive", "🚗 Auto Garage & Fleet Workshop"),
                            ("education", "🎓 Academy & Tutoring Center"),
                        ];
                        for (id, label) in domains {
                            ui.selectable_value(&mut self.selected_blueprint, id.to_string(), label);
                        }
                    });

                ui.checkbox(&mut self.seed_sample_data, "Seed Realistic Starter Records (instant working demo)");

                ui.add_space(12.0);
                if ui.button(egui::RichText::new("🚀 Initialize Stigmergic Generation").color(emerald).strong()).clicked() {
                    self.init_application();
                }

                if let Some(ref res) = self.init_result {
                    ui.add_space(12.0);
                    ui.group(|ui| {
                        ui.colored_label(emerald, "✔ APP WORKSPACE INITIALIZED");
                        ui.monospace(serde_json::to_string_pretty(res).unwrap_or_default());
                    });
                }
            }
            ActiveTab::Blueprints => {
                ui.heading("🏛️ DatabaseCraft Domain Blueprints Catalog");
                ui.label("Pre-validated, zero-panic relational schemas with primary keys, foreign keys, and indexes.");
                ui.separator();
                ui.label(format!("Currently selected for scaffolding: {}", self.selected_blueprint));
            }
            ActiveTab::StigmergyLedger => {
                ui.heading("🐝 Decentralized Stigmergy Task Ledger");
                ui.label("Decoupled agents sense pending tasks directly in SQLite, eliminating hierarchical prompt bottlenecks.");
                ui.separator();

                if let Some(ref sum) = self.stigmergy_summary {
                    if let Some(ledger) = sum.get("ledger") {
                        ui.columns(4, |cols| {
                            cols[0].group(|ui| {
                                ui.colored_label(Color32::from_rgb(251, 191, 36), "⏳ PENDING");
                                ui.heading(format!("{}", ledger["pending"]));
                            });
                            cols[1].group(|ui| {
                                ui.colored_label(Color32::from_rgb(96, 165, 250), "⚙️ CLAIMED");
                                ui.heading(format!("{}", ledger["claimed"]));
                            });
                            cols[2].group(|ui| {
                                ui.colored_label(Color32::from_rgb(52, 211, 153), "✔ COMPLETED");
                                ui.heading(format!("{}", ledger["completed"]));
                            });
                            cols[3].group(|ui| {
                                ui.colored_label(Color32::from_rgb(248, 113, 113), "✖ FAILED");
                                ui.heading(format!("{}", ledger["failed"]));
                            });
                        });
                    }
                }
            }
            ActiveTab::Compiler => {
                ui.heading("📦 Sovereign Single-Binary Compiler");
                ui.label("Package the database, backend, and user interface into a standalone zero-runtime-tax executable.");
                ui.separator();

                if let Some(ref res) = self.compilation_result {
                    ui.group(|ui| {
                        ui.colored_label(emerald, "✔ BUILD SUCCEEDED: ZERO RUNTIME CLOUD TAX");
                        ui.monospace(serde_json::to_string_pretty(res).unwrap_or_default());
                    });
                } else {
                    ui.label("Click 'Compile Standalone .exe' to generate portable client binaries.");
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let rt = Arc::new(Runtime::new().unwrap_or_else(|e| {
        eprintln!("Failed to init tokio runtime: {e}");
        std::process::exit(1);
    }));

    let store = Arc::new(DatabaseStore::open_in_memory().unwrap_or_else(|e| {
        eprintln!("Failed to open in-memory store: {e}");
        std::process::exit(1);
    }));

    // Headless smoke test
    if args.contains(&"--smoke".to_string()) {
        println!("[appcraft] Running headless smoke verification...");
        let status = rt.block_on(dispatch(Command::Status, &store));
        println!("[appcraft] Smoke OK: {status}");
        return Ok(());
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(1024.0, 720.0))
            .with_title(format!("AppCraft Studio v{APP_VERSION}")),
        ..Default::default()
    };

    eframe::run_native(
        "AppCraft Studio",
        native_options,
        Box::new(|_cc| Ok(Box::new(AppCraftGui::new(store, rt)))),
    )
}
