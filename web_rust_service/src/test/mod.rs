pub mod test_login;

#[cfg(test)]
mod test {
    #[allow(dead_code)]
    #[allow(unused)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    use crate::config::RedisConfig;
    use crate::models::{AppState, CgmType, UserSetting};
    use crate::services::ns_service::{utc_to_east8_string, NSSource, TrendConverter};
    use crate::services::sugar_service::{
        carelink_login, update_statistics, DictKeys, CARELINK_DATA_URL,
        CARELINK_REFRESH_TOKEN_URL, HTTP_TIMEOUT, UA,
    };
    use crate::test::test_login::test_carelink_login;
    use crate::utils;
    use crate::utils::ar2::forecast_ar2_sg;
    use crate::utils::jwt_token::JwtConfig;
    use crate::utils::mail::EmailService;
    use crate::utils::redis_client::{create_redis_pool, RedisResult, RedisService};
    use crate::utils::task::{ScheduleType, TaskManager};
    use crate::utils::{DateUtils, JsonHelp};
    use axum::http::StatusCode;
    use chrono::{DateTime, Local, NaiveDateTime, Utc};
    use reqwest::cookie::Jar;
    use reqwest::redirect::Policy;
    use reqwest::Client;
    use serde_json::{json, Value};
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;
    use tracing::{error, info};

    #[test]
    fn common_test() {
        let t = Value::from(96.0);
        println!("{}", t.as_f64().unwrap());
    }
    #[test]
    fn test_time() {
        let utc_now = Utc::now();
        println!("UTC 当前时间: {}", utc_now);
        println!("UTC 时间戳(秒): {}", utc_now.timestamp());
        println!("UTC 时间戳(毫秒): {}", utc_now.timestamp_millis());
        println!("UTC 时间戳(微秒): {}", utc_now.timestamp_micros());
        println!("UTC 时间戳(纳秒): {:?}", utc_now.timestamp_nanos_opt());

        // 获取本地时间戳
        let local_now = Local::now();
        println!("本地当前时间: {}", local_now.format("%Y-%m-%d %H:%M:%S"));
        println!("本地时间戳(秒): {}", local_now.timestamp());
        println!("本地时间戳(毫秒): {}", local_now.timestamp_millis());

        // 从字符串解析时间
        let datetime_str = "2023-01-01 12:00:00";
        let naive_datetime =
            NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").unwrap();
        println!("解析的时间戳(秒): {}", naive_datetime.and_utc().timestamp());

        // 时间戳转 DateTime
        let timestamp = 1672531200; // 2023-01-01 00:00:00 UTC
        let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
        println!("时间戳转时间: {}", datetime);

        // 获取当前时间字符串
        let formatted = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        println!("格式化时间: {}", formatted);

        // 创建指定时间的 UTC 时间
        let naive_time =
            NaiveDateTime::parse_from_str(DateUtils::datetime().as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let date_time_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);
        let time_str = date_time_utc.to_rfc3339();
        println!("指定时间 (UTC): {}", time_str);
        println!(
            "{:?}",
            DateTime::parse_from_rfc3339("2024-10-09T10:03:01.000-00:00")
        );
    }
    #[test]
    fn test_default() {
        let json = json!(
      { "cgm": "carelink", "patient_id": "date13", "username": "date13", "role": "patient", "admin": true,
        "carelink_token_refresh_interval": 150, "carelink_data_refresh_interval": 55 });
        json.show();
        println!("{}", json.get_bool("admin"));
        println!("{}", json.get_i64("carelink_data_refresh_interval"));
    }

    async fn get_redis() -> (&'static str, RedisService, Client) {
        let config = RedisConfig {
            url: "redis://localhost:6379/13".to_string(),
            max_connections: 20,
            min_connections: 1,
            connection_timeout: 30,
            max_lifetime: 1800,
        };
        let redis_pool = create_redis_pool(&config).await.unwrap();
        let http_client = Client::builder()
            .redirect(Policy::none())
            .cookie_store(true)
            .use_rustls_tls()
            .timeout(Duration::from_secs(HTTP_TIMEOUT))
            .build()
            .expect("Failed to build reqwest client");
        ("alex", RedisService::new(redis_pool), http_client)
    }

    #[allow(dead_code)]
    #[allow(unused)]
    #[allow(unused_variables)]
    async fn get_state_and_user_setting() -> (AppState, UserSetting) {
        let config = RedisConfig {
            url: "redis://localhost:6379/13".to_string(),
            max_connections: 20,
            min_connections: 1,
            connection_timeout: 30,
            max_lifetime: 1800,
        };
        let cookie_jar = Jar::default();
        let redis_pool = create_redis_pool(&config).await.unwrap();
        let state = AppState::new(
            RedisService::new(redis_pool),
            JwtConfig::new("123".to_string(), 1),
            TaskManager::new().await,
            EmailService::new(
                false,
                "localhost".to_string(),
                455,
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
            ),
        );
        (
            state,
            UserSetting {
                user_key: "alex".to_string(),
                admin: true,
                carelink_password: "pigMing1983!".to_string(),
                carelink_data_refresh_interval: 160,
                carelink_token_refresh_interval: 350,
                cgm: CgmType::Carelink,
                patient_id: "date13".to_string(),
                role: "patient".to_string(),
                ..UserSetting::null()
            },
        )
    }

    #[allow(dead_code)]
    #[allow(unused)]
    #[allow(unused_variables)]
    #[tokio::test]
    async fn test_carewlink_login() {
        utils::init_logger("info");
        let (state, user_setting) = get_state_and_user_setting().await;
        match carelink_login(&state, &user_setting).await {
            Ok(_) => {}
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }

    #[allow(dead_code)]
    #[allow(unused)]
    #[allow(unused_variables)]
    #[tokio::test]
    async fn test_json() {
        let (name, redis_service, client) = get_redis().await;
        let data_key = format!("{}:{}", name, DictKeys::DATA);

        if let Ok(Some(mut org_data)) = redis_service.get_json::<Value>(data_key.as_str()).await {
            let mut org_sugar_data = &mut org_data["data"];
            // save_history_data(&mut org_sugar_data, &redis_service, name.to_string()).await;
            // update_luck_data(&mut org_sugar_data, &redis_service, name.to_string()).await;
            // update_carelink_my_data_yesterday_data(
            //     &mut org_sugar_data,
            //     &redis_service,
            //     name.to_string(),
            // )
            //     .await;
            update_statistics(&mut org_data, &redis_service, name.to_string()).await;
        }

        // // 假设 redis 连接已经建立
        // if let Ok(Some(mut org_data)) = redis_service.get_json::<Value>(data_key.as_str()).await {
        //     let sgs = org_data["data"]["sgs"]
        //         .as_array()
        //         .map(|array| {
        //             array.iter()
        //                 .filter_map(|item| {
        //                     Some(item.get_i64("sg", 0) as f64)
        //                 }).collect::<Vec<f64>>()
        //         })
        //         .unwrap_or_default();
        //
        //     org_data["forecast"] = json!({
        //       "ar2": forecast_ar2_sg(&sgs,20)
        //     });
        //     println!("{:#?}", org_data);
        // }

        // let password = "205DED2984E1DAF6E3B9373767E8BF6E";
        // if let Ok(config) = redis_service.hget("user", "alex").await.get_json() {
        //     config.show();
        //     assert_eq!(config.get_string("password", ""), password);
        //     println!(
        //         "{},{}",
        //         config.get_string("password", ""),
        //         config.get_string("password", "") == password
        //     );
        // }
    }

    #[tokio::test]
    async fn test_date() {
        info!("Carelink refresh data");
        let (name, redis, client) = get_redis().await;
        let params = HashMap::from([
            ("patientId", "date13"),
            ("username", "date13"),
            ("role", "patient"),
        ]);
        let auth_key = format!("{}:{}", name, DictKeys::AUTH);
        match redis.get_json::<Value>(&auth_key).await.get_json() {
            Ok(auth_data) => {
                let token = auth_data.get_string("token");
                let response = client
                    .post(CARELINK_DATA_URL)
                    .json(&params)
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", token))
                    .header("User-Agent", UA)
                    .send()
                    .await
                    .expect("Failed to send data request");

                let status = response.status();
                if status == StatusCode::OK {
                    let result = response
                        .json::<Value>()
                        .await
                        .expect("Failed to parse JSON response");
                    println!("用户:{} 远程读取 carelinkData 数据成功!!!", name);
                    print!("result:{}", result);
                    // result
                } else {
                    let text = format!("用户:{} 读取 carelinkData 数据失败!!!:{}", name, status);
                    println!("{}", text);
                    // send_mail(&text); // 邮件发送函数需要另外实现
                    // Err(reqwest::Error::from("")
                }
            }
            _ => {}
        }
    }

    #[allow(dead_code)]
    #[allow(unused)]
    #[allow(unused_variables)]
    #[tokio::test]
    async fn test_token() {
        let (name, redis, client) = get_redis().await;

        let auth_key = format!("{}:{}", name, DictKeys::AUTH);
        match redis.get_json::<Value>(&auth_key).await.get_json() {
            Ok(mut auth_data) => {
                let token = auth_data.get_string("token");
                let params = HashMap::from([("country", "HK"), ("locale", "zh")]);
                println!("token:{}", token);
                let response = client
                    .post(CARELINK_REFRESH_TOKEN_URL)
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", token).to_string())
                    .header("User-Agent", UA)
                    .send()
                    .await
                    .expect("Failed to send token request");

                let status = response.status();
                println!("status:{}", status);
                if status == StatusCode::OK {
                    // 获取所有 cookies
                    let cookies: Vec<reqwest::cookie::Cookie> = response.cookies().collect();
                    let auth_tmp_token = cookies
                        .iter()
                        .find(|cookie| cookie.name() == "auth_tmp_token")
                        .map(|cookie| cookie.value().to_string());
                    auth_data["token"] = Value::from(auth_tmp_token.unwrap().to_string());
                    println!("用户:{} 获取 carelinkToken 数据成功!!!", name);
                    // result
                } else {
                    let text = format!("用户:{} 获取 carelinkToken 数据失败!!!:{}", name, status);
                    println!("{}", text);
                }
            }
            Err(e) => {
                error!("Carelink refresh token failed: {}", e);
            }
        }
    }

    // Needs multi_thread to test, otherwise it hangs on scheduler.add()
    #[tokio::test]
    // #[tokio::test]
    async fn test_schedule() {
        let task_manager = Arc::new(TaskManager::new().await);
        // 添加示例任务
        let tm_clone = task_manager.clone();
        let a = Arc::new("test");
        println!("count after creating a = {}", Arc::strong_count(&a));

        tm_clone
            .add_task(
                "task1",
                "数据同步任务",
                &ScheduleType::Repeated(Duration::from_secs(5)),
                move || {
                    let a = Arc::clone(&a);
                    Box::pin(async move {
                        println!("正在同步数据...,{}", a);
                    })
                },
            )
            .await;
        //
        // // 添加任务2：每分钟执行一次
        // tm_clone
        //     .add_task(
        //         "task2",
        //         "清理任务",
        //         "0 * * * * *", // 每分钟的0秒
        //         || {
        //             Box::pin(async move {
        //                 println!("正在清理临时文件...");
        //             })
        //         },
        //     )
        //     .await;

        // 启动所有任务
        tm_clone.start().await;
        // 运行一段时间
        sleep(Duration::from_secs(15)).await;
        tm_clone.shutdown().await;
    }

    #[test]
    fn test_ar2_forecast() {
        // 测试数据：模拟血糖值
        let test_data: Vec<f64> = vec![
            99.0, 101.0, 90.0, 74.0, 59.0, 52.0, 58.0, 73.0, 83.0, 80.0, 77.0, 71.0, 69.0, 67.0,
            67.0, 68.0, 69.0, 81.0, 80.0, 81.0, 81.0, 81.0, 82.0, 79.0, 79.0, 79.0, 75.0, 78.0,
            93.0, 120.0, 146.0, 167.0, 189.0, 196.0, 211.0, 230.0, 221.0, 214.0, 209.0, 205.0,
            197.0, 185.0, 177.0, 160.0, 149.0, 134.0, 122.0, 131.0, 127.0, 125.0, 124.0, 122.0,
            118.0, 114.0, 109.0, 96.0, 90.0, 87.0, 80.0, 62.0, 54.0, 56.0, 63.0, 71.0, 88.0, 92.0,
            96.0, 98.0, 90.0, 68.0, 66.0, 62.0, 63.0, 63.0, 62.0, 70.0, 70.0, 71.0, 72.0, 81.0,
            97.0, 108.0, 114.0, 117.0, 119.0, 119.0, 116.0, 115.0, 116.0, 123.0, 120.0, 119.0,
            116.0, 117.0, 120.0, 118.0, 117.0, 114.0, 110.0, 112.0, 120.0, 135.0, 154.0, 168.0,
            172.0, 168.0, 164.0, 157.0, 152.0, 155.0, 157.0, 163.0, 172.0, 166.0, 161.0, 148.0,
            146.0, 143.0, 141.0, 139.0, 136.0, 132.0, 136.0, 138.0, 148.0, 154.0, 147.0, 144.0,
            140.0, 127.0, 117.0, 102.0, 94.0, 84.0, 73.0, 66.0, 61.0, 57.0, 60.0, 67.0, 74.0, 89.0,
            100.0, 109.0, 112.0, 117.0, 121.0, 117.0, 113.0, 114.0, 112.0, 110.0, 109.0, 104.0,
            100.0, 94.0, 92.0, 96.0, 99.0, 101.0, 107.0, 105.0, 105.0, 104.0, 101.0, 96.0, 94.0,
            94.0, 95.0, 96.0, 98.0, 100.0, 101.0, 101.0, 101.0, 99.0, 97.0, 92.0, 87.0, 83.0, 78.0,
            74.0, 74.0, 73.0, 75.0, 78.0, 80.0, 85.0, 87.0, 88.0, 90.0, 91.0, 90.0, 90.0, 89.0,
            88.0, 87.0, 84.0, 80.0, 78.0, 74.0, 73.0, 72.0, 75.0, 78.0, 81.0, 84.0, 87.0, 92.0,
            95.0, 94.0, 93.0, 90.0, 87.0, 83.0, 80.0, 77.0, 76.0, 75.0, 75.0, 75.0, 76.0, 76.0,
            76.0, 75.0, 74.0, 75.0, 77.0, 77.0, 80.0, 83.0, 86.0, 91.0, 91.0, 93.0, 97.0, 95.0,
            93.0, 91.0, 90.0, 89.0, 86.0, 84.0, 83.0, 82.0, 82.0, 81.0, 83.0, 84.0, 91.0, 93.0,
            97.0, 94.0, 92.0, 95.0, 102.0, 111.0, 118.0, 117.0, 113.0, 112.0, 110.0, 107.0, 104.0,
            107.0, 106.0, 104.0, 104.0, 104.0, 106.0, 109.0, 110.0, 112.0, 110.0, 109.0, 106.0,
            102.0, 98.0, 95.0, 94.0, 94.0, 94.0, 94.0, 94.0, 87.0, 76.0,
        ];

        let forecast = forecast_ar2_sg(&test_data, 20);

        println!("预测结果: {:?}", forecast);
        assert_eq!(forecast.len(), 20);
        //
        // // 检查预测值是否合理
        // for &value in &forecast {
        //     assert!(value >= 5 && value <= 7, "预测值 {} 超出合理范围", value);
        // }
    }

    #[test]
    fn test_with_zeros() {
        // 测试包含零值的数据
        let test_data = vec![0.0, 5.6, 0.0, 5.8, 5.7, 0.0, 5.9, 6.0];

        let forecast = forecast_ar2_sg(&test_data, 3);
        println!("包含零值的预测: {:?}", forecast);
    }

    #[test]
    fn test_datetime_change() {
        let new_time = utc_to_east8_string("2026-02-06T11:56:55Z");

        println!("新时间: {:?}", new_time);
    }

    #[test]
    fn test_trend() {
        let converter = TrendConverter::new();
        if let Some(carelink) = converter.convert_str("FortyFiveDown") {
            println!("trend -> {:?}", carelink);
        }
        println!("source:{}", NSSource::Ottai.get_source());
    }

    #[tokio::test]
    async fn test_mail() {
        let email = EmailService::new(
            true,
            "smtp.exmail.qq.com".to_string(),
            465,
            "shanghaiyiyiba2012nan@sweetie-online.com".to_string(),
            "Dwnt6x2mUUY5xftM".to_string(),
            "shanghaiyiyiba2012nan@sweetie-online.com".to_string(),
            "date13@qq.com".to_string(),
        );
        match email
            .send_text_email("test", "中文测试".to_string())
            .await
        {
            Err(e) => {
                println!("send mail error:{}", e);
            }
            Ok(_) => {
                println!("send mail success");
            }
        }
    }

    #[tokio::test]
    async fn test_interval() {
        // let current_time = Local::now();
        // println!("{}", current_time);
        // println!("{:?}", DateTime::parse_from_rfc3339("2026-02-10T09:12:02.000Z").ok());

        let now_local = DateTime::parse_from_rfc3339("2026-02-10T09:14:02.000+08:00")
            .ok()
            .unwrap();

        // 解析目标时间（注意时区是 -00:00，即 UTC）
        let target_time = DateTime::parse_from_rfc3339("2026-02-10T09:11:49.000+08:00")
            .expect("Failed to parse datetime");

        // 将本地时间转换为 UTC 以便比较
        // let now_utc = now_local.with_timezone(&chrono::Utc);
        // let target_utc = target_time.with_timezone(&chrono::Utc);
        // println!("{}", now_utc);
        // 计算时间差
        let duration = now_local.signed_duration_since(target_time);

        // 转换为分钟
        println!("{}", duration.num_minutes());

        // println!("{}", DateUtils::datetime_local());
    }
    #[tokio::test]
    async fn test_login() {
        match test_carelink_login().await {
            Ok(_) => {
                println!("login success");
            }
            Err(e) => {
                println!("login error:{}", e);
            }
        }
    }
}
