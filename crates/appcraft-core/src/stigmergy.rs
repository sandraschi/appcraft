//! Stigmergic Ledger and Environment-Mediated Task Engine.
//!
//! Agents coordinate indirectly by sensing and marking tasks in the
//! workspace's `_stigmergy_ledger` table, eliminating hierarchical prompt bottlenecks.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use crate::error::AppCraftError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Claimed,
    Completed,
    Failed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "PENDING",
            Self::Claimed => "CLAIMED",
            Self::Completed => "COMPLETED",
            Self::Failed => "FAILED",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "CLAIMED" => Self::Claimed,
            "COMPLETED" => Self::Completed,
            "FAILED" => Self::Failed,
            _ => Self::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StigmergicTask {
    pub id: i64,
    pub task_type: String,
    pub target_entity: String,
    pub payload: String,
    pub status: TaskStatus,
    pub agent_claimer: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

pub struct StigmergyEngine;

impl StigmergyEngine {
    pub fn init_ledger(store: &databasecraft_store::DatabaseStore) -> Result<(), AppCraftError> {
        store.execute(
            "CREATE TABLE IF NOT EXISTS _stigmergy_ledger (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task_type TEXT NOT NULL,
                target_entity TEXT NOT NULL,
                payload TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'PENDING',
                agent_claimer TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                completed_at DATETIME
            );
            CREATE INDEX IF NOT EXISTS idx_stigmergy_status ON _stigmergy_ledger(status);
            CREATE INDEX IF NOT EXISTS idx_stigmergy_type ON _stigmergy_ledger(task_type);",
        )
        .map_err(|e| AppCraftError::Storage(e.to_string()))?;
        Ok(())
    }

    pub fn emit_task(
        store: &databasecraft_store::DatabaseStore,
        task_type: &str,
        target_entity: &str,
        payload: &str,
    ) -> Result<i64, AppCraftError> {
        let sql = format!(
            "INSERT INTO _stigmergy_ledger (task_type, target_entity, payload, status) VALUES ('{}', '{}', '{}', 'PENDING');",
            task_type.replace('\'', "''"),
            target_entity.replace('\'', "''"),
            payload.replace('\'', "''")
        );
        store.execute(&sql).map_err(|e| AppCraftError::Storage(e.to_string()))?;

        let res = store.query("SELECT last_insert_rowid() AS id;").map_err(|e| AppCraftError::Storage(e.to_string()))?;
        let id = res.rows.first().and_then(|r| r.first()).and_then(|v| v.as_i64()).unwrap_or(0);
        Ok(id)
    }

    pub fn claim_task(
        store: &databasecraft_store::DatabaseStore,
        task_type: &str,
        agent_id: &str,
    ) -> Result<Option<StigmergicTask>, AppCraftError> {
        let sql = format!(
            "SELECT id, task_type, target_entity, payload, status, agent_claimer, created_at, completed_at \
             FROM _stigmergy_ledger WHERE status = 'PENDING' AND task_type = '{}' ORDER BY id ASC LIMIT 1;",
            task_type.replace('\'', "''")
        );
        let res = store.query(&sql).map_err(|e| AppCraftError::Storage(e.to_string()))?;
        if let Some(row) = res.rows.first() {
            let id = row.get(0).and_then(|v| v.as_i64()).unwrap_or(0);
            let t_type = row.get(1).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let target = row.get(2).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let payload = row.get(3).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let created = row.get(6).and_then(|v| v.as_str()).unwrap_or("").to_string();

            let update_sql = format!(
                "UPDATE _stigmergy_ledger SET status = 'CLAIMED', agent_claimer = '{}' WHERE id = {};",
                agent_id.replace('\'', "''"),
                id
            );
            store.execute(&update_sql).map_err(|e| AppCraftError::Storage(e.to_string()))?;

            Ok(Some(StigmergicTask {
                id,
                task_type: t_type,
                target_entity: target,
                payload,
                status: TaskStatus::Claimed,
                agent_claimer: Some(agent_id.to_string()),
                created_at: created,
                completed_at: None,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn complete_task(
        store: &databasecraft_store::DatabaseStore,
        task_id: i64,
        success: bool,
    ) -> Result<(), AppCraftError> {
        let status = if success { "COMPLETED" } else { "FAILED" };
        let sql = format!(
            "UPDATE _stigmergy_ledger SET status = '{status}', completed_at = CURRENT_TIMESTAMP WHERE id = {task_id};"
        );
        store.execute(&sql).map_err(|e| AppCraftError::Storage(e.to_string()))?;
        Ok(())
    }

    pub fn ledger_summary(store: &databasecraft_store::DatabaseStore) -> Result<serde_json::Value, AppCraftError> {
        let res = store.query("SELECT status, count(*) FROM _stigmergy_ledger GROUP BY status;")
            .map_err(|e| AppCraftError::Storage(e.to_string()))?;

        let mut pending = 0;
        let mut claimed = 0;
        let mut completed = 0;
        let mut failed = 0;

        for row in res.rows {
            let status = row.get(0).and_then(|v| v.as_str()).unwrap_or("");
            let count = row.get(1).and_then(|v| v.as_i64()).unwrap_or(0);
            match status {
                "PENDING" => pending = count,
                "CLAIMED" => claimed = count,
                "COMPLETED" => completed = count,
                "FAILED" => failed = count,
                _ => {}
            }
        }

        Ok(serde_json::json!({
            "pending": pending,
            "claimed": claimed,
            "completed": completed,
            "failed": failed,
            "total": pending + claimed + completed + failed,
        }))
    }
}
