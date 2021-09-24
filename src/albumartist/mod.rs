use chrono::{DateTime, Local};
use crate::{
    db::DbConnection,
    sql_utils::value,
};
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
