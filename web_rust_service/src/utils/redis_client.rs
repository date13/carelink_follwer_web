#![allow(dead_code)]

use crate::config::RedisConfig;
use crate::models::{ApiResponse, ApiResult, AppError, OperationStatus, OperationStatusTrait};
use crate::utils::parse_json;
use anyhow::Error;
use axum::http::StatusCode;
use bb8::{Pool, PooledConnection};
use bb8_redis::redis::AsyncTypedCommands;
use bb8_redis::RedisConnectionManager;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use std::fmt::Display;
use std::time::Duration;

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConnection = PooledConnection<'static, RedisConnectionManager>;
// 创建 Redis 连接池
pub async fn create_redis_pool(
    config: &RedisConfig,
) -> Result<RedisPool, Error> {
    let manager = RedisConnectionManager::new(config.url.clone())?;

    let pool = Pool::builder()
        .max_size(config.max_connections)
        .min_idle(Some(config.min_connections))
        .connection_timeout(Duration::from_secs(config.connection_timeout))
        .max_lifetime(Duration::from_secs(config.max_lifetime))
        .build(manager)
        .await?;

    Ok(pool)
}

#[derive(Clone, Debug)]
pub struct RedisService {
    pool: RedisPool,
}

impl RedisService {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    pub async fn get_conn(
        &self,
    ) -> Result<PooledConnection<'_, RedisConnectionManager>, Error> {
        let conn = self.pool.get().await?;
        Ok(conn)
    }

    // 设置字符串值
    pub async fn set(
        &self,
        key: &str,
        value: &str,
        ttl: Option<u64>,
    ) -> Result<Option<Value>, Error> {
        let mut conn = self.get_conn().await?;

        if let Some(ttl_seconds) = ttl {
            conn.set_ex(key, value, ttl_seconds).await?;
        } else {
            conn.set(key, value).await?;
        }

        Ok(Some(OperationStatus::ok()))
    }

    // 获取字符串值
    pub async fn get(&self, key: &str) -> Result<Option<String>, Error> {
        let mut conn = self.get_conn().await?;
        let value: Option<String> = conn.get(key).await?;
        Ok(value)
    }

    // 删除键
    pub async fn del(&self, key: &str) -> Result<(), Error> {
        let mut conn = self.get_conn().await?;
        conn.del(key).await?;
        Ok(())
    }

    // 设置 JSON 值
    pub async fn set_json<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<u64>,
    ) -> Result<Option<Value>, Error> {
        let json = serde_json::to_string(value)?;
        Ok(self.set(key, &json, ttl).await?)
    }

    // 获取 JSON 值
    pub async fn get_json<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, Error> {
        if let Some(json_str) = self.get(key).await? {
            let value: T = serde_json::from_str(&json_str)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    // 设置哈希表字段
    pub async fn hset(
        &self,
        key: &str,
        field: &str,
        value: &str,
    ) -> Result<Option<Value>, Error> {
        let mut conn = self.get_conn().await?;
        conn.hset(key, field, value).await?;
        Ok(Some(OperationStatus::ok()))
    }

    pub async fn hset_json<T: Serialize>(
        &self,
        key: &str,
        field: &str,
        value: &T,
    ) -> Result<Option<Value>, Error> {
        let json = serde_json::to_string(value)?;
        Ok(self.hset(key, field, &json).await?)
    }

    pub async fn hdel(
        &self,
        key: &str,
        field: &str,
    ) -> Result<Option<Value>, Error> {
        let mut conn = self.get_conn().await?;
        conn.hdel(key, field).await?;
        Ok(Some(OperationStatus::ok()))
    }

    // 获取哈希表字段
    pub async fn hget(
        &self,
        key: &str,
        field: &str,
    ) -> Result<Option<String>, Error> {
        let mut conn = self.get_conn().await?;
        let value: Option<String> = conn.hget(key, field).await?;
        Ok(value)
    }

    pub async fn hmget(
        &self,
        key: &str,
        field: &Vec<String>,
    ) -> Result<Vec<String>, Error> {
        let mut conn = self.get_conn().await?;
        let value: Vec<String> = conn.hmget(key, field).await?;
        Ok(value)
    }

    pub async fn hget_all(&self, key: &str) -> Result<Option<Value>, Error> {
        let mut conn = self.get_conn().await?;
        let value = conn.hgetall(key).await?;
        Ok(Some(json!(value)))
    }

    // 设置过期时间
    pub async fn expire(&self, key: &str, seconds: i64) -> Result<(), Error> {
        let mut conn = self.get_conn().await?;
        conn.expire(key, seconds).await?;
        Ok(())
    }

    // 检查键是否存在
    pub async fn exists(&self, key: &str) -> Result<bool, Error> {
        let mut conn = self.get_conn().await?;
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }

    // 递增
    pub async fn incr(
        &self,
        key: &str,
        increment: i64,
    ) -> Result<isize, Error> {
        let mut conn = self.get_conn().await?;
        let result = conn.incr(key, increment).await?;
        Ok(result)
    }
}
// 定义一个 trait 来扩展 RedisClient
pub trait RedisResult {
    // fn get_string_result(&self) -> ApiResponse<String>;
    fn get_json_result(&self) -> ApiResponse<Value>;

    // fn get_message_result(&self, msg: &str) -> ApiResponse<String>;

    fn get_json(&self) -> Result<Value, AppError>;
}

trait RedisResultExtract<T> {
    //这个方法直接在路由里面调用,直接可以返回了
    fn result_extract<F, R>(&self, f: F) -> ApiResponse<R>
    where
        F: FnOnce(&T) -> R;

    //这个方法在路由里面调用还要再处理才能返回
    fn extract<F, R>(&self, f: F) -> Result<R, AppError>
    where
        F: FnOnce(&T) -> R;
}

impl<T> RedisResultExtract<T> for Result<Option<T>, Error> {
    fn result_extract<F, R>(&self, f: F) -> ApiResponse<R>
    where
        F: FnOnce(&T) -> R,
    {
        match self {
            Ok(Some(data)) => ApiResult::success(f(data)),
            Ok(None) => ApiResult::error(-404, "Data not found in Redis".to_string()),
            Err(e) => ApiResult::error(-2, e.to_string()),
        }
    }

    fn extract<F, R>(&self, f: F) -> Result<R, AppError>
    where
        F: FnOnce(&T) -> R,
    {
        match self {
            Ok(Some(data)) => Ok(f(data)),
            Ok(None) => AppError::new(StatusCode::NOT_FOUND, "not data found".to_string()),
            Err(e) => AppError::new(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
        }
    }
}
impl<T> RedisResult for Result<Option<T>, Error>
where
    T: Serialize + Display,
{
    // fn get_string_result(&self) -> ApiResponse<String> {
    //     self.result_extract(|data| data.to_string())
    // }

    fn get_json_result(&self) -> ApiResponse<Value> {
        self.result_extract(|data| parse_json(data.to_string().as_str()))
    }

    // fn get_message_result(&self, msg: &str) -> ApiResponse<String> {
    //     self.result_extract(|_| msg.to_string())
    // }

    fn get_json(&self) -> Result<Value, AppError> {
        self.extract(|data| parse_json(data.to_string().as_str()))
    }
}
