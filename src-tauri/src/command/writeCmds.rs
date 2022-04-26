use crate::entity::Chapter;
use crate::{writeCmds, DB_POOL};

#[tauri::command]
pub async fn save_chapter(chapter: Chapter) {
    let pool = DB_POOL.get().unwrap();
    log::info!("chapter info is {:?}", chapter);
    sqlx::query(" insert into chapter (id, name, content, revision, create_time) values (?,?,?,?,?,datetime())")
        .bind(&chapter.id)
        .bind(&chapter.name)
        .bind(&chapter.content)
        .execute(pool)
        .await
        .unwrap();
}
