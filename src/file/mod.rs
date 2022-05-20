use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::FromRow,
};
#[derive(FromRow)]
pub struct File {
    id: i64,
    file_blob: Vec<u8>,
    mime_type: String,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
