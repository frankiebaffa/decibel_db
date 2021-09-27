use chrono::{DateTime, Local};
use crate::sql_utils::value;
use rusqlite::Connection;
pub struct AlbumArtist {
    id: i64,
    artist_id: i64,
    album_id: i64,
    artisttype_id: i64,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl AlbumArtist {

}
