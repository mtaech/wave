#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use command::writeCmds;
use once_cell::sync::OnceCell;
use sqlx::{Pool, Sqlite};
use tauri::async_runtime::block_on;

mod command;
mod entity;
mod init;

static DB_POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

pub struct GlobalEnv {}

impl GlobalEnv {
    pub fn global_db_pool() -> &'static Pool<Sqlite> {
        DB_POOL.get().expect("get sqlite connect instance error")
    }
}

fn main() {
    init::setup_logger().expect("set logger error");
    let pool: Pool<Sqlite> = block_on(init::set_db_conn());
    DB_POOL.set(pool);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![writeCmds::save_chapter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
