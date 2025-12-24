pub mod ar2;
pub mod jwt_token;
pub mod redis_client;
pub mod task;
pub mod mail;

use crate::models::AppError;
use axum::http::StatusCode;
use chrono::Local;
use garde::Report;
use serde_json::{from_str, Value};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
pub fn init_logger(log_level: &str) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_timer(ChronoLocal::rfc_3339()))
        .init();
}

pub fn format_err(e: Report) -> String {
    e.to_string()
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join(", ")
}

pub trait JsonHelp {
    fn show(&self);
    fn get_val(&self, key: &str) -> Result<&str, AppError>;
    fn get_string(&self, key: &str) -> String;
    fn get_string_or(&self, key: &str, default: &str) -> String;
    fn get_bool(&self, key: &str) -> bool;
    fn get_bool_or(&self, key: &str, default: bool) -> bool;
    fn get_i64(&self, key: &str) -> i64;
    fn get_i64_or(&self, key: &str, default: i64) -> i64;
    fn get_f64(&self, key: &str) -> f64;
    fn get_f64_or(&self, key: &str, default: f64) -> f64;
}

impl JsonHelp for Value {
    fn show(&self) {
        match self {
            Value::Null => println!("null"),
            Value::Bool(b) => println!("bool: {}", b),
            Value::Number(n) => println!("number: {}", n),
            Value::String(s) => println!("string: {}", s),
            Value::Array(arr) => println!("array: {:?}", arr),
            Value::Object(obj) => println!("object: {:?}", obj),
        }
    }
    fn get_val(&self, key: &str) -> Result<&str, AppError> {
        match self.get(key) {
            Some(Value::String(s)) => Ok(s),
            _ => AppError::new(StatusCode::NOT_FOUND, format!("{} not found", key)),
        }
    }

    fn get_string(&self, key: &str) -> String {
        self.get_string_or(key, "")
    }

    fn get_string_or(&self, key: &str, default: &str) -> String {
        self.get(key).map_or(default.to_string(), |v| {
            v.as_str().unwrap_or("").to_string()
        })
    }

    fn get_bool(&self, key: &str) -> bool {
        self.get_bool_or(key, false)
    }

    fn get_bool_or(&self, key: &str, default: bool) -> bool {
        self.get(key)
            .map_or(default, |v| v.as_bool().unwrap_or(default))
    }

    fn get_i64(&self, key: &str) -> i64 {
        self.get_i64_or(key, 0)
    }

    fn get_i64_or(&self, key: &str, default: i64) -> i64 {
        self.get(key)
            .map_or(default, |v| v.as_i64().unwrap_or(default))
    }

    fn get_f64(&self, key: &str) -> f64 {
        self.get_f64_or(key, 0.0)
    }

    fn get_f64_or(&self, key: &str, default: f64) -> f64 {
        self.get(key)
            .map_or(default, |v| v.as_f64().unwrap_or(default))
    }
}

// pub fn parse_json_result(str: &str) -> Result<Value, AppError> {
//     from_str::<Value>(str).map_err(|e| AppError {
//         status: StatusCode::UNPROCESSABLE_ENTITY,
//         msg: format!("json parse error,{}", e),
//     })
// }

pub fn parse_json(str: &str) -> Value {
    from_str(str).unwrap_or(Value::Null)
}

pub struct DateUtils;

impl DateUtils {
    pub const DATE_TIME: &'static str = "%Y-%m-%d %H:%M:%S";
    pub const DATE: &'static str = "%Y-%m-%d";
    // pub const TIME: &'static str = "%H:%M:%S";

    pub fn datetime() -> String {
        Local::now().format(Self::DATE_TIME).to_string()
    }
}
#[cfg(test)]
mod utils_test {
    use super::*;
    use tracing::{debug, info, warn};

    #[test]
    fn test_log() {
        init_logger("warn");
        warn!("warn test");
        info!("info test");
        debug!("debug test");
    }
}
