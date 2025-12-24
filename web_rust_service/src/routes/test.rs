use crate::models::{ApiResponse, ApiResult, User};
use crate::routes::AppState;
use crate::utils::redis_client::RedisResult;
use crate::utils::JsonHelp;
use axum::extract::State;
use axum::routing::post;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashRequest {
  // 定义你期望的字段
  pub key: String,
  pub value: String,
}

pub fn test_router() -> Router<AppState> {
  Router::new()
      .route("/get_user_info", get(get_user_info))
      .route("/get_json_from_redis", get(get_json_from_redis))
      .route("/get_json_hash_from_redis", get(get_json_hash_from_redis))
      .route("/set_json_handler", post(set_json_handler))
      .route("/set_hash_handler", post(set_hash_handler))
      .route("/del_hash_handler", post(del_hash_handler))
  // .route("/get_hash_from_redis", get(get_hash_from_redis))
}

pub async fn get_json_from_redis(user: User, State(state): State<AppState>) -> ApiResponse<Value> {
  let key = &format!("{}:luck", user.name.unwrap());
  // state.redis.get_json_result::<Value>(&key).await
  state.redis.get_json::<Value>(&key).await.get_json_result()
}

pub async fn get_json_hash_from_redis(
  user: User,
  State(state): State<AppState>,
) -> ApiResponse<Value> {
  let key = &format!("{}:food", user.name.unwrap());
  // state.redis.get_json_result::<Value>(&key).await
  state.redis.hget_all(&key).await.get_json_result()
}

pub async fn set_hash_handler(
  user: User,
  State(state): State<AppState>,
  Json(hash_request): Json<HashRequest>,
) -> ApiResponse<Value> {
  let key = &format!("{}:food", user.name.unwrap());
  state
      .redis
      .hset(key, hash_request.key.as_str(), hash_request.value.as_str())
      .await
      .get_json_result()
}

pub async fn del_hash_handler(
  user: User,
  State(state): State<AppState>,
  Json(payload): Json<Value>,
) -> ApiResponse<Value> {
  let key = &format!("{}:food", user.name.unwrap());
  // 安全地获取 key 字段
  let field = payload.get_val("key")?;
  state.redis.hdel(key, field).await.get_json_result()
}

pub async fn set_json_handler(
  user: User,
  State(state): State<AppState>,
  Json(payload): Json<Value>,
) -> ApiResponse<Value> {
  let key = &format!("{}:luck", user.name.unwrap());
  // state.redis.get_json_result::<Value>(&key).await
  println!("{},{}", key, payload);
  state
      .redis
      .set_json(&key, &payload, None)
      .await
      .get_json_result()
  // .get_message_result("ok")
}

pub async fn get_user_info(user: User) -> ApiResponse<User> {
  ApiResult::success(User {
    id: user.id,
    name: user.name,
    password: None,
  })
}
