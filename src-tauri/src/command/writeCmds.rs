use crate::entity::chapter;
use crate::entity::chapter::ActiveModel as ChapterModel;
use crate::DB_CONN;

#[tauri::command]
pub async fn save_chapter(model: ChapterModel) {
    let conn = DB_CONN.get().unwrap();
    let data = chapter::ActiveModel { id: "1", ..model };
    data.insert(conn).await;
}
