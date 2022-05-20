use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::FromRow,
};
#[derive(FromRow)]
pub struct Artist {
    id: i64,
    name: String,
    bio: Option<String>,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
