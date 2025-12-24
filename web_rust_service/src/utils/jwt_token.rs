use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// JWT 声明
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,        // 用户ID
  pub exp: usize,         // 过期时间
  pub iat: usize,         // 签发时间
  pub username: String,   // 用户名
}

// JWT 配置
#[derive(Clone)]
pub struct JwtConfig {
  pub secret: String,
  pub expiration_hours: i64,
}

impl JwtConfig {
  pub fn new(secret: String, expiration_hours: i64) -> Self {
    Self {
      secret,
      expiration_hours,
    }
  }
}

// 认证错误类型
#[derive(Debug)]
pub enum AuthError {
  InvalidToken,
  ExpiredToken,
  MissingToken,
  InvalidHeader,
}

impl std::fmt::Display for AuthError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AuthError::InvalidToken => write!(f, "无效的Token"),
      AuthError::ExpiredToken => write!(f, "Token已过期"),
      AuthError::MissingToken => write!(f, "缺少Token"),
      AuthError::InvalidHeader => write!(f, "无效的认证头"),
    }
  }
}

pub struct JwtUtil;

impl JwtUtil {
  // 生成 Token
  pub fn generate_token(
    user_id: &str,
    username: &str,
    config: &JwtConfig,
  ) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::hours(config.expiration_hours);

    let claims = Claims {
      sub: user_id.to_string(),
      exp: exp.timestamp() as usize,
      iat: now.timestamp() as usize,
      username: username.to_string(),
    };

    encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(config.secret.as_ref()),
    )
  }

  // 验证 Token
  pub fn verify_token(
    token: &str,
    config: &JwtConfig,
  ) -> Result<Claims, AuthError> {
    let validation = Validation::default();

    let token_data = decode::<Claims>(
      token,
      &DecodingKey::from_secret(config.secret.as_ref()),
      &validation,
    ).map_err(|e| match e.kind() {
      jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
      _ => AuthError::InvalidToken,
    })?;

    Ok(token_data.claims)
  }

  // 从 Header 中提取 Token
  pub fn extract_token_from_headers(headers: &HeaderMap) -> Result<String, AuthError> {
    let auth_header = headers
        .get("authorization")
        .ok_or(AuthError::MissingToken)?
        .to_str()
        .map_err(|_| AuthError::InvalidHeader)?;

    if !auth_header.starts_with("Bearer ") {
      return Err(AuthError::InvalidHeader);
    }

    Ok(auth_header[7..].to_string()) // 去掉 "Bearer " 前缀
  }
}