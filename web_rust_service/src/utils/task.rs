use crate::models::{AppState, CgmType, UserSetting};
use crate::services::sugar_service::{
    carelink_refresh_data, carelink_refresh_history, carelink_refresh_token,
};
use crate::utils::DateUtils;
use anyhow::Error;
use chrono::{DateTime, Local};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::log::{debug, info};
use uuid::Uuid;

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

// 任务管理器
pub struct TaskManager {
    scheduler: Arc<Mutex<JobScheduler>>,
    tasks: Arc<DashMap<String, TaskInfo>>,
    job_store: Arc<DashMap<String, Job>>,
}

impl TaskManager {
    pub async fn new() -> Self {
        let scheduler = JobScheduler::new()
            .await
            .expect("Failed to create scheduler");
        Self {
            scheduler: Arc::new(Mutex::new(scheduler)),
            tasks: Arc::new(DashMap::new()),
            job_store: Arc::new(DashMap::new()),
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
        Fut: Future<Output=()> + Send + 'static,
    {
        let task = Arc::new(task);
        let id_arc = Arc::new(id.to_string());
        let tasks = Arc::clone(&self.tasks);
        let closure_fn = move |_uuid, mut _l: JobScheduler| {
            let task_clone = Arc::clone(&task);
            let tasks_clone = Arc::clone(&tasks);
            let id = id_arc.clone();
            Box::pin(async move {
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
            ScheduleType::Cron(schedule) => {
                Job::new_async(&schedule, move |_uuid, mut _l: JobScheduler| {
                    closure_fn(_uuid, _l)
                })
                    .expect("Failed to create Cron job")
            }
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
        self.job_store.insert(id.to_string(), job);

        info!("任务已添加: {}", id);
    }

    // 启动所有任务
    pub async fn start(&self) {
        let scheduler = self.scheduler.lock().await;
        scheduler.start().await.expect("Failed to start scheduler");
        info!("任务调度器已启动");
    }

    // 停止特定任务
    pub async fn stop_task(&self, job_id: &str) -> bool {
        if let Some(mut task) = self.tasks.get_mut(job_id) {
            let scheduler = self.scheduler.lock().await;
            if scheduler
                .remove(
                    Uuid::try_parse(&task.job_id)
                        .as_ref()
                        .expect("parse uuid error"),
                )
                .await
                .is_ok()
            {
                task.status = TaskStatus::Stopped;
                task.next_run_time = "".to_string();
                info!("任务已停止: {}", job_id);
                return true;
            }
        }
        false
    }

    /// 恢复任务
    pub async fn resume_task(&self, job_id: &str) -> bool {
        if let Some(task) = self.tasks.get_mut(job_id) {
            if let Some(job) = self.job_store.get_mut(job_id) {
                let scheduler = self.scheduler.lock().await;
                scheduler.add(job.clone()).await.expect("Failed to add job");
            }
        }
        false
    }

    // 暂停所有任务
    // pub async fn pause_all(&self) {
    //     let mut scheduler = self.scheduler.lock().await;
    //     scheduler
    //         .shutdown()
    //         .await
    //         .expect("Failed to pause scheduler");
    //
    //     for mut task in self.tasks.iter_mut() {
    //         if task.status == TaskStatus::Running {
    //             task.status = TaskStatus::Paused;
    //         }
    //     }
    //     info!("所有任务已暂停");
    // }

    // 恢复所有任务
    // pub async fn resume_all(&self) {
    //     let scheduler = self.scheduler.lock().await;
    //     scheduler.start().await.expect("Failed to resume scheduler");
    //
    //     for mut task in self.tasks.iter_mut() {
    //         if task.status == TaskStatus::Paused {
    //             task.status = TaskStatus::Running;
    //         }
    //     }
    //     info!("所有任务已恢复");
    // }

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
        Fut: Future<Output=()> + Send + 'static,
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
            carelink_refresh_token(&app_state, &setting).await;
            carelink_refresh_data(&app_state, &setting).await;

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
                        carelink_refresh_data(&state, &user_setting).await;
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
