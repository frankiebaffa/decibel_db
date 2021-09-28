use chrono::{DateTime, Local};
struct AlbumTrackArtist {
    id: i64,
    artist_id: i64,
    albumtrack_id: i64,
    artisttype_id: i64,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl AlbumTrackArtist {

}
