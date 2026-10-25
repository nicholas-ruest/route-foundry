use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use rf_auth::authenticate;
use rf_errors::FoundryError;
use rf_middleware::validate_idempotency;
use rf_orchestrator::{FoundryService, RouteCommand};
use rf_types::WorkloadId;
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use uuid::Uuid;
#[derive(Clone)]
pub struct AppState {
    pub service: FoundryService,
    pub auth_mode: String,
}
#[derive(Serialize)]
struct Health {
    status: &'static str,
}
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(|| async { Json(Health { status: "ok" }) }))
        .route(
            "/readyz",
            get(|| async { Json(Health { status: "ready" }) }),
        )
        .route("/api/v1/routes", post(route_workload))
        .route("/api/v1/routes/{id}", get(get_route))
        .route("/api/v1/audit", get(get_audit))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(5),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state))
}
fn auth(headers: &HeaderMap, mode: &str) -> Result<(), ApiError> {
    let cookie = headers.get("cookie").and_then(|v| v.to_str().ok());
    authenticate(cookie, mode).map(|_| ()).map_err(ApiError)
}
async fn route_workload(
    State(s): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(cmd): Json<RouteCommand>,
) -> Result<Json<rf_types::RoutingResult>, ApiError> {
    auth(&headers, &s.auth_mode)?;
    validate_idempotency(headers.get("idempotency-key").and_then(|v| v.to_str().ok()))
        .map_err(ApiError)?;
    let corr = headers
        .get("x-correlation-id")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    Ok(Json(s.service.route(cmd, &corr).map_err(ApiError)?))
}
async fn get_route(
    State(s): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<rf_types::RoutingResult>, ApiError> {
    auth(&headers, &s.auth_mode)?;
    let id = WorkloadId::new(id).map_err(|e| ApiError(FoundryError::Invalid(e)))?;
    Ok(Json(s.service.get(&id).map_err(ApiError)?))
}
async fn get_audit(
    State(s): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<rf_types::StoredRecord>>, ApiError> {
    auth(&headers, &s.auth_mode)?;
    Ok(Json(s.service.audit().map_err(ApiError)?))
}
struct ApiError(FoundryError);
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            FoundryError::Invalid(_) => StatusCode::UNPROCESSABLE_ENTITY,
            FoundryError::Unauthorized => StatusCode::UNAUTHORIZED,
            FoundryError::Forbidden(_) => StatusCode::FORBIDDEN,
            FoundryError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_GATEWAY,
        };
        (
            status,
            Json(serde_json::json!({"code":status.as_u16(),"message":self.0.to_string()})),
        )
            .into_response()
    }
}
