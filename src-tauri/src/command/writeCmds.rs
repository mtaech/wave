use crate::entity::Chapter;
use crate::{writeCmds, DB_POOL};
use log::error;

#[tauri::command]
pub async fn save_chapter(chapter: Chapter) {
    let pool = DB_POOL.get().unwrap();
    log::info!("chapter info is {:?}", chapter);

}
