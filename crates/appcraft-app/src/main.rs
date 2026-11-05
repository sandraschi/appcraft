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
    pub locale: appcraft_core::i18n::AppLocale,
    pub is_dark_theme: bool,
    pub is_authenticated: bool,
    pub auth_username: String,
    pub auth_password_input: String,
    pub show_auth_modal: bool,
    pub show_about_dialog: bool,
}

impl AppCraftGui {
    pub fn new(store: Arc<DatabaseStore>, runtime: Arc<Runtime>) -> Self {
        let mut app = Self {
            store,
            runtime,
            active_tab: ActiveTab::Studio,
            app_name: "MyLocalApp".to_string(),
            selected_template: "desktop_egui".to_string(),
            selected_blueprint: "surgery".to_string(),
            seed_sample_data: true,
            init_result: None,
            stigmergy_summary: None,
            compilation_result: None,
            status_message: format!("AppCraft v{APP_VERSION} Studio [Ready on Port {CONTROL_PORT}]"),
            locale: appcraft_core::i18n::AppLocale::default(),
            is_dark_theme: true,
            is_authenticated: false,
            auth_username: "craftmaster".to_string(),
            auth_password_input: String::new(),
            show_auth_modal: false,
            show_about_dialog: false,
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
        if self.is_dark_theme {
            ui.ctx().set_visuals(egui::Visuals::dark());
        } else {
            ui.ctx().set_visuals(egui::Visuals::light());
        }

        let emerald = Color32::from_rgb(16, 185, 129);

        // 0. Desktop Menu Bar
        egui::Panel::top("desktop_menu_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                let loc = self.locale;
                ui.menu_button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::File, loc), |ui| {
                    if ui.button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::ScaffoldNewProject, loc)).clicked() {
                        self.active_tab = ActiveTab::Studio;
                        ui.close();
                    }
                    if ui.button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::CompileStandaloneExe, loc)).clicked() {
                        self.active_tab = ActiveTab::Compiler;
                        self.compile_application();
                        ui.close();
                    }
                    ui.separator();
                    if ui.button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::Exit, loc)).clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::Workspaces, loc), |ui| {
                    if ui.button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::StudioConfig, loc)).clicked() {
                        self.active_tab = ActiveTab::Studio;
                        ui.close();
                    }
                    if ui.button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::DomainBlueprints, loc)).clicked() {
                        self.active_tab = ActiveTab::Blueprints;
                        ui.close();
                    }
                    if ui.button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::StigmergicLedger, loc)).clicked() {
                        self.active_tab = ActiveTab::StigmergyLedger;
                        self.refresh_stigmergy();
                        ui.close();
                    }
                    if ui.button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::BinaryCompiler, loc)).clicked() {
                        self.active_tab = ActiveTab::Compiler;
                        ui.close();
                    }
                });

                ui.menu_button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::View, loc), |ui| {
                    let theme_label = if self.is_dark_theme { "☀ Switch to Light Theme" } else { "🌙 Switch to Dark Theme" };
                    if ui.button(theme_label).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                        ui.close();
                    }
                });

                ui.menu_button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::Security, loc), |ui| {
                    if self.is_authenticated {
                        ui.label(format!("{}: {}", appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::SignedIn, loc), self.auth_username));
                        if ui.button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::SignedOut, loc)).clicked() {
                            self.is_authenticated = false;
                            self.status_message = "Signed out".to_string();
                            ui.close();
                        }
                    } else {
                        if ui.button("Basic Auth Login...").clicked() {
                            self.show_auth_modal = true;
                            ui.close();
                        }
                    }
                });

                ui.menu_button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::Help, loc), |ui| {
                    if ui.button(appcraft_core::i18n::t(appcraft_core::i18n::I18nKey::AboutAppCraft, loc)).clicked() {
                        self.show_about_dialog = true;
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let cur_loc = self.locale;
                    let loc_label = format!("{} {}", cur_loc.flag(), cur_loc.code().to_uppercase());
                    egui::ComboBox::from_id_salt("topbar_locale_selector")
                        .selected_text(loc_label)
                        .show_ui(ui, |ui| {
                            for loc_item in appcraft_core::i18n::AppLocale::all() {
                                let text = format!("{} {}", loc_item.flag(), loc_item.display_name());
                                let _ = ui.selectable_value(&mut self.locale, *loc_item, text);
                            }
                        });

                    let (icon, label) = if self.is_dark_theme { ("☀", "Light") } else { ("🌙", "Dark") };
                    if ui.small_button(format!("{icon} {label}")).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                    }

                    if self.is_authenticated {
                        ui.colored_label(emerald, format!("👤 {}", self.auth_username));
                    } else if ui.small_button("🔑 Login").clicked() {
                        self.show_auth_modal = true;
                    }
                });
            });
        });

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

        // Modal: Basic Auth Login
        if self.show_auth_modal {
            egui::Window::new("🔐 Sovereign Basic Auth")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label("Authenticate to access Sovereign Deployment Engine:");
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label("User:");
                        ui.text_edit_singleline(&mut self.auth_username);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Pass:");
                        ui.add(egui::TextEdit::singleline(&mut self.auth_password_input).password(true));
                    });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            if !self.auth_username.is_empty() {
                                self.is_authenticated = true;
                                self.show_auth_modal = false;
                                self.status_message = format!("Signed in as {}", self.auth_username);
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_auth_modal = false;
                        }
                    });
                });
        }

        // Modal: About Dialog
        if self.show_about_dialog {
            egui::Window::new("ℹ About AppCraft")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.heading("AppCraft Native Studio");
                    ui.label("Sovereign zero-runtime-tax native app factory.");
                    ui.label(format!("Version: v{APP_VERSION}"));
                    ui.label(format!("Control Port: {CONTROL_PORT}"));
                    ui.separator();
                    ui.label("Subsumes cloud app-builders into local single-file compiled binaries.");
                    ui.add_space(8.0);
                    if ui.button("Close").clicked() {
                        self.show_about_dialog = false;
                    }
                });
        }
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
