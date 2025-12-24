use crate::models::{ApiResponse, ApiResult, AppError, AuthUser, User};
use crate::routes::AppState;
use crate::utils::jwt_token::JwtUtil;
use crate::utils::redis_client::RedisResult;
use crate::utils::{format_err, JsonHelp};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use garde::Validate;
use serde_json::{json, Value};


pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(user_login))
        .route("/get_user_setting", get(get_user_setting))
    // .route("/get_hash_from_redis", get(get_hash_from_redis))
}

// #[axum::debug_handler]
pub async fn user_login(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> ApiResponse<AuthUser> {
    if let Err(e) = payload.validate() {
        return AppError::new_resp(StatusCode::UNPROCESSABLE_ENTITY, format_err(e));
    }
    let name = payload.name.unwrap_or("".to_string());
    let password = payload.password.unwrap_or("".to_string());
    let config = state.redis.hget("user", name.as_str()).await.get_json()?;
    if config.get_string("password").to_lowercase() != password.to_lowercase() {
        return ApiResult::error(-1, "登录用户密码不正确".to_string());
    }
    let token = match JwtUtil::generate_token(&name, &name, &state.jwt_config) {
        Ok(token) => token,
        Err(e) => {
            return AppError::new_resp(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("生成token错误,{}", e),
            );
        }
    };
    ApiResult::success(AuthUser {
        name,
        admin: config.get_bool("admin"),
        cgm: config.get_string("cgm"),
        token,
    })
}

pub async fn get_user_setting(user: User, State(state): State<AppState>) -> ApiResponse<Value> {
    let setting = state.get_user_settings(user.name.unwrap().as_str()).await;
    println!("{:?}", setting);
    ApiResult::success(json!(setting))
}
