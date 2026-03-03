use crate::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pipelines")
            .route("", web::get().to(handlers::get_pipelines))
            .route("", web::post().to(handlers::create_pipeline))
            .route("/{id}", web::get().to(handlers::get_pipeline))
            .route("/{id}", web::put().to(handlers::update_pipeline))
            .route("/{id}", web::delete().to(handlers::delete_pipeline)),
    )
    .service(
        web::scope("/stages")
            .route("", web::post().to(handlers::create_stage))
            .route(
                "/{id}/status",
                web::patch().to(handlers::update_stage_status),
            )
            .route("/{id}", web::delete().to(handlers::delete_stage)),
    );
}
