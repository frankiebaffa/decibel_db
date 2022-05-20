use {
    chrono::{DateTime, Utc},
    sqlx::FromRow,
};
#[derive(FromRow)]
pub struct AlbumTrack {
    id: i64,
    album_id: i64,
    song_id: i64,
    file_id: i64,
    track_number: i8,
    version: String,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
