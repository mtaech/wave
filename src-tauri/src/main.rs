#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use command::writeCmds;
use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;
use std::env;
use tauri::async_runtime::block_on;

mod command;
mod entity;
mod init;

static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::new();

pub struct GlobalEnv {}

impl GlobalEnv {
    pub fn global_db_pool() -> &'static DatabaseConnection {
        DB_CONN.get().expect("get sqlite connect instance error")
    }
}

fn main() {
    init::setup_logger();
    let connection: DatabaseConnection = block_on(init::set_db_pool());
    DB_CONN.set(connection);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![writeCmds::save_chapter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
