use crate::models::{ApiResponse, ApiResult, AppError, User};
use crate::routes::AppState;
use crate::services::sugar_service::{carelink_refresh_data, DictKeys};
use crate::utils::redis_client::RedisResult;
use crate::utils::JsonHelp;
use axum::extract::{Path, State};
use axum::response::sse::Event;
use axum::response::Sse;
use axum::routing::{delete, get, put};
use axum::{Json, Router};
use chrono::Local;
use futures::stream::{self, Stream};
use serde_json::{json, Value};
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::StreamExt as _;

pub fn sugar_router() -> Router<AppState> {
    Router::new()
        .route("/index", put(load_data))
        .route("/sse", put(load_data_sse))
        .route("/test", get(test_service))
        .route("/foods", get(load_foods))
        .route("/food", put(set_food))
        .route("/food/{key}", delete(del_food))
        .route("/refreshCarelinkData", put(refresh_carelink_data))
    // .route("/get_hash_from_redis", get(get_hash_from_redis))
}

async fn load_sugar_data(name: &str, state: AppState) -> Result<(Value, Value), AppError> {
    let data_key = format!("{}:carelinkData", name);
    let my_data_key = format!("{}:carelinkMyData", name);

    let data = state.redis.get_json::<Value>(&data_key).await.get_json()?;
    let my_data = state
        .redis
        .get_json::<Value>(&my_data_key)
        .await
        .get_json()?;
    Ok((data, my_data))
}

pub async fn load_data(user: User, State(state): State<AppState>) -> ApiResponse<Value> {
    let name = user.name.unwrap_or("".to_string());
    let (data, my_data) = load_sugar_data(&name, state).await?;
    ApiResult::success(json!({
        "data":data,
        "myData":my_data,
    }))
}

pub async fn load_data_sse(
    user: User,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item=Result<Event, Infallible>>> {
    let name = user.name.unwrap_or("".to_string());
    let setting = state.get_user_settings(&name).await;

    let stream = stream::repeat_with(move || {
        // 克隆需要的状态
        let state_clone = state.clone();
        let name_clone = name.clone();
        async move {
            // 每次迭代都重新加载最新数据
            let (data, my_data) = load_sugar_data(&name_clone, state_clone)
                .await
                .unwrap_or((Value::Null, Value::Null));

            Event::default()
                .event(format!("{}:sugar_data", &name_clone))
                .id(Local::now().timestamp_millis().to_string())
                .json_data(json!({
                    "data": data,
                    "myData": my_data,
                }))
                .unwrap()
        }
    })
        // 使用 then 处理异步操作
        .then(|f| f)
        .map(Ok)
        .throttle(Duration::from_secs(setting.sse_interval as u64));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive"),
    )
}

pub async fn refresh_carelink_data(
    user: User,
    State(state): State<AppState>,
) -> ApiResponse<Value> {
    let name = user.name.unwrap_or("".to_string());
    let setting = state.get_user_settings(&name).await;
    carelink_refresh_data(&state, &setting).await;
    let (data, my_data) = load_sugar_data(&name, state).await?;
    ApiResult::success(json!({
        "data":data,
        "myData":my_data,
    }))
}

async fn load_foods(user: User, state: State<AppState>) -> ApiResponse<Value> {
    let name = user.name.unwrap_or("".to_string());
    state
        .redis
        .hget_all(&format!("{}:food", name))
        .await
        .get_json_result()
}
async fn set_food(
    user: User,
    state: State<AppState>,
    Json(payload): Json<Value>,
) -> ApiResponse<Value> {
    let name = user.name.unwrap_or("".to_string());
    let params_key = payload.get_string("key");
    let params_val = payload.get_i64("val");

    state
        .redis
        .hset(
            format!("{}:{}", name, DictKeys::FOOD).as_str(),
            params_key.as_str(),
            params_val.to_string().as_str(),
        )
        .await
        .get_json_result()
}

async fn del_food(
    Path(key): Path<String>,
    user: User,
    state: State<AppState>,
) -> ApiResponse<Value> {
    let name = user.name.unwrap_or("".to_string());

    state
        .redis
        .hdel(
            format!("{}:{}", name, DictKeys::FOOD).as_str(),
            key.as_str(),
        )
        .await
        .get_json_result()
}

async fn test_service() -> Json<serde_json::Value> {
    Json(json!({
        "code":0,
        "data":{
            "car": {
                "name": "new car",
                "color": "blue",
            },
            "boat": {
                "name": "old boat",
                "color":"red"
            }
        },
        "msg":"success"
    }))
}
