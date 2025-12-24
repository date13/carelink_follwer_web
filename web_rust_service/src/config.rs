use clap::Parser;
use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::env;

#[derive(Parser)]
pub struct Cli {
    #[clap(long, default_value = "development")]
    run_mode: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub default: DefaultConfig,
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub log: LogConfig,
    pub scheduler: SchedulerConfig,
    pub mail: MailConfig,
}
#[derive(Debug, Deserialize, Clone)]
pub struct DefaultConfig {
    pub env: String,
    pub jwt_secret: String,
    pub jwt_exp_hours: i64,
    pub cors: CorsConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[warn(unused_variables)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    // pub allow_credentials: bool,
    pub max_age_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
    pub max_lifetime: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SchedulerConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MailConfig {
    pub enabled: bool,
    pub user: String,
    pub password: String,
    pub to: String,
    pub smtp_host: String,
    pub smtp_port: u16,
}
impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let args = Cli::parse();

        // 手动设置环境变量
        unsafe {
            env::set_var("RUN_MODE", &args.run_mode);
        }
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        println!("RUN_MODE: {}", run_mode);
        let config = Config::builder()
            .add_source(File::new("config/development", FileFormat::Toml))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    pub fn is_production(&self) -> bool {
        self.default.env == "production"
    }
}
