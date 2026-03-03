use crate::models::{AppState, CreatePipelineDto, CreateStageDto, Pipeline, Stage, StageStatus, UpdatePipelineDto, UpdateStageStatusDto};
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use uuid::Uuid;

pub async fn get_pipelines(state: web::Data<AppState>) -> Result<HttpResponse> {
    let pipelines = state.pipelines.lock().unwrap();
    Ok(HttpResponse::Ok().json(pipelines.clone()))
}

pub async fn get_pipeline(state: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse> {
    let pipelines = state.pipelines.lock().unwrap();
    let pipeline = pipelines
        .iter()
        .find(|p| p.id.to_string() == *id)
        .cloned();
    
    match pipeline {
        Some(p) => Ok(HttpResponse::Ok().json(p)),
        None => Ok(HttpResponse::NotFound().json("Pipeline not found")),
    }
}

pub async fn create_pipeline(
    state: web::Data<AppState>,
    dto: web::Json<CreatePipelineDto>,
) -> Result<HttpResponse> {
    let now = Utc::now();
    let pipeline = Pipeline {
        id: Uuid::new_v4(),
        name: dto.name.clone(),
        description: dto.description.clone(),
        stages: Vec::new(),
        created_at: now,
        updated_at: now,
    };
    
    let mut pipelines = state.pipelines.lock().unwrap();
    pipelines.push(pipeline.clone());
    
    Ok(HttpResponse::Created().json(pipeline))
}

pub async fn update_pipeline(
    state: web::Data<AppState>,
    id: web::Path<String>,
    dto: web::Json<UpdatePipelineDto>,
) -> Result<HttpResponse> {
    let mut pipelines = state.pipelines.lock().unwrap();
    
    if let Some(pipeline) = pipelines.iter_mut().find(|p| p.id.to_string() == *id) {
        if let Some(name) = dto.name.clone() {
            pipeline.name = name;
        }
        if let Some(description) = dto.description.clone() {
            pipeline.description = description;
        }
        pipeline.updated_at = Utc::now();
        return Ok(HttpResponse::Ok().json(pipeline.clone()));
    }
    
    Ok(HttpResponse::NotFound().json("Pipeline not found"))
}

pub async fn delete_pipeline(state: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse> {
    let mut pipelines = state.pipelines.lock().unwrap();
    let len_before = pipelines.len();
    pipelines.retain(|p| p.id.to_string() != *id);
    
    if pipelines.len() < len_before {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().json("Pipeline not found"))
    }
}

pub async fn create_stage(
    state: web::Data<AppState>,
    dto: web::Json<CreateStageDto>,
) -> Result<HttpResponse> {
    let pipeline_id = match Uuid::parse_str(&dto.pipeline_id) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid UUID")),
    };
    
    let now = Utc::now();
    let stage = Stage {
        id: Uuid::new_v4(),
        name: dto.name.clone(),
        order: dto.order,
        status: StageStatus::Pending,
        pipeline_id,
        created_at: now,
        updated_at: now,
    };
    
    let mut pipelines = state.pipelines.lock().unwrap();
    
    if let Some(pipeline) = pipelines.iter_mut().find(|p| p.id == pipeline_id) {
        pipeline.stages.push(stage.clone());
        pipeline.updated_at = now;
        return Ok(HttpResponse::Created().json(stage));
    }
    
    Ok(HttpResponse::NotFound().json("Pipeline not found"))
}

pub async fn update_stage_status(
    state: web::Data<AppState>,
    id: web::Path<String>,
    dto: web::Json<UpdateStageStatusDto>,
) -> Result<HttpResponse> {
    let stage_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid UUID")),
    };
    let mut pipelines = state.pipelines.lock().unwrap();
    
    for pipeline in pipelines.iter_mut() {
        if let Some(stage) = pipeline.stages.iter_mut().find(|s| s.id == stage_id) {
            stage.status = dto.status.clone();
            stage.updated_at = Utc::now();
            pipeline.updated_at = Utc::now();
            return Ok(HttpResponse::Ok().json(stage.clone()));
        }
    }
    
    Ok(HttpResponse::NotFound().json("Stage not found"))
}

pub async fn delete_stage(state: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse> {
    let stage_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid UUID")),
    };
    let mut pipelines = state.pipelines.lock().unwrap();
    
    for pipeline in pipelines.iter_mut() {
        let len_before = pipeline.stages.len();
        pipeline.stages.retain(|s| s.id != stage_id);
        
        if pipeline.stages.len() < len_before {
            pipeline.updated_at = Utc::now();
            return Ok(HttpResponse::NoContent().finish());
        }
    }
    
    Ok(HttpResponse::NotFound().json("Stage not found"))
}
