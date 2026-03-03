use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "varchar20")]
pub enum StageStatus {
    Pending,
    Running,
    Success,
    Failed,
}

impl Default for StageStatus {
    fn default() -> Self {
        StageStatus::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    pub id: Uuid,
    pub name: String,
    pub order: i32,
    pub status: StageStatus,
    pub pipeline_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub stages: Vec<Stage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePipelineDto {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePipelineDto {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStageDto {
    pub pipeline_id: String,
    pub name: String,
    pub order: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStageStatusDto {
    pub status: StageStatus,
}

pub mod state;
pub use state::AppState;
