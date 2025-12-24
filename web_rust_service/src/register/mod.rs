use crate::config::AppConfig;
use crate::models::{AppError, User};
use crate::utils::jwt_token::{JwtConfig, JwtUtil};
use axum::extract::State;
use axum::http::{HeaderValue, StatusCode};
use axum::{
    body::Body, extract::Request, http::Method, middleware::Next, response::Response, Json,
};
use serde_json::Value;
use std::time::Duration;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tracing::info;
use uuid::Uuid;

// 请求日志中间件
pub async fn logging_middleware(mut request: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();
    let start_time = std::time::Instant::now();

    // 提取请求信息
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();
    let query = uri.query().map(|q| q.to_string());
    // 提取头部信息
    // let headers = request.headers();
    // let user_agent = headers
    //     .get("user-agent")
    //     .and_then(|v| v.to_str().ok())
    //     .map(|s| s.to_string());
    //
    // let client_ip = headers
    //     .get("x-forwarded-for")
    //     .or_else(|| headers.get("x-real-ip"))
    //     .and_then(|v| v.to_str().ok())
    //     .map(|s| s.to_string());
    // 检查是否需要跳过日志记录（只跳过 OPTIONS 方法）
    let should_skip = matches!(method, Method::OPTIONS);

    if !should_skip {
        let body = extract_body(&mut request).await;
        info!(
            request_id = %request_id,
            method = %method,
            path = %path,
            query = ?query,
            body = %body.as_deref().unwrap_or(""),
            // user_agent = ?user_agent,
            // client_ip = ?client_ip,
            "Request started"
        );
    }
    // 处理请求
    let response = next.run(request).await;

    // 计算响应时间
    let response_time = start_time.elapsed().as_millis();
    let status_code = response.status().as_u16();

    // 记录请求完成
    if !should_skip {
        info!(
            request_id = %request_id,
            status_code = status_code,
            response_time_ms = response_time,
            "Request completed"
        );
    }
    response
}

async fn extract_body(request: &mut Request) -> Option<String> {
    // 克隆请求体
    let body = std::mem::replace(request.body_mut(), Body::from(""));

    // 将 Body 转换为字节
    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    // 将字节转换回 Body 并放回请求中
    *request.body_mut() = Body::from(bytes.clone());
    match String::from_utf8(bytes.to_vec()) {
        Ok(s) if !s.is_empty() => clean_json_string(&s),
        Ok(_) => None,
        Err(_) => Some("<binary data>".to_string()),
    }
}

fn clean_json_string(s: &str) -> Option<String> {
    // 尝试解析为 JSON
    match serde_json::from_str::<Value>(s) {
        Ok(json_value) => {
            // 重新序列化为紧凑的 JSON 字符串
            serde_json::to_string(&json_value).ok()
        }
        Err(_) => {
            // 如果不是有效的 JSON，返回原始字符串
            Some(s.to_string())
        }
    }
}

pub fn cors_layer(config: &AppConfig) -> CorsLayer {
    if config.is_production() {
        let origins: Vec<HeaderValue> = config
            .default
            .cors
            .allowed_origins
            .iter()
            .filter_map(|origin| origin.parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers(Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(config.default.cors.max_age_seconds))
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(config.default.cors.max_age_seconds))
    }
}

const SKIP_PATHS: &[&str] = &["/public", "/user/login"];
// 认证中间件
pub async fn auth_middleware(
    State(jwt_config): State<JwtConfig>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    // 跳过认证的路由
    let path = request.uri().path();
    if SKIP_PATHS.iter().any(|prefix| path.starts_with(prefix)) {
        return Ok(next.run(request).await);
    }

    // 提取 Token
    let token = match JwtUtil::extract_token_from_headers(request.headers()) {
        Ok(token) => token,
        Err(e) => return AppError::new_middle(StatusCode::UNAUTHORIZED, &e.to_string()),
    };

    // 验证 Token
    let claims = match JwtUtil::verify_token(&token, &jwt_config) {
        Ok(claims) => claims,
        Err(e) => return AppError::new_middle(StatusCode::UNAUTHORIZED, &e.to_string()),
    };

    // 将用户信息添加到请求扩展中
    let current_user = User {
        id: claims.sub.parse().ok(),
        name: Some(claims.username),
      password: None
    };

    request.extensions_mut().insert(current_user);

    Ok(next.run(request).await)
}

