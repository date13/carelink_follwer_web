use crate::models::{AppState, UserSetting};
use crate::utils::ar2::forecast_ar2_sg;
use crate::utils::mail::EmailService;
use crate::utils::redis_client::{RedisResult, RedisService};
use crate::utils::{parse_json, DateUtils, JsonHelp};
use axum::http::StatusCode;
use chrono::{Duration, Local};
use garde::rules::pattern::regex::Regex;
use reqwest::{Client, Response};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::string::ToString;
use std::sync::Arc;
use tracing::{debug, error, info};
use url::Url;

pub const CARELINK_BASE_URL: &str = "https://carelink.minimed.eu";
pub const CARELINK_LOGIN_URL: &str = "https://carelink-login.minimed.eu";
pub const CARELINK_REFRESH_TOKEN_URL: &str = "/patient/sso/reauth";
pub const CARELINK_DATA_URL: &str =
    "https://clcloud.minimed.eu/connect/carepartner/v6/display/message";
pub const HTTP_TIMEOUT: u64 = 120;
pub const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0";
const FORECAST_COUNT: usize = 30;

const LUCK_LIMIT: u8 = 92;
const MIN_LUCK_CV: u8 = 27;
// 或者使用 associated constants
pub struct DictKeys {}

impl DictKeys {
    pub const LUCK: &'static str = "luck";
    pub const AUTH: &'static str = "carelinkAuth";
    pub const DATA: &'static str = "carelinkData";
    pub const MY_DATA: &'static str = "carelinkMyData";
    pub const HISTORY: &'static str = "history";
    // pub const SETTING: &'static str = "setting";
    pub const FOOD: &'static str = "food";
}

pub async fn carelink_refresh_token(state: &AppState, user_setting: &UserSetting) {
    // state.redis.get_json()
    info!("Carelink refresh token");
    let user_key = user_setting.user_key.as_str();
    let auth_key = format!("{}:{}", user_key, DictKeys::AUTH);
    match state.redis.get_json::<Value>(&auth_key).await.get_json() {
        Ok(mut auth_data) => {
            let token = auth_data.get_string("token");
            let status = auth_data.get_i64("status");
            if status != 200 {
                info!(
                    "用户:{} refreshCarelinkToken token失效,请手动刷新Token,当前状态:{}",
                    user_key, status
                );
            } else {
                let response = state
                    .http_client
                    .post(format!(
                        "{}{}",
                        CARELINK_BASE_URL, CARELINK_REFRESH_TOKEN_URL
                    ))
                    // .json(&params)
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", token))
                    .header("User-Agent", UA)
                    .send()
                    .await
                    .inspect_err(|e| {
                        send_mail(state.email.clone(), format!("刷新token错误,{:#?}", e));
                    })
                    .expect("Failed to send token request");

                let status = response.status();
                let user = &user_setting.user_key;
                auth_data["status"] = status.as_u16().into();
                if status == StatusCode::OK {
                    // 获取所有 cookies
                    let cookies: Vec<reqwest::cookie::Cookie> = response.cookies().collect();
                    let auth_tmp_token = cookies
                        .iter()
                        .find(|cookie| cookie.name() == "auth_tmp_token")
                        .map(|cookie| cookie.value().to_string());
                    auth_data["token"] = Value::from(auth_tmp_token.unwrap().to_string());
                    debug!("用户:{} 获取 carelinkToken 数据成功!!!", user);
                    // result
                } else {
                    let text = format!("用户:{} 获取 carelinkToken 数据失败!!!:{}", user, status);
                    send_mail(state.email.clone(), text.clone());
                    error!("{}", text)
                }
                update_carelink_auth_data_to_redis(&mut auth_data, &state.redis, user_key).await;
            }
        }
        Err(e) => {
            error!("Carelink refresh token failed: {}", e);
        }
    }
}

pub async fn carelink_refresh_data(state: &AppState, user_setting: &UserSetting) {
    // state.redis.get_json()
    info!("Carelink refresh data");
    let auth_key = format!("{}:{}", user_setting.user_key, DictKeys::AUTH);
    let redis = state.redis.clone(); // 确保 RedisService 实现了 Clone + Send
    let setting = user_setting.clone();
    match state.redis.get_json::<Value>(&auth_key).await.get_json() {
        Ok(data) => {
            let status = data.get_i64("status") as u16;
            if status == StatusCode::UNAUTHORIZED.as_u16() {
                info!(
                    "用户:{} refreshCarelinkData token失效,开始自动刷新Token,当前状态:{}",
                    user_setting.user_key, status
                );
                match carelink_login(&state, &setting).await {
                    Ok(_) => {}
                  Err(_) => {
                        update_carelink_data(
                            &redis, // 注意：这里传递的是 &RedisService
                            &setting.user_key,
                            status as i32,
                            None,
                        )
                            .await
                    }
                }
            } else {
                let token = data.get_string("token");
                if token.is_empty() {
                    info!(
                        "用户:{} refreshCarelinkData token为空,请手动刷新Token",
                        user_setting.user_key
                    );
                } else {
                    // 克隆需要的部分 - 确保都是 Send 的
                    let (status, json_data) = load_carelink_data(
                        &state.http_client,
                        state.email.clone(),
                        &token,
                        setting.clone(),
                    )
                        .await;
                    update_carelink_data(
                        &redis, // 注意：这里传递的是 &RedisService
                        &setting.user_key,
                        status,
                        json_data,
                    )
                        .await;
                }
            }
        }
        Err(e) => {
            error!("Carelink refresh data failed: {}", e);
        }
    }
}

pub async fn carelink_refresh_history(state: &AppState, user_setting: &UserSetting) {
    // state.redis.get_json()
    info!("Carelink refresh history");
    let user_key = user_setting.user_key.clone();
    let data_key = format!("{}:{}", user_key, DictKeys::DATA);
    // 假设 redis 连接已经建立
    if let Ok(Some(mut org_data)) = state.redis.get_json::<Value>(data_key.as_str()).await {
        let mut org_sugar_data = &mut org_data["data"];
        update_carelink_my_data_yesterday_data(&mut org_sugar_data, &state.redis, user_key.clone())
            .await;
        save_history_data(&mut org_sugar_data, &state.redis, user_key.clone()).await;
        update_luck_data(&mut org_sugar_data, &state.redis, user_key.clone()).await;
        update_statistics(&mut org_data, &state.redis, user_key.clone()).await;
    } else {
        error!(
            "Failed to get carelink data when carelink_refresh_history: {:?}",
            data_key
        );
    }
}
pub async fn load_carelink_data(
    client: &Client,
    email: Arc<EmailService>,
    token: &str,
    user_setting: UserSetting,
) -> (i32, Option<Value>) {
    let params = HashMap::from([
        ("patientId", &user_setting.patient_id),
        ("username", &user_setting.username),
        ("role", &user_setting.role),
    ]);

    let response = client
        .post(CARELINK_DATA_URL)
        .json(&params)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", UA)
        .send()
        .await
        .inspect_err(|e| {
            send_mail(email.clone(), format!("刷新CarelinkDate错误,{:#?}", e));
        })
        .expect("Failed to send data request");

    let status = response.status();
    let user = &user_setting.user_key;
    if status == StatusCode::OK {
        match response.json::<Value>().await {
            Ok(value) => {
                info!("用户:{:?} 远程读取 carelinkData 数据成功!!!", user);
                (StatusCode::OK.as_u16() as i32, Some(value))
            }
            Err(e) => {
                send_mail(email.clone(), format!("解析CarelinkDate JSON错误,{:#?}", e));
                // 返回错误状态码和 None
                (StatusCode::UNPROCESSABLE_ENTITY.as_u16() as i32, None)
            }
        }
        // result
    } else {
        let text = format!("用户:{} 读取 carelinkData 数据失败!!!:{}", user, status);
        send_mail(email.clone(), text.clone());
        error!("{}", text);
        // send_mail(&text); // 邮件发送函数需要另外实现
        // Err(reqwest::Error::from("")
        (status.as_u16() as i32, None)
    }
}

// 更新Redis数据
async fn update_carelink_data(
    redis: &RedisService,
    user_key: &str,
    status: i32,
    data: Option<Value>,
) {
    let data_key = format!("{}:{}", user_key, DictKeys::DATA);
    // 假设 redis 连接已经建立
    if let Ok(Some(mut org_data)) = redis.get_json::<Value>(data_key.as_str()).await {
        org_data["status"] = status.into();

        if let Some(data) = data {
            let system_status = data.get_string("systemStatusMessage");
            deal_warm_up(system_status, &mut org_data);
            org_data["data"] = data;
            let sgs = org_data["data"]["sgs"]
                .as_array()
                .map(|array| {
                    array
                        .iter()
                        .filter_map(|item| Some(item.get_i64("sg") as f64))
                        .collect::<Vec<f64>>()
                })
                .unwrap_or_default();

            org_data["forecast"] = json!({
              "ar2": forecast_ar2_sg(&sgs,FORECAST_COUNT)
            });
        }
        update_carelink_data_to_redis(&mut org_data, redis, user_key).await;
    } else {
        error!("Failed to get carelink data from redis: {:?}", data_key);
    }
}

const NEXT_START_TIME_KEY: &str = "nextStartTime";
fn deal_warm_up(system_status: String, org_data: &mut Value) {
    let next_start: i64 = org_data[NEXT_START_TIME_KEY].as_i64().unwrap_or(0);
    if system_status == "WARM_UP" && next_start == -1 {
        org_data[NEXT_START_TIME_KEY] =
            Value::from((Local::now() + Duration::hours(2)).timestamp_millis());
    }
    if system_status == "NO_ERROR_MESSAGE" && next_start != -1 {
        org_data[NEXT_START_TIME_KEY] = Value::from(-1);
    }
}

async fn update_carelink_auth_data_to_redis(
    data: &mut Value,
    redis: &RedisService,
    user_key: &str,
) {
    let auth_key = format!("{}:{}", user_key, DictKeys::AUTH);
    data["update_time"] = DateUtils::datetime().into();
    match redis.set_json(&auth_key, &data, None).await {
        Ok(_) => {
            info!("用户:{} 刷新 carelinkAuthData 数据成功!!!", auth_key);
        }
        Err(e) => {
            error!("用户:{} 刷新 carelinkAuthData 数据失败!!!:{}", auth_key, e);
        }
    }
}

async fn update_carelink_data_to_redis(data: &mut Value, redis: &RedisService, user_key: &str) {
    let data_key = format!("{}:{}", user_key, DictKeys::DATA);
    data["update_time"] = DateUtils::datetime().into();
    match redis.set_json(&data_key, &data, None).await {
        Ok(_) => {
            info!("用户:{} 刷新 carelinkData 数据成功!!!", data_key);
        }
        Err(e) => {
            error!("用户:{} 刷新 carelinkData 数据失败!!!:{}", data_key, e);
        }
    }
}

const YESTERDAY_KEY: &str = "yesterday";

pub async fn update_carelink_my_data_yesterday_data(
    org_yesterday_data: &mut Value,
    redis: &RedisService,
    user_key: String,
) {
    let my_data_key = format!("{}:{}", user_key, DictKeys::MY_DATA);
    if let Ok(Some(mut org_my_data)) = redis.get_json::<Value>(&my_data_key).await {
        let sgs_data = org_yesterday_data["sgs"].as_array_mut().unwrap();
        let yes_sgs_arr = org_my_data[YESTERDAY_KEY]["sgs"].as_array_mut().unwrap();
        let yes_sgs_arr_len = yes_sgs_arr.len();
        deal_yes_data(yes_sgs_arr, sgs_data);

        //这里一定要先处理完上面的sgs_data的借用,才能再次处理marker的值
        let markers_data = org_yesterday_data["markers"].as_array_mut().unwrap();
        let yes_markers_arr = org_my_data[YESTERDAY_KEY]["markers"]
            .as_array_mut()
            .unwrap();
        let yes_markers_arr_len = yes_markers_arr.len();
        deal_yes_data(yes_markers_arr, markers_data);

        org_my_data["update_time"] = DateUtils::datetime().into();
        match redis.set_json(&my_data_key, &org_my_data, None).await {
            Ok(_) => {
                info!(
                    "用户:{} 刷新 carelinkYesterdayData 数据成功!!!",
                    my_data_key
                );
                info!(
                    "用户:{} 更新carelinkYesterdayData数据,sgsArr:{} markersArr:{}",
                    user_key, yes_sgs_arr_len, yes_markers_arr_len
                );
            }
            Err(e) => {
                error!(
                    "用户:{} 刷新 carelinkYesterdayData 数据失败!!!:{}",
                    my_data_key, e
                );
            }
        }
    } else {
        error!(
            "Failed to get carelink my_data when carelink_refresh_history: {:?}",
            my_data_key
        );
    }
}

fn deal_yes_data(yes_arr: &mut Vec<Value>, yes_data: &mut Vec<Value>) {
    let len = yes_arr.len();
    if len < 2 {
        yes_arr.append(yes_data);
    } else if len == 2 {
        // 如果 yes_arr 长度等于2
        // 1. 将 [1] 移动到 [0]
        yes_arr[0] = std::mem::replace(&mut yes_arr[1], Value::Array(yes_data.clone()));
        // yes_arr[1] = Value::Array(yes_data.clone());
    }
}

pub async fn save_history_data(
    org_yesterday_data: &mut Value,
    redis: &RedisService,
    user_key: String,
) {
    let mut sum_recommended: f64 = 0.0;
    let mut sum_auto_correction: f64 = 0.0;
    let mut sum_basal: f64 = 0.0;
    for item in org_yesterday_data["markers"].as_array().unwrap().iter() {
        let item_type = item["type"].as_str().unwrap();

        if item_type == "INSULIN" {
            let item_activation_type = item["activationType"].as_str().unwrap();
            if item_activation_type == "RECOMMENDED" {
                sum_recommended += item.get_f64("deliveredFastAmount");
            } else if item_activation_type == "AUTOCORRECTION" {
                sum_auto_correction += item.get_f64("deliveredFastAmount");
            }
        } else if item_type == "AUTO_BASAL_DELIVERY" {
            sum_basal += item.get_f64("bolusAmount");
        }
    }

    let history_data = json!({
        "averageSG": org_yesterday_data["averageSG"].as_f64().unwrap(),
        "belowHypoLimit": org_yesterday_data["belowHypoLimit"].as_f64().unwrap(),
        "aboveHyperLimit": org_yesterday_data["aboveHyperLimit"].as_f64().unwrap(),
        "timeInRange": org_yesterday_data["timeInRange"].as_f64().unwrap(),
        "averageSGFloat": org_yesterday_data["averageSGFloat"].as_f64().unwrap(),
        "insulin": {
            "recommended": sum_recommended,
            "autoCorrection": sum_auto_correction,
            "basal": sum_basal
        }
    });
    let history_key = format!("{}:{}", user_key, DictKeys::HISTORY);
    match redis
        .hset_json(
            &history_key,
            (Local::now() - Duration::days(1))
                .format(DateUtils::DATE)
                .to_string()
                .as_str(),
            &history_data,
        )
        .await
    {
        Ok(Some(data)) => {
            info!("用户:{} 历史数据保存成功， 数据:{}", user_key, data);
        }
        Err(e) => {
            error!("用户:{} 历史数据保存失败:{}", user_key, e);
        }
        Ok(None) => {
            error!("用户:{} 历史数据保存失败,无保存数据", user_key);
        }
    }
}
#[allow(unused_variables)]
pub async fn update_statistics(org_data: &mut Value, redis: &RedisService, user_key: String) {
    match redis
        .hget_all(format!("{}:{}", user_key, DictKeys::HISTORY).as_str())
        .await
    {
        Ok(Some(data)) => {
            if let Some(obj) = data.as_object() {
                let mut history_arr: Vec<(String, Value)> =
                    obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                history_arr.sort_by(|a, b| b.0.cmp(&a.0));
                let mut sum_avg: f64 = 0.0;
                // let mut sum_below: f64 = 0.0;
                // let mut sum_above: f64 = 0.0;
                let mut day_30 = get_sum_obj();
                let mut day_90 = get_sum_obj();
                for (i, item) in history_arr.iter().enumerate() {
                    let history_data = parse_json(item.1.as_str().unwrap());
                    if i < 30 {
                        calc_day_obj_data(&mut day_30, &history_data);
                    }
                    if i < 90 {
                        calc_day_obj_data(&mut day_90, &history_data);
                    }
                    sum_avg += history_data.get_f64("averageSGFloat");
                    // sum_below += history_data.get_f64("belowHypoLimit");
                    // sum_above += history_data.get_f64("aboveHyperLimit");
                }
                let total_avg_sg = sum_avg / history_arr.len() as f64;

                org_data["GMI"] = fix2(3.31 + 0.02392 * total_avg_sg);
                if let Some(statistics) = org_data.get_mut("statistics") {
                    let day_30_sum = &day_30["sum"];
                    let day_30_count = day_30.get_f64("count");
                    let day_30_insulin_count = day_30.get_f64("insulinCount");
                    let day_90_sum = &day_90["sum"];
                    let day_90_count = day_90.get_f64("count");
                    let day_90_insulin_count = day_90.get_f64("insulinCount");

                    statistics["day30"] = json!({
                        "avg": fix2(day_30_sum.get_f64("avg")  / day_30_count),
                        "below":fix2(day_30_sum.get_f64("below")  / day_30_count),
                        "above":fix2(day_30_sum.get_f64("above")  / day_30_count),
                        "recommended":fix2(day_30_sum.get_f64("recommended")  / day_30_insulin_count),
                        "autoCorrection":fix2(day_30_sum.get_f64("autoCorrection")  / day_30_insulin_count),
                        "basal":fix2(day_30_sum.get_f64("basal")  / day_30_insulin_count),
                    });

                    statistics["day90"] = json!({
                        "avg": fix2(day_90_sum.get_f64("avg")  / day_90_count),
                        "below":fix2(day_90_sum.get_f64("below")  / day_90_count),
                        "above":fix2(day_90_sum.get_f64("above")  / day_90_count),
                        "recommended":fix2(day_90_sum.get_f64("recommended")  / day_90_insulin_count),
                        "autoCorrection":fix2(day_90_sum.get_f64("autoCorrection")  / day_90_insulin_count),
                        "basal":fix2(day_90_sum.get_f64("basal")  / day_90_insulin_count),
                    });
                }

                update_carelink_data_to_redis(org_data, redis, &user_key).await;
            }
        }
        Err(e) => {
            error!("load history data failed:{}", e);
        }
        _ => {
            error!("load history data failed");
        }
    }
}

pub async fn update_luck_data(org_sugar_data: &mut Value, redis: &RedisService, user_key: String) {
    let luck_key = format!("{}:{}", user_key, DictKeys::LUCK);
    match redis.get_json::<Value>(&luck_key).await {
        Ok(Some(mut data)) => {
            let tie = org_sugar_data.get_i64("timeInRange");
            let avg_sg = org_sugar_data.get_f64("averageSG");

            // 5. 过滤和提取血糖值数组
            let glucose_values = extract_glucose_values(org_sugar_data);
            if glucose_values.is_empty() {
                error!("用户:{} 没有有效的血糖数据", user_key);
            }
            // 6. 计算 CV（变异系数）
            let cv = calculate_cv(&glucose_values, avg_sg);
            // 7. 更新 luck 数据
            if tie >= LUCK_LIMIT as i64 && cv <= MIN_LUCK_CV as f64 {
                data["yes"] = Value::from(data.get_i64("yes") + 1);
            } else {
                data["no"] = Value::from(data.get_i64("no") + 1);
            }

            // 8. 更新更新时间
            data["update_time"] = Value::from(DateUtils::datetime());

            // 9. 保存回 Redis
            match redis.set_json(&luck_key, &data, None).await {
                Ok(_) => {
                    // 10. 记录日志
                    info!(
                        "用户:{} tir:{:.1} cv:{:.2} 保存luck数据成功",
                        user_key, tie, cv
                    );
                }
                Err(_) => {
                    error!(
                        "用户:{} tir:{:.1} cv:{:.2} 保存luck数据失败",
                        user_key, tie, cv
                    )
                }
            }
        }
        Err(e) => {
            error!("get user:{} luck data failed:{}", user_key, e);
        }
        _ => {
            error!("get user:{} luck data failed", user_key);
        }
    }
}

/// 提取有效的血糖值数组
fn extract_glucose_values(sg_data: &Value) -> Vec<f64> {
    let sgs_array = match sg_data.get("sgs") {
        Some(Value::Array(arr)) => arr,
        _ => return Vec::new(),
    };

    sgs_array
        .iter()
        .filter_map(|item| {
            // 检查条件
            let kind = item.get_string("kind");
            let sensor_state = item.get_string("sensorState");
            let sg_value = item.get_i64("sg");

            // 条件1: kind == "SG" AND sensor_state == "NO_ERROR_MESSAGE" AND sg != 0
            let condition1 = kind == "SG" && sensor_state == "NO_ERROR_MESSAGE" && sg_value != 0;

            // 条件2: sensor_state == "SG_BELOW_40_MGDL" (这种状态下可能没有sg字段)
            let condition2 = sensor_state == "SG_BELOW_40_MGDL";

            if condition1 || condition2 {
                // 对于 SG_BELOW_40_MGDL，如果没有sg字段，可以返回一个特殊值如 39.0
                if condition2 && sg_value == 0 {
                    Some(39.0) // 假设低于40的值
                } else {
                    Some(sg_value as f64)
                }
            } else {
                None
            }
        })
        .collect()
}

/// 计算变异系数 (CV = (标准差 / 平均值) * 100)
fn calculate_cv(values: &[f64], average_sg: f64) -> f64 {
    if values.len() < 2 || average_sg == 0.0 {
        return 0.0;
    }

    // 计算标准差
    let std_dev = calculate_std_dev(values, average_sg);

    // 计算 CV
    (std_dev / average_sg) * 100.0
}

/// 计算标准差
fn calculate_std_dev(values: &[f64], mean: f64) -> f64 {
    let variance: f64 = values
        .iter()
        .map(|&x| {
            let diff = x - mean;
            diff * diff
        })
        .sum::<f64>()
        / values.len() as f64;

    variance.sqrt()
}

#[allow(dead_code)]
/// 或者使用更完整的统计计算
fn calculate_statistics(values: &[f64]) -> (f64, f64, f64) {
    let count = values.len();
    if count == 0 {
        return (0.0, 0.0, 0.0);
    }

    let sum: f64 = values.iter().sum();
    let mean = sum / count as f64;

    let variance = if count > 1 {
        values
            .iter()
            .map(|&x| {
                let diff = x - mean;
                diff * diff
            })
            .sum::<f64>()
            / (count - 1) as f64 // 样本标准差
    } else {
        0.0
    };

    let std_dev = variance.sqrt();
    let cv = if mean != 0.0 {
        (std_dev / mean) * 100.0
    } else {
        0.0
    };

    (mean, std_dev, cv)
}

fn fix2(v: f64) -> Value {
    Value::from(format!("{:.2}", v))
}
fn get_sum_obj() -> Value {
    json!({
        "count": 0,
        "insulinCount": 0,
        "sum":{
            "avg": 0,
            "below": 0,
            "above": 0,
            "recommended": 0,
            "autoCorrection": 0,
            "basal": 0,
        }
    })
}

fn calc_day_obj_data(day_obj: &mut Value, item: &Value) {
    // 更新 count
    increment_field(day_obj, "count", 1);
    let sum_obj = day_obj["sum"].as_object_mut().unwrap();

    // 更新 sum 字段
    update_field(sum_obj, "avg", item, "timeInRange");
    update_field(sum_obj, "below", item, "belowHypoLimit");
    update_field(sum_obj, "above", item, "aboveHyperLimit");
    if let Some(insulin) = item.get("insulin") {
        update_field(sum_obj, "recommended", insulin, "recommended");
        update_field(sum_obj, "autoCorrection", insulin, "autoCorrection");
        update_field(sum_obj, "basal", insulin, "basal");

        // 更新 insulinCount
        increment_field(day_obj, "insulinCount", 1);
    }
}

// 辅助函数：安全地更新浮点数字段
fn update_field(
    sum_obj: &mut serde_json::Map<String, Value>,
    field: &str,
    item: &Value,
    item_field: &str,
) {
    if let (Some(current), Some(add_value)) = (
        sum_obj.get(field).and_then(|v| v.as_f64()),
        item.get(item_field).and_then(|v| v.as_f64()),
    ) {
        sum_obj.insert(field.to_string(), Value::from(current + add_value));
    }
}

/// 递增数值字段
fn increment_field(day_obj: &mut Value, field: &str, increment: u64) {
    if let Some(current) = day_obj.get(field).and_then(|v| v.as_u64()) {
        day_obj[field] = Value::from(current + increment);
    } else {
        day_obj[field] = Value::from(increment);
    }
}

pub fn send_mail(email_service: Arc<EmailService>, text: String) {
    tokio::spawn(async move {
        match email_service
            .send_text_email(
                email_service.to.as_str(),
                "carelink_follower_web警报",
                text.as_str(),
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                error!("send mail error:{:#?}", e);
            }
        };
    });
}

pub async fn carelink_login(
    state: &AppState,
    user_setting: &UserSetting,
) -> Result<bool, LoginError> {
    let carelink_login = CarelinkLogin::new(
        state.http_client.clone(),
        user_setting.username.clone(),
        user_setting.carelink_password.clone(),
    );

    match carelink_login.login().await {
        Ok(token) => {
            info!("Carelink auto login successful: {}", token);
            let user_key = user_setting.user_key.as_str();
            let auth_key = format!("{}:{}", user_key, DictKeys::AUTH);
            match state.redis.get_json::<Value>(&auth_key).await.get_json() {
                Ok(mut auth_data) => {
                    auth_data["status"] = Value::from(200);
                    auth_data["token"] = Value::from(token.to_string());
                    update_carelink_auth_data_to_redis(&mut auth_data, &state.redis, user_key)
                        .await;
                    info!("Carelink token refresh successful");
                    send_mail(
                        state.email.clone(),
                        format!("Carelink token refresh successful, token is:{}", token),
                    );
                    Ok(true)
                }
                Err(e) => {
                    Err(LoginError::LoginFailed(format!("Carelink get user auth data error: {:?}", e)))
                }
            }
        }
        Err(e) => {
            let msg = format!("Carelink login error: {:?}", e);
            error!(msg);
            send_mail(state.email.clone(), msg.clone());
            Err(LoginError::LoginFailed(msg))
        }
    }
}

// 错误类型
#[derive(Debug)]
#[allow(dead_code)]
pub enum LoginError {
    HttpError(reqwest::Error),
    UrlParseError(url::ParseError),
    MissingLocationHeader,
    AuthTokenNotFound,
    LoginFailed(String),
}

impl From<reqwest::Error> for LoginError {
    fn from(err: reqwest::Error) -> Self {
        LoginError::HttpError(err)
    }
}

impl From<url::ParseError> for LoginError {
    fn from(err: url::ParseError) -> Self {
        LoginError::UrlParseError(err)
    }
}

struct CarelinkLogin {
    client: Arc<Client>,
    username: String,
    password: String,
}

impl CarelinkLogin {
    fn new(client: Arc<Client>, username: String, password: String) -> CarelinkLogin {
        Self {
            client,
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    // 主登录方法
    pub async fn login(&self) -> Result<String, LoginError> {
        info!("开始登录流程...");

        // 步骤1：获取初始重定向地址
        let auth_url = self.get_initial_redirect().await?;

        // 步骤2：获取登录页面地址
        let login_page_url = self.get_login_page(&auth_url).await?;

        // 步骤3：访问登录页面，获取state参数
        let state = self.visit_login_page(&login_page_url).await?;

        // 步骤4：提交登录表单
        let resume_url = self.submit_login(&login_page_url, &state).await?;

        // 步骤5：访问授权继续页面
        let final_url = self.resume_authorization(&resume_url).await?;

        // 步骤6：获取最终的auth token
        let auth_token = self.get_auth_token(&final_url).await?;

        info!("登录成功！");
        Ok(auth_token)
    }

    fn show_status(&self, response: &Response) {
        info!("response status:{}", response.status());
    }

    async fn get_initial_redirect(&self) -> Result<String, LoginError> {
        let url = format!(
            "{}{}",
            CARELINK_BASE_URL, "/patient/sso/login?country=hk&lang=zh"
        );
        info!("步骤1：访问 {}", url);
        let response = self.client.get(url).send().await?;
        // 从Location头部获取重定向地址
        let location = response
            .headers()
            .get("location")
            .ok_or(LoginError::MissingLocationHeader)?
            .to_str()
            .map_err(|_| LoginError::LoginFailed("无效的Location头部".to_string()))?;
        self.show_status(&response);
        info!("步骤1完成：重定向到 {}", location);
        Ok(location.to_string())
    }

    // 步骤2：获取登录页面地址
    async fn get_login_page(&self, auth_url: &str) -> Result<String, LoginError> {
        info!("步骤2：访问 {}", auth_url);

        let response = self.client.get(auth_url).send().await?;

        // 从Location头部获取重定向地址
        let location = response
            .headers()
            .get("location")
            .ok_or(LoginError::MissingLocationHeader)?
            .to_str()
            .map_err(|_| LoginError::LoginFailed("无效的Location头部".to_string()))?;

        // 构建完整的URL
        let full_url = if location.starts_with("http") {
            location.to_string()
        } else {
            let base = Url::parse(CARELINK_LOGIN_URL)?;
            let url = base.join(location)?;
            url.to_string()
        };
        self.show_status(&response);
        info!("步骤2完成：登录页面 {}", full_url);
        Ok(full_url)
    }

    // 步骤3：访问登录页面（获取必要参数）
    async fn visit_login_page(&self, login_url: &str) -> Result<String, LoginError> {
        info!("步骤3：访问登录页面 {}", login_url);

        let response = self.client.get(login_url).send().await?;

        // 检查状态码
        if !response.status().is_success() {
            return Err(LoginError::LoginFailed(format!(
                "登录页面访问失败: {}",
                response.status()
            )));
        }

        // let _ = &response.text().await?;
        // 从URL中提取state参数
        let url = Url::parse(login_url)?;
        let pairs: Vec<(String, String)> = url.query_pairs().into_owned().collect();

        let state = pairs
            .iter()
            .find(|(key, _)| key == "state")
            .map(|(_, value)| value.clone())
            .ok_or(LoginError::LoginFailed("未找到state参数".to_string()))?;
        self.show_status(&response);
        info!("步骤3完成：获取到state参数");
        Ok(state)
    }

    // 步骤4：提交登录表单
    async fn submit_login(&self, login_url: &str, state: &str) -> Result<String, LoginError> {
        info!("步骤4：提交登录表单到 {}", login_url);

        let mut form_data = HashMap::new();
        form_data.insert("state", state);
        form_data.insert("username", self.username.as_str());
        form_data.insert("password", self.password.as_str());
        form_data.insert("action", "default");

        let response = self
            .client
            .post(login_url)
            .form(&form_data)
            // .header("Content-Type", "application/x-www-form-urlencoded")
            // .header("User-Agent", UA)
            .send()
            .await?;

        let status = response.status();
        // 检查状态码是否为302
        if status != 302 {
            // 可能是登录失败，检查响应内容
            let body = response.text().await?;
            if body.contains("Invalid username or password") || body.contains("登录失败") {
                return Err(LoginError::LoginFailed("用户名或密码错误".to_string()));
            }
            return Err(LoginError::LoginFailed(format!(
                "登录失败，状态码: {}",
                status
            )));
        }
        self.show_status(&response);
        // 从Location头部获取重定向地址
        let location = response
            .headers()
            .get("location")
            .ok_or(LoginError::MissingLocationHeader)?
            .to_str()
            .map_err(|_| LoginError::LoginFailed("无效的Location头部".to_string()))?;

        // 构建完整的URL
        let full_url = if location.starts_with("http") {
            location.to_string()
        } else {
            let base = Url::parse(CARELINK_LOGIN_URL)?;
            let url = base.join(location)?;
            url.to_string()
        };

        info!("步骤4完成：重定向到 {}", full_url);
        Ok(full_url)
    }

    // 步骤5：访问授权继续页面
    async fn resume_authorization(&self, resume_url: &str) -> Result<String, LoginError> {
        info!("步骤5：访问 {}", resume_url);

        let response = self.client.get(resume_url).send().await?;

        // 检查状态码是否为302
        if response.status() != 302 {
            return Err(LoginError::LoginFailed(format!(
                "期望状态码302，得到{}",
                response.status()
            )));
        }

        // 从Location头部获取重定向地址
        let location = response
            .headers()
            .get("location")
            .ok_or(LoginError::MissingLocationHeader)?
            .to_str()
            .map_err(|_| LoginError::LoginFailed("无效的Location头部".to_string()))?;
        self.show_status(&response);
        info!("步骤5完成：重定向到 {}", location);
        Ok(location.to_string())
    }

    // 步骤6：获取最终的auth token
    async fn get_auth_token(&self, final_url: &str) -> Result<String, LoginError> {
        info!("步骤6：访问 {}", final_url);

        let response = self.client.get(final_url).send().await?;

        // 检查状态码是否为307
        if response.status() != 307 {
            return Err(LoginError::LoginFailed(format!(
                "期望状态码307，得到{}",
                response.status()
            )));
        }
        self.show_status(&response);
        // 从Set-Cookie头部提取auth_tmp_token
        let cookies = response
            .headers()
            .get_all("set-cookie")
            .iter()
            .filter_map(|h| h.to_str().ok())
            .collect::<Vec<_>>();

        // 查找包含auth_tmp_token的cookie
        let token_regex = Regex::new(r"auth_tmp_token=([^;]+)").unwrap();

        for cookie in cookies {
            if let Some(captures) = token_regex.captures(cookie) {
                if let Some(token) = captures.get(1) {
                    let auth_token = token.as_str();
                    info!("步骤6完成：成功获取auth_tmp_token");
                    return Ok(auth_token.to_string());
                }
            }
        }
        Err(LoginError::AuthTokenNotFound)
    }
}
