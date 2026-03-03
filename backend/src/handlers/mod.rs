use crate::models::{AppState, CreatePipelineDto, CreateStageDto, Pipeline, Stage, StageStatus, UpdatePipelineDto, UpdateStageStatusDto};
use actix_web::{web, HttpResponse, Result};
use sqlx::Row;

pub async fn get_pipelines(state: web::Data<AppState>) -> Result<HttpResponse> {
    let pipelines = sqlx::query_as::<_, PipelineRow>(
        "SELECT id, name, description, created_at, updated_at FROM pipelines ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await;

    match pipelines {
        Ok(rows) => {
            let mut result = Vec::new();
            for row in rows {
                let stages = sqlx::query_as::<_, StageRow>(
                    "SELECT id, pipeline_id, name, stage_order as `order`, status, created_at, updated_at FROM stages WHERE pipeline_id = $1 ORDER BY stage_order"
                )
                .fetch_all(&state.db)
                .await
                .unwrap_or_default();

                result.push(Pipeline {
                    id: row.id,
                    name: row.name,
                    description: row.description,
                    stages: stages.into_iter().map(|s| Stage {
                        id: s.id,
                        pipeline_id: s.pipeline_id,
                        name: s.name,
                        order: s.order,
                        status: s.status,
                        created_at: s.created_at,
                        updated_at: s.updated_at,
                    }).collect(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                });
            }
            Ok(HttpResponse::Ok().json(result))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

pub async fn get_pipeline(state: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse> {
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid UUID")),
    };

    let row = sqlx::query_as::<_, PipelineRow>(
        "SELECT id, name, description, created_at, updated_at FROM pipelines WHERE id = $1"
    )
    .bind(uuid)
    .fetch_optional(&state.db)
    .await;

    match row {
        Ok(Some(row)) => {
            let stages = sqlx::query_as::<_, StageRow>(
                "SELECT id, pipeline_id, name, stage_order as `order`, status, created_at, updated_at FROM stages WHERE pipeline_id = $1 ORDER BY stage_order"
            )
            .fetch_all(&state.db)
            .await
            .unwrap_or_default();

            let pipeline = Pipeline {
                id: row.id,
                name: row.name,
                description: row.description,
                stages: stages.into_iter().map(|s| Stage {
                    id: s.id,
                    pipeline_id: s.pipeline_id,
                    name: s.name,
                    order: s.order,
                    status: s.status,
                    created_at: s.created_at,
                    updated_at: s.updated_at,
                }).collect(),
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            Ok(HttpResponse::Ok().json(pipeline))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("Pipeline not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

pub async fn create_pipeline(
    state: web::Data<AppState>,
    dto: web::Json<CreatePipelineDto>,
) -> Result<HttpResponse> {
    let result = sqlx::query(
        "INSERT INTO pipelines (name, description) VALUES ($1, $2) RETURNING id, name, description, created_at, updated_at"
    )
    .bind(&dto.name)
    .bind(&dto.description)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            let pipeline = Pipeline {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                stages: Vec::new(),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(HttpResponse::Created().json(pipeline))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

pub async fn update_pipeline(
    state: web::Data<AppState>,
    id: web::Path<String>,
    dto: web::Json<UpdatePipelineDto>,
) -> Result<HttpResponse> {
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid UUID")),
    };

    let result = sqlx::query(
        "UPDATE pipelines SET name = COALESCE($1, name), description = COALESCE($2, description), updated_at = NOW() WHERE id = $3 RETURNING id, name, description, created_at, updated_at"
    )
    .bind(&dto.name)
    .bind(&dto.description)
    .bind(uuid)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(row)) => {
            let pipeline = Pipeline {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                stages: Vec::new(),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(HttpResponse::Ok().json(pipeline))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("Pipeline not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

pub async fn delete_pipeline(state: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse> {
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid UUID")),
    };

    let result = sqlx::query("DELETE FROM pipelines WHERE id = $1")
        .bind(uuid)
        .execute(&state.db)
        .await;

    match result {
        Ok(r) => {
            if r.rows_affected() > 0 {
                Ok(HttpResponse::NoContent().finish())
            } else {
                Ok(HttpResponse::NotFound().json("Pipeline not found"))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

pub async fn create_stage(
    state: web::Data<AppState>,
    dto: web::Json<CreateStageDto>,
) -> Result<HttpResponse> {
    let pipeline_id = match uuid::Uuid::parse_str(&dto.pipeline_id) {
        Ok(u) => u,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid pipeline UUID")),
    };

    let result = sqlx::query(
        "INSERT INTO stages (pipeline_id, name, stage_order, status) VALUES ($1, $2, $3, 'pending') RETURNING id, pipeline_id, name, stage_order as `order`, status, created_at, updated_at"
    )
    .bind(pipeline_id)
    .bind(&dto.name)
    .bind(dto.order)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            let stage = Stage {
                id: row.get("id"),
                pipeline_id: row.get("pipeline_id"),
                name: row.get("name"),
                order: row.get("order"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(HttpResponse::Created().json(stage))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

pub async fn update_stage_status(
    state: web::Data<AppState>,
    id: web::Path<String>,
    dto: web::Json<UpdateStageStatusDto>,
) -> Result<HttpResponse> {
    let stage_id = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid UUID")),
    };

    let result = sqlx::query(
        "UPDATE stages SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING id, pipeline_id, name, stage_order as `order`, status, created_at, updated_at"
    )
    .bind(&dto.status)
    .bind(stage_id)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(row)) => {
            let stage = Stage {
                id: row.get("id"),
                pipeline_id: row.get("pipeline_id"),
                name: row.get("name"),
                order: row.get("order"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(HttpResponse::Ok().json(stage))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("Stage not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

pub async fn delete_stage(state: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse> {
    let stage_id = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid UUID")),
    };

    let result = sqlx::query("DELETE FROM stages WHERE id = $1")
        .bind(stage_id)
        .execute(&state.db)
        .await;

    match result {
        Ok(r) => {
            if r.rows_affected() > 0 {
                Ok(HttpResponse::NoContent().finish())
            } else {
                Ok(HttpResponse::NotFound().json("Stage not found"))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

#[derive(sqlx::FromRow)]
struct PipelineRow {
    id: uuid::Uuid,
    name: String,
    description: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct StageRow {
    id: uuid::Uuid,
    pipeline_id: uuid::Uuid,
    name: String,
    order: i32,
    status: StageStatus,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}
