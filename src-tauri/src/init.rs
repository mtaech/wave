use std::ops::Add;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs};

use log::{error, LevelFilter};
use sqlx::sqlite::{SqliteConnection, SqlitePoolOptions};
use sqlx::{prelude, Error, Pool, Sqlite};

pub fn setup_logger() -> Result<(), fern::InitError> {
    let log_path = get_log_path();
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("sqlx", LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}

pub async fn set_db_conn() -> Pool<Sqlite> {
    let home_path = get_current_dir_path();
    let db_dir = home_path.join("db");
    if !db_dir.exists() {
        match fs::create_dir_all(&db_dir) {
            Ok(_) => {}
            Err(error) => {
                log::error!("create db dir error reason:{:?}", error);
            }
        }
    }
    let db_path = db_dir
        .join("wave.db")
        .into_os_string()
        .into_string()
        .unwrap();
    let db_url = String::from("sqlite:")
        .add(db_path.as_str())
        .add("?mode=rwc"); //auto create db file if not exist
    log::warn!("db url path is {}", db_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await;
    match pool {
        Ok(pool) => pool,
        Err(error) => {
            log::error!("connect to db failed reason :{:?}", error);
            panic!()
        }
    }
}

fn get_home_path() -> PathBuf {
    let home = env!("HOME");
    Path::new(home).join(".wave")
}

fn get_current_dir_path() -> PathBuf {
    env::current_dir().unwrap()
}

fn get_log_path() -> String {
    let wave_path = get_home_path();
    let path = wave_path
        .join("wave.log")
        .into_os_string()
        .into_string()
        .unwrap();
    if !wave_path.exists() {
        fs::create_dir_all(wave_path).unwrap();
    }
    path
}
