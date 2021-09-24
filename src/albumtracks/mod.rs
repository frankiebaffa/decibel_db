use chrono::{DateTime, Local};
use crate::{db::DbConnection, sql_utils::value};
struct AlbumTracks {
    id: i64,
    album_id: i64,
    song_id: i64,
    file_id: i64,
    tracknumber: i8,
    version: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl AlbumTracks {

}
