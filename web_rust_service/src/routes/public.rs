#[warn(unused_variables)]
use crate::models::{ApiResponse, ApiResult};
use crate::routes::AppState;
use crate::services::public::HelloWorld;
use crate::services::sugar_service::DictKeys;
use crate::utils::redis_client::RedisResult;
use crate::utils::JsonHelp;
use axum::extract::State;
use axum::routing::post;
use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(hello_world))
        .route("/dict", post(update_user_token))
        // .route("/user/{id}", get(get_user_by_id_param))
        // .route("/get_user_by_id", get(get_user_by_id))
        // .route("/create_user", post(create_user))
        .route("/test", get(test_service))
}

pub async fn update_user_token(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> ApiResponse<Value> {
    let params_val = payload.get_string("val");
    state
        .redis
        .set(
            format!("{}:{}", "alex", DictKeys::AUTH).as_str(),
            params_val.as_str(),
            None,
        )
        .await
        .get_json_result()
}

//
// #[derive(Debug, Deserialize)]
// struct SearchQuery {
//     id: i32,
//     name: Option<String>,
//     // age: Option<i32>,
// }
// #[warn(unused_variables)]
// async fn create_user(
//     State(state): State<AppState>,
//     Json(payload): Json<User>,
// ) -> ApiResponse<User> {
//     if let Err(e) = payload.validate() {
//         return AppError::new_resp(StatusCode::UNPROCESSABLE_ENTITY, format_err(e));
//     }
//     if payload.name.is_none() {
//         return AppError::new_resp(
//             StatusCode::UNPROCESSABLE_ENTITY,
//             "Username cannot be empty".to_string(),
//         );
//     }
//     ApiResult::success(User {
//         id: payload.id,
//         name: payload.name,
//         password: None,
//     })
// }
//
// async fn get_user_by_id(Query(query): Query<SearchQuery>) -> ApiResponse<User> {
//     ApiResult::success(User {
//         id: Some(query.id),
//         name: Some(query.name.unwrap_or("date13".to_string())),
//         password: None,
//     })
// }
//
// async fn get_user_by_id_param(Path(id): Path<i32>) -> ApiResponse<User> {
//     if id == 0 {
//         return ApiResult::error(-1, format!("not user for id:{}", id));
//     }
//
//     if id < 0 {
//         return AppError::new_resp(
//             StatusCode::BAD_GATEWAY,
//             format!("no user found for id {}", id),
//         );
//     }
//
//     ApiResult::success(User {
//         id: Some(id),
//         name: Some(String::from("date13")),
//         password: None,
//     })
// }
// async fn health_check() -> &'static str {
//     "OK"
// }
pub(crate) async fn hello_world() -> ApiResponse<HelloWorld> {
    ApiResult::success(HelloWorld::new(String::from("Hello World"), 25))
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
