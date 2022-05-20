use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::FromRow,
};
#[derive(FromRow)]
pub struct AlbumArtist {
    id: i64,
    artist_id: i64,
    album_id: i64,
    artisttype_id: i64,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}

