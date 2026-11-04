//! Template definitions and code generation templates for AppCraft.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StackType {
    NativeDesktopEgui,
    FastApiReact,
    SingleBinaryAxumWebview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateManifest {
    pub id: String,
    pub name: String,
    pub stack: StackType,
    pub description: String,
    pub default_entrypoint: String,
}

pub fn available_templates() -> Vec<TemplateManifest> {
    vec![
        TemplateManifest {
            id: "desktop_egui".to_string(),
            name: "Sovereign Native Desktop (Rust + egui)".to_string(),
            stack: StackType::NativeDesktopEgui,
            description: "Zero-dependency 60fps native desktop application with embedded SQLite B-Tree engine.".to_string(),
            default_entrypoint: "src/main.rs".to_string(),
        },
        TemplateManifest {
            id: "fastapi_react".to_string(),
            name: "Modern Local Web (FastAPI + React/Tailwind)".to_string(),
            stack: StackType::FastApiReact,
            description: "High-productivity Python FastAPI backend with Vite + React + Tailwind frontend.".to_string(),
            default_entrypoint: "backend/main.py".to_string(),
        },
        TemplateManifest {
            id: "axum_webview".to_string(),
            name: "Single-Binary Hybrid (Rust Axum + Webview)".to_string(),
            stack: StackType::SingleBinaryAxumWebview,
            description: "Compiles complete HTML/JS/CSS bundle inside a single portable zero-runtime .exe file.".to_string(),
            default_entrypoint: "src/main.rs".to_string(),
        },
    ]
}
