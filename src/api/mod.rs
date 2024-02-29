use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod health;
pub mod process;

#[derive(OpenApi)]
#[openapi(paths(process::process, health::health))]
struct ApiDoc;

pub fn app() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/api/doc").url("/api/doc/swagger.json", ApiDoc::openapi()))
        .route("/api/process", post(process::process))
        .route("/health", get(health::health))
}
