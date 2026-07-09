use crate::models::{AppState, CgmType, UserSetting};
use crate::services::sugar_service::{
    carelink_refresh_data, carelink_refresh_history, carelink_refresh_token,
};
use crate::utils::DateUtils;
use anyhow::Error;
use chrono::{DateTime, Local};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::log::{debug, info};

// 任务状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Running,
    Stopped,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: String,
    pub name: String,
    pub schedule_type: ScheduleType,
    pub next_run_time: String,
    pub status: TaskStatus,
    pub job_id: String,
}

/// 任务执行中的守护结构：离开作用域（正常结束或被取消/panic 卸载）时自动清除“执行中”标记，
/// 避免某次运行异常导致该任务永远被重入保护卡住。
struct RunningGuard(Arc<AtomicBool>);
impl Drop for RunningGuard {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

// 任务管理器
pub struct TaskManager {
    scheduler: Arc<Mutex<JobScheduler>>,
    tasks: Arc<DashMap<String, TaskInfo>>,
    // 记录每个任务是否正在执行，用于防止重复触发（重入保护）
    running: Arc<DashMap<String, Arc<AtomicBool>>>,
}

#[allow(dead_code)]
impl TaskManager {
    pub async fn new() -> Self {
        let scheduler = JobScheduler::new()
            .await
            .expect("Failed to create scheduler");
        Self {
            scheduler: Arc::new(Mutex::new(scheduler)),
            tasks: Arc::new(DashMap::new()),
            running: Arc::new(DashMap::new()),
            // job_store: Arc::new(DashMap::new()),
        }
    }

    pub async fn add_task<F, Fut>(
        &self,
        id: &str,
        name: &str,
        schedule_type: &ScheduleType,
        task: F,
    ) where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(task);
        let id_arc = Arc::new(id.to_string());
        let tasks = Arc::clone(&self.tasks);
        // 重入保护标记：同一任务上一次未结束时跳过本次触发
        let running = Arc::new(AtomicBool::new(false));
        let running_for_map = Arc::clone(&running);
        let closure_fn = move |_uuid, mut _l: JobScheduler| {
            let task_clone = Arc::clone(&task);
            let tasks_clone = Arc::clone(&tasks);
            let running_flag = Arc::clone(&running);
            let id = id_arc.clone();
            Box::pin(async move {
                // 检查任务是否处于运行状态，非运行状态则跳过
                if let Some(task_info) = tasks_clone.get(&id.to_string()) {
                    if task_info.status != TaskStatus::Running {
                        return;
                    }
                }
                // 重入保护：若上一次仍在执行，则跳过本次触发，避免重叠运行
                if running_flag
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    .is_err()
                {
                    debug!("本次任务:{} 仍在执行中,跳过重叠触发", &id);
                    return;
                }
                // 守护对象：无论任务正常结束还是被取消/panic，都会自动清除“执行中”标记
                let _guard = RunningGuard(Arc::clone(&running_flag));
                debug!("本次任务:{},开始: {}", &id.clone(), DateUtils::datetime());
                task_clone().await;
                if let Ok(Some(next_tick)) = _l.next_tick_for_job(_uuid).await {
                    let local_time: DateTime<Local> = DateTime::from(next_tick);
                    if let Some(mut task) = tasks_clone.get_mut(&id.to_string().clone()) {
                        task.next_run_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
                    }
                    debug!(
                        "本次任务:{},结束:{},下次执行时间: {}",
                        &id.clone(),
                        DateUtils::datetime(),
                        local_time.format("%Y-%m-%d %H:%M:%S"),
                    );
                }
            })
        };

        let job = match schedule_type {
            ScheduleType::Cron(schedule) => Job::new_cron_job_async_tz(
                &schedule,
                chrono_tz::Asia::Shanghai,
                move |_uuid, mut _l: JobScheduler| closure_fn(_uuid, _l),
            )
            .expect("Failed to create Cron job"),
            ScheduleType::Repeated(interval) => {
                Job::new_repeated_async(*interval, move |_uuid, mut _l: JobScheduler| {
                    closure_fn(_uuid, _l)
                })
                .expect("Failed to create Repeated job")
            }
        };

        let scheduler = self.scheduler.lock().await;
        let job_id = scheduler.add(job.clone()).await.expect("Failed to add job");

        self.tasks.insert(
            id.to_string(),
            TaskInfo {
                id: id.to_string(),
                name: name.to_string(),
                schedule_type: schedule_type.clone(),
                next_run_time: "".to_string(),
                status: TaskStatus::Running,
                job_id: job_id.to_string(),
            },
        );
        // 记录重入保护标记
        self.running.insert(id.to_string(), running_for_map);
        // self.job_store.insert(id.to_string(), job);

        info!("任务已添加: {}", id);
    }

    // 启动所有任务
    pub async fn start(&self) {
        let scheduler = self.scheduler.lock().await;
        scheduler.start().await.expect("Failed to start scheduler");
        info!("任务调度器已启动");
    }

    // 按任务 id 或 job_id 查找真实的 map key（TaskInfo.id）
    fn find_task_key(&self, task_id: &str) -> Option<String> {
        self.tasks
            .iter()
            .find(|t| t.id == task_id || t.job_id == task_id)
            .map(|t| t.id.clone())
    }

    // 停止特定任务（task_id 既可以是任务 id，也可以是调度器返回的 job_id）
    pub async fn stop_task(&self, job_id: &str) -> bool {
        if let Some(key) = self.find_task_key(job_id) {
            if let Some(mut task) = self.tasks.get_mut(&key) {
                task.status = TaskStatus::Stopped;
                task.next_run_time = "".to_string();
                info!("任务已停止: {}", key);
                return true;
            }
        }
        false
    }

    #[allow(unused_variables)]
    /// 恢复任务
    pub async fn resume_task(&self, job_id: &str) -> bool {
        if let Some(key) = self.find_task_key(job_id) {
            if let Some(mut task) = self.tasks.get_mut(&key) {
                if task.status == TaskStatus::Stopped {
                    task.status = TaskStatus::Running;
                    info!("任务已恢复: {}", key);
                    return true;
                }
            }
        }
        false
    }

    // 获取任务列表
    pub async fn list_tasks(&self) -> Vec<TaskInfo> {
        let mut result = vec![];
        for task in self.tasks.iter() {
            result.push(task.clone());
        }
        result
    }

    // 停止调度器
    pub async fn shutdown(&self) {
        let mut scheduler = self.scheduler.lock().await;
        scheduler
            .shutdown()
            .await
            .expect("Failed to shutdown scheduler");
        info!("任务调度器已停止");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleType {
    Cron(String), // Cron 表达式
    Repeated(Duration), // 重复间隔
                  // RepeatedAt(DateTime<Utc>, Duration), // 指定时间开始的重复
                  // OneShotAt(DateTime<Utc>),            // 一次性任务
}

pub struct TaskBuilder {
    state: Arc<AppState>,
    setting: Arc<UserSetting>,
    schedule_type: Arc<ScheduleType>,
}

impl TaskBuilder {
    pub fn new(state: Arc<AppState>, setting: UserSetting, schedule_type: ScheduleType) -> Self {
        Self {
            state,
            setting: Arc::new(setting),
            schedule_type: Arc::new(schedule_type),
        }
    }

    pub async fn build<F, Fut>(&self, id: &str, name: &str, f: F) -> &Self
    where
        F: Fn(Arc<AppState>, Arc<UserSetting>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let id_arc = Arc::new(id.to_string());
        let state_clone = Arc::clone(&self.state);
        let setting_clone = Arc::clone(&self.setting);
        let schedule_type = Arc::clone(&self.schedule_type);
        // 创建任务闭包
        let task_closure = {
            move || {
                let setting = Arc::clone(&setting_clone);
                let state = Arc::clone(&state_clone);
                let f = f.clone();
                Box::pin(async move {
                    f(state, setting).await;
                })
            }
        };
        // 添加到任务管理器
        self.state
            .task_manager
            .add_task(id_arc.as_str(), name, &schedule_type, task_closure)
            .await;
        &self
    }
}

// 使用示例
pub async fn add_scheduler_job(state: AppState, setting: UserSetting) -> Result<(), Error> {
    let user_key = setting.user_key.as_str();

    match setting.cgm {
        CgmType::Carelink => {
            let token_key = format!("CarelinkTokenTask:{}", user_key);
            let data_key = format!("CarelinkDataTask:{}", user_key);
            let history_key = format!("CarelinkHistoryTask:{}", user_key);
            let app_state = Arc::new(state);
            debug!("{:?}", setting);
            // 首次刷新放到后台执行，避免阻塞启动；失败不影响调度器注册
            let boot_state = Arc::clone(&app_state);
            let boot_setting = setting.clone();
            tokio::spawn(async move {
                carelink_refresh_token(&boot_state, &boot_setting).await;
                carelink_refresh_data(&boot_state, &boot_setting.user_key).await;
            });
            // TaskBuilder::new(
            //     Arc::clone(&app_state),
            //     setting.clone(),
            //     ScheduleType::Repeated(Duration::from_secs(10 as u64)),
            // )
            // .build(
            //     "test task",
            //     format!("test schedule:{}", user_key).as_ref(),
            //     |state, user_setting| async move {
            //         println!("task run:{}", DateUtils::datetime());
            //     },
            // )
            // .await;
            TaskBuilder::new(
                Arc::clone(&app_state),
                setting.clone(),
                ScheduleType::Repeated(Duration::from_secs(
                    setting.carelink_token_refresh_interval as u64,
                )),
            )
            .build(
                &token_key,
                format!("刷新carelinkToken:{}", user_key).as_ref(),
                |state, user_setting| async move {
                    carelink_refresh_token(&state, &user_setting).await;
                },
            )
            .await;
            TaskBuilder::new(
                Arc::clone(&app_state),
                setting.clone(),
                ScheduleType::Repeated(Duration::from_secs(
                    // 3,
                    setting.carelink_data_refresh_interval as u64,
                )),
            )
            .build(
                &data_key,
                format!("刷新carelinkData:{}", user_key).as_ref(),
                |state, user_setting| async move {
                    carelink_refresh_data(&state, &user_setting.user_key).await;
                },
            )
            .await;

            TaskBuilder::new(
                Arc::clone(&app_state),
                setting.clone(),
                ScheduleType::Cron("0 0 0 * * *".to_string()),
            )
            .build(
                &history_key,
                format!("刷新carelinkHistory:{}", user_key).as_ref(),
                |state, user_setting| async move {
                    carelink_refresh_history(&state, &user_setting).await;
                },
            )
            .await;
        }
        CgmType::Dexcom => {
            // Dexcom 相关任务
        }
    }

    Ok(())
}
