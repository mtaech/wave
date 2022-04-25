#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;
use std::env;
use tauri::async_runtime::block_on;

mod init;

static DB_POOL: OnceCell<DatabaseConnection> = OnceCell::new();

pub struct GlobalEnv {}

impl GlobalEnv {
    pub fn global_db_pool() -> &'static DatabaseConnection {
        DB_POOL.get().expect("get sqlite connect instance error")
    }
}

fn main() {
    init::setup_logger();
    let connection: DatabaseConnection = block_on(init::set_db_pool());
    DB_POOL.set(connection);
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
