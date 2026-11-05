//! Integration and dispatch verification tests for AppCraft.
use appcraft_core::{dispatch, Command};
use databasecraft_store::DatabaseStore;

#[tokio::test]
async fn test_appcraft_status_and_control_port() -> Result<(), Box<dyn std::error::Error>> {
    let store = DatabaseStore::open_in_memory()?;
    let res = dispatch(Command::Status, &store).await;
    assert_eq!(res["success"], true);
    assert_eq!(res["app"], "appcraft");
    assert_eq!(res["control_port"], 14115);
    Ok(())
}

#[tokio::test]
async fn test_appcraft_init_with_domain_blueprint_and_stigmergy() -> Result<(), Box<dyn std::error::Error>> {
    let store = DatabaseStore::open_in_memory()?;

    // 1. Initialize an app with medical surgery blueprint
    let init_res = dispatch(
        Command::InitApp {
            name: "Oakhaven Surgery Assistant".to_string(),
            template_id: "desktop_egui".to_string(),
            blueprint_id: Some("surgery".to_string()),
            seed_data: Some(true),
            out_dir: None,
        },
        &store,
    )
    .await;

    assert_eq!(init_res["success"], true);
    assert_eq!(init_res["blueprint"], "Medical & Dental Surgery Clinic");
    assert_eq!(init_res["tables_scaffolded"].as_array().map(|a| a.len()), Some(6));
    let emitted = init_res["stigmergic_tasks_emitted"].as_u64().unwrap_or(0);
    assert!(emitted >= 12); // 2 tasks (API + UI) per 6 tables

    // 2. Query Stigmergy summary
    let sum_res = dispatch(Command::StigmergySummary, &store).await;
    assert_eq!(sum_res["success"], true);
    assert_eq!(sum_res["ledger"]["pending"], emitted as i64);

    // 3. Worker agent claims a task
    let claim_res = dispatch(
        Command::StigmergyClaim {
            task_type: "API_SCAFFOLD".to_string(),
            agent_id: "agent_backend_specialist_01".to_string(),
        },
        &store,
    )
    .await;

    assert_eq!(claim_res["success"], true);
    let task_id = claim_res["task"]["id"].as_i64().ok_or("missing task id")?;

    // 4. Worker completes the task
    let comp_res = dispatch(
        Command::StigmergyComplete {
            task_id,
            success: true,
        },
        &store,
    )
    .await;
    assert_eq!(comp_res["success"], true);

    // 5. Verify updated summary
    let updated_sum = dispatch(Command::StigmergySummary, &store).await;
    assert_eq!(updated_sum["ledger"]["completed"], 1);

    Ok(())
}

#[tokio::test]
async fn test_appcraft_init_emits_desktop_egui_tree() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::temp_dir().join(format!("appcraft-golden-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let store = DatabaseStore::open_in_memory()?;

    // Golden fixture: fixed name + surgery blueprint, no seed (deterministic).
    let init_res = dispatch(
        Command::InitApp {
            name: "Golden Surgery".to_string(),
            template_id: "desktop_egui".to_string(),
            blueprint_id: Some("surgery".to_string()),
            seed_data: Some(false),
            out_dir: Some(dir.to_string_lossy().to_string()),
        },
        &store,
    )
    .await;

    assert_eq!(init_res["success"], true);
    let files = init_res["emitted"]["files"]
        .as_array()
        .ok_or("missing emitted.files")?;
    assert_eq!(files.len(), 7);

    // Byte-identical against the checked-in golden tree.
    for rel in ["Cargo.toml", "src/main.rs", "portal/main.rs", "README.md", ".gitignore", "start.ps1", "start.bat"] {
        let got = std::fs::read_to_string(dir.join(rel))?;
        let want = std::fs::read_to_string(format!("tests/golden/golden-surgery/{rel}"))?;
        assert_eq!(got, want, "mismatch in {rel}");
    }
    // The app database is emitted too (schema only, no seed here).
    assert!(dir.join("golden-surgery.dbcraft").exists());

    let _ = std::fs::remove_dir_all(&dir);
    Ok(())
}
