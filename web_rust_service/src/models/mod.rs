use crate::services::sugar_service::HTTP_TIMEOUT;
use crate::utils::jwt_token::JwtConfig;
use crate::utils::mail::EmailService;
use crate::utils::redis_client::RedisService;
use crate::utils::task::TaskManager;
use anyhow::Result;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use garde::Validate;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum CgmType {
    Carelink,
    Dexcom,
}

impl CgmType {
    pub fn new(val: &str) -> Self {
        match val {
            "carelink" => CgmType::Carelink,
            "dexcom" => CgmType::Dexcom,
            _ => CgmType::Carelink,
        }
    }
}
// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub redis: Arc<RedisService>,
    pub http_client: Arc<Client>,
    pub jwt_config: JwtConfig,
    pub user_settings: Arc<RwLock<HashMap<String, UserSetting>>>,
    pub task_manager: Arc<TaskManager>,
    pub email: Arc<EmailService>,
}

impl AppState {
    pub fn new(
        redis: RedisService,
        jwt_config: JwtConfig,
        task_manager: TaskManager,
        email: EmailService,
    ) -> Self {
        Self {
            redis: Arc::new(redis),
            http_client: Arc::new(
                Client::builder()
                    .cookie_store(true)
                    .use_rustls_tls()
                    .timeout(Duration::from_secs(HTTP_TIMEOUT))
                    .build()
                    .expect("Failed to build reqwest client"),
            ),
            jwt_config,
            user_settings: Arc::new(RwLock::new(HashMap::new())),
            task_manager: Arc::new(task_manager),
            email: Arc::new(email),
        }
    }

    // 获取用户设置
    pub async fn get_user_settings(&self, user_name: &str) -> UserSetting {
        let settings_map = self.user_settings.read().await;
        settings_map
            .get(user_name)
            .map_or(UserSetting::null(), |v| v.clone())
    }

    // 保存用户设置
    pub async fn save_user_settings(
        &self,
        user_name: &str,
        settings: UserSetting,
    ) -> Option<UserSetting> {
        let mut settings_map = self.user_settings.write().await;
        settings_map.insert(user_name.to_string(), settings)
    }
}

pub type ApiResponse<T> = anyhow::Result<ApiResult<T>, AppError>;
pub trait OperationStatusTrait {
    fn ok() -> Value;
    fn fail() -> Value;
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationStatusEnum {
    Success,
    Ok,
    Error,
    Fail,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationStatus {
    pub success: bool,
    pub message: OperationStatusEnum,
}

impl OperationStatusTrait for OperationStatus {
    fn ok() -> Value {
        json!(Self {
            success: true,
            message: OperationStatusEnum::Ok
        })
    }

    fn fail() -> Value {
        json!(Self {
            success: false,
            message: OperationStatusEnum::Fail
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: String,
}

impl<T> ApiResult<T> {
    pub fn success(data: T) -> ApiResponse<T> {
        Ok(ApiResult {
            code: 0,
            data: Some(data),
            msg: String::new(),
        })
    }

    pub fn error(code: i32, msg: String) -> ApiResponse<T> {
        Ok(ApiResult {
            code,
            data: None,
            msg,
        })
    }
}

// 为 ApiResponse 实现 IntoResponse
impl<T> IntoResponse for ApiResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub msg: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.msg)
    }
}

impl AppError {
    pub fn new_resp<T>(status: StatusCode, msg: String) -> ApiResponse<T> {
        Err(AppError { status, msg })
    }

    pub fn new<T>(status: StatusCode, msg: String) -> Result<T, AppError> {
        Err(AppError { status, msg })
    }

    pub fn new_middle<T>(status: StatusCode, msg: &String) -> Result<T, (StatusCode, Json<Value>)> {
        Err((status, Json(json!({ "msg":msg }))))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = ApiResult::<()>::error(self.status.as_u16() as i32, self.msg);
        (self.status, body).into_response()
    }
}
//
// #[derive(Debug, Deserialize)]
// struct Pagination {
//     page: Option<u32>,
//     limit: Option<u32>,
//     sort: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct PageResult<T> {
//     pub list: Vec<T>,
//     pub total: i32,
// }

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct User {
    #[garde(skip)]
    pub id: Option<i32>,
    #[garde(required)]
    pub name: Option<String>,
    #[garde(required, length(min = 4, max = 50))]
    pub password: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSetting {
    pub user_key: String,
    pub cgm: CgmType,
    pub patient_id: String,
    pub username: String,
    pub role: String,
    pub admin: bool,
    pub sse_interval: i64,
    pub carelink_token_refresh_interval: i64,
    pub carelink_data_refresh_interval: i64,
}

impl UserSetting {
    pub fn null() -> Self {
        UserSetting {
            user_key: String::new(),
            cgm: CgmType::Carelink,
            patient_id: "".to_string(),
            username: "".to_string(),
            role: "".to_string(),
            admin: false,
            sse_interval: 30,
            carelink_token_refresh_interval: 3600,
            carelink_data_refresh_interval: 3600,
        }
    }
}

// 提取当前用户的提取器
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<User>()
            .ok_or((StatusCode::UNAUTHORIZED, "未认证的用户"))?;

        Ok(user.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub name: String,
    pub admin: bool,
    pub cgm: String,
    pub token: String,
}
