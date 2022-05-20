use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::FromRow
};
#[derive(FromRow)]
pub struct Album {
    id: i64,
    albumtype_id: i64,
    cover_id: i64,
    name: String,
    blurb: Option<String>,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
