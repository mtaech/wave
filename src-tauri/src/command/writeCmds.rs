use crate::entity::Chapter;
use crate::{writeCmds, DB_POOL};
use log::error;
use rbatis::crud::CRUD;

#[tauri::command]
pub async fn save_chapter(chapter: Chapter) {
    let pool = DB_POOL.get().unwrap();
    log::info!("chapter info is {:?}", chapter);
    match pool.save::<Chapter>(&chapter, &[]).await {
        Ok(_) => {}
        Err(error) => {
            error!("save chapter error reason:{:?}", error)
        }
    };
}
