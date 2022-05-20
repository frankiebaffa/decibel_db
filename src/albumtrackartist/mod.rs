use {
    chrono::{DateTime, Utc},
    sqlx::FromRow,
};
#[derive(FromRow)]
pub struct AlbumTrackArtist {
    id: i64,
    artist_id: i64,
    albumtrack_id: i64,
    artisttype_id: i64,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
