use crate::models::{ApiResponse, ApiResult, AppError, User};
use crate::routes::AppState;
use crate::utils::redis_client::RedisResult;
use crate::utils::{format_err, JsonHelp};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use garde::Validate;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub fn system_router() -> Router<AppState> {
    Router::new()
        .route("/dict/{name}/{key}", get(load_dict_user_data))
        .route("/dict/{key}", get(load_dict_data))

        .route("/dict_hash/{name}/{key}", get(load_dict_user_hash_data))
        .route("/dict_hash/{key}", get(load_dict_hash_data))

        .route("/dict", post(update_dict))
        .route("/dict/{key}", post(update_user_dict))

        .route("/user_setting", get(load_user_setting))
        .route("/user_setting", post(update_user_setting))
        .route("/tasks", get(load_scheduler_tasks))
        .route("/task/{task_id}/stop", get(stop_scheduler_tasks))
        .route("/task/{task_id}/start", get(start_scheduler_tasks))
    // .route("/get_hash_from_redis", get(get_hash_from_redis))
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct UserSettingFrom {
    #[garde(required)]
    pub admin: Option<bool>,
    #[garde(required)]
    pub carelink_data_refresh_interval: Option<i32>,
    #[garde(required)]
    pub carelink_token_refresh_interval: Option<i32>,
    #[garde(required)]
    pub sse_interval: Option<i32>,
}
// pub async fn load_setting(
//     Path(name): Path<String>,
//     State(state): State<AppState>,
// ) -> ApiResponse<Value> {
//     let data = state
//         .redis
//         .get_json::<Value>(&format!("{}:{}", name, DictKeys::SETTING))
//         .await
//         .get_json()?;
//     ApiResult::success(data)
// }

pub async fn load_dict_user_data(
    Path((name, key)): Path<(String, String)>,
    State(state): State<AppState>,
) -> ApiResponse<Value> {
    state
        .redis
        .get_json::<Value>(&format!("{}:{}", name, key))
        .await
        .get_json_result()
}

pub async fn load_dict_user_hash_data(
    Path((name, key)): Path<(String, String)>,
    State(state): State<AppState>,
) -> ApiResponse<Value> {
    state
        .redis
        .hget_all(&format!("{}:{}", name, key))
        .await
        .get_json_result()
}

pub async fn load_dict_data(
    Path(key): Path<String>,
    State(state): State<AppState>,
) -> ApiResponse<Value> {
    state.redis.get_json::<Value>(&key).await.get_json_result()
}

pub async fn load_dict_hash_data(
    Path(key): Path<String>,
    State(state): State<AppState>,
) -> ApiResponse<Value> {
    state.redis.hget_all(key.as_str()).await.get_json_result()
}

pub async fn update_dict(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> ApiResponse<Value> {
    let params_key = payload.get_string("key");
    let params_sub_key = payload.get_string("subKey");
    let params_val = payload.get_string("val");

    if !params_sub_key.is_empty() {
        state
            .redis
            .hset(
                params_key.as_str(),
                params_sub_key.as_str(),
                params_val.as_str(),
            )
            .await
            .get_json_result()
    } else {
        state
            .redis
            .set(params_key.as_str(), params_val.as_str(), None)
            .await
            .get_json_result()
    }
}
pub async fn update_user_dict(
    Path(key): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> ApiResponse<Value> {
    let params_key = payload.get_string("key");
    let params_sub_key = payload.get_string("subKey");
    let params_val = payload.get_string("val");

    let data_key = format!("{}:{}", key, params_key);

    if !params_sub_key.is_empty() {
        state
            .redis
            .hset(
                data_key.as_str(),
                params_sub_key.as_str(),
                params_val.as_str(),
            )
            .await
            .get_json_result()
    } else {
        state
            .redis
            .set(&data_key, params_val.as_str(), None)
            .await
            .get_json_result()
    }
}
pub async fn load_user_setting(user: User, State(state): State<AppState>) -> ApiResponse<Value> {
    ApiResult::success(
        state
            .redis
            .hget("user", &user.name.unwrap())
            .await
            .get_json()?,
    )
}

pub async fn update_user_setting(
    user: User,
    State(state): State<AppState>,
    Json(payload): Json<UserSettingFrom>,
) -> ApiResponse<Value> {
    if let Err(e) = payload.validate() {
        return AppError::new_resp(StatusCode::UNPROCESSABLE_ENTITY, format_err(e));
    }
    let name = user.name.unwrap();
    let mut config = state.redis.hget("user", &name).await.get_json()?;
    config["carelink_token_refresh_interval"] = payload.carelink_token_refresh_interval.into();
    config["carelink_data_refresh_interval"] = payload.carelink_data_refresh_interval.into();
    config["sse_interval"] = payload.sse_interval.into();
    config["admin"] = payload.admin.into();

    match state.redis.hset_json("user", &name, &config).await {
        Ok(Some(data)) => {
            let mut user_setting = state.get_user_settings(&name).await;
            if user_setting.username != String::new() {
                user_setting.carelink_token_refresh_interval =
                    payload.carelink_token_refresh_interval.unwrap() as i64;
                user_setting.carelink_data_refresh_interval =
                    payload.carelink_data_refresh_interval.unwrap() as i64;
                user_setting.sse_interval = payload.sse_interval.unwrap() as i64;
                user_setting.admin = payload.admin.unwrap();
                state.save_user_settings(&name, user_setting).await;
            }
            ApiResult::success(data)
        }
        Err(e) => ApiResult::error(-1, e.to_string()),
        _ => ApiResult::error(-2, format!("{} is not exist", name)),
    }
}

pub async fn load_scheduler_tasks(State(state): State<AppState>) -> ApiResponse<Value> {
    ApiResult::success(json!(state.task_manager.list_tasks().await))
}

pub async fn stop_scheduler_tasks(
    Path(task_id): Path<String>,
    State(state): State<AppState>,
) -> ApiResponse<Value> {
    ApiResult::success(json!(state.task_manager.stop_task(task_id.as_str()).await))
}

pub async fn start_scheduler_tasks(
    Path(task_id): Path<String>,
    State(state): State<AppState>,
) -> ApiResponse<Value> {
    ApiResult::success(json!(
        state.task_manager.resume_task(task_id.as_str()).await
    ))
}
