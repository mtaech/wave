use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub content: String,
    pub revision: String,
    pub create_time: DateTime<Local>,
}

impl Chapter {
    pub fn new() -> Chapter {
        Chapter {
            id: "".to_string(),
            name: "".to_string(),
            content: "".to_string(),
            revision: "".to_string(),
            create_time: Local::now(),
        }
    }
}
