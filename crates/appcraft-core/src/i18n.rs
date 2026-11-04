//! AppCraft Internationalization (i18n) Engine.
//!
//! Follows the fleet standard: typed locale enums, zero-panic string lookups,
//! multi-language translations (EN, DE, FR, ES, JA, KO, ZH),
//! covering application scaffolding, domain blueprints, stigmergy, and compilation.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppLocale {
    #[default]
    En, // English (Global Default)
    De, // German
    Fr, // French
    Es, // Spanish
    Ja, // Japanese
    Ko, // Korean
    Zh, // Chinese
}

impl AppLocale {
    pub fn all() -> &'static [AppLocale] {
        &[
            AppLocale::En,
            AppLocale::De,
            AppLocale::Fr,
            AppLocale::Es,
            AppLocale::Ja,
            AppLocale::Ko,
            AppLocale::Zh,
        ]
    }

    pub fn code(&self) -> &'static str {
        match self {
            AppLocale::En => "en",
            AppLocale::De => "de",
            AppLocale::Fr => "fr",
            AppLocale::Es => "es",
            AppLocale::Ja => "ja",
            AppLocale::Ko => "ko",
            AppLocale::Zh => "zh",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AppLocale::En => "English (US/UK)",
            AppLocale::De => "Deutsch (DE/AT/CH)",
            AppLocale::Fr => "Français (FR)",
            AppLocale::Es => "Español (ES/LATAM)",
            AppLocale::Ja => "日本語 (JA)",
            AppLocale::Ko => "한국어 (KO)",
            AppLocale::Zh => "中文 (ZH)",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            AppLocale::En => "🇬🇧",
            AppLocale::De => "🇩🇪",
            AppLocale::Fr => "🇫🇷",
            AppLocale::Es => "🇪🇸",
            AppLocale::Ja => "🇯🇵",
            AppLocale::Ko => "🇰🇷",
            AppLocale::Zh => "🇨🇳",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum I18nKey {
    // Menu Bar Top Items
    File,
    Workspaces,
    View,
    Security,
    Help,

    // File Actions
    ScaffoldNewProject,
    CompileStandaloneExe,
    Exit,

    // Workspaces Tabs
    StudioConfig,
    DomainBlueprints,
    StigmergicLedger,
    BinaryCompiler,

    // Controls & Forms
    AppName,
    Template,
    Blueprint,
    SeedSampleData,
    InitializeApp,
    CompileApp,

    // Badges & Status
    SignedIn,
    SignedOut,
    Login,
    AboutAppCraft,
    StatusReady,
}

pub fn t(key: I18nKey, locale: AppLocale) -> &'static str {
    match (key, locale) {
        // Menu Bar Top Items
        (I18nKey::File, AppLocale::En) => "File",
        (I18nKey::File, AppLocale::De) => "Datei",
        (I18nKey::File, AppLocale::Fr) => "Fichier",
        (I18nKey::File, AppLocale::Es) => "Archivo",
        (I18nKey::File, AppLocale::Ja) => "ファイル",
        (I18nKey::File, AppLocale::Ko) => "파일",
        (I18nKey::File, AppLocale::Zh) => "文件",

        (I18nKey::Workspaces, AppLocale::En) => "Workspaces",
        (I18nKey::Workspaces, AppLocale::De) => "Arbeitsbereiche",
        (I18nKey::Workspaces, AppLocale::Fr) => "Espaces de travail",
        (I18nKey::Workspaces, AppLocale::Es) => "Espacios de trabajo",
        (I18nKey::Workspaces, AppLocale::Ja) => "ワークスペース",
        (I18nKey::Workspaces, AppLocale::Ko) => "작업공간",
        (I18nKey::Workspaces, AppLocale::Zh) => "工作区",

        (I18nKey::View, AppLocale::En) => "View",
        (I18nKey::View, AppLocale::De) => "Ansicht",
        (I18nKey::View, AppLocale::Fr) => "Affichage",
        (I18nKey::View, AppLocale::Es) => "Vista",
        (I18nKey::View, AppLocale::Ja) => "表示",
        (I18nKey::View, AppLocale::Ko) => "보기",
        (I18nKey::View, AppLocale::Zh) => "视图",

        (I18nKey::Security, AppLocale::En) => "Security",
        (I18nKey::Security, AppLocale::De) => "Sicherheit",
        (I18nKey::Security, AppLocale::Fr) => "Sécurité",
        (I18nKey::Security, AppLocale::Es) => "Seguridad",
        (I18nKey::Security, AppLocale::Ja) => "セキュリティ",
        (I18nKey::Security, AppLocale::Ko) => "보안",
        (I18nKey::Security, AppLocale::Zh) => "安全与鉴权",

        (I18nKey::Help, AppLocale::En) => "Help",
        (I18nKey::Help, AppLocale::De) => "Hilfe",
        (I18nKey::Help, AppLocale::Fr) => "Aide",
        (I18nKey::Help, AppLocale::Es) => "Ayuda",
        (I18nKey::Help, AppLocale::Ja) => "ヘルプ",
        (I18nKey::Help, AppLocale::Ko) => "도움말",
        (I18nKey::Help, AppLocale::Zh) => "帮助与文档",

        // File Actions
        (I18nKey::ScaffoldNewProject, AppLocale::En) => "Scaffold New Project...",
        (I18nKey::ScaffoldNewProject, AppLocale::De) => "Neues Projekt gerüsten...",
        (I18nKey::ScaffoldNewProject, AppLocale::Fr) => "Échafauder un nouveau projet...",
        (I18nKey::ScaffoldNewProject, AppLocale::Es) => "Crear estructura de nuevo proyecto...",
        (I18nKey::ScaffoldNewProject, AppLocale::Ja) => "新規プロジェクトを足場作成...",
        (I18nKey::ScaffoldNewProject, AppLocale::Ko) => "새 프로젝트 스캐폴딩...",
        (I18nKey::ScaffoldNewProject, AppLocale::Zh) => "脚手架生成新工程...",

        (I18nKey::CompileStandaloneExe, AppLocale::En) => "Compile Standalone .exe",
        (I18nKey::CompileStandaloneExe, AppLocale::De) => "Eigenständige .exe kompilieren",
        (I18nKey::CompileStandaloneExe, AppLocale::Fr) => "Compiler l'exécutable autonome .exe",
        (I18nKey::CompileStandaloneExe, AppLocale::Es) => "Compilar .exe independiente",
        (I18nKey::CompileStandaloneExe, AppLocale::Ja) => "単一バイナリ .exe をコンパイル",
        (I18nKey::CompileStandaloneExe, AppLocale::Ko) => "단일 실행 파일 (.exe) 컴파일",
        (I18nKey::CompileStandaloneExe, AppLocale::Zh) => "编译独立单文件可执行程序 (.exe)",

        (I18nKey::Exit, AppLocale::En) => "Exit",
        (I18nKey::Exit, AppLocale::De) => "Beenden",
        (I18nKey::Exit, AppLocale::Fr) => "Quitter",
        (I18nKey::Exit, AppLocale::Es) => "Salir",
        (I18nKey::Exit, AppLocale::Ja) => "終了",
        (I18nKey::Exit, AppLocale::Ko) => "종료",
        (I18nKey::Exit, AppLocale::Zh) => "退出",

        // Workspaces Tabs
        (I18nKey::StudioConfig, AppLocale::En) => "Studio Config",
        (I18nKey::StudioConfig, AppLocale::De) => "Studio-Konfiguration",
        (I18nKey::StudioConfig, AppLocale::Fr) => "Configuration Studio",
        (I18nKey::StudioConfig, AppLocale::Es) => "Configuración de Studio",
        (I18nKey::StudioConfig, AppLocale::Ja) => "スタジオ設定",
        (I18nKey::StudioConfig, AppLocale::Ko) => "스튜디오 설정",
        (I18nKey::StudioConfig, AppLocale::Zh) => "工作台配置",

        (I18nKey::DomainBlueprints, AppLocale::En) => "Domain Blueprints",
        (I18nKey::DomainBlueprints, AppLocale::De) => "Domänen-Baupläne",
        (I18nKey::DomainBlueprints, AppLocale::Fr) => "Plans directeurs de domaine",
        (I18nKey::DomainBlueprints, AppLocale::Es) => "Planos de dominio",
        (I18nKey::DomainBlueprints, AppLocale::Ja) => "ドメイン・ブループリント",
        (I18nKey::DomainBlueprints, AppLocale::Ko) => "도메인 블루프린트",
        (I18nKey::DomainBlueprints, AppLocale::Zh) => "领域架构设计图",

        (I18nKey::StigmergicLedger, AppLocale::En) => "Stigmergic Ledger",
        (I18nKey::StigmergicLedger, AppLocale::De) => "Stigmergisches Hauptbuch",
        (I18nKey::StigmergicLedger, AppLocale::Fr) => "Grand livre stigmergique",
        (I18nKey::StigmergicLedger, AppLocale::Es) => "Libro mayor estigmérgico",
        (I18nKey::StigmergicLedger, AppLocale::Ja) => "スティグマジー協調元帳",
        (I18nKey::StigmergicLedger, AppLocale::Ko) => "스티그머지 협동 원장",
        (I18nKey::StigmergicLedger, AppLocale::Zh) => "蚁群协同任务账本",

        (I18nKey::BinaryCompiler, AppLocale::En) => "Binary Compiler",
        (I18nKey::BinaryCompiler, AppLocale::De) => "Binär-Compiler",
        (I18nKey::BinaryCompiler, AppLocale::Fr) => "Compilateur binaire",
        (I18nKey::BinaryCompiler, AppLocale::Es) => "Compilador binario",
        (I18nKey::BinaryCompiler, AppLocale::Ja) => "バイナリ・コンパイラ",
        (I18nKey::BinaryCompiler, AppLocale::Ko) => "바이너리 컴파일러",
        (I18nKey::BinaryCompiler, AppLocale::Zh) => "二进制编译生成器",

        // Controls & Forms
        (I18nKey::AppName, AppLocale::En) => "Application Name",
        (I18nKey::AppName, AppLocale::De) => "Anwendungsname",
        (I18nKey::AppName, AppLocale::Fr) => "Nom de l'application",
        (I18nKey::AppName, AppLocale::Es) => "Nombre de la aplicación",
        (I18nKey::AppName, AppLocale::Ja) => "アプリケーション名",
        (I18nKey::AppName, AppLocale::Ko) => "애플리케이션 이름",
        (I18nKey::AppName, AppLocale::Zh) => "应用系统名称",

        (I18nKey::Template, AppLocale::En) => "Template",
        (I18nKey::Template, AppLocale::De) => "Vorlage",
        (I18nKey::Template, AppLocale::Fr) => "Modèle",
        (I18nKey::Template, AppLocale::Es) => "Plantilla",
        (I18nKey::Template, AppLocale::Ja) => "テンプレート",
        (I18nKey::Template, AppLocale::Ko) => "템플릿",
        (I18nKey::Template, AppLocale::Zh) => "脚手架模板",

        (I18nKey::Blueprint, AppLocale::En) => "Database Blueprint",
        (I18nKey::Blueprint, AppLocale::De) => "Datenbank-Bauplan",
        (I18nKey::Blueprint, AppLocale::Fr) => "Plan de base de données",
        (I18nKey::Blueprint, AppLocale::Es) => "Plano de base de datos",
        (I18nKey::Blueprint, AppLocale::Ja) => "データベース・ブループリント",
        (I18nKey::Blueprint, AppLocale::Ko) => "데이터베이스 청사진",
        (I18nKey::Blueprint, AppLocale::Zh) => "领域数据库架构",

        (I18nKey::SeedSampleData, AppLocale::En) => "Seed Realistic Sample Data",
        (I18nKey::SeedSampleData, AppLocale::De) => "Realistische Beispieldaten befüllen",
        (I18nKey::SeedSampleData, AppLocale::Fr) => "Remplir avec des données d'exemple réalistes",
        (I18nKey::SeedSampleData, AppLocale::Es) => "Sembrar datos de muestra realistas",
        (I18nKey::SeedSampleData, AppLocale::Ja) => "リアルな初期サンプルデータを投入",
        (I18nKey::SeedSampleData, AppLocale::Ko) => "실제 샘플 데이터 시딩",
        (I18nKey::SeedSampleData, AppLocale::Zh) => "自动注入全保真业务仿真数据",

        (I18nKey::InitializeApp, AppLocale::En) => "Initialize Application",
        (I18nKey::InitializeApp, AppLocale::De) => "Anwendung initialisieren",
        (I18nKey::InitializeApp, AppLocale::Fr) => "Initialiser l'application",
        (I18nKey::InitializeApp, AppLocale::Es) => "Inicializar aplicación",
        (I18nKey::InitializeApp, AppLocale::Ja) => "アプリケーションを初期化",
        (I18nKey::InitializeApp, AppLocale::Ko) => "애플리케이션 초기화",
        (I18nKey::InitializeApp, AppLocale::Zh) => "初始化创建应用程序",

        (I18nKey::CompileApp, AppLocale::En) => "Compile Single Binary",
        (I18nKey::CompileApp, AppLocale::De) => "Einzelne Binärdatei kompilieren",
        (I18nKey::CompileApp, AppLocale::Fr) => "Compiler en binaire unique",
        (I18nKey::CompileApp, AppLocale::Es) => "Compilar binario único",
        (I18nKey::CompileApp, AppLocale::Ja) => "単一バイナリをビルド",
        (I18nKey::CompileApp, AppLocale::Ko) => "단일 바이너리 빌드",
        (I18nKey::CompileApp, AppLocale::Zh) => "一键编译单文件二进制",

        // Badges & Status
        (I18nKey::SignedIn, AppLocale::En) => "Signed In",
        (I18nKey::SignedIn, AppLocale::De) => "Angemeldet",
        (I18nKey::SignedIn, AppLocale::Fr) => "Connecté",
        (I18nKey::SignedIn, AppLocale::Es) => "Conectado",
        (I18nKey::SignedIn, AppLocale::Ja) => "ログイン済み",
        (I18nKey::SignedIn, AppLocale::Ko) => "로그인됨",
        (I18nKey::SignedIn, AppLocale::Zh) => "已认证登录",

        (I18nKey::SignedOut, AppLocale::En) => "Signed Out",
        (I18nKey::SignedOut, AppLocale::De) => "Abgemeldet",
        (I18nKey::SignedOut, AppLocale::Fr) => "Déconnecté",
        (I18nKey::SignedOut, AppLocale::Es) => "Desconectado",
        (I18nKey::SignedOut, AppLocale::Ja) => "ログアウト済み",
        (I18nKey::SignedOut, AppLocale::Ko) => "로그아웃됨",
        (I18nKey::SignedOut, AppLocale::Zh) => "已退出登录",

        (I18nKey::Login, AppLocale::En) => "Login",
        (I18nKey::Login, AppLocale::De) => "Anmelden",
        (I18nKey::Login, AppLocale::Fr) => "Connexion",
        (I18nKey::Login, AppLocale::Es) => "Iniciar sesión",
        (I18nKey::Login, AppLocale::Ja) => "ログイン",
        (I18nKey::Login, AppLocale::Ko) => "로그인",
        (I18nKey::Login, AppLocale::Zh) => "登录",

        (I18nKey::AboutAppCraft, AppLocale::En) => "About AppCraft",
        (I18nKey::AboutAppCraft, AppLocale::De) => "Über AppCraft",
        (I18nKey::AboutAppCraft, AppLocale::Fr) => "À propos d'AppCraft",
        (I18nKey::AboutAppCraft, AppLocale::Es) => "Acerca de AppCraft",
        (I18nKey::AboutAppCraft, AppLocale::Ja) => "AppCraft について",
        (I18nKey::AboutAppCraft, AppLocale::Ko) => "AppCraft 정보",
        (I18nKey::AboutAppCraft, AppLocale::Zh) => "关于 AppCraft",

        (I18nKey::StatusReady, AppLocale::En) => "AppCraft Sovereign Studio Ready",
        (I18nKey::StatusReady, AppLocale::De) => "AppCraft Sovereign Studio bereit",
        (I18nKey::StatusReady, AppLocale::Fr) => "Studio souverain AppCraft prêt",
        (I18nKey::StatusReady, AppLocale::Es) => "Estudio soberano AppCraft listo",
        (I18nKey::StatusReady, AppLocale::Ja) => "AppCraft ソブリンスタジオ 準備完了",
        (I18nKey::StatusReady, AppLocale::Ko) => "AppCraft 주권 스튜디오 준비완료",
        (I18nKey::StatusReady, AppLocale::Zh) => "AppCraft 自主应用工坊就绪",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appcraft_i18n_locales() {
        assert_eq!(AppLocale::all().len(), 7);
        for loc in AppLocale::all() {
            assert!(!loc.code().is_empty());
            assert!(!loc.display_name().is_empty());
            assert!(!loc.flag().is_empty());
            assert!(!t(I18nKey::File, *loc).is_empty());
            assert!(!t(I18nKey::Workspaces, *loc).is_empty());
            assert!(!t(I18nKey::View, *loc).is_empty());
            assert!(!t(I18nKey::Security, *loc).is_empty());
            assert!(!t(I18nKey::Help, *loc).is_empty());
            assert!(!t(I18nKey::StatusReady, *loc).is_empty());
        }
    }
}
