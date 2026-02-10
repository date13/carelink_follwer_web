use crate::models::AppState;
use crate::services::sugar_service::DictKeys;
use crate::utils::{DateUtils, JsonHelp};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::sync::OnceLock;
use tracing::{error, info};
use uuid::Uuid;

const MAX_ENTRIES: usize = 288;
const HIGH_LIMIT: f32 = 180.0;
const LOW_LIMIT: f32 = 70.2;

const WARN_EMERG_INTERVAL: u8 = 15;
const WARN_EARLY_INTERVAL: u8 = 30;
const WARN_EARLY_HIGH: f32 = 160.2;
const WARN_EARLY_LOW: f32 = 90.0;

const MAX_NOTIFICATION_COUNT: usize = 30;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationObj {
    dateTime: String,
    instanceId: String,
    messageId: String,
    triggeredDateTime: String,
    sg: i64,
    source: String,
}

#[allow(non_snake_case)]
impl NotificationObj {
    pub fn new(messageId: String, source: String, alert_time: String) -> Self {
        NotificationObj::new_for_sg(messageId, source, 0, alert_time)
    }
    pub fn new_for_sg(
        messageId: String,
        source: String,
        sg: i64,
        alert_time: String,
    ) -> NotificationObj {
        Self {
            dateTime: alert_time.clone(),
            instanceId: Uuid::new_v4().simple().to_string(),
            messageId,
            sg,
            triggeredDateTime: alert_time,
            source,
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
enum NOTIFICATION {
    BC_SID_SG_APPROACH_HIGH_LIMIT_CHECK_BG,
    BC_SID_HIGH_SG_CHECK_BG,
    BC_SID_SG_APPROACH_LOW_LIMIT_CHECK_BG,
    BC_SID_LOW_SD_CHECK_BG,
    BC_SID_SG_RISE_RAPID,
}

impl NOTIFICATION {
    pub fn get_str(&self) -> String {
        format!("{:?}", self)
    }
}
#[derive(Debug, Clone)]
pub enum NSSource {
    Ottai,
}

impl NSSource {
    pub fn get_source(&self) -> String {
        format!("{:?}", self)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NSTrend {
    Flat,
    FortyFiveUp,
    FortyFiveDown,
    SingleUp,
    SingleDown,
    DoubleUp,
    DoubleDown,
    TripleUp,
    TripleDown,
    None,
}

impl NSTrend {
    pub fn get_str(&self) -> String {
        format!("{:?}", self)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarelinkTrend {
    FLAT,
    FORTY_FIVE_UP,
    FORTY_FIVE_DOWN,
    UP,
    DOWN,
    UP_DOUBLE,
    DOWN_DOUBLE,
    UP_TRIPLE,
    DOWN_TRIPLE,
    NONE,
}

pub struct TrendConverter {
    mapping: HashMap<NSTrend, CarelinkTrend>,
}

impl TrendConverter {
    pub fn new() -> Self {
        let mut mapping = HashMap::new();

        mapping.insert(NSTrend::Flat, CarelinkTrend::FLAT);
        mapping.insert(NSTrend::FortyFiveUp, CarelinkTrend::FORTY_FIVE_UP);
        mapping.insert(NSTrend::FortyFiveDown, CarelinkTrend::FORTY_FIVE_DOWN);
        mapping.insert(NSTrend::SingleUp, CarelinkTrend::UP);
        mapping.insert(NSTrend::SingleDown, CarelinkTrend::DOWN);
        mapping.insert(NSTrend::DoubleUp, CarelinkTrend::UP_DOUBLE);
        mapping.insert(NSTrend::DoubleDown, CarelinkTrend::DOWN_DOUBLE);
        mapping.insert(NSTrend::TripleUp, CarelinkTrend::UP_TRIPLE);
        mapping.insert(NSTrend::TripleDown, CarelinkTrend::DOWN_TRIPLE);
        mapping.insert(NSTrend::None, CarelinkTrend::NONE);

        TrendConverter { mapping }
    }

    pub fn convert(&self, ns_trend: &NSTrend) -> Option<&CarelinkTrend> {
        self.mapping.get(ns_trend)
    }

    pub fn convert_str(&self, ns_trend: &str) -> Option<&CarelinkTrend> {
        let ns_enum = match ns_trend {
            "Flat" => NSTrend::Flat,
            "FortyFiveUp" => NSTrend::FortyFiveUp,
            "FortyFiveDown" => NSTrend::FortyFiveDown,
            "SingleUp" => NSTrend::SingleUp,
            "SingleDown" => NSTrend::SingleDown,
            "DoubleUp" => NSTrend::DoubleUp,
            "DoubleDown" => NSTrend::DoubleDown,
            "TripleUp" => NSTrend::TripleUp,
            "TripleDown" => NSTrend::TripleDown,
            "None" => NSTrend::None,
            _ => return None,
        };

        self.convert(&ns_enum)
    }

    // 获取全局实例
    pub fn global() -> &'static TrendConverter {
        static INSTANCE: OnceLock<TrendConverter> = OnceLock::new();
        INSTANCE.get_or_init(|| TrendConverter::new())
    }
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct SgObj {
    sg: i64,
    datetime: String,
    timeChange: bool,
    sensorState: String,
    kind: String,
    version: i8,
    direction: String,
    source: String,
}

impl SgObj {
    fn new(sg: i64, date: String, direction: String, source: String) -> Self {
        Self {
            sg,
            datetime: date,
            timeChange: false,
            sensorState: "NO_ERROR_MESSAGE".to_string(),
            kind: "SG".to_string(),
            version: 1,
            direction,
            source,
        }
    }
}

pub async fn ns_receive_data(
    state: &AppState,
    key: &str,
    data: &mut Vec<Value>,
    user_key: &str,
) -> Option<Vec<Value>> {
    let ns_key = format!("{}:{}", user_key, DictKeys::NS);
    if let Ok(Some(mut ng_data)) = state.redis.get_json::<Value>(ns_key.as_str()).await {
        ng_data["update_time"] = DateUtils::datetime().into();
        // 获取当前数组（如果不存在则创建）
        if !ng_data[key].is_array() {
            ng_data[key] = Value::Array(Vec::new());
        }
        let current_array = ng_data[key].as_array_mut().unwrap();
        for item in data {
            current_array.push(item.clone());
        }
        // println!("{},{:?}", current_array.len(), current_array);
        // ✅ 正确逻辑：检查 current_array 的长度，而不是 data 的长度
        // 当 current_array 超过最大限制时，移除最旧的元素
        while current_array.len() > MAX_ENTRIES {
            // println!("remove");
            current_array.remove(0); // 这会直接修改 ng_data[key]
        }
        current_array.sort_by(|a, b| a.get_i64("date").cmp(&b.get_i64("date")));
        let clone = current_array.clone();
        match state.redis.set_json(&ns_key, &ng_data, None).await {
            Ok(_) => {
                info!("用户:{} 刷新 ns {}数据成功!!!", user_key, key);
            }
            Err(e) => {
                error!("用户:{} 刷新 ns {}数据失败!!!:{}", user_key, key, e);
            }
        }
        return Some(clone);
    }
    None
}

/// 方法1：将 UTC 时间字符串转换为东八区时间字符串
pub fn utc_to_east8_string(utc_str: &str) -> Result<String, chrono::ParseError> {
    // 1. 解析 UTC 时间字符串
    let utc_dt: DateTime<Utc> = DateTime::parse_from_rfc3339(utc_str)?.with_timezone(&Utc);

    // 2. 创建东八区时区 (+08:00)
    let east8_offset = FixedOffset::east_opt(8 * 3600).unwrap(); // +08:00

    // 3. 转换到东八区
    let east8_dt = utc_dt.with_timezone(&east8_offset);

    let naive_east8 = east8_dt.naive_local();
    let east8_as_utc = Utc.from_utc_datetime(&naive_east8);

    // 4. 格式化为指定格式
    Ok(east8_as_utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
}

pub async fn ns_refresh_entries_data(state: &AppState, entries: &mut Vec<Value>, user_key: &str) {
    // let mut user_setting = state.get_user_settings(user_key).await;
    let data_key = format!("{}:{}", user_key, DictKeys::DATA);
    //保存数据先
    if let Some(origin_entries) = ns_receive_data(state, "entries", entries, &user_key).await {
        let user_setting = state.get_user_settings(&user_key).await;
        if !user_setting.ns {
            return;
        }
        if let Ok(Some(mut org_data)) = state.redis.get_json::<Value>(data_key.as_str()).await {
            let current_data = org_data["data"].as_object_mut().unwrap();
            // 处理血糖数据
            deal_sgs_data(current_data, &origin_entries);
            // 处理警告信息
            deal_notification_data(current_data);
            org_data["update_time"] = DateUtils::datetime().into();
            match state.redis.set_json(&data_key, &org_data, None).await {
                Ok(_) => {
                    info!("用户:{} 保存 ns 数据到carelink成功!!!", user_key);
                }
                Err(e) => {
                    error!("用户:{} 保存 ns 数据到 carelink失败!!!:{}", user_key, e);
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn deal_sgs_data(current_data: &mut Map<String, Value>, origin_entries: &Vec<Value>) {
    let last_sgs = &origin_entries[origin_entries.len() - 1];
    current_data["lastSG"] = json!(SgObj::new(
        last_sgs.get_i64("sgv"),
        utc_to_east8_string(last_sgs.get_string("dateString").as_str()).unwrap(),
        last_sgs.get_string("direction"),
        last_sgs.get_string("device"),
    ));

    let sgs_array = current_data["sgs"].as_array_mut().unwrap();
    let last_org_sg = sgs_array[sgs_array.len() - 1].get_i64("sg");
    sgs_array.clear();

    let mut sum_sg: f32 = 0.0;
    let mut hypo_count = 0;
    let mut hyper_count = 0;

    for entry in origin_entries.iter() {
        let sg = entry.get_i64("sgv") as f32;
        sum_sg += sg;
        // 统计分类
        if sg <= LOW_LIMIT {
            hypo_count += 1;
        } else if sg >= HIGH_LIMIT {
            hyper_count += 1;
        }
        sgs_array.push(json!(SgObj::new(
            entry.get_i64("sgv"),
            utc_to_east8_string(entry.get_string("dateString").as_str()).unwrap(),
            entry.get_string("direction"),
            entry.get_string("device"),
        )))
    }
    // 数字差距 1 不显示箭头
    if (last_sgs.get_i64("sgv") - last_org_sg).abs() <= 1 {
        current_data["lastSGTrend"] = Value::String(format!("{:?}", CarelinkTrend::NONE));
    } else {
        let converter = TrendConverter::global();
        if let Some(carelink) = converter.convert_str(last_sgs["direction"].as_str().unwrap()) {
            current_data["lastSGTrend"] = Value::String(format!("{:?}", carelink));
        }
    }

    let total_count = origin_entries.len() as i32;
    let (averageSGFloat, averageSG, belowHypoLimit, aboveHyperLimit, timeInRange) =
        calculate(sum_sg, hypo_count, hyper_count, total_count);
    current_data["averageSGFloat"] = Value::from(averageSGFloat);
    current_data["averageSG"] = Value::from(averageSG);
    current_data["belowHypoLimit"] = Value::from(belowHypoLimit);
    current_data["aboveHyperLimit"] = Value::from(aboveHyperLimit);
    current_data["timeInRange"] = Value::from(timeInRange);
}

fn deal_notification_data(current_data: &mut Map<String, Value>) {
    //取出处理完的sgs数据
    let cur_sgs_data = current_data["sgs"].as_array().unwrap().clone();
    let last_sgs = cur_sgs_data[cur_sgs_data.len() - 1].clone();

    let source = last_sgs.get_string("source");
    //取出警告数据
    let notification_data = current_data["notificationHistory"]["clearedNotifications"]
        .as_array_mut()
        .unwrap();

    let last_alter_org_time = last_sgs.get_string("datetime");
    let last_alter_notification_time = last_alter_org_time.replace("Z", "-00:00");
    // 快速上升直接报警,不用检查间隔
    if last_sgs.get_string("direction") == NSTrend::TripleUp.get_str() {
        notification_data.push(json!(NotificationObj::new(
            NOTIFICATION::BC_SID_SG_RISE_RAPID.get_str(),
            source.clone(),
            last_alter_notification_time
        )));
    } else {
        let last_sg = last_sgs.get_i64("sg") as f32;
        let last_alert_time =
            DateTime::parse_from_rfc3339(last_alter_org_time.replace("Z", "+08:00").as_str())
                .unwrap();
        // 高于接近值
        if last_sg >= WARN_EARLY_HIGH {
            //高于限定值,并且间隔超过了15分钟报警一次
            if last_sg >= HIGH_LIMIT
                && !has_recent_notification(
                notification_data,
                NOTIFICATION::BC_SID_HIGH_SG_CHECK_BG.get_str().as_str(),
                WARN_EMERG_INTERVAL,
                &last_alert_time,
            )
            {
                notification_data.push(json!(NotificationObj::new_for_sg(
                    NOTIFICATION::BC_SID_HIGH_SG_CHECK_BG.get_str(),
                    source.clone(),
                    last_sg as i64,
                    last_alter_notification_time
                )));
            } else {
                // 高于接近值并且间隔超过了30分钟报警一次
                if !has_recent_notification(
                    notification_data,
                    NOTIFICATION::BC_SID_SG_APPROACH_HIGH_LIMIT_CHECK_BG
                        .get_str()
                        .as_str(),
                    WARN_EARLY_INTERVAL,
                    &last_alert_time,
                ) {
                    notification_data.push(json!(NotificationObj::new(
                        NOTIFICATION::BC_SID_SG_APPROACH_HIGH_LIMIT_CHECK_BG.get_str(),
                        source.clone(),
                        last_alter_notification_time
                    )));
                }
            }
            // 低于接近值
        } else if last_sg <= WARN_EARLY_LOW {
            //低于于限定值,并且间隔超过了15分钟报警一次
            if last_sg <= LOW_LIMIT
                && !has_recent_notification(
                notification_data,
                NOTIFICATION::BC_SID_LOW_SD_CHECK_BG.get_str().as_str(),
                WARN_EMERG_INTERVAL,
                &last_alert_time,
            )
            {
                notification_data.push(json!(NotificationObj::new_for_sg(
                    NOTIFICATION::BC_SID_LOW_SD_CHECK_BG.get_str(),
                    source.clone(),
                    last_sg as i64,
                    last_alter_notification_time
                )));
            } else {
                // 低于接近值并且间隔超过了30分钟报警一次
                if !has_recent_notification(
                    notification_data,
                    NOTIFICATION::BC_SID_SG_APPROACH_LOW_LIMIT_CHECK_BG
                        .get_str()
                        .as_str(),
                    WARN_EARLY_INTERVAL,
                    &last_alert_time,
                ) {
                    notification_data.push(json!(NotificationObj::new(
                        NOTIFICATION::BC_SID_SG_APPROACH_LOW_LIMIT_CHECK_BG.get_str(),
                        source.clone(),
                        last_alter_notification_time
                    )));
                }
            }
        }
    }
    // 超过数量去掉多余的警告
    while notification_data.len() > MAX_NOTIFICATION_COUNT {
        notification_data.remove(0);
    }
}

fn has_recent_notification(
    notifications: &[Value],
    message_id: &str,
    minutes: u8,
    last_alert_time: &DateTime<FixedOffset>,
) -> bool {
    notifications.iter().rev().any(|notification| {
        // 检查消息类型
        let msg_id_match = notification
            .get("messageId")
            .and_then(|v| v.as_str())
            .map(|msg| msg == message_id)
            .unwrap_or(false);

        if !msg_id_match {
            return false;
        }

        // 检查时间
        notification
            .get("triggeredDateTime")
            .and_then(|v| v.as_str())
            .and_then(|time_str| {
                DateTime::parse_from_rfc3339(time_str.replace("-00:00", "+08:00").as_str()).ok()
            })
            .map(|triggered_time| {
                let duration = last_alert_time.signed_duration_since(triggered_time);
                info!("duration: {:?}", duration.num_minutes());
                duration.num_minutes() <= minutes as i64
            })
            .unwrap_or(false)
    })
}

fn calculate(
    sum_sg: f32,
    hypo_count: i32,
    hyper_count: i32,
    total_count: i32,
) -> (f32, i32, i32, i32, i32) {
    // 计算各项指标
    let average_sg = if total_count > 0 {
        sum_sg / total_count as f32
    } else {
        0.0
    };

    let below_hypo_limit = if total_count > 0 {
        ((hypo_count as f32 / total_count as f32) * 100.0) as i32
    } else {
        0
    };

    let above_hyper_limit = if total_count > 0 {
        ((hyper_count as f32 / total_count as f32) * 100.0) as i32
    } else {
        0
    };

    let time_in_range = 100 - below_hypo_limit - above_hyper_limit;
    (
        average_sg,
        average_sg as i32,
        below_hypo_limit,
        above_hyper_limit,
        time_in_range,
    )
}
pub async fn ns_refresh_treatments_data(
    state: &AppState,
    treatments: &mut Vec<Value>,
    user_key: &str,
) {
    ns_receive_data(state, "treatments", treatments, &user_key).await;
}

pub async fn ns_refresh_devicestatus_data(
    state: &AppState,
    devicestatus: &mut Vec<Value>,
    user_key: &str,
) {
    ns_receive_data(state, "devicestatus", devicestatus, &user_key).await;
}
