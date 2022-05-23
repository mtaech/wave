use chrono::{DateTime, Local};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub id: Option<String>,
    pub name: Option<String>,
    pub content: Option<String>,
    pub revision: Option<String>,
    pub create_time: Option<DateTime<Local>>,
}

impl Chapter {
    pub fn new() -> Chapter {
        Chapter {
            id: Some(nanoid!(10)),
            name: None,
            content: None,
            revision: None,
            create_time: None,
        }
    }
}
