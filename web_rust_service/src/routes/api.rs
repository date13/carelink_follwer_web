use crate::models::User;
#[warn(unused_variables)]
use crate::models::{ApiResponse, ApiResult};
use crate::routes::AppState;
use crate::services::ns_service::{
    ns_refresh_devicestatus_data, ns_refresh_entries_data, ns_refresh_treatments_data,
};
use axum::extract::State;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{response::Json, Router};
use chrono::Utc;
use serde_json::{json, Value};
use tracing::info;

pub fn ns_api_router() -> Router<AppState> {
  Router::new()
      .route("/status.json", get(status_json))
      .route("/status", get(status))
      .route("/verifyauth", get(verifyauth))
      .route("/entries", post(add_entries))
      .route("/treatments", post(add_treatments))
      .route("/devicestatus", post(add_devicestatus))
}

pub async fn add_entries(
  user: User,
  State(state): State<AppState>,
  Json(mut payload): Json<Value>,
) -> ApiResponse<Value> {
  info!("receive ns entries data: {:?}", payload);
    match payload.as_array_mut() {
    None => ApiResult::success(json!({})),
    Some(entries) => {
      ns_refresh_entries_data(&state, entries, user.name.unwrap().as_str()).await;
      ApiResult::success(Value::String("data receive".to_string()))
    }
  }
}

pub async fn add_treatments(
  user: User,
  State(state): State<AppState>,
  Json(mut payload): Json<Value>,
) -> ApiResponse<Value> {
  info!("receive ns treatments data: {:?}", payload);
    match payload.as_array_mut() {
    None => ApiResult::success(json!({})),
    Some(treatments) => {
      ns_refresh_treatments_data(&state, treatments, user.name.unwrap().as_str()).await;
      ApiResult::success(Value::String("data receive".to_string()))
    }
  }
}

pub async fn add_devicestatus(
  user: User,
  State(state): State<AppState>,
  Json(mut payload): Json<Value>,
) -> ApiResponse<Value> {
  info!("receive ns devicestatus data: {:?}", payload);
    match payload.as_array_mut() {
    None => ApiResult::success(json!({})),
    Some(treatments) => {
      ns_refresh_devicestatus_data(&state, treatments, user.name.unwrap().as_str()).await;
      ApiResult::success(Value::String("data receive".to_string()))
    }
  }
}

pub async fn verifyauth() -> Json<Value> {
  Json(json!({
      "status": 200,
      "message": {
        "canRead": false,
        "canWrite": true,
        "isAdmin":false,
        "message": "OK",
        "rolefound": "FOUND",
        "permissions": "DEFAULT"
      }
   }))
}

pub async fn status_json() -> Json<Value> {
  let now = Utc::now();
  Json(json!({
        "status": "ok",
        "name": "Nightscout-Follower",
        "version": "14.2.6",
        "serverTime": now.to_rfc3339(),
        "serverTimeEpoch": now.timestamp(),
        "apiEnabled": true,
        "careportalEnabled": true,
        "boluscalcEnabled": true,
        "settings": {
            "units": "mg/dl",
            "timeFormat": 24,
            "customTitle": "Alex Nightscout",
            "nightMode": false,
            "editMode": true,
            "showRawbg": "never"
        }
    }))
}

pub async fn ng_version() -> Json<Value> {
  Json(json!([
    {
        "version": "1.0.0",
        "url": "/api/v1"
    }]))
}

pub async fn status() -> Html<&'static str> {
  Html("<h1>STATUS OK</h1>")
}
