mod public;
mod sugar;
mod system;
mod test;
mod user;

use crate::config::AppConfig;
use crate::models::{AppState, CgmType, UserSetting};
use crate::register::{auth_middleware, cors_layer, logging_middleware};
use crate::routes::public::public_router;
use crate::routes::sugar::sugar_router;
use crate::routes::system::system_router;
use crate::routes::test::test_router;
use crate::routes::user::user_router;
use crate::utils::jwt_token::JwtConfig;
use crate::utils::mail::EmailService;
use crate::utils::redis_client::{create_redis_pool, RedisResult, RedisService};
use crate::utils::task::{add_scheduler_job, TaskManager};
use crate::utils::{parse_json, JsonHelp};
use axum::http::StatusCode;
use axum::{middleware, Router};
use tracing::{error, info};

pub async fn create_routes_and_init_app_state(config: &AppConfig) -> Router {
    let jwt_config = JwtConfig::new(
        config.default.jwt_secret.to_string(), // 生产环境应该使用环境变量
        config.default.jwt_exp_hours,          // 24小时过期
    );
    // 创建 Redis 连接池
    let redis_pool = create_redis_pool(&config.redis).await.unwrap();
    let redis_service = RedisService::new(redis_pool);
    let task_manager = TaskManager::new().await;
    let email = EmailService::new(
        config.mail.smtp_host.to_string(),
        config.mail.smtp_port,
        config.mail.user.to_string(),
        config.mail.password.to_string(),
        config.mail.user.to_string(),
        config.mail.to.to_string(),
    );
    // 应用状态
    let state = AppState::new(redis_service, jwt_config, task_manager, email);

    //初始化用户状态以及开启定时任务
    init_user_state(state.clone(), config.scheduler.enabled).await;
    Router::new().nest(
        "/api",
        api_routes(state.clone())
            .layer(middleware::from_fn_with_state(
                state.jwt_config,
                auth_middleware,
            ))
            .layer(cors_layer(config))
            .layer(middleware::from_fn(logging_middleware)),
    )
}

fn api_routes(app_state: AppState) -> Router {
    Router::new()
        .nest("/public", public_router())
        .nest("/test", test_router())
        .nest("/user", user_router())
        .nest("/sugar", sugar_router())
        .nest("/system", system_router())
        .fallback(|| async { (StatusCode::NOT_FOUND, "404 Not Found") })
        .with_state(app_state)
}

async fn init_user_state(state: AppState, is_scheduler: bool) {
    let result = state.redis.hget_all("user").await.get_json();
    match result {
        Ok(data) => {
            if let Some(obj) = data.as_object() {
                for (name, value) in obj {
                    if let Some(json_str) = value.as_str() {
                        // 只解析需要的字段
                        let config = parse_json(json_str);
                        let username = config.get_string("username");
                        if username != "" {
                            let setting = UserSetting {
                                user_key: name.to_string(),
                                carelink_password: config.get_string("carelink_password"),
                                cgm: CgmType::new(config.get_string("cgm").as_str()),
                                patient_id: config.get_string("patientId"),
                                username: config.get_string("username"),
                                role: config.get_string("role"),
                                admin: config.get_bool("admin"),
                                sse_interval: config.get_i64_or("sse_interval", 30),
                                carelink_token_refresh_interval: config
                                    .get_i64_or("carelink_token_refresh_interval", 150),
                                carelink_data_refresh_interval: config
                                    .get_i64_or("carelink_data_refresh_interval", 150),
                            };
                            state.save_user_settings(&name, setting.clone()).await;
                            info!("用户:{}数据初始化完成", username);
                            if is_scheduler {
                                match add_scheduler_job(state.clone(), setting.clone()).await {
                                    Ok(_) => info!("用户:{}计划任务初始化完成", username),
                                    Err(e) => error!("用户:{}任务初始化错误: {:#?}", username, e),
                                }
                                state.task_manager.start().await;
                            }
                        } else {
                            error!("用户:{}缺少数据,初始化失败", name);
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("数据初始画错误: {:#?}", e);
        }
    }
}
