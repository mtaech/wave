#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate rbatis;
use command::writeCmds;
use log::error;
use once_cell::sync::OnceCell;
use rbatis::rbatis::Rbatis;
use std::env;
use tauri::async_runtime::block_on;

mod command;
mod entity;
mod init;

static DB_POOL: OnceCell<Rbatis> = OnceCell::new();

fn main() {
    init::setup_logger();
    block_on(init_db());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![writeCmds::save_chapter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn init_db() {
    let db_url = init::set_db_conn();
    let rb = Rbatis::new();
    match rb.link(&db_url).await {
        Ok(_) => {
            DB_POOL.set(rb);
        }
        Err(error) => {
            error!("db connect error reason:{:?}", error)
        }
    }
}
