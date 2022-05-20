use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::FromRow,
};
#[derive(FromRow)]
pub struct Song {
    id: i64,
    name: String,
    blurb: String,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
