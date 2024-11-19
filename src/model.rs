use chrono::{DateTime, Utc};

pub struct Todo {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub done: i64,
    pub title: String,
    pub start_date: Option<DateTime<Utc>>,
    pub start_time: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub url: Option<String>,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, created_at: {}, updated_at: {}, title: {}, done: {}, description: {:?}, url: {:?}, due_date: {:?}",
            self.id, self.created_at.to_rfc3339(), self.updated_at.to_rfc3339(), self.title, self.done, self.description, self.url, self.start_date
        )
    }
}
